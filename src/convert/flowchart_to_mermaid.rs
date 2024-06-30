use itertools::{chain, Itertools as _};

use crate::{
    flowchart::{Flowchart, FlowchartItem},
    mermaid::{MermaidFlowchart, MermaidGraph, NodeId, NodeShape},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PreviousNode {
    pub nid: NodeId,
    pub label: Option<String>,
    pub is_terminal: bool,
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
    let nid_end = mg.add_node(&fc.end, end_shape);

    add_edges_to(mg, &prev_nodes, &nid_begin);

    let mut prev_nodes = vec![PreviousNode {
        nid: nid_begin.clone(),
        label: None,
        is_terminal: false,
    }];
    for item in &fc.items {
        prev_nodes = add_item_to(mg, nid_begin.clone(), nid_end.clone(), prev_nodes, item);
    }

    add_edges_to_end(mg, &prev_nodes, &nid_end);

    vec![PreviousNode {
        nid: nid_end,
        label: None,
        is_terminal: false,
    }]
}

pub fn add_item_to(
    mg: &mut MermaidGraph,
    nid_begin: NodeId,
    nid_end: NodeId,
    prev_nodes: Vec<PreviousNode>,
    item: &FlowchartItem,
) -> Vec<PreviousNode> {
    match item {
        FlowchartItem::Step(step) => {
            let nid = mg.add_node(&step.label, NodeShape::Rectangle);
            add_edges_to(mg, &prev_nodes, &nid);

            vec![PreviousNode {
                nid,
                label: None,
                is_terminal: false,
            }]
        }
        FlowchartItem::Condition(cond) => {
            let nid_cond = mg.add_node(&cond.condition, NodeShape::Diamond);
            add_edges_to(mg, &prev_nodes, &nid_cond);

            let mut then_nodes = vec![PreviousNode {
                nid: nid_cond.clone(),
                label: Some("T".into()),
                is_terminal: false,
            }];
            for then_item in &cond.then_items {
                then_nodes = add_item_to(
                    mg,
                    nid_begin.clone(),
                    nid_end.clone(),
                    then_nodes,
                    then_item,
                );
            }

            let mut else_nodes = vec![PreviousNode {
                nid: nid_cond.clone(),
                label: Some("F".into()),
                is_terminal: false,
            }];
            for else_item in &cond.else_items {
                else_nodes = add_item_to(
                    mg,
                    nid_begin.clone(),
                    nid_end.clone(),
                    else_nodes,
                    else_item,
                );
            }

            chain!(then_nodes.clone(), else_nodes.clone()).collect_vec()
        }
        FlowchartItem::Continue(continue_) => {
            let nid = mg.add_node(&continue_.label, NodeShape::Rectangle);
            add_edges_to(mg, &prev_nodes, &nid);
            add_edges_to(
                mg,
                &[PreviousNode {
                    nid: nid.clone(),
                    label: None,
                    is_terminal: false,
                }],
                &nid_begin,
            );

            vec![PreviousNode {
                nid,
                label: None,
                is_terminal: true,
            }]
        }
        FlowchartItem::Break(break_) => {
            let nid = mg.add_node(&break_.label, NodeShape::Rectangle);
            add_edges_to(mg, &prev_nodes, &nid);
            add_edges_to(
                mg,
                &[PreviousNode {
                    nid,
                    label: None,
                    is_terminal: false,
                }],
                &nid_end,
            );

            vec![]
        }
        FlowchartItem::Terminal(terminal) => {
            let nid = mg.add_node(&terminal.label, NodeShape::Rounded);
            add_edges_to(mg, &prev_nodes, &nid);

            vec![PreviousNode {
                nid,
                label: None,
                is_terminal: true,
            }]
        }
        FlowchartItem::SubFlowchart(sub_fc) => {
            let mg = mg.add_subgraph(sub_fc.begin.clone());
            add_flowchart_to(mg, prev_nodes.clone(), sub_fc)
        }
    }
}

pub fn add_edges_to(mg: &mut MermaidGraph, prev_nodes: &[PreviousNode], next_node: &NodeId) {
    add_edges_impl(mg, prev_nodes, next_node, false)
}

pub fn add_edges_to_end(mg: &mut MermaidGraph, prev_nodes: &[PreviousNode], end_node: &NodeId) {
    add_edges_impl(mg, prev_nodes, end_node, true)
}

pub fn add_edges_impl(
    mg: &mut MermaidGraph,
    prev_nodes: &[PreviousNode],
    next_node: &NodeId,
    is_end: bool,
) {
    for prev_node in prev_nodes {
        if prev_node.is_terminal && !is_end {
            continue;
        }

        mg.add_edge(&prev_node.nid, next_node, prev_node.label.as_deref());
    }
}
