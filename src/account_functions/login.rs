use std::io::Write;
extern crate magic_crypt;
use magic_crypt::{MagicCryptTrait, new_magic_crypt};
use postgres::{Client, Error, NoTls};
use crate::db_functions::check::{check_if_logged_in};
pub(crate) fn login() -> Result<(), Error>{
    // CHANGE ACCORDING TO YOUR POSTGRES USERNAME & DATABASE
    let mut client = Client::connect("postgresql://grimgram:grimgram@localhost/rust", NoTls)?;
    //check_if_logged_in();
    let mut username  = String::new();
    let mut email = String::new();
    let mut password = String::new();
    println!("Enter Username: ");
    std::io::stdin().read_line(&mut username).unwrap();
    println!("Enter Email: ");
    std::io::stdin().read_line(&mut email).unwrap();
    println!("Enter Password: ");
    std::io::stdin().read_line(&mut password).unwrap();
                                                                // must be the same as create_user key
    let encrypt = new_magic_crypt!("your_password_key");
    let encrypt_password = encrypt.encrypt_str_to_base64(password);

    // username: user
    // email: email@gmail.com
    //password: 12345678
    for row in client.query("SELECT username, password, email FROM users WHERE email=$1", &[&email])? {
        let username_match: &str = row.get(0);
        let password_match: &str = row.get(1);
        let email_match: &str = row.get(2);

        //println!("here: {}", password_match);
        //println!("original {}", encrypt_password);


        if password_match != encrypt_password {
            println!("Password does not match.");
            login();
        } else if password_match == encrypt_password {
            println!("✅");
        }

        if username_match != username {
            println!("Username does not match.");
            login();
        } else if username_match == username {
            println!("✅");
        }

        let en_user = new_magic_crypt!("your_user_key");
        let encrypt_username = en_user.encrypt_str_to_base64(username_match);
        let mut login_cache = std::fs::File::create(".login_cache").expect("create failed");
        login_cache.write_all(encrypt_username.as_bytes()).expect("write failed");
        login_cache.write_all("\n".as_bytes()).expect("write failed");
        login_cache.write_all(password_match.as_bytes()).expect("write failed");


    }



    Ok(())
}