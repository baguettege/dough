use std::rc::Rc;
use crate::bytecode::proto::Proto;

pub struct Program {
    main: Rc<Proto>,
    functions: Vec<Rc<Proto>>
}

impl Program {
    pub(crate) fn new(main: Proto, functions: Vec<Proto>) -> Self {
        Self {
            main: Rc::new(main),
            functions: functions.into_iter().map(|proto| Rc::new(proto)).collect()
        }
    }

    pub(crate) fn main(&self) -> Rc<Proto> {
        self.main.clone()
    }

    pub(crate) fn function(&self, index: usize) -> Rc<Proto> {
        self.functions[index].clone()
    }
}