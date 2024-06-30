use std::{cell::RefCell, fmt, rc::Rc};

use itertools::Itertools as _;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NodeId(String);

impl fmt::Display for NodeId {
    fn fmt(&self, b: &mut fmt::Formatter) -> fmt::Result {
        write!(b, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
struct NodeIdGenerator {
    digits: Vec<char>,
}

impl NodeIdGenerator {
    pub fn new() -> Self {
        Self { digits: vec![] }
    }

    pub fn step(&mut self) {
        for i in (0..self.digits.len()).rev() {
            if self.digits[i] != 'Z' {
                self.digits[i] = (self.digits[i] as u8 + 1) as char;
                return;
            }

            self.digits[i] = 'A';
        }

        // All characters are 'Z' if we reach this point
        self.digits.push('A');
    }

    pub fn generate(&mut self) -> NodeId {
        self.step();
        NodeId(self.to_string())
    }
}

impl fmt::Display for NodeIdGenerator {
    fn fmt(&self, b: &mut fmt::Formatter) -> fmt::Result {
        write!(b, "{}", self.digits.iter().format(""))
    }
}

impl Iterator for NodeIdGenerator {
    type Item = NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        self.step();
        Some(self.generate())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Node {
    id: NodeId,
    label: String,
    shape: NodeShape,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.shape {
            NodeShape::Rounded => write!(f, r#"{}("{}")"#, self.id, self.label),
            NodeShape::Rectangle => write!(f, r#"{}["{}"]"#, self.id, self.label),
            NodeShape::Diamond => write!(f, r#"{}{{"{}"}}"#, self.id, self.label),
            NodeShape::Trapezoid => write!(f, r#"{}[/"{}"\]"#, self.id, self.label),
            NodeShape::InvertedTrapezoid => write!(f, r#"{}[\"{}"/]"#, self.id, self.label),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NodeShape {
    Rounded,
    Rectangle,
    Diamond,
    Trapezoid,
    InvertedTrapezoid,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Edge {
    id0: NodeId,
    id1: NodeId,
    label: Option<String>,
}

#[derive(Debug, Clone)]
pub struct MermaidFlowchart {
    graph: MermaidGraph,
}

#[derive(Debug, Clone)]
pub struct MermaidGraph {
    id_gen: Rc<RefCell<NodeIdGenerator>>,
    label: Option<String>,
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    subgraphs: Vec<MermaidGraph>,
}

impl MermaidFlowchart {
    pub fn new() -> Self {
        let id_gen = Rc::new(RefCell::new(NodeIdGenerator::new()));
        let graph = MermaidGraph::new(Rc::clone(&id_gen));
        Self { graph }
    }

    pub fn graph_mut(&mut self) -> &mut MermaidGraph {
        &mut self.graph
    }

    pub fn render<W: fmt::Write>(self, writer: &mut W) {
        writeln!(writer, "flowchart TD;").unwrap();
        self.graph.render(writer);
    }
}

impl Default for MermaidFlowchart {
    fn default() -> Self {
        Self::new()
    }
}

impl MermaidGraph {
    fn new(id_gen: Rc<RefCell<NodeIdGenerator>>) -> Self {
        Self {
            id_gen,
            label: None,
            nodes: vec![],
            edges: vec![],
            subgraphs: vec![],
        }
    }

    fn with_label(id_gen: Rc<RefCell<NodeIdGenerator>>, label: String) -> Self {
        Self {
            id_gen,
            label: Some(label),
            nodes: vec![],
            edges: vec![],
            subgraphs: vec![],
        }
    }

    pub fn add_node(&mut self, label: impl Into<String>, shape: NodeShape) -> NodeId {
        let label = label.into();
        let id = self.id_gen.borrow_mut().generate();
        self.nodes.push(Node {
            id: id.clone(),
            label,
            shape,
        });

        id
    }

    pub fn add_edge(&mut self, id0: &NodeId, id1: &NodeId, label: Option<&str>) {
        self.edges.push(Edge {
            id0: id0.clone(),
            id1: id1.clone(),
            label: label.map(|s| s.to_string()),
        });
    }

    pub fn add_subgraph(&mut self, label: String) -> &mut MermaidGraph {
        let subgraph = MermaidGraph::with_label(Rc::clone(&self.id_gen), label);
        self.subgraphs.push(subgraph);
        self.subgraphs.last_mut().unwrap()
    }

    pub fn render<W: fmt::Write>(self, writer: &mut W) {
        self.render_nodes(writer);
        self.render_edges(writer);
    }

    fn render_nodes<W: fmt::Write>(&self, writer: &mut W) {
        for node in &self.nodes {
            writeln!(writer, "{};", node).unwrap();
        }

        for subgraph in &self.subgraphs {
            writeln!(
                writer,
                "subgraph \"{}\"",
                subgraph.label.as_deref().unwrap_or("")
            )
            .unwrap();
            subgraph.render_nodes(writer);
            writeln!(writer, "end").unwrap();
        }
    }

    fn render_edges<W: fmt::Write>(&self, writer: &mut W) {
        for edge in &self.edges {
            if let Some(label) = &edge.label {
                writeln!(writer, r#"{} -->|"{}"| {};"#, edge.id0, label, edge.id1).unwrap();
            } else {
                writeln!(writer, "{} --> {};", edge.id0, edge.id1).unwrap();
            }
        }

        for subgraph in &self.subgraphs {
            // Do not include edges within `subgraph ... end` block
            subgraph.render_edges(writer);
        }
    }
}
