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
    let put_result = cache::put_struct(name.as_ref(), &example).unwrap();
    //Print the operation result.
    println!("{:?}",&put_result);

    // Get the result from redis.
    let get_result: ExampleStruct = cache::get_struct(name.as_ref()).unwrap();
    // Print the result.
    println!("{:?}",&get_result);
    assert_eq!(example.name, put_result.name, "Names must be same Original: {}, Redis(Put): {}", example.name, put_result.name);
    assert_eq!(example.name, get_result.name, "Names must be same Original: {}, Redis(Get): {}", example.name, get_result.name);
}



