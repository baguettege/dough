use ast::typed::{Item, Program, Func};
use crate::func_table::FuncTable;

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
