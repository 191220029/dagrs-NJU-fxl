//! # Example: hello_dagrs
//! Creates a `DefaultNode` that returns with "Hello Dagrs",
//! add this node to a new `Graph` and run.

use std::sync::Arc;

use async_trait::async_trait;
use dagrs::{
    Action, Content, DefaultNode, EnvVar, Graph, InChannels, Node, NodeTable, OutChannels, Output,
};

#[derive(Default)]
pub struct HelloAction;
#[async_trait]
impl Action for HelloAction {
    async fn run(&self, _: &mut InChannels, _: &OutChannels, _: Arc<EnvVar>) -> Output {
        Output::Out(Some(Content::new("Hello Dagrs".to_string())))
    }
}

fn main() {
    let mut node_table = NodeTable::new();
    let hello_node = DefaultNode::with_action(
        "Hello Dagrs".to_string(),
        HelloAction::default(),
        &mut node_table,
    );
    let id: &dagrs::NodeId = &hello_node.id();
    let mut graph = Graph::new();
    graph.add_node(hello_node);
    graph.run();

    let outputs = graph.get_outputs();
    assert_eq!(outputs.len(), 1);

    let content = outputs.get(id).unwrap().get_out().unwrap();
    let node_output = content.get::<String>().unwrap();
    assert_eq!(node_output, "Hello Dagrs")
}
