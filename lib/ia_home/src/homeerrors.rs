use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum HomeErrors {
    NotExist(String),
    AlreadyExist(String),
}

impl Display for HomeErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            HomeErrors::NotExist(x) => format!("element with name '{}' not exist", x),
            HomeErrors::AlreadyExist(x) => format!("element with name '{}' already exist", x),
        };
        f.write_str(&msg)
    }
}

impl Error for HomeErrors {}
