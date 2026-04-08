use ast::{Node, NodeId};
use ast::typed::Block;
use bytecode::{Idx, Reg};
use crate::emitter::chunk::Builder;
use crate::emitter::Emitter;
use crate::Result;
use crate::slot::Slot;

impl Emitter<'_> {
    pub(super) fn emit_block(&self, builder: &mut Builder, block: &Block) -> Result<()> {
        block.iter().try_for_each(|stmt| self.emit_stmt(builder, stmt))
    }
}

impl Emitter<'_> {
    pub(super) fn slot(&self, node: &impl Node) -> Slot {
        self.layout.slots().get(node)
    }

    pub(super) fn binding_slot(&self, id: NodeId) -> Slot {
        self.layout.slots().get_binding(id)
    }

    pub(super) fn local(&self, node: &impl Node) -> Reg {
        match self.slot(node) {
            Slot::Local(reg) => reg,
            _ => unreachable!(),
        }
    }

    pub(super) fn global(&self, node: &impl Node) -> Idx {
        match self.slot(node) {
            Slot::Global(idx) => idx,
            _ => unreachable!(),
        }
    }
    
    pub(super) fn fn_idx(&self, node: &impl Node) -> Idx {
        match self.slot(node) {
            Slot::Fn(idx) => idx,
            _ => unreachable!(),
        }
    }
}
