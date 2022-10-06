use std::ops::{Add,Sub,AddAssign};
use log::{ info, error, debug, /*warn,*/ trace };

use std::fmt;
use crate::minmax::MinMax::{Value,NA};

#[derive(Debug,Clone,Copy,PartialOrd,Ord,PartialEq,Eq)]
pub enum MinMax<T> where T: Clone {
    Min,
    Value(T),
    Max,
    NA,
}

// Implement `Display` for `MinMax`.
impl<T: fmt::Display+ std::clone::Clone> fmt::Display for MinMax<T>

{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MinMax::Min => f.pad(&format!("Min")),
            MinMax::Max => f.pad(&format!("Max")),
            MinMax::NA =>  f.pad(&format!("NA")),
            MinMax::Value(ref x) =>  f.pad(&format!("{}", x))
        }
    }
}

impl<T: std::ops::Add + std::cmp::PartialEq + Add<Output = T>+ std::clone::Clone> Add for MinMax<T> {
    type Output = MinMax<T>;

    fn add(self, other: MinMax<T>) -> MinMax<T> {

        match (self, other ) {
            (MinMax::Min, MinMax::Min)  | (MinMax::Min,MinMax::NA) | (MinMax::NA, MinMax::Min) =>  MinMax::Min,
            (MinMax::NA, MinMax::NA) => MinMax::NA,
            (Value(op1), MinMax::Min) | (Value(op1), MinMax::NA) => Value(op1),
            (MinMax::Min, Value(op2)) | (MinMax::NA, Value(op2)) => Value(op2),
            (MinMax::Max,_) | (_, MinMax::Max) => MinMax::Max,
            (Value(op1), Value(op2)) => Value(op1+op2),
        }
    }
}

impl<T: std::ops::Sub + std::cmp::PartialEq + Sub<Output = T>+ std::clone::Clone> Sub for MinMax<T> {
    type Output = MinMax<T>;

    fn sub(self, other: MinMax<T>) -> MinMax<T> {

        match (self, other ) {
            (MinMax::Min, MinMax::Min)  | (MinMax::Min,MinMax::NA) | (MinMax::NA, MinMax::Min) =>  MinMax::Min,
            (MinMax::NA, MinMax::NA) => MinMax::NA,
            (Value(op1), MinMax::Min) | (Value(op1), MinMax::NA) => Value(op1),
            (MinMax::Min, Value(_)) => MinMax::Min,
            (MinMax::NA, Value(_)) => MinMax::Min,
            (MinMax::Max,_) => MinMax::Max,
            (_, MinMax::Max) => MinMax::Min,
            (Value(op1), Value(op2)) => Value(op1-op2),
        }
    }
}


impl<T: std::ops::Add + std::cmp::PartialEq + Add<Output = T>+std::clone::Clone> AddAssign for MinMax<T> {

    fn add_assign(&mut self, other: Self) {
        *self =  self.clone() + other;
    }
}


//impl<T: std::ops::Add + std::cmp::PartialEq + Add<Output = T>> MinMax<T> {
impl<T: std::fmt::Display+Clone> MinMax<T> {

    pub fn unwrap_value(&self) -> &T {
        match self {
            Value(obj) => obj,
            _ => panic!("Non-Value minmax {}", self)
        }
    }

    pub fn  is_value(&self) -> bool {
        match self {
            Value(obj) => true,
            _ => false,
        }
    }
    pub fn unwrap_value_or<'a>(&'a self, alt_value: &'a T) -> &T {
        match self {
            Value(obj) => obj,
            _ => alt_value,
        }
    }

}

