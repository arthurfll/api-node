use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{Error ,Surreal};

use crate::models::user_model::User;

#[derive(Clone)]
pub struct Database {
    pub client: Surreal<Client>,
    pub name_space: String,
    pub db_name: String,
}

impl Database {
    pub async fn init() -> Result<Self,Error> {
        let client = Surreal::new::<Ws>("127.0.0.1:8000").await?;
        client.signin(Root {
            username: "root",
            password: "root"
        })
        .await?;
        client.use_ns("surreal").use_db("database").await.unwrap();
        Ok(Database {
            client,
            name_space: String::from("surreal"),
            db_name: String::from("database")
        })
    }

    pub async fn get_all_users(&self) -> Option<Vec<User>> {
        let result: Result<Vec<_>, Error>  = self.client.select("user").await;
        match result {
            Ok(all_users) => Some(all_users),
            Err(_) => None,
        }
    }

    pub async fn db_cadastro(&self,new_user: User) -> Option<User> {
        let created_user = self
            .client
            .create(("user",new_user.uuid.clone()))
            .content(new_user)
            .await;

        match created_user {
            Ok(created) => created,
            Err(_) => None,
        }
    }
}

