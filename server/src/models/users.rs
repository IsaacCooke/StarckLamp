use diesel::prelude::*;
use async_graphql::{types::ID, Object};

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive!(Insertable, AsChangeset)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[Object]
impl User {
    async fn id(&self) -> ID {
        ID::from(self.id)
    }
    async fn username(&self) -> &str {
        &self.username
    }
    async fn email(&self) -> &str {
        &self.email
    }
    async fn created_at(&self) -> chrono::NaiveDateTime {
        self.created_at
    }
}