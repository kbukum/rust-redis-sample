extern crate serde_json;
use std::string::String;
extern crate redis;

use self::redis::Commands;



use serde::{Serialize, Deserialize};

pub fn put_struct<T: Serialize>(key: &str, st: T) -> redis::RedisResult<T> {
    // Connect to a local Redis
    let client = try!(redis::Client::open("redis://127.0.0.1/"));
    let conn = try!(client.get_connection());
    // Encode our Job in JSON
    let serialized = serde_json::to_string(&st).unwrap();
    println!("\tSerialized : {}", &serialized);
    let _: () = try!(conn.set(key, &serialized));
    // println!("Struct: {:?}", st);
    Ok(st)
}


pub fn get_struct<T: Deserialize>(key: &str) -> redis::RedisResult<T> {
    // Connect to a local Redis
    let client = try!(redis::Client::open("redis://127.0.0.1/"));

    let conn = try!(client.get_connection());
    let serialized_struct: String = try!(conn.get(key));
    Ok(serde_json::from_str(&serialized_struct).unwrap())
}

