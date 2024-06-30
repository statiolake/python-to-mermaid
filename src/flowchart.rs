#[derive(Debug, Clone)]
pub struct Flowchart {
    pub is_root: bool,
    pub begin: String,
    pub end: String,
    pub items: Vec<FlowchartItem>,
}

impl Flowchart {
    pub fn new(label: impl AsRef<str>) -> Self {
        let label = label.as_ref();

        Self {
            is_root: true,
            begin: format!("Begin: {label}"),
            end: format!("End: {label}"),
            items: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum FlowchartItem {
    Step(Step),
    Condition(Condition),
    SubFlowchart(Flowchart),
}

#[derive(Debug, Clone)]
pub struct Step {
    pub label: String,
}

impl Step {
    pub fn new(label: impl AsRef<str>) -> Self {
        Self {
            label: label.as_ref().to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Condition {
    pub condition: String,
    pub then_items: Vec<FlowchartItem>,
    pub else_items: Vec<FlowchartItem>,
}

impl Condition {
    pub fn new(condition: impl AsRef<str>) -> Self {
        Self {
            condition: condition.as_ref().to_string(),
            then_items: Vec::new(),
            else_items: Vec::new(),
        }
    }
}
