use std::path::Path; // path input output
use std::{thread, time::Duration}; // this for sleeping

mod chat;
mod config;
mod colors;
mod clear; // This for clear function
mod user_variable;

fn main() {
    let config_path = "config.txt";

    if !Path::new(config_path).exists() { 
        println!("First setup?"); // setup
        config::config(); // This calling config function
         // printinng the escape codes and flush stdout to ensure it happens immediately

        clear::clear();
        println!("Terminal cleared!");
        thread::sleep(Duration::from_secs(1));

        clear::clear();
        println!("Ready!");
        thread::sleep(Duration::from_secs(2));

        chat::chat();
    } else {
        clear::clear();

        println!("Welcome Back!");

        clear::clear();


        chat::chat();
    }
}
