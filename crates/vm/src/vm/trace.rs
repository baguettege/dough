use bytecode::Instr;
use crate::value::Value;
use crate::vm::Vm;

impl Vm<'_> {
    pub(super) fn trace(&mut self, instr: Instr) {
        let values = self.operands
            .iter()
            .copied()
            .collect::<Vec<_>>();
        let stack = values
            .iter()
            .map(|val| self.fmt_value(val))
            .collect::<Vec<_>>();

        eprintln!("{} | [{}]", instr, stack.join(", "));
    }

    fn fmt_value(&mut self, value: &Value) -> String {
        match value {
            Value::Str(handle) => self.heap.with(
                *handle, |obj| format!("{obj}")),
            _ => format!("{value}")
        }
    }
}
