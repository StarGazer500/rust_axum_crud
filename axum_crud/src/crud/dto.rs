
use serde::{Deserialize};

#[derive(Deserialize,Debug)]
pub struct RequestCredentials{
    pub email : String,
    pub password : String
}

#[derive(serde::Deserialize)]
pub struct GetByEmailRequest {
    pub email: String,
}