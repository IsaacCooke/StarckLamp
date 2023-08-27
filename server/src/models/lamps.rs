use diesel::prelude::*;
use async_graphql::{types::ID, Object};

#[derive(Queryable)]
pub struct Lamp {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub red: i8,
    pub green: i8,
    pub blue: i8,
    pub is_on: bool,
    pub user_id: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive!(Insertable, AsChangeset)]
#[diesel(table_name = "lamps")]
pub struct NewLamp {
    pub name: String,
    pub red: i8,
    pub green: i8,
    pub blue: i8,
    pub is_on: bool,
}

#[Object]
impl Lamp {
    async fn id(&self) -> ID {
        ID::from(self.id)
    }
    async fn name(&self) -> &str {
        &self.name
    }
    async fn red(&self) -> i8 {
        self.red
    }
    async fn green(&self) -> i8 {
        self.green
    }
    async fn blue(&self) -> i8 {
        self.blue
    }
    async fn is_on(&self) -> bool {
        self.is_on
    }
    async fn user_id(&self) -> i32 {
        self.user_id
    }
    async fn created_at(&self) -> chrono::NaiveDateTime {
        self.created_at
    }
    async fn updated_at(&self) -> chrono::NaiveDateTime {
        self.updated_at
    }
}