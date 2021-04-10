#![deny(warnings)]
#![feature(proc_macro_hygiene, decl_macro)]

mod graphql_schema;

#[macro_use] extern crate rocket;

use juniper::{
    EmptySubscription, EmptyMutation,
  };
use rocket::{response::content, State};
use juniper_rocket;

// #[get("/")]
// fn index() -> &'static str {
//     "Hello, world!"
// }
#[rocket::get("/")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql", None)
}

#[rocket::post("/graphql", data = "<request>")]
fn post_graphql_handler(
    _context: State<graphql_schema::Schema>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<graphql_schema::Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute_sync(&schema, &())
}

fn main() {
    rocket::ignite()
        .manage(graphql_schema::Schema::new(
            graphql_schema::Query,
            EmptyMutation::<()>::new(),
            EmptySubscription::<()>::new(),
        ))
        .mount("/", routes![graphiql,post_graphql_handler])
        .launch();
}