use std::fmt;

use anyhow;

pub trait Solver {
    fn solve(self: Box<Self>, input: String) -> Result<String, anyhow::Error>;
}

impl<F, T: fmt::Display, E: Into<anyhow::Error>> Solver for F
    where F: Fn(String) -> Result<T, E>
{
    fn solve(self: Box<Self>, input: String) -> Result<String, anyhow::Error>
    {
        match self(input) {
            Ok(v) => Ok(v.to_string()),
            Err(e) => Err(e.into()),
        }
    }
}
