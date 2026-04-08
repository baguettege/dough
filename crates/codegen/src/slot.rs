use std::collections::HashMap;
use ast::{Node, NodeId};
use bytecode::{Idx, Reg};

#[derive(Copy, Clone)]
pub(crate) enum Slot {
    Global(Idx),
    Local(Reg),
    Fn(Idx),
}

#[derive(Default)]
pub(crate) struct Table(HashMap<NodeId, Slot>);

impl Table {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn insert(&mut self, node: &impl Node, slot: Slot) {
        self.0.insert(node.id(), slot);
    }

    pub(crate) fn get(&self, node: &impl Node) -> Slot {
        self.get_binding(node.id())
    }

    pub(crate) fn get_binding(&self, id: NodeId) -> Slot {
        self.0.get(&id)
            .copied()
            .expect("compiler bug: slot missing")
    }
}
