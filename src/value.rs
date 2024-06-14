use petgraph::{Directed, Graph};
use std::fmt::{Display, Formatter};
use std::ops::{Add, Mul};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub enum Op {
    ADD,
    MUL,
    NONE,
}

#[derive(Debug, Clone)]
pub struct Value<T> {
    pub data: T,
}

impl Add for Value<f64> {
    type Output = Value<f64>;

    fn add(self, other: Value<f64>) -> Value<f64> {
        Value {
            data: self.data + other.data,
        }
    }
}

impl Mul for Value<f64> {
    type Output = Value<f64>;

    fn mul(self, other: Value<f64>) -> Value<f64> {
        Value {
            data: self.data * other.data,
        }
    }
}
