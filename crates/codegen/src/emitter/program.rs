use crate::emitter::chunk::Builder;
use crate::emitter::Emitter;
use crate::Result;
use ast::typed::{Fn, Item, Program, Static};
use bytecode::{Chunk, Idx, Instr, Reg};

impl Emitter<'_> {
    pub(super) fn emit_fns(&self, program: &Program) -> Result<Vec<Chunk>> {
        let mut fns = Vec::new();

        for item in program {
            if let Item::Fn(node) = item {
                let chunk = self.emit_fn(node)?;
                // fn chunks must be emitted in the same top level encounter
                // order used by `Allocator` when assigning `Slot::Fn(idx)`
                fns.push(chunk);
            }
        }

        Ok(fns)
    }

    fn emit_fn(&self, node: &Fn) -> Result<Chunk> {
        let mut builder = Builder::new();

        self.emit_block(&mut builder, node.body())?;

        let idx = self.fn_idx(node);
        let local_count = self.layout.fn_local_counts()[idx as usize];

        Ok(builder.build(local_count))
    }
}

impl Emitter<'_> {
    pub(super) fn emit_entry(&self, program: &Program) -> Result<Chunk> {
        let mut builder = Builder::new();

        for item in program {
            if let Item::Static(node) = item {
                self.emit_static(&mut builder, node)?;
            }
        }

        let idx = self.main_fn_idx(program);
        let entry_local_count = self.layout.entry_local_count();
        // `Allocator` must ensure the final register is allocated for the dst of 'main'
        let dst: Reg = (entry_local_count - 1)
            .try_into()
            .expect("compiler bug: entry call dst out of range");

        builder.emit(Instr::Call { dst, idx, argc: 0 });
        builder.emit(Instr::Halt {});

        Ok(builder.build(entry_local_count))
    }

    fn main_fn_idx(&self, program: &Program) -> Idx {
        program
            .iter()
            .find_map(|item| {
                match item {
                    Item::Fn(node) if node.ident() == "main" =>
                        Some(self.fn_idx(node)),
                    _ => None,
                }
            }).expect("no 'main' fn found in AST: semantic analyzer bug")
    }

    fn emit_static(&self, builder: &mut Builder, node: &Static) -> Result<()> {
        let idx = self.global(node);

        let src = self.emit_expr(builder, node.init())?;
        builder.emit(Instr::Stg { idx, src });

        Ok(())
    }
}
