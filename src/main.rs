mod processes;
mod state;
mod to_do;

use processes::process_input;
use serde_json::value::Value;
use serde_json::Map;
use state::read_file;
use std::env;
use to_do::todo_factory;

// use to_do::structs::done::Done;
// use to_do::structs::pending::Pending;
// use to_do::structs::traits::create::Create;
// use to_do::todo_factory;
// use to_do::ItemTypes;

fn main() {
    // let todo_item: Result<ItemTypes, &'static str> = todo_factory("pending", "make");

    // match todo_item.unwrap() {
    //     ItemTypes::Pending(item) => item.create(&item.super_struct.title),
    //     ItemTypes::Done(item) => println!(
    //         "It's a done item with the title: {}",
    //         item.super_struct.title
    //     ),
    // }
    // let args: Vec<String> = env::args().collect();
    // let status: &String = &args[1];
    // let title: &String = &args[2];

    // let mut state: Map<String, Value> = read_file(&String::from("./state.json"));

    // state.insert(title.to_string(), json!(status));
    // write_to_file("./state.json", &mut state);

    // println!("{:?}", state);

    let args: Vec<String> = env::args().collect();

    let command: &String = &args[1];
    let title: &String = &args[2];

    let state: Map<String, Value> = read_file(&"./state.json".to_string());

    let mut status: String;

    match &state.get(*&title) {
        Some(result) => {
            println!("result.to_string(): {}", result);
            status = result.to_string().replace('\"', "");
        }
        None => status = String::from("pending"),
    }

    let item = todo_factory(&status, title).expect(&status);

    process_input(item, command.to_string(), &state);
}
