#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct NodeId(u32);

impl NodeId {
    pub fn new(value: u32) -> Self {
        Self(value)
    }
    
    pub fn as_u32(&self) -> u32 {
        self.0
    }
}

pub trait Node {
    fn id(&self) -> NodeId;
}