use crate::value::array::DoughArray;
use crate::value::DoughValue;

mod value;
mod heap;
mod vm;
mod bytecode;

fn main() {
    let x = dough_array![DoughValue::Bool(true),];
}