#![deny(warnings)]
#![feature(proc_macro_hygiene, decl_macro)]

/// PAY ATTENTION TO VERSIONS!!!!!

mod graphql_schema;

#[macro_use] extern crate rocket;

use juniper_rocket;
use juniper_warp;
use rocket::{response::content, State};
use warp::{Filter};
use juniper::{
    EmptySubscription, EmptyMutation,
};

fn schema() -> graphql_schema::Schema {
    graphql_schema::Schema::new(
        graphql_schema::Query,
        EmptyMutation::<()>::new(),
        EmptySubscription::<()>::new(),
    )
}

fn main() {
    let mut user_input = String::new();
    println!("Choose which server you'd like to run:\n[1] warp\n[2] Rocket");
    std::io::stdin().read_line(&mut user_input).expect("Failed");
    // Cast String input into i32 type
    let user_choice: i32 = user_input.trim().parse().expect("Failed to convert user input to i32");

    if 1 == user_choice {
        println!("Hello from warp server\nListening on http://127.0.0.1:3030. Please visit http://127.0.0.1:3030/graphiql to access the GraphQL playground.");
        let mut rt = tokio::runtime::Runtime::new().unwrap();
        let future = warp_server();
        rt.block_on(future);
    } else if 2 == user_choice {
        println!("Hello from Rocket server\nListening on http://127.0.0.1:8080. Please visit http://127.0.0.1:8080 to access the GraphQL playground.");
        rocket_server();
    } else {
        println!("Wrong input, please choose either [1,2]");
    }
    
}

async fn warp_server() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::get()
        .and(warp::path("hello").and(warp::path::param())
            .map(|name: String| format!("Hello, {}!", name)));

    let graphiql_path = warp::get()
        .and(warp::path("graphiql"))
        .and(juniper_warp::graphiql_filter("/graphql", None));

    let state = warp::any().map(move || () ).boxed(); // TODO: Do we need this?
    let graphql_filter = warp::path("graphql")
        .and(juniper_warp::make_graphql_filter(schema(), state));

    let routes = hello
        .or(graphiql_path)
        .or(graphql_filter);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

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

fn rocket_server() {
    rocket::ignite()
        .manage(graphql_schema::Schema::new(
            graphql_schema::Query,
            EmptyMutation::<()>::new(),
            EmptySubscription::<()>::new(),
        ))
        .mount("/", routes![graphiql,post_graphql_handler])
        .launch();
}