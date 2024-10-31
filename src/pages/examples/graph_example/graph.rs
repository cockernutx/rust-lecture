use std::{collections::HashSet, fmt::Display, vec};
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Edge {
    pub r#in: String,
    pub out: String,
    pub relation_type: String,
}
#[allow(dead_code)]
impl Edge {
    pub fn new(r#in: String, out: String, relation_type: String) -> Self {
        Self {
            r#in,
            out,
            relation_type,
        }
    }
}
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Node<T: Clone> {
    pub id: String,
    pub content: T,
}
#[derive(Debug, Clone)]
pub enum Error {
    NodeNotFound(String),
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NodeNotFound(id) => write!(f, "node {id} not found"),
        }
    }
}
type Result<T> = std::result::Result<T, Error>;
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Graph<T: Clone> {
    pub nodes: Vec<Node<T>>,
    pub edges: HashSet<Edge>,
}
#[allow(dead_code)]
impl<T: Clone> Graph<T> {
    pub fn new() -> Self {
        Self {
            nodes: vec![],
            edges: HashSet::new(),
        }
    }
    pub fn add_node(&mut self, content: T, id: String) -> String {
        let node = Node {
            id: id.clone(),
            content,
        };
        self.nodes.push(node);
        id
    }
    pub fn get_by_id_mut(&mut self, id: String) -> Option<&mut Node<T>> {
        self.nodes.iter_mut().find(|x| x.id == id)
    }
    pub fn get_by_id(&self, id: String) -> Option<&Node<T>> {
        self.nodes.iter().find(|x| x.id == id)
    }
    pub fn relate(&mut self, r#in: String, out: String, relation_type: String) -> Result<()> {
        let Some(_) = self.get_by_id(r#in.clone()) else {
            return Err(Error::NodeNotFound(r#in));
        };
        let Some(_) = self.get_by_id(out.clone()) else {
            return Err(Error::NodeNotFound(out));
        };
        let edge = Edge {
            r#in,
            out,
            relation_type,
        };
        self.edges.insert(edge);
        Ok(())
    }
    pub fn get_edges(&self, node: String) -> Result<Vec<&Edge>> {
        let Some(node) = self.get_by_id(node.clone()) else {
            return Err(Error::NodeNotFound(node));
        };
        Ok(self
            .edges
            .iter()
            .filter(|x| x.r#in == node.id || x.out == node.id)
            .collect())
    }
}
impl<T: Clone> IntoIterator for Graph<T> {
    type Item = (Node<T>, Vec<Edge>);
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        let mut i = vec![];
        for node in self.nodes.clone() {
            let edges = self.get_edges(node.id.clone()).unwrap();
            let edges: Vec<Edge> = edges.into_iter().map(|x| x.clone()).collect();
            i.push((node, edges));
        }
        i.into_iter()
    }
}
