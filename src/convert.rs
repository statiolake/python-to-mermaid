use itertools::{chain, Itertools as _};

use crate::{
    flowchart::{Flowchart, FlowchartItem},
    mermaid::{MermaidFlowchart, MermaidGraph, NodeId, NodeShape},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PreviousNode {
    pub id: NodeId,
    pub label: Option<String>,
}

impl PreviousNode {
    pub fn new(id: NodeId, label: Option<String>) -> Self {
        Self { id, label }
    }
}

pub fn convert(fc: &Flowchart) -> MermaidFlowchart {
    let mut mfc = MermaidFlowchart::new();
    let mg = mfc.graph_mut();
    add_flowchart_to(mg, vec![], fc);
    mfc
}

pub fn add_flowchart_to(
    mg: &mut MermaidGraph,
    prev_nodes: Vec<PreviousNode>,
    fc: &Flowchart,
) -> Vec<PreviousNode> {
    let (begin_shape, end_shape) = if fc.is_root {
        (NodeShape::Rounded, NodeShape::Rounded)
    } else {
        (NodeShape::Trapezoid, NodeShape::InvertedTrapezoid)
    };

    let nid_begin = mg.add_node(&fc.begin, begin_shape);
    add_edges_to(mg, &prev_nodes, &nid_begin);

    let mut prev_nodes = vec![PreviousNode::new(nid_begin, None)];
    for item in &fc.items {
        prev_nodes = add_item_to(mg, prev_nodes, item);
    }

    let nid_end = mg.add_node(&fc.end, end_shape);
    add_edges_to(mg, &prev_nodes, &nid_end);

    vec![PreviousNode::new(nid_end, None)]
}

pub fn add_item_to(
    mg: &mut MermaidGraph,
    prev_nodes: Vec<PreviousNode>,
    item: &FlowchartItem,
) -> Vec<PreviousNode> {
    match item {
        FlowchartItem::Step(step) => {
            let nid = mg.add_node(&step.label, NodeShape::Rectangle);
            add_edges_to(mg, &prev_nodes, &nid);

            vec![PreviousNode::new(nid, None)]
        }
        FlowchartItem::Condition(cond) => {
            let nid_cond = mg.add_node(&cond.condition, NodeShape::Diamond);
            add_edges_to(mg, &prev_nodes, &nid_cond);

            let mut then_nodes = vec![PreviousNode::new(nid_cond.clone(), Some("T".into()))];
            for then_item in &cond.then_items {
                then_nodes = add_item_to(mg, then_nodes, then_item);
            }

            let mut else_nodes = vec![PreviousNode::new(nid_cond.clone(), Some("F".into()))];
            for else_item in &cond.else_items {
                else_nodes = add_item_to(mg, else_nodes, else_item);
            }

            chain!(then_nodes.clone(), else_nodes.clone()).collect_vec()
        }
        FlowchartItem::SubFlowchart(sub_fc) => {
            let mg = mg.add_subgraph(sub_fc.begin.clone());
            add_flowchart_to(mg, prev_nodes.clone(), sub_fc)
        }
    }
}

pub fn add_edges_to(mg: &mut MermaidGraph, prev_nodes: &[PreviousNode], next_node: &NodeId) {
    for prev_node in prev_nodes {
        mg.add_edge(&prev_node.id, next_node, prev_node.label.as_deref());
    }
}
