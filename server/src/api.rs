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
    async fn update_email(&self, _ctx: &Context<'_>, id: i32, username: String, email: String) -> User {
        Ok(update_email(id, username, email))
    }
    async fn update_password(&self, _ctx: &Context<'_>, id: i32, username: String, password: String) -> User {
        Ok(update_password(id, username, password))
    }
    async fn delete_user(&self, _ctx: &Context<'_>, id: i32) -> User {
        Ok(delete_user(id))
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