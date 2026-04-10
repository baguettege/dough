use std::collections::HashMap;
use ast::{Node, NodeId};
use bytecode::Idx;

#[derive(Default)]
pub(crate) struct FuncTable(HashMap<NodeId, Idx>);

impl FuncTable {
    pub(crate) fn insert(&mut self, node: &impl Node) -> Idx {
        let idx: Idx = self.0.len()
            .try_into()
            .expect("exceeded max fn indices");
        self.0.insert(node.id(), idx);
        idx
    }

    pub(crate) fn get(&self, id: NodeId) -> Idx {
        self.0.get(&id)
            .copied()
            .expect("compiler bug: fn not found")
    }
}
