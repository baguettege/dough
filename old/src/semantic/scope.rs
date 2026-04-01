use std::collections::HashMap;
use crate::ty::DoughType;

pub(super) enum Symbol {
    Local { ty: DoughType },
    Global { ty: DoughType },
    Func { params: Vec<DoughType>, return_type: DoughType },
}

pub(super) struct Scope {
    symbols: HashMap<String, Symbol>,
}

impl Scope {
    pub(super) fn new() -> Self {
        Self { symbols: HashMap::new() }
    }
}
