use serde::{
    Serialize, Deserialize
};

#[derive(Serialize,Deserialize)]
pub struct Cadastro {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email:String,
    pub password: String,
    pub saldo: i128
}
