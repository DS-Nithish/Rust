//A CLI app to `put`, `get`, and `delete` key-value pairs.
// _Use `HashMap`, optionally persist using `serde`._

/*
#get retrieving the key value pair key and value for detailed data
#and if you only want the key you can use key overall key is a must
Example: todo get Name

Name: Nithis
Name: Mugen

#put adding key value to the app must ne key and value if only one
#is present error
Example: todo put Age 19

*if sucessfully added print the message*
Added!

#delete deleting the key value pair options: deleting with key which is risky
#Or give the key value pair

Example: todo delete Age 19

Deleted Age:19


so if we want to add put get delete method we can use the if or match or both of them
to get the key-value pairs the question from where to i get and add this is it a file
i dont know maybe files are the best option but how to we know the file contains a hashmap

i can create a cli app which stores data and we can retrieve it like  a password getting or adding

how to add  the get_table function to add the value to the table hashmap now i only referencing to the new hashmap
I am not changing or adding anything i know how to insert but how to do it with another not inside fn main()


*/

use clap::Parser;
use std::{
    collections::{
        HashMap,
        hash_map::{Keys, Values},
    },
    io::{self},
};

// struct Method {
//     get: i32,
// }
//

fn main() {
    // let get = std::env::args().nth(3).expect("Argument failed");

    let map_one = function_one();
    let mut map_two = HashMap::new();
    let table_three = function_two(map_two, map_one);
    println!("{:?}", table_three);
}

fn function_two(
    mut map_two: HashMap<String, String>,
    map_one: HashMap<String, String>,
) -> HashMap<String, String> {
    // let mut table: HashMap<String, String> = HashMap::new();

    // table.insert("Name".to_string(), "Nithish".to_string());

    map_two.extend(map_one);

    println!("{:#?}", &map_two);

    map_two
}

fn function_one() -> HashMap<String, String> {
    let mut key = String::new();
    let mut value = String::new();

    io::stdin().read_line(&mut key).expect("Key not read");
    io::stdin().read_line(&mut value).expect("Key not read");
    let mut map_one = HashMap::new();
    map_one.insert(key.trim().to_string(), value.trim().to_string());
    map_one
}
