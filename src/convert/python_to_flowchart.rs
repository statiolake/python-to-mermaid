use crate::flowchart::{self, Flowchart, FlowchartItem};
use anyhow::Result;
use ast::{StmtFunctionDef, Visitor as _};
use rustpython_parser::{ast, Parse};

pub fn enumerate_fn_defs(source: &str) -> Result<Vec<FnDef>> {
    let ast = ast::ModModule::parse(source, ".")?;
    let mut collector = FnDefCollector::new();
    for stmt in ast.body {
        collector.visit_stmt(stmt);
    }

    Ok(collector
        .fn_defs
        .into_iter()
        .map(|(name, fn_def)| FnDef { name, fn_def })
        .collect())
}

pub fn convert(fn_def: FnDef) -> Result<Flowchart> {
    let mut fc = Flowchart::new(fn_def.name);
    let mut gen = FlowchartGenerator::new(&mut fc);
    gen.visit_stmts(fn_def.fn_def.body);
    Ok(fc)
}

#[derive(Debug, Clone)]
pub struct FnDef {
    pub name: String,
    pub fn_def: ast::StmtFunctionDef,
}

trait VisitorExt: ast::Visitor {
    fn visit_stmts<I: IntoIterator<Item = ast::Stmt>>(&mut self, stmts: I) {
        for stmt in stmts {
            self.visit_stmt(stmt);
        }
    }
}

impl<V: ast::Visitor> VisitorExt for V {}

struct FnDefCollector {
    path: Vec<String>,
    fn_defs: Vec<(String, ast::StmtFunctionDef)>,
}

impl FnDefCollector {
    pub fn new() -> Self {
        Self {
            path: Vec::new(),
            fn_defs: Vec::new(),
        }
    }
}

impl ast::Visitor for FnDefCollector {
    fn visit_stmt_class_def(&mut self, node: ast::StmtClassDef) {
        self.path.push(node.name.to_string());
        self.generic_visit_stmt_class_def(node);
        self.path.pop();
    }

    fn visit_stmt_function_def(&mut self, node: StmtFunctionDef) {
        self.path.push(node.name.to_string());
        self.fn_defs.push((self.path.join("."), node.clone()));
        self.generic_visit_stmt_function_def(node);
        self.path.pop();
    }
}

struct FlowchartGenerator<'f> {
    fc: &'f mut Flowchart,
}

impl<'f> FlowchartGenerator<'f> {
    pub fn new(fc: &'f mut Flowchart) -> Self {
        Self { fc }
    }
}

impl<'f> ast::Visitor for FlowchartGenerator<'f> {
    fn visit_stmt_return(&mut self, node: ast::StmtReturn) {
        self.fc
            .items
            .push(FlowchartItem::Terminal(flowchart::Terminal::new("Return")));
    }

    fn visit_stmt_delete(&mut self, node: ast::StmtDelete) {
        self.fc
            .items
            .push(FlowchartItem::Step(flowchart::Step::new("Delete")));
    }

    fn visit_stmt_assign(&mut self, node: ast::StmtAssign) {
        self.fc
            .items
            .push(FlowchartItem::Step(flowchart::Step::new("Assign")));
    }

    fn visit_stmt_aug_assign(&mut self, node: ast::StmtAugAssign) {
        self.fc
            .items
            .push(FlowchartItem::Step(flowchart::Step::new("AugAssign")));
    }

    fn visit_stmt_ann_assign(&mut self, node: ast::StmtAnnAssign) {
        self.fc
            .items
            .push(FlowchartItem::Step(flowchart::Step::new("AnnAssign")));
    }

    fn visit_stmt_for(&mut self, node: ast::StmtFor) {
        let mut fc = Flowchart::new("For");
        let mut gen = FlowchartGenerator::new(&mut fc);
        gen.visit_stmts(node.body);
        self.fc.items.push(FlowchartItem::SubFlowchart(fc));
    }

    fn visit_stmt_while(&mut self, node: ast::StmtWhile) {
        let mut fc = Flowchart::new("While");
        let mut gen = FlowchartGenerator::new(&mut fc);
        gen.visit_stmts(node.body);
        self.fc.items.push(FlowchartItem::SubFlowchart(fc));
    }

    fn visit_stmt_if(&mut self, node: ast::StmtIf) {
        let then_items = {
            let mut fc = Flowchart::new("If");
            let mut gen = FlowchartGenerator::new(&mut fc);
            gen.visit_stmts(node.body);
            fc.items
        };

        let else_items = {
            let mut fc = Flowchart::new("If");
            let mut gen = FlowchartGenerator::new(&mut fc);
            gen.visit_stmts(node.orelse);
            fc.items
        };

        self.fc
            .items
            .push(FlowchartItem::Condition(flowchart::Condition {
                condition: "If".to_string(),
                then_items,
                else_items,
            }));
    }

    fn visit_stmt_with(&mut self, node: ast::StmtWith) {
        let mut fc = Flowchart::new("With");
        let mut gen = FlowchartGenerator::new(&mut fc);
        gen.visit_stmts(node.body);
        self.fc.items.push(FlowchartItem::SubFlowchart(fc));
    }

    fn visit_stmt_match(&mut self, node: ast::StmtMatch) {
        let mut fc = Flowchart::new("Match");

        let mut curr = &mut fc.items;
        for case in node.cases {
            let items = {
                let mut fc = Flowchart::new("");
                let mut gen = FlowchartGenerator::new(&mut fc);
                gen.visit_stmts(case.body);
                fc.items
            };

            if matches!(
                case.pattern,
                ast::Pattern::MatchAs(ast::PatternMatchAs {
                    pattern: None,
                    name: None,
                    ..
                })
            ) {
                // Wildcard. Add items into current case.
                curr.extend(items);
            } else {
                // Not wildcard.
                curr.push(FlowchartItem::Condition(flowchart::Condition::new(
                    "Case ?",
                )));
                let FlowchartItem::Condition(cond) = curr.last_mut().unwrap() else {
                    unreachable!("bug: last item is not a condition");
                };
                cond.then_items = items;
                curr = &mut cond.else_items;
            }
        }

        self.fc.items.push(FlowchartItem::SubFlowchart(fc));
    }

    fn visit_stmt_raise(&mut self, node: ast::StmtRaise) {
        self.fc
            .items
            .push(FlowchartItem::Terminal(flowchart::Terminal::new("Raise")));
    }

    fn visit_stmt_try(&mut self, node: ast::StmtTry) {
        let mut fc = Flowchart::new("Try");
        let mut gen = FlowchartGenerator::new(&mut fc);
        gen.visit_stmts(node.body);
        self.fc.items.push(FlowchartItem::SubFlowchart(fc));
    }

    fn visit_stmt_expr(&mut self, node: ast::StmtExpr) {
        self.fc.items.push(FlowchartItem::Step(flowchart::Step::new(
            node.value.to_string().replace('"', ""),
        )));
    }

    fn visit_stmt_break(&mut self, node: ast::StmtBreak) {
        self.fc
            .items
            .push(FlowchartItem::Break(flowchart::Break::new("Break")));
    }

    fn visit_stmt_continue(&mut self, node: ast::StmtContinue) {
        self.fc
            .items
            .push(FlowchartItem::Continue(flowchart::Continue::new(
                "Continue",
            )));
    }
}
