mod api;
mod schema;
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

fn main() {
    println!("Hello, world!");
}
