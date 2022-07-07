use std::error::Error;

pub type ConlangString = String;
pub type IpaString = String;
pub type ResultAnyError<T> = Result<T, Box<dyn Error>>;