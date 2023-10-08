use bson::{doc, Document};
use futures::TryStreamExt;
use mongodb::{error::Error, results::{InsertOneResult, UpdateResult}, Collection};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub user_name: String,
    pub password: String,
    pub email: String,
}

pub struct UserService {
    collection: Collection<User>
}

fn _user_to_document(user: &User) -> Document {
    let User {
        first_name,
        last_name,
        user_name,
        password,
        email,
    } = user;
    doc! {
        "first_name": first_name,
        "last_name": last_name,
        "user_name": user_name,
        "password": password,
        "email": email,
    }
}

impl UserService {
    pub fn new(collection: Collection<User>) -> UserService {
        UserService { collection }
    }

    pub async fn create(&self, user: &User) -> Result<InsertOneResult, Error> {
        self.collection.insert_one(user, None).await
    }

    pub async fn update(&self, user: &User) -> Result<UpdateResult, Error> {
        let User {
            first_name: _first_name,
            last_name: _last_name,
            user_name: _user_name,
            password: _password,
            email,
        } = user;

        self.collection.update_one(doc! { "email": email }, doc! { "$set": _user_to_document(user) }, None).await
    }

    pub async fn delete(&self, email: &String) {
        let _ = self.collection.delete_one(doc!{ "email": email }, None).await;
    }

    pub async fn get_user_email(&self, email: &String) -> Option<User> {
        let user = self.collection.find_one(doc!{ "email": email}, None).await.expect("Error to get user");
        return user
    }

    pub async fn get(&self) -> Result<Vec<User>, Error> {
        let mut cursor = self.collection.find(None, None).await.expect("Error to get all users");
        let mut data: Vec<User> = Vec::new();

        while let Some(result) = cursor.try_next().await.expect("Error retrieving document from cursor") {
            data.push(result);
        }

        Ok(data)
    }

}