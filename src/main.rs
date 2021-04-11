#![deny(warnings)]
#![feature(proc_macro_hygiene, decl_macro)]

/// PAY ATTENTION TO VERSIONS!!!!!

mod graphql_schema;

// #[macro_use] extern crate rocket;

use juniper::{
    EmptySubscription, EmptyMutation,
  };
// use rocket::{response::content, State};
// use juniper_rocket;
use juniper_warp;

use warp::{Filter};

fn schema() -> graphql_schema::Schema {
    graphql_schema::Schema::new(
        graphql_schema::Query,
        EmptyMutation::<()>::new(),
        EmptySubscription::<()>::new(),
    )
}

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::get()
        .and(warp::path("hello").and(warp::path::param())
            .map(|name: String| format!("Hello, {}!", name)));

    let graphiql_path = warp::get()
        .and(warp::path("graphiql"))
        .and(juniper_warp::graphiql_filter("/graphql", None));

    let state = warp::any().map(move || () ).boxed();
    let graphql_filter = warp::path("graphql")
        .and(juniper_warp::make_graphql_filter(schema(), state));

    let routes = hello
        .or(graphiql_path)
        .or(graphql_filter);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

// #[rocket::get("/")]
// fn graphiql() -> content::Html<String> {
//     juniper_rocket::graphiql_source("/graphql", None)
// }

// #[rocket::post("/graphql", data = "<request>")]
// fn post_graphql_handler(
//     _context: State<graphql_schema::Schema>,
//     request: juniper_rocket::GraphQLRequest,
//     schema: State<graphql_schema::Schema>,
// ) -> juniper_rocket::GraphQLResponse {
//     request.execute_sync(&schema, &())
// }

// fn main() {
//     rocket::ignite()
//         .manage(graphql_schema::Schema::new(
//             graphql_schema::Query,
//             EmptyMutation::<()>::new(),
//             EmptySubscription::<()>::new(),
//         ))
//         .mount("/", routes![graphiql,post_graphql_handler])
//         .launch();
// }