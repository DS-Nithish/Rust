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
#[allow(unused_imports)]
use clap::{Args, Parser, Subcommand};
use serde_json;
#[allow(unused_assignments, unused_variables)]
use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{Read, Write},
};
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    method: Method,
}
#[derive(Subcommand)]
enum Method {
    Put { key: String, value: String },
    Get { key: String },
    Delete { key: String },
}

fn main() {
    let args = Cli::parse();

    let mut table = load_data().unwrap_or_else(|_| {
        println!("No existing data found or error reading file. Starting with empty table.");
        HashMap::new()
    });

    match args.method {
        Method::Put { key, value } => {
            table.insert(key, value);
        }
        Method::Get { key } => {
            match table.get(&key) {
                Some(value) => println!("{}:{}", key, value),
                None => println!("NO VALUE FOUND FOR {}", key),
            };
        }
        Method::Delete { key } => {
            match table.remove(&key) {
                Some(value) => println!("DELETED  {}:{}", key, value),
                None => println!("Nothing matches {key}"),
            };
        }
    }
    save_data(&table).expect("");
}

fn load_data() -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let filename = "my_data.json";

    let file = File::open(filename);

    if let Err(_) = file {
        return Ok(HashMap::new());
    }
    let mut file = file?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    if data.is_empty() {
        return Ok(HashMap::new());
    }

    let deserialized_data: HashMap<String, String> = serde_json::from_str(&data)?;
    Ok(deserialized_data)
}

fn save_data(table: &HashMap<String, String>) -> Result<(), Box<dyn std::error::Error>> {
    let filename = "my_data.json";

    let serialized_data = serde_json::to_string(table)?;
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(filename)?;

    file.write_all(serialized_data.as_bytes())?;
    Ok(())
}
