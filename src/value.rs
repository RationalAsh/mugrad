use std::fmt::{Display, Formatter};
use std::ops::{Add, Mul};

/// Struct to represent a value.
#[derive(Debug, Clone)]
pub struct Value<T> {
    data: T,
}

/// Implement the Value struct
impl<T> Value<T> {
    /// Create a new value.
    pub fn new(data: T) -> Self {
        Value { data }
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
