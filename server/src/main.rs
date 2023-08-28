mod api;
mod schema;
mod server;
mod data {
    pub mod postgresql;
}
mod models {
    pub mod users;
    pub mod lamps;
}
mod resolvers {
    pub mod users_resolver;
    pub mod lamps_resolver;
}

use server::run_server;

#[actix_web::main]
fn main() -> std::io::Result<()> {
    run_server()
}
