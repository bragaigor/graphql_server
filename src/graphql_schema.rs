// use rocket::{response::content, State};
use juniper::{
  graphql_object, EmptySubscription, FieldResult, GraphQLEnum, 
  GraphQLInputObject, GraphQLObject, RootNode,
  EmptyMutation,
};

#[derive(GraphQLEnum)]
enum Episode {
    NewHope,
    Empire,
    Jedi,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct Human {
    id: String,
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
}

// There is also a custom derive for mapping GraphQL input objects.

#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct NewHuman {
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
}

// To make our context usable by Juniper, we have to implement a marker trait.
// impl juniper::Context for dyn Context {}

pub struct Query;

#[graphql_object()]
impl Query {
  fn apiVersion() -> &str {
      "1.0"
  }

  // Arguments to resolvers can either be simple types or input objects.
  // To gain access to the context, we specify a argument
  // that is a reference to the Context type.
  // Juniper automatically injects the correct context here.
  fn human(_id: String) -> FieldResult<Human> {
    println!("human resolver was called with id: {}!!", _id);
    let my_vec: Vec<Episode> = vec![Episode::Empire, Episode::Jedi];
    let human = Human{
        id: String::from("11228899"), 
        name: String::from("Igor Braga"), 
        appears_in: my_vec, 
        home_planet: String::from("Janut")};
    // Return the result.
    return Ok(human);
  }
}

pub type Schema = RootNode<'static, Query, EmptyMutation<()>, EmptySubscription<()>>;