use crate::schema::lamps;
use async_graphql::{types::ID, Object};
use diesel::prelude::*;

#[derive(Queryable)]
pub struct Lamp {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub red: i16,
    pub green: i16,
    pub blue: i16,
    pub is_on: bool,
    pub user_id: Option<i32>,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = lamps)]
pub struct NewLamp {
    pub name: String,
    pub description: String,
    pub red: i16,
    pub green: i16,
    pub blue: i16,
    pub is_on: bool,
    pub user_id: Option<i32>,
}

#[Object]
impl Lamp {
    async fn id(&self) -> ID {
        ID::from(self.id)
    }
    async fn name(&self) -> &str {
        &self.name
    }
    async fn description(&self) -> &str {
        &self.description
    }
    async fn red(&self) -> i16 {
        self.red
    }
    async fn green(&self) -> i16 {
        self.green
    }
    async fn blue(&self) -> i16 {
        self.blue
    }
    async fn is_on(&self) -> bool {
        self.is_on
    }
    async fn user_id(&self) -> Option<i32> {
        self.user_id
    }
}
