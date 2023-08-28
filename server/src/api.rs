use async_graphql::{Context, Data, Object, Result, Schema, Subscription};
use futures_util::Stream;
use serde::Deserialize;
use crate::resolvers::users_resolver::*;
use crate::resolvers::lamps_resolver::*;
use crate::models::users::User;
use crate::models::lamps::Lamp;

pub type TokenSchema = Schema<QueryRoot, MutationRoot, SubscriptionRoot>;

pub struct Token(pub String);

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn current_token<'a>(&self, ctx: &'a Context<'_>) -> Option<&'a str> {
        ctx.data_opt::<Token>().map(|token| token.0.as_str())
    }
    async fn users(&self, limit: i64) -> Vec<User> {
        get_users(limit);
    }
    async fn user(&self, id: i32) -> User {
        get_user_by_id(id);
    }
    async fn lamps(&self, limit: i64) -> Vec<Lamp> {
        get_lamps(limit);
    }
    async fn lamp(&self, id: i32) -> Lamp {
        get_lamp_by_id(id);
    }
    async fn lamps_by_user(&self, user_id: i32) -> Vec<Lamp> {
        get_lamp_by_user(user_id);
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_user(&self, _ctx: &Context<'_>, username: String, email: String, password: String) -> User {
        Ok(add_user(username, email, password))
    }
    async fn update_username(&self, _ctx: &Context<'_>, id: i32, username: String) -> User {
        Ok(update_username(id, username))
    }
    async fn update_email(&self, _ctx: &Context<'_>, id: i32, email: String) -> User {
        Ok(update_email(id, email))
    }
    async fn update_password(&self, _ctx: &Context<'_>, id: i32, password: String) -> User {
        Ok(update_password(id, password))
    }
    async fn delete_user(&self, _ctx: &Context<'_>, id: i32) -> User {
        Ok(delete_user(id))
    }
    async fn create_lamp(&self, _ctx: &Context<'_>, user_id: i32, name: String, description: String) -> Lamp {
        Ok(add_lamp(name, description, user_id))
    }
    async fn set_lamp_color(&self, _ctx: &Context<'_>, id: i32, red: i16, green: i16, blue: i16) -> Lamp {
        Ok(set_color(id, red, green, blue))
    }
    async fn set_lamp_is_on(&self, _ctx: &Context<'_>, id: i32, is_on: bool) -> Lamp {
        Ok(toggle_on(id, is_on))
    }
    async fn update_lamp_name(&self, _ctx: &Context<'_>, id: i32, name: String) -> Lamp {
        Ok(update_name(id, name))
    }
    async fn update_lamp_description(&self, _ctx: &Context<'_>, id: i32, description: String) -> Lamp {
        Ok(update_description(id, description))
    }
    async fn delete_lamp(&self, _ctx: &Context<'_>, id: i32) -> Lamp {
        Ok(delete_lamp(id))
    }
}

pub struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
    async fn values(&self, ctx: &Context<'_>) -> Result<impl Stream<Item = i32>> {
        if ctx.data::<Token>()?.0 != "123456" {
            return Err("Forbidden".into());
        }
        Ok(futures_util::stream::once(async move { 10 }))
    }
}

pub async fn on_connection_init(value: serde_json::Value) -> Result<Data> {
    #[derive(Deserialize)]
    struct Payload {
        token: String,
    }

    // Validate the token
    if let Ok(payload) = serde_json::from_value::<Payload>(value) {
        let mut data = Data::default();
        data.insert(Token(payload.token));
        Ok(data)
    } else {
        Err("Unauthorized".into())
    }
}