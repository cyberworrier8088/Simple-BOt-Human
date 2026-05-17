use crate::user_variable::load_config;
use std::io;
use std::io::Write;

// Chat function
pub fn chat() {
    let user_config = load_config();

    println!("###########################################");
    println!("#               WELCOME BACK              #");
    println!("###########################################");
    println!("--- Session Info ---");
    println!("[+] Name:           {}", user_config.name);
    println!("[+] Age:            {}", user_config.age);
    println!("[+] Gender:         {}", user_config.gender);
    println!("[+] Location:       {}", user_config.location);
    println!("[+] Hobby:          {}", user_config.hobby);
    println!("[+] Favorite Color: {}", user_config.fav_color);
    println!("--------------------");

    loop {
        let mut chat = String::new();
        print!("Chat: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut chat).unwrap();

        match chat.trim().to_lowercase().as_str() {
            "hello" | "hi" | "hey" => {
                println!(
                    "Hello my dear {}! How can I help you today?",
                    user_config.name
                );
            }
            "how are you" => {
                println!(
                    "I am doing great, thank you for asking! How are you doing, {}?",
                    user_config.name
                );
            }
            "what is your name" | "who are you" => {
                println!(
                    "I am your friendly ur assistant,  for you, {}!",
                    user_config.name
                );
            }
            "where am i from" | "where do i live" | "my location" | "location" => {
                println!(
                    "ur currently based in the beautiful place: {}!",
                    user_config.location
                );
            }
            "what is my hobby" | "hobby" | "my hobby" => {
                println!(
                    "ur favorite hobby is {}, which sounds incredibly fun and creative!",
                    user_config.hobby
                );
            }
            "what is my favorite color" | "color" | "my color" | "favorite color" => {
                println!(
                    "ur favorite color is {}! It's a gorgeous and vibrant choice.",
                    user_config.fav_color
                );
            }
            "how old am i" | "my age" | "age" | "am i old" => {
                println!(
                    "ur are {} years young, {}! Age is just a number.",
                    user_config.age, user_config.name
                );
            }
            "tell me a joke" | "joke" => {
                println!("Why do programmers wear glasses? Because they can't C#! 😁");
            }
            "bye" | "exit" | "quit" => {
                println!(
                    "Goodbye, {}! Have a wonderful and productive day!",
                    user_config.name
                );
                break;
            }
            "help" => {
                println!("You can ask me questions like:");
                println!(" - hello / hi / hey");
                println!(" - how are you");
                println!(" - who are you / what is your name");
                println!(" - where do i live / my location");
                println!(" - what is my hobby / hobby");
                println!(" - what is my favorite color / color");
                println!(" - how old am i / age");
                println!(" - tell me a joke / joke");
                println!(" - bye / exit / quit");
            }
            _ => {
                println!(
                    "That's interesting! Let's talk more about that, or ask me for 'help' to see what I can do."
                );
            }
        }
    }
}
