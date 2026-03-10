use crate::value::DoughValue;

pub(super) struct Registers {
    regs: Vec<DoughValue>
}

impl Registers {
    pub(super) fn new(count: usize) -> Self {
        Self { regs: vec![DoughValue::Unit; count] }
    }
    
    pub(super) fn get(&self, index: usize) -> DoughValue {
        self.regs[index]
    }
    
    pub(super) fn set(&mut self, index: usize, value: DoughValue) {
        self.regs[index] = value;
    }
}