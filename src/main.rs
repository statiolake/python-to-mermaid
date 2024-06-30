use anyhow::Result;
use ast::{StmtFunctionDef, Visitor};
use python_to_mermaid::{
    convert::convert,
    flowchart::{self, Flowchart, FlowchartItem},
};
use rustpython_parser::{ast, Parse};
use std::io::{stdin, Read};

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
            .push(FlowchartItem::Step(flowchart::Step::new("Return")));
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
            .push(FlowchartItem::Step(flowchart::Step::new("Raise")));
    }

    fn visit_stmt_try(&mut self, node: ast::StmtTry) {
        let mut fc = Flowchart::new("Try");
        let mut gen = FlowchartGenerator::new(&mut fc);
        gen.visit_stmts(node.body);
        self.fc.items.push(FlowchartItem::SubFlowchart(fc));
    }

    fn visit_stmt_expr(&mut self, node: ast::StmtExpr) {
        self.fc.items.push(FlowchartItem::Step(flowchart::Step::new(
            node.value.to_string(),
        )));
    }
}

fn main() -> Result<()> {
    let source = {
        let mut buf = String::new();
        stdin().read_to_string(&mut buf)?;
        buf
    };

    let ast = ast::ModModule::parse(&source, ".")?;
    let mut collector = FnDefCollector::new();
    for stmt in ast.body {
        collector.visit_stmt(stmt);
    }

    for (name, fn_def) in collector.fn_defs {
        println!("## {name}");
        println!();
        println!("```mermaid");
        let mut fc = Flowchart::new(name);
        let mut gen = FlowchartGenerator::new(&mut fc);
        gen.visit_stmts(fn_def.body);
        let mfc = convert(&fc);
        let mut s = String::new();
        mfc.render(&mut s);
        println!("{}", s);
        println!("```");
        println!();
    }

    Ok(())
}
