#![feature(decl_macro, proc_macro_hygiene)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate uuid;
extern crate log;
extern crate fern;

extern crate chrono;

mod services;
mod adapters;
use crate::adapters::graphql::{RootContext, Schema, QueryRoot, MutationRoot};
use rocket::{response::content, State};
use juniper::{
    EmptySubscription
};
use dotenv::dotenv;
use std::env;

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}

#[rocket::get("/")]
pub fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql", None)
}

#[rocket::get("/graphql?<request>")]
pub fn get_graphql_handler(
    context: State<RootContext>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute_sync(&schema, &context)
}

#[rocket::post("/graphql", data = "<request>")]
pub fn post_graphql_handler(
    context: State<RootContext>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute_sync(&schema, &context)
}

fn main() {
    dotenv().ok();

    setup_logger().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    rocket::ignite()
        .manage(RootContext::new(&database_url))
        .manage(Schema::new(
            QueryRoot,
            MutationRoot,
            EmptySubscription::<RootContext>::new(),
        ))
        .mount(
            "/",
            rocket::routes![graphiql, get_graphql_handler, post_graphql_handler],
        )
        .launch();
}