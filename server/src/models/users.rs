use async_graphql::{types::ID, Object};
use diesel::prelude::*;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::users)]
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
}
