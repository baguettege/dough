use std::collections::HashMap;
use ast::{Node, NodeId};
use crate::symbol::Symbol;

#[derive(Default)]
pub struct Table(HashMap<NodeId, Symbol>);

impl Table {
    pub(crate) fn insert(&mut self, node: &impl Node, symbol: Symbol) {
        self.0.insert(node.id(), symbol);
    }

    pub fn get(&self, node: &impl Node) -> &Symbol {
        self.0.get(&node.id()).expect("symbol not found: resolver bug")
    }
}
