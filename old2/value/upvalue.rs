use crate::value::DoughValue;

pub(crate) enum DoughUpvalue {
    Open(usize), // register index
    Closed(DoughValue)
}