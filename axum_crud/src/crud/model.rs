
use serde::{Serialize};


#[derive(Serialize)]
pub struct ResponseCredentials{
   pub email : String,
   pub password : String
}