use std::rc::Rc;
use crate::bytecode::function::Function;

pub struct Program {
    main: Rc<Function>,
    functions: Vec<Rc<Function>>,
    global_count: usize
}

impl Program {
    pub(crate) fn new(
        main: Function,
        functions: Vec<Function>,
        global_count: usize
    ) -> Self {
        Self {
            main: Rc::new(main),
            functions: functions.into_iter().map(|proto| Rc::new(proto)).collect(),
            global_count
        }
    }

    pub(crate) fn main(&self) -> Rc<Function> {
        self.main.clone()
    }

    pub(crate) fn function(&self, index: usize) -> Rc<Function> {
        self.functions[index].clone()
    }

    pub(crate) fn global_count(&self ) -> usize {
        self.global_count
    }
}