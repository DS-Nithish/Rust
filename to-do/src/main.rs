/*!
**To-Do List CLI App**
  Manage tasks with options like `add`, `remove`, `list`.
  _Use enums and structs for statuses (`Pending`, `Done`)._/

  so  first

  to-do add to add egg --done

  so do we pass a string or

  SAVING

  How to save the file to i use the previous methode i used on data-store to add,get,delete the key-value pairs
  But this is usefull for simple tasks how can i use this in this programme where i have to index the string


  REMOVING


  So we want to put it in a array or a vec and index the number to remove the Question is how do we put the
  status do we put the status at the front or back well we want the status to be front how can i put it at front
  do i pass them as flags or do a something like

  As it is  a app do i make it interactive that is the way to go i think like this

  to-do list

  ⏳- TO the dishes
  ✅ - Clean the house

  So i have to make it go up and down
  So if i press something like Spacebar it should change
  I think first we can
*/

#[allow(unused_imports)]
use std::{
    error::Error,
    fs::{File, OpenOptions},
    io::{Read, Write},
};

use clap::{Args, Parser, Subcommand};
use std::collections::BTreeSet;

#[derive(Parser)]
#[command(
    version,
    name = "todo",
    about = "The intial to-do task app that i created at april 2025"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand)]
enum Commands {
    Task(TaskCommand),
    Status(Status),
}
#[derive(Subcommand)]
enum TaskSubCommand {
    Add(AddTask),

    Remove { remove: usize },
    List,
}
#[derive(Args)]
struct TaskCommand {
    #[command(subcommand)]
    subcommand: TaskSubCommand,
}
#[derive(Args)]
struct AddTask {
    description: String,
    #[arg(long, short)]
    status: Option<String>,
}
#[derive(Subcommand)]
enum StatusStatusSub {
    Done,
    Pending,
}
#[derive(Args)]
struct Status {
    id: usize,
    #[command(subcommand)]
    status: StatusStatusSub,
}

fn remove_duplicates(data: Vec<String>) -> Vec<String> {
    let set: BTreeSet<_> = data.into_iter().collect();
    set.into_iter().collect()
}
fn main() {
    let args = Cli::parse();

    let mut tasks = load_data().unwrap_or_else(|_| {
        println!("No existing data or error loading the data");
        Vec::new()
    });

    match args.command {
        Commands::Task(task) => match task.subcommand {
            TaskSubCommand::Add(add) => {
                tasks.push(add.description);
                println!("Added! Sucessfully");
            }
            TaskSubCommand::Remove { remove } => {
                tasks.remove(remove);
                println!("Removed! Successfully");
            }
            TaskSubCommand::List => {
                for i in &tasks {
                    println!("{}", i);
                }
            }
        },

        /*
        1. TO CHANGE IF STATUS EXIST REPLACE WITH THE NEW ONE
        so if i want to change the status of the task i have to put a algorithm
        that checks if the task already has a status if it has change the status to the
        one we want to change

        2. IF ACCIDENTALLY ENTERED THE STATUS WHICH IS ALREADY THE CURRENT STATUS
        i think the best way is to just to add a replace or return false
        */
        Commands::Status(status) => match status.status {
            StatusStatusSub::Pending => {
                // for string in &mut tasks {
                //     *string = string.replace("✅ - ", "⏳ - ");
                // }
                //we could also use replacerange
                if let Some(s) = tasks.get_mut(status.id) {
                    let word = "✅ - ".to_string();
                    let same = "⏳ - ".to_string();
                    if s.contains(&word) {
                        *s = s.replace("✅ - ", "⏳ - ");
                    } else if s.contains(&same) {
                        *s = s.replace("⏳ - ", "⏳ - ");
                    } else {
                        s.insert_str(0, "⏳ - ");
                    }
                }
                println!("{:#?}", tasks);
            }
            StatusStatusSub::Done => {
                if let Some(s) = tasks.get_mut(status.id) {
                    let word = "⏳ - ".to_string();
                    let same = "✅ - ".to_string();
                    if s.contains(&word) {
                        *s = s.replace("⏳ - ", "✅ - ");
                    } else if s.contains(&same) {
                        *s = s.replace("✅ - ", "✅ - ");
                    } else {
                        s.insert_str(0, "✅ - ");
                    }
                }
                println!("{:#?}", tasks);
            }
        },
    }
    let unique_data = remove_duplicates(tasks);
    // println!("{:?}", unique_data);

    saved_data(&unique_data).expect("Unable to save data");
}
fn load_data() -> Result<Vec<String>, Box<dyn Error>> {
    let filename = "data.json";

    let file = File::open(filename);
    if let Err(_) = file {
        return Ok(Vec::new());
    }

    let mut file = file?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    if data.is_empty() {
        return Ok(Vec::new());
    }

    let deserialized_data: Vec<String> = serde_json::from_str(&data)?;

    Ok(deserialized_data)
}

fn saved_data(data: &Vec<String>) -> Result<(), Box<dyn Error>> {
    let filename = "data.json";

    let serialed_data = serde_json::to_string(data)?;
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&filename)?;

    file.write_all(serialed_data.as_bytes())?;
    Ok(())
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}
