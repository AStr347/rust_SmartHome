use thiserror::Error;

#[derive(Error, Debug)]
pub enum HomeErrors {
    #[error("element with name '{0}' not exist")]
    NotExist(String),
    #[error("element with name '{0}' already exist")]
    AlreadyExist(String),
}
