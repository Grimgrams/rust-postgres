extern crate magic_crypt;
use magic_crypt::{MagicCryptTrait, new_magic_crypt};
use std::process::exit;
use std::io::Write;
use postgres::{Client, Error, NoTls};

pub(crate) fn check_user_details(username: &str, email: &str) -> Result<(), Error>{
    // CHANGE ACCORDING TO YOUR POSTGRES USERNAME & DATABASE
    let mut client = Client::connect("postgresql://grimgram:grimgram@localhost/rust", NoTls)?;

    for row in client.query("SELECT * FROM users WHERE username=$1", &[&username])? {
        let username_match: &str = row.get(1);

        if username_match == username {
            println!("USERNAME {} IS TAKEN, PLEASE CHOOSE ANOTHER ONE.", username);
            exit(1);
        }

    }

    for row in client.query("SELECT * FROM users WHERE email=$1", &[&email])? {
        let email_match: &str = row.get(3);

        if email_match == email {
            println!("EMAIL {} IS TAKEN, PLEASE CHOOSE ANOTHER ONE.", email);
            exit(1);
        }
    }
    Ok(())
}
use std::io::Read;
pub(crate) fn check_if_logged_in(){
    let mut  is_logged_in_file_check = std::fs::File::open(".login_cache").expect("FILE NOT FOUND, YOU MUST LOGIN AGAIN!");
    let mut contents = String::new();
    is_logged_in_file_check.read_to_string(&mut contents).unwrap();
}

/*
pub(crate) fn check_login(username: &str, email: &str, password: &str) -> Result<(), Error>{

    // these values are for when the program makes recursion (line 43, 48 53)
    let username_store: &str = username;
    let email_store: &str = email;
    let password_store: &str = password; // TODO update when password encryption is implemented
    /// CHANGE ACCORDING TO YOUR POSTGRES USERNAME & DATABASE
    let mut client = Client::connect("postgresql://grimgram:grimgram@localhost/rust", NoTls)?;

    for row in client.query("SELECT username, password, email FROM users WHERE email=$1", &[&email])? {
        let username_match: &str = row.get(1);
        let email_match: &str = row.get(2);
        let password_match: &str = row.get(3);

        if username_match != username {
            println!("Username does not match.");
            check_login(username_store, email_store, password_store);
        }

        if email_match != email {
            println!("Email does not match.");
            check_login(username_store, email_store, password_store);
        }

        if password_match != password {
            println!("Password does not match.");
            check_login(username_store, email_store, password_store);
        }
    }

    Ok(())
}



pub(crate) fn check_if_logged_in(username: &str, password: &str){
                         // Your key of choice (any)
    let mcrypt = new_magic_crypt!("your_key", 256);
    let user_en = mcrypt.encrypt_str_to_base64(username);
    let password_en = mcrypt.encrypt_str_to_base64(password);
    let mut login_cache = std::fs::File::create(".login_cache").expect("create failed");

    login_cache.write_all(user_en.as_bytes()).expect("write failed");
    login_cache.write_all("\n".as_bytes()).expect("write failed");
    login_cache.write_all(password_en.as_bytes()).expect("write failed");
    println!("Cache created successfully");
}

 */