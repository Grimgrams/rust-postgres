use regex::Regex;
extern crate magic_crypt;
use magic_crypt::{MagicCryptTrait, new_magic_crypt};
use postgres::{Client, Error, NoTls};
use crate::db_functions::check::check_user_details;
extern crate dotenv;

use dotenv::dotenv;
use std::env;


pub(crate) fn create_user() -> Result<(), Error>{
    dotenv().ok();
    let pskey = "PSQLHOST";
    let psvalue= dotenv::var(pskey).unwrap();
    // CHANGE ACCORDING TO YOUR POSTGRES USERNAME & DATABASE
    let mut client = Client::connect(&*psvalue, NoTls)?;

    let mut username= String::new();
    let mut password = String::new();
    let mut email = String::new();
    let mut a_type = String::new();



        println!("Enter Username (min 4): ");
        std::io::stdin().read_line(&mut username).unwrap();
        println!("Enter Email (no caps): ");
        std::io::stdin().read_line(&mut email).unwrap();
        println!("Enter Password (min 8): ");
        std::io::stdin().read_line(&mut password).unwrap();
        a_type = "000".parse().unwrap();

        if username.chars().count() < 4 {
            println!("USERNAME I TOO SHORT (MIN 8)");
            create_user();
        }
        if username.chars().count() > 26 {
            println!("USERNAME IS TOO LONG (25 CHARS ONLY)");
            create_user();
        }

        if password.chars().count() > 126 {
            println!("PASSWORD IS TOO LONG (sorry)");
            create_user();
        }

        if password.chars().count() < 9 {
            println!("PASSWORD IS TOO SHORT");
            create_user();
        }

    //TODO fix email validation

    // Your key of choice (any)
    let mcrypt = new_magic_crypt!("your_password_key");
    let password_en = mcrypt.encrypt_str_to_base64(password);

    let email_regex =  Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").unwrap();
                                                // ^ this may be why your getting an invalid email, change [a-z0-9_+] -> [a-zA-Z0-9_+] to fix email check error (below)
    let mut email_check: String = email;
    let is_email_val = email_regex.is_match(&*email_check);
    if is_email_val == false {
        println!("INVALID EMAIL");
        create_user();
    } else { println!("checking database...") }

    // check function checks db for accounts with the same input
    // !!goes before insert query so no error gets executed
    check_user_details(&*username, &*email_check);

    client.execute(
        "INSERT INTO users (username, password, email, a_type) VALUES ($1, $2, $3, $4)",
        &[&username, &password_en, &email_check, &a_type],
    )?;

    println!("User registration successful!");

    Ok(())
}
