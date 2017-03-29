extern crate serde;
extern crate serde_json;
extern crate redis;
use std::string::String;
mod cache;


#[doc(hidden)]
#[macro_use]
extern crate serde_derive; // we have to define it here because macros must be at root 
//These are for auto ser,de founction generation.
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
/// A sample struct.
pub struct ExampleStruct {
     name: String
}


fn main() {
    println!("Hello, world!");
    // Create a Struct
    let example: ExampleStruct = ExampleStruct { name: "Kamil Bukum".to_string()};
    let name = "example".to_string();
    cache::put_struct(name.as_ref(), example);
}



