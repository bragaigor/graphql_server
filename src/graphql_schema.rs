// use rocket::{response::content, State};
use juniper::{
  graphql_object, EmptySubscription, FieldResult, GraphQLEnum, 
  GraphQLInputObject, GraphQLObject, RootNode,
  EmptyMutation,
};
// use std::collections::HashMap;

#[derive(GraphQLEnum)]
enum Powers {
    Fly,
    SuperStrength,
    Invisibility,
	Magic,
	Telepaty,
	Luck,
	Fire,
	Ice,
	Wind
}

// struct Database {
// 	super_heroes: HashMap<String, SuperHero>
// }
// TODO: Extend this implementation and create a Database struct where we can include and delete
//    elements from it. 

// Context: The context type is a feature in Juniper that lets field resolvers access global data,
// 		most commonly database connections or authentication information.

#[derive(GraphQLObject)]
// #[graphql(description = "A humanoid creature in the Star Wars universe")] this is the equivalent of
/// A humanoid creature in the Star Wars universe
struct SuperHero {
    id: String,
	/// SuperHero's name
    name: String,
    super_powers: Vec<Powers>,
    home_planet: String, // This field is converted to camelCase name homePlanet
}

#[derive(GraphQLObject)]
struct Weapon {
	id: String,
	name: String,
	power_level: i32,
	magicals: Vec<Powers>
}

// TODO: What is this? This is probably used to insert new elements into the Database
#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct NewSuperHero {
    name: String,
    super_powers: Vec<Powers>,
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
  fn super_hero(_id: String) -> FieldResult<SuperHero> {
    println!("SuperHero resolver was called with id: {}!!", _id);
    let my_vec: Vec<Powers> = vec![Powers::SuperStrength, Powers::Invisibility];
    let super_hero = SuperHero{
        id: String::from("11228899"), 
        name: String::from("Igor Braga"), 
        super_powers: my_vec, 
        home_planet: String::from("Earth")};
    // Return the result.
    return Ok(super_hero);
  }

  fn random_weapon() -> FieldResult<Weapon> {
	let my_vec: Vec<Powers> = vec![Powers::Fire, Powers::Wind];
	let weapon = Weapon {
		id: String::from("6666666"), 
        name: String::from("Excalibur"), 
        power_level: 989, 
        magicals: my_vec
		};
	return Ok(weapon);
  }
}

pub type Schema = RootNode<'static, Query, EmptyMutation<()>, EmptySubscription<()>>;