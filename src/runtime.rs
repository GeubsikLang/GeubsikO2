use crate::syntax::*;
use std::borrow::Cow;

pub struct Variable<'a> {
    ident: Identifier<'a>,
    value: Value,
}

#[derive(Clone)]
pub enum Value {
    Undefined,
    Number(f64),
    String(String),
}

pub enum ArithmeticCast<'a> {
    Undefined,
    Numbers(f64, f64),
    Strings(Cow<'a, str>, Cow<'a, str>),
}

impl Value {
    pub fn to_boolean(&self) -> bool {
        match self {
            Value::Number(num) => num.abs() > std::f64::EPSILON,
            Value::String(s) => !s.is_empty(),
            Value::Undefined => false,
        }
    }

    pub fn negate(&self) -> bool {
        !self.to_boolean()
    }

    pub fn arithmetic_cast<'a>(left: &'a Value, right: &'a Value) -> ArithmeticCast<'a> {
        match (left, right) {
            (Value::Undefined, Value::Undefined) => ArithmeticCast::Undefined,
            (Value::Undefined, Value::Number(num)) => ArithmeticCast::Numbers(0f64, *num),
            (Value::Number(num), Value::Undefined) => ArithmeticCast::Numbers(*num, 0f64),
            (Value::Undefined, Value::String(s)) => ArithmeticCast::Strings("".into(), s.into()),
            (Value::String(s), Value::Undefined) => ArithmeticCast::Strings(s.into(), "".into()),
            (Value::Number(left), Value::Number(right)) => ArithmeticCast::Numbers(*left, *right),
            (Value::Number(num), Value::String(s)) => {
                ArithmeticCast::Numbers(*num, s.parse().unwrap_or(0f64))
            }
            (Value::String(s), Value::Number(num)) => {
                ArithmeticCast::Numbers(s.parse().unwrap_or(0f64), *num)
            }
            (Value::String(left), Value::String(right)) => {
                ArithmeticCast::Strings(left.into(), right.into())
            }
        }
    }

    pub fn add(&self, other: &Value) -> Value {
        match Self::arithmetic_cast(self, other) {
            ArithmeticCast::Undefined => Value::Number(1f64),
            ArithmeticCast::Numbers(left, right) => Value::Number(left + right),
            ArithmeticCast::Strings(left, right) => Value::String((left + right).into()),
        }
    }

    pub fn subtract(&self, other: &Value) -> Value {
        match Self::arithmetic_cast(self, other) {
            ArithmeticCast::Undefined => Value::Number(1f64),
            ArithmeticCast::Numbers(left, right) => Value::Number(left - right),
            ArithmeticCast::Strings(_, _) => Value::Undefined,
        }
    }

    pub fn multiply(&self, other: &Value) -> Value {
        match Self::arithmetic_cast(self, other) {
            ArithmeticCast::Undefined => Value::Number(1f64),
            ArithmeticCast::Numbers(left, right) => Value::Number(left - right),
            ArithmeticCast::Strings(_, _) => Value::Undefined,
        }
    }

    pub fn divide(&self, other: &Value) -> Value {
        match Self::arithmetic_cast(self, other) {
            ArithmeticCast::Undefined => Value::Number(1f64),
            ArithmeticCast::Numbers(left, right) => Value::Number(left / right),
            ArithmeticCast::Strings(_, _) => Value::Undefined,
        }
    }

    pub fn modulo(&self, other: &Value) -> Value {
        match Self::arithmetic_cast(self, other) {
            ArithmeticCast::Undefined => Value::Number(1f64),
            ArithmeticCast::Numbers(left, right) => Value::Number(left % right),
            ArithmeticCast::Strings(_, _) => Value::Undefined,
        }
    }

    pub fn equal(&self, other: &Value) -> bool {
        match Self::arithmetic_cast(self, other) {
            ArithmeticCast::Undefined => true,
            ArithmeticCast::Numbers(left, right) => (left - right).abs() < std::f64::EPSILON,
            ArithmeticCast::Strings(left, right) => left == right,
        }
    }

    pub fn strict_equal(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Undefined, Value::Undefined) => true,
            (Value::Number(left), Value::Number(right)) => (left - right).abs() < std::f64::EPSILON,
            (Value::String(left), Value::String(right)) => left == right,
            _ => false,
        }
    }

    pub fn greater_than(&self, other: &Value) -> bool {
        match Self::arithmetic_cast(self, other) {
            ArithmeticCast::Undefined => false,
            ArithmeticCast::Numbers(left, right) => left > right,
            ArithmeticCast::Strings(left, right) => left > right,
        }
    }

    pub fn less_than(&self, other: &Value) -> bool {
        match Self::arithmetic_cast(self, other) {
            ArithmeticCast::Undefined => false,
            ArithmeticCast::Numbers(left, right) => left < right,
            ArithmeticCast::Strings(left, right) => left < right,
        }
    }
}
