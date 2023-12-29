#[derive(Debug, Serialize, Deserialize)]
pub struct UserLicence {
   pub username: String,
   pub email   : String,
   pub licence : String,
   pub periodo : String,
}