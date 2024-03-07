use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize)]
pub struct UserCadastro {
    pub username : String,
    pub email : String,
    pub first_name : String,
    pub last_name : String , 
    pub password : String
}

#[derive(Serialize,Deserialize)]
pub struct EditPerfilURL {
    pub uuid: String
}

#[derive(Serialize,Deserialize)]
pub struct User {
    pub uuid: String,
    pub username : String,
    pub email : String,
    pub first_name : String,
    pub last_name : String , 
    pub password : String
}

impl Pizza {
    pub fn new(username: String, email: String, first_name: String, last_name: String, password: String) -> User {
        User {
            uuid,username,first_name,last_name, password
        }
    }
}