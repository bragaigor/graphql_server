#[warn(unused_variables)]
#[allow(dead_code)]
// use rocket::{response::content, State};
use juniper::{
  graphql_object, EmptySubscription, FieldResult, GraphQLEnum, 
  GraphQLInputObject, GraphQLObject, RootNode,
  EmptyMutation,
};
use std::collections::HashMap;

#[derive(Clone, GraphQLEnum)]
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

// TODO: Extend this implementation and create a Database struct where we can include and delete
//    elements from it. 

// Context: The context type is a feature in Juniper that lets field resolvers access global data,
// 		most commonly database connections or authentication information.

#[derive(Clone, GraphQLObject)]
// #[graphql(description = "A humanoid creature in the Star Wars universe")] this is the equivalent of
/// A humanoid creature in the Star Wars universe
pub struct SuperHero {
    id: String,
	/// SuperHero's name
    name: String,
    super_powers: Vec<Powers>,
    home_planet: String, // This field is converted to camelCase name homePlanet
}

impl SuperHero {
	fn new(
		id: &str,
		name: &str,
		super_powers: &[Powers],
		home_planet: &str
	) -> SuperHero {
		return Self {
			id: id.to_owned(),
			name: String::from(name), // Equivalent to "".to_owned()
			super_powers: super_powers.to_vec(),
			home_planet: home_planet.to_string() // Equivalent to String::from("")
		};
	}
}

#[derive(Clone, GraphQLObject)]
pub struct Weapon {
	id: String,
	name: String,
	power_level: i32,
	magicals: Vec<Powers>,
}

impl Weapon {
	fn new(
		id: &str,
		name: &str,
		power_level: i32,
		magicals: &[Powers]
	) -> Weapon {
		return Self {
			id: id.to_owned(),
			name: String::from(name), // Equivalent to "".to_owned()
			power_level: power_level,
			magicals: magicals.to_vec() // Equivalent to String::from("")
		};
	}
}

pub struct Database {
	super_heroes: HashMap<String, SuperHero>,
	weapons_list: HashMap<String, Weapon>,
}

impl Database {
	fn new() -> Database {
		let mut super_heroes = HashMap::new();
		let mut weapons_list = HashMap::new();

		super_heroes.insert(
			"Iron man".to_owned(),
			SuperHero::new(
				"11228899", 
				"Igor Braga", 
				&[Powers::Fire, Powers::Wind], 
				"Earth" )
		);

		weapons_list.insert(
			"Excalibur".to_owned(),
			Weapon::new(
				"6666666", 
				"Excalibur", 
				989, 
				&[Powers::SuperStrength, Powers::Invisibility] )
		);

		return Database {super_heroes, weapons_list};
	}
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


pub fn simple_test() {
	let my_database = Database::new();

	// let super_hero_name: String = my_database.super_heroes[&"Iron Man"].name.clone();
	// let weapon_name: String = my_database.weapons_list["Excalibur"].name.clone();

	// How to iterate over a hashmap
	for (key, value) in &my_database.super_heroes {
        println!("{} / {}", key, value.name);
    }

	// How to fetch item from hash table safely
	// Is this faster then the following????
	// This version doesn't deal with if a key is not found in the hashmap
	// let clone_super_heroe2: SuperHero = my_database.super_heroes["Iron man"].clone();
	// println!("clone super hero name: {}", clone_super_heroe2.name);
	// let _option_super_heroe: Option<&SuperHero> = my_database.super_heroes.get("Iron Man");
	// or
	match my_database.super_heroes.get(&"Iron man".to_string()) {
		Some(super_hero) => { 
			match my_database.weapons_list.get(&"Excalibur".to_string()) {
				Some(the_weapon) => {
					println!("{} and weapon {}", super_hero.name, the_weapon.name); 
				},
				None => { println!("No weapons found with thta name"); }
			}
		},
		None => { println!("No superheroes found with that name"); }
	}
}
