#[derive(Debug)]
pub(crate) struct Param {
    name: String,
    type_name: String
}

impl Param {
    pub(crate) fn new(
        name: impl Into<String>,
        type_name: impl Into<String>
    ) -> Self {
        Self {
            name: name.into(),
            type_name: type_name.into()
        }
    }

    pub(crate) fn name(&self) -> &str {
        self.name.as_str()
    }

    pub(crate) fn type_name(&self) -> &str {
        self.type_name.as_str()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum UnaryOp {
    Not,
    Neg
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BinaryOp {
    Mul,
    Div,

    Add,
    Sub,

    Lt,
    Le,
    Gt,
    Ge,

    Eq,
    Ne,

    And,

    Or
}

impl BinaryOp {
    const MIN_PRECEDENCE: i32 = i32::MIN;
    
    pub(crate) fn precedence(&self) -> i32 {
        match self {
            Self::Mul | Self::Div => 5,
            Self::Add | Self::Sub => 4,
            Self::Lt | Self::Le | Self::Gt | Self::Ge => 3,
            Self::Eq | Self::Ne => 2,
            Self::And => 1,
            Self::Or => 0
        }
    }
}