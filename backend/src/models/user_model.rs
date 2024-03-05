use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize)]
pub struct UserCadastro {
    pub username : String,
    pub email : String,
    pub first_name : String,
    pub last_name : String , 
    pub password : String
}