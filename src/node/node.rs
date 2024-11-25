use std::{collections::HashMap, sync::Arc};

use crate::{
    connection::{in_channel::InChannels, out_channel::OutChannels},
    utils::{env::EnvVar, output::Output},
};

use super::id_allocate::alloc_id;

///# The [`Node`] trait
///
/// Nodes are the basic scheduling units of Graph. They can be identified by
/// a globally assigned [`NodeId`] and a user-provided name.
///
/// Nodes can communicate with others asynchronously through [`InChannels`] and [`OutChannels`].
///
/// In addition to the above properties, users can also customize some other attributes.
pub trait Node: Send + Sync {
    /// id is the unique identifier of each node, it will be assigned by the [`NodeTable`]
    /// when creating a new node, you can find this node through this identifier.
    fn id(&self) -> NodeId;
    /// The node's name.
    fn name(&self) -> NodeName;
    /// Input Channels of this node.
    fn input_channels(&mut self) -> &mut InChannels;
    /// Output Channels of this node.
    fn output_channels(&mut self) -> &mut OutChannels;
    /// Execute a run of this node.
    fn run(&mut self, env: Arc<EnvVar>) -> Output;
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct NodeId(pub(crate) usize);

pub type NodeName = String;

/// [NodeTable]: a mapping from [Node]'s name to [NodeId].
#[derive(Default)]
pub struct NodeTable(pub(crate) HashMap<NodeName, NodeId>);

/// [NodeTable]'s name in [`EnvVar`].
pub const NODE_TABLE_STR: &str = "node_table";

impl NodeTable {
    /// Alloc a new [NodeId] for a [Node].
    ///
    /// If there is a Node requesting for an ID with a duplicate name,
    /// the older one's info will be overwritten.
    pub fn alloc_id_for(&mut self, name: &str) -> NodeId {
        let id = alloc_id();
        log::debug!("alloc id {:?} for {:?}", id, name);

        if let Some(v) = self.0.insert(name.to_string(), id.clone()) {
            log::warn!("Node {} is already allocated with id {:?}.", name, v);
        };
        id
    }

    pub fn get(&self, name: &str) -> Option<&NodeId> {
        self.0.get(name)
    }

    pub fn new() -> Self {
        Self::default()
    }
}

impl EnvVar {
    /// Get a [`Node`]'s [`NodeId`] by providing its name.
    pub fn get_node_id(&self, node_name: &str) -> Option<&NodeId> {
        let node_table: &NodeTable = self.get_ref(NODE_TABLE_STR).unwrap();
        node_table.get(node_name)
    }
}