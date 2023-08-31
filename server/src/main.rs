mod api;
mod schema;
mod server;
mod data {
    pub mod postgresql;
}
mod models {
    pub mod lamps;
    pub mod users;
}
mod resolvers {
    pub mod lamps_resolver;
    pub mod users_resolver;
}

use server::run_server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    run_server().await
}
