use serde::{Serialize,Deserialize};


#[derive(Serialize,Deserialize)]
pub struct RequestUser {
    pub username: String
}

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
    pub uuid: String,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct User {
    pub uuid: String,
    pub username : String,
    pub email : String,
    pub first_name : String,
    pub last_name : String , 
    pub password : String
}

impl User {
    pub fn new(uuid: String, username: String, email: String, first_name: String, last_name: String, password: String) -> User {
        User {
            uuid,username,email, first_name,last_name, password
        }
    }
}

