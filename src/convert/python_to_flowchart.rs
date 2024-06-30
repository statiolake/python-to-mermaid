use crate::flowchart::{self, Flowchart, FlowchartItem};
use anyhow::Result;
use ast::{StmtFunctionDef, Visitor as _};
use itertools::Itertools as _;
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
    let mut fc = Flowchart::new_root(fn_def.name);
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
        let label = format!(
            "return: {}",
            node.value.map_or("".to_string(), |v| v.to_string())
        );

        self.fc
            .items
            .push(FlowchartItem::Terminal(flowchart::Terminal::new(label)));
    }

    fn visit_stmt_delete(&mut self, node: ast::StmtDelete) {
        let label = format!("delete: {}", node.targets.iter().format(", "));

        self.fc
            .items
            .push(FlowchartItem::Step(flowchart::Step::new(label)));
    }

    fn visit_stmt_assign(&mut self, node: ast::StmtAssign) {
        let label = format!("{} = {}", node.targets.iter().format(", "), node.value);

        self.fc
            .items
            .push(FlowchartItem::Step(flowchart::Step::new(label)));
    }

    fn visit_stmt_aug_assign(&mut self, node: ast::StmtAugAssign) {
        let op = op_to_string(&node.op);
        let label = format!("{} {}= {}", node.target, op, node.value);

        self.fc
            .items
            .push(FlowchartItem::Step(flowchart::Step::new(label)));
    }

    fn visit_stmt_ann_assign(&mut self, node: ast::StmtAnnAssign) {
        let label = format!(
            "{}: {}{}",
            node.target,
            node.annotation,
            node.value.map_or_else(String::new, |v| format!(" = {}", v))
        );

        self.fc
            .items
            .push(FlowchartItem::Step(flowchart::Step::new(label)));
    }

    fn visit_stmt_for(&mut self, node: ast::StmtFor) {
        let label = format!("for {} in {}", node.target, node.iter);
        let mut fc = Flowchart::new_sub(label);
        let mut gen = FlowchartGenerator::new(&mut fc);
        gen.visit_stmts(node.body);
        self.fc.items.push(FlowchartItem::SubFlowchart(fc));
    }

    fn visit_stmt_while(&mut self, node: ast::StmtWhile) {
        let label = format!("while {}", node.test);
        let mut fc = Flowchart::new_sub(label);
        let mut gen = FlowchartGenerator::new(&mut fc);
        gen.visit_stmts(node.body);
        self.fc.items.push(FlowchartItem::SubFlowchart(fc));
    }

    fn visit_stmt_if(&mut self, node: ast::StmtIf) {
        let then_items = {
            let mut fc = Flowchart::new_unlabeled_sub();
            let mut gen = FlowchartGenerator::new(&mut fc);
            gen.visit_stmts(node.body);
            fc.items
        };

        let else_items = {
            let mut fc = Flowchart::new_unlabeled_sub();
            let mut gen = FlowchartGenerator::new(&mut fc);
            gen.visit_stmts(node.orelse);
            fc.items
        };

        self.fc
            .items
            .push(FlowchartItem::Condition(flowchart::Condition {
                condition: format!("if: {} ?", node.test),
                then_items,
                else_items,
            }));
    }

    fn visit_stmt_with(&mut self, node: ast::StmtWith) {
        let label = format!(
            "with {}",
            node.items
                .iter()
                .map(|item| &item.context_expr)
                .format(", ")
        );
        let mut fc = Flowchart::new_sub(label);
        let mut gen = FlowchartGenerator::new(&mut fc);
        gen.visit_stmts(node.body);
        self.fc.items.push(FlowchartItem::SubFlowchart(fc));
    }

    fn visit_stmt_match(&mut self, node: ast::StmtMatch) {
        let label = format!("match {}", node.subject);
        let mut fc = Flowchart::new_sub(label);

        let mut curr = &mut fc.items;
        for case in node.cases {
            let items = {
                let mut fc = Flowchart::new_unlabeled_sub();
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
                let label = format!("case: {} ?", pattern_to_string(&case.pattern));
                curr.push(FlowchartItem::Condition(flowchart::Condition::new(label)));
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
        let label = format!(
            "raise {}",
            node.exc.map_or_else(String::new, |e| e.to_string())
        );

        self.fc
            .items
            .push(FlowchartItem::Terminal(flowchart::Terminal::new(label)));
    }

    fn visit_stmt_try(&mut self, node: ast::StmtTry) {
        let mut fc = Flowchart::new_sub("try");
        let mut gen = FlowchartGenerator::new(&mut fc);
        gen.visit_stmts(node.body);
        self.fc.items.push(FlowchartItem::SubFlowchart(fc));

        let mut curr = &mut self.fc.items;
        for handler in node.handlers {
            let ast::ExceptHandler::ExceptHandler(handler) = handler;

            let label = match (&handler.type_, &handler.name) {
                (None, None) => "except ?".to_string(),
                (Some(t), None) => format!("except {} ?", t),
                (None, Some(n)) => format!("except as {} ?", n),
                (Some(t), Some(n)) => format!("except {} as {} ?", t, n),
            };

            let items = {
                let mut fc = Flowchart::new_unlabeled_sub();
                let mut gen = FlowchartGenerator::new(&mut fc);
                gen.visit_stmts(handler.body);
                fc.items
            };

            curr.push(FlowchartItem::Condition(flowchart::Condition::new(label)));
            let FlowchartItem::Condition(cond) = curr.last_mut().unwrap() else {
                unreachable!("bug: last item is not a condition");
            };

            cond.then_items = items;
            curr = &mut cond.else_items;
        }
    }

    fn visit_stmt_expr(&mut self, node: ast::StmtExpr) {
        let label = node.value.to_string().replace('"', "");

        self.fc
            .items
            .push(FlowchartItem::Step(flowchart::Step::new(label)));
    }

    fn visit_stmt_break(&mut self, _node: ast::StmtBreak) {
        let label = "break".to_string();
        self.fc
            .items
            .push(FlowchartItem::Break(flowchart::Break::new(label)));
    }

    fn visit_stmt_continue(&mut self, _node: ast::StmtContinue) {
        let label = "continue".to_string();
        self.fc
            .items
            .push(FlowchartItem::Continue(flowchart::Continue::new(label)));
    }
}

fn op_to_string(op: &ast::Operator) -> &str {
    match op {
        ast::Operator::Add => "+",
        ast::Operator::Sub => "-",
        ast::Operator::Mult => "*",
        ast::Operator::MatMult => "@",
        ast::Operator::Div => "/",
        ast::Operator::Mod => "%",
        ast::Operator::Pow => "**",
        ast::Operator::LShift => "<<",
        ast::Operator::RShift => ">>",
        ast::Operator::BitOr => "|",
        ast::Operator::BitXor => "^",
        ast::Operator::BitAnd => "&",
        ast::Operator::FloorDiv => "//",
    }
}

fn pattern_to_string(pattern: &ast::Pattern) -> String {
    match pattern {
        ast::Pattern::MatchSingleton(ast::PatternMatchSingleton { value, .. }) => value.to_string(),
        ast::Pattern::MatchValue(ast::PatternMatchValue { value, .. }) => value.to_string(),
        ast::Pattern::MatchAs(ast::PatternMatchAs { pattern, name, .. }) => {
            let pattern = pattern
                .as_ref()
                .map_or_else(|| "_".to_string(), |p| pattern_to_string(p));
            if let Some(name) = name {
                format!("{} as {}", pattern, name)
            } else {
                pattern
            }
        }
        ast::Pattern::MatchSequence(ast::PatternMatchSequence { .. }) => "[...]".to_string(),
        ast::Pattern::MatchMapping(ast::PatternMatchMapping { .. }) => "{...}".to_string(),
        ast::Pattern::MatchClass(ast::PatternMatchClass { cls, .. }) => {
            format!("{}()", cls)
        }
        ast::Pattern::MatchStar(ast::PatternMatchStar { name, .. }) => {
            if let Some(name) = name {
                format!("*{}", name)
            } else {
                "*".to_string()
            }
        }
        ast::Pattern::MatchOr(ast::PatternMatchOr { patterns, .. }) => {
            format!("({})", patterns.iter().map(pattern_to_string).format(" | "))
        }
    }
}
