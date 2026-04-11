use std::collections::HashMap;
use ast::{Node, NodeId};
use bytecode::Idx;

#[derive(Default)]
pub(super) struct LocalAllocator {
    locals: HashMap<NodeId, Idx>,
    next: Idx,
}

impl LocalAllocator {
    pub(super) fn new() -> Self {
        Self::default()
    }

    pub(super) fn alloc(&mut self, node: &impl Node) -> Idx {
        let idx = self.next;
        self.locals.insert(node.id(), idx);
        self.next = self.next
            .checked_add(1)
            .expect("exceeded max local indices");
        idx
    }

    pub(super) fn get(&self, id: NodeId) -> Idx {
        self.locals.get(&id)
            .copied().expect("compiler bug: variable not found")
    }
    
    pub(super) fn count(&self) -> usize {
        self.next as usize
    }
}
