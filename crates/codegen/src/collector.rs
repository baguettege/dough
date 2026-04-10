use std::collections::HashMap;
use ast::{Node, NodeId};
use ast::typed::{Func, Item, Program};
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

#[derive(Default)]
struct Collector {
    funcs: FuncTable,
}

impl Collector {
    fn new() -> Self {
        Self::default()
    }

    fn collect(mut self, program: &Program) -> FuncTable {
        for item in program {
            self.collect_item(item);
        }
        self.funcs
    }

    fn collect_item(&mut self, item: &Item) {
        match item {
            Item::Func(node) => self.collect_fn(node),
        }
    }

    fn collect_fn(&mut self, node: &Func) {
        self.funcs.insert(node);
    }
}

pub(super) fn collect(program: &Program) -> FuncTable {
    Collector::new().collect(program)
}
