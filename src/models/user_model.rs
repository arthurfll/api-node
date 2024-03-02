use serde::{Serialize,Deserialize}

#[derive(Deserialize,Serialize)]
struct User {
    id: i128
    username: String,
    first_name: String,
    last_name: String,
    email: String,
    password: String,
    email_confirm: bool,
    is_licenced: bool,
}
