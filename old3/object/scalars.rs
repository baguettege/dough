#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct DoughInt(i64);

impl DoughInt {
    pub fn new(value: i64) -> Self {
        Self(value)
    }

    pub fn as_i64(&self) -> i64 {
        self.0
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct DoughFloat(f64);

impl DoughFloat {
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    pub fn as_f64(&self) -> f64 {
        self.0
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct DoughBool(bool);

impl DoughBool {
    pub fn new(value: bool) -> Self {
        Self(value)
    }

    pub fn as_bool(&self) -> bool {
        self.0
    }
}