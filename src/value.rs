use std::fmt::{Display, Formatter};
use std::ops::{Add, Mul};

#[derive(Debug, Clone)]
pub enum Op {
    ADD,
    MUL,
    NONE,
}

/// Struct to represent a value.
#[derive(Debug, Clone)]
pub struct Value<T> {
    data: T,
    op: Op,
}

/// Implement the Value struct
impl<T> Value<T> {
    /// Create a new value.
    pub fn new(data: T) -> Self {
        Value { data, op: Op::NONE }
    }
}

/// Implement the Add trait for the Value struct
impl<T> Add for Value<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Value<T>;

    fn add(self, other: Value<T>) -> Value<T> {
        Value {
            data: self.data + other.data,
            op: Op::ADD,
        }
    }
}

/// Implement the Mul trait for the Value struct
impl<T> Mul for Value<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Value<T>;

    fn mul(self, other: Value<T>) -> Value<T> {
        Value {
            data: self.data * other.data,
            op: Op::MUL,
        }
    }
}

/// Implement the Display trait for the Value struct
impl<T> Display for Value<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Value({})", self.data)
    }
}
