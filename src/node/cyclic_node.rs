use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::Mutex;

use crate::{EnvVar, InChannels, Node, NodeId, NodeName, OutChannels, Output};

/// # Cyclic node type
/// A `CyclicNode` represents a sub-graph of `Graph`, which contains a group of
/// [`Node`]s connected in a circle.
pub struct CyclicNode {
    entry: NodeId,
    exit: NodeId,
    nodes: Vec<Arc<Mutex<dyn Node>>>,
    inner_edges: Vec<(NodeId, Vec<NodeId>)>,
}

impl CyclicNode {
    pub fn new(nodes: Vec<impl Node + 'static>, entry: NodeId, exit: NodeId) -> Result<Self, CyclicNodeErr> {
        let mut cyclic_node = Self {
            entry,
            exit,
            nodes: vec![],
            inner_edges: vec![],
        };
        if nodes.iter().filter(|node| node.id() == entry).next().is_none() {
            Err(CyclicNodeErr::InvalidEntry(entry))
        }
        else if nodes.iter().filter(|node| node.id() == exit).next().is_none() {
            Err(CyclicNodeErr::InvalidExit(exit))
        }
        else {
            nodes.into_iter().for_each(|node| {
                cyclic_node.nodes.push(Arc::new(Mutex::new(node)));
            });
            Ok(cyclic_node)
        }
    }

    pub fn add_inner_edge(&mut self, src: NodeId, dests: Vec<NodeId>) {
        self.inner_edges.push((src, dests));
    }
}

#[async_trait]
impl Node for CyclicNode {
    fn id(&self) -> NodeId {
        panic!()
    }

    fn name(&self) -> NodeName {
        panic!()
    }

    fn input_channels(&mut self) ->  &mut InChannels {
        panic!()
    }

    fn output_channels(&mut self) ->  &mut OutChannels {
        panic!()
    }

    async fn run(&mut self, _:Arc<EnvVar>) ->  Output {
        panic!("CyclicNode itself should never run.")
    }
}

pub enum CyclicNodeErr {
    InvalidEntry(NodeId),
    InvalidExit(NodeId),
}