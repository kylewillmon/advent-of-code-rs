use std::fmt;
use std::error::Error;

pub trait Solver {
    fn solve(self: Box<Self>, input: String) -> Result<String, Box<dyn 'static + Error>>;
}

impl<F, T: fmt::Display, E: 'static + Error> Solver for F
    where F: Fn(String) -> Result<T, E>
{
    fn solve(self: Box<Self>, input: String) -> Result<String, Box<dyn 'static + Error>>
    {
        match self(input) {
            Ok(v) => Ok(v.to_string()),
            Err(e) => Err(e.into()),
        }
    }
}
