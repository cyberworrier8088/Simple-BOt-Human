use crate::colors;
use std::fs;
use std::io;
use std::io::Write;

pub fn config() {
    // This for user name
    let mut name = String::new();
    print!("Enter You are name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name).unwrap();

    // This for user Age
    let mut age = String::new();
    print!("Enter You are age: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut age).unwrap();

    // This for user gender
    let mut gender = String::new();
    print!("Enter you are gender: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut gender).unwrap();

    // gender lowercase do and trim
    let gender_enhnaced = gender.trim().to_lowercase();

    let gender = match gender_enhnaced.as_str() {
        "boy" | "male" | "man" | "m" => "Boy",             // boy
        "girl" | "female" | "woman" | "f" => "Girl",       // girl
        "nonbinary" | "non-binary" | "nb" => "Non-Binary", // non-binary
        "trans" | "transgender" => "Transy",               // transgender
        "other" | "custom" => "Non",                       // other
        _ => {
            println!(
                "{}Invalid gender! Defaulting to Other.{}",
                colors::RED,
                colors::RESET
            ); // Invalid gender
            "Other"
        }
    };

    // This for user location
    let mut location = String::new();
    print!("Enter your location/country: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut location).unwrap();

    // This for user hobby
    let mut hobby = String::new();
    print!("Enter your favorite hobby: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut hobby).unwrap();

    // This for user favorite color
    let mut fav_color = String::new();
    print!("Enter your favorite color: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut fav_color).unwrap();

    // Data formating
    let data = format! {
        "name={}\nage={}\ngender={}\nlocation={}\nhobby={}\nfav_color={}",
        name.trim(),
        age.trim(),
        gender.trim(),
        location.trim(),
        hobby.trim(),
        fav_color.trim(),
    };
    // checking error
    if let Err(e) = fs::write("config.txt", data) {
        println!("{}Error saving config: {}{}", colors::RED, e, colors::RESET);
        println!("{}Pls check: {}{}", colors::RED, e, colors::RESET);
    } else {
        println!(
            "\n{}✓ Configuration Saved Successfully!{}",
            colors::GREEN,
            colors::RESET
        );
    }
}
