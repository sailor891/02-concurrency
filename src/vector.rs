use anyhow::{anyhow, Result};
use std::{
    fmt,
    iter::Sum,
    ops::{Add, AddAssign, Deref, Mul},
};

pub struct Vector<T> {
    pub data: Vec<T>,
}

impl<T> Vector<T> {
    pub fn new(data: impl Into<Vec<T>>) -> Vector<T> {
        Vector { data: data.into() }
    }
}

impl<T> Vector<T>
where
    T: fmt::Debug + Default + Add<Output = T> + AddAssign + Mul<Output = T> + Copy + Sum<T>,
{
    pub fn dot_product(&self, rhs: &Vector<T>) -> Result<T> {
        if self.data.len() != rhs.data.len() {
            return Err(anyhow!("Vector dimensions do not match"));
        }
        let sum = self
            .data
            .iter()
            .zip(rhs.data.iter())
            .map(|(&a, &b)| a * b)
            .sum();
        Ok(sum)
    }
}

impl<T> Deref for Vector<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
