mod handle;
mod heap;
mod gc;

pub use heap::Heap;
pub use gc::{Tracer, Trace};
pub use handle::Handle;

pub trait Object: Sized + Trace {}
