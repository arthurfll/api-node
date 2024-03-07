use std::env;

pub fn get_ip() -> String{
    let ip = env::var("IP").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("{}:{}",ip,port);
    addr
}
