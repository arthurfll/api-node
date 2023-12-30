use serde::{Serialize,Deserialize};


#[derive(Serialize, Deserialize)]
pub struct UserLicence {
   pub username: String,
   pub email   : String,
   pub licence : String,
   pub periodo : String,
}
