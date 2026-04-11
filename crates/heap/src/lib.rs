mod handle;
mod heap;
mod gc;

pub use heap::Heap;
pub use gc::{Tracer, Trace};

pub trait Object: Sized + Trace {}
