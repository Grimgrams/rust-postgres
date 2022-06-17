extern crate magic_crypt;
extern crate dotenv;
use magic_crypt::{MagicCryptTrait, new_magic_crypt};
use postgres::{Client, Error, NoTls};
use dotenv::dotenv;
use std::env;

pub(crate) fn update_username() -> Result<(), Error>{
    dotenv().ok();
    let pskey = "PSQLHOST";
    let psvalue= dotenv::var(pskey).unwrap();
    // CHANGE ACCORDING TO YOUR POSTGRES USERNAME & DATABASE
    let mut client = Client::connect(&*psvalue, NoTls)?;

    let mut email = String::new();
    let mut old_username = String::new();
    let mut password = String::new();
    let mut new_username= String::new();


    println!("Enter Email: ");
    std::io::stdin().read_line(&mut email).unwrap();
    println!("Enter Old Username: ");
    std::io::stdin().read_line(&mut old_username).unwrap();
    println!("Enter Password: ");
    std::io::stdin().read_line(&mut password);
    println!("Enter *NEW* Username (min 4): ");
    std::io::stdin().read_line(&mut new_username).unwrap();

    let mcrypt = new_magic_crypt!("your_password_key");
    let password_en = mcrypt.encrypt_str_to_base64(password);

    for row in client.query("SELECT username, password FROM users WHERE email=$1", &[&email])? {
        let username_match: &str = row.get(0);
        let password_match: &str = row.get(1);

        if username_match != old_username {
            println!("Username DOES NOT MATCH\n============");
            update_username();
        }
        if password_match != password_en {
            println!("Passwords DO NOT MATCH!\n===========");
            update_username();
        }

        if username_match == new_username {
            println!("NEW USERNAME CANNOT BE YOUR OLD USERNAME!\n============");
            update_username();
        }

    }

    check_if_username_is_taken(&new_username);




    client.execute("UPDATE users SET username=$1 WHERE email=$2", &[&new_username, &email])?;

    Ok(())
}

fn check_if_username_is_taken(username: &str) -> Result<(), Error> {
    // CHANGE ACCORDING TO YOUR POSTGRES USERNAME & DATABASE
    let mut client = Client::connect("postgresql://grimgram:grimgram@localhost/rust", NoTls)?;
    for row in client.query("SELECT * FROM users WHERE username=$1", &[&username])? {
        let username_match: &str = row.get(1);

        if username_match == username {
            println!("USERNAME {} IS TAKEN, PLEASE CHOOSE ANOTHER ONE.", username);
            update_username();
        }

    }
    Ok(())
}




pub(crate) fn update_email() -> Result<(), Error>{
    // CHANGE ACCORDING TO YOUR POSTGRES USERNAME & DATABASE
    let mut client = Client::connect("postgresql://grimgram:grimgram@localhost/rust", NoTls)?;

    let mut old_email = String::new();
    let mut password = String::new();
    let mut new_email= String::new();


    println!("Enter Old Email: ");
    std::io::stdin().read_line(&mut old_email).unwrap();
    println!("Enter Password: ");
    std::io::stdin().read_line(&mut password);
    println!("Enter *NEW* Email: ");
    std::io::stdin().read_line(&mut new_email).unwrap();

    let mcrypt = new_magic_crypt!("your_password_key");
    let password_en = mcrypt.encrypt_str_to_base64(password);

    for row in client.query("SELECT email, password FROM users WHERE email=$1", &[&old_email])? {
        let email_match: &str = row.get(0);
        let password_match: &str = row.get(1);

        if email_match != old_email {
            println!("Email DOES NOT MATCH\n============");
            update_email();
        }
        if password_match != password_en {
            println!("Passwords DO NOT MATCH!\n===========");
            update_email();
        }


        if email_match == new_email {
            println!("NEW EMAIL CANNOT BE YOUR OLD EMAIL!\n============");
            update_email();
        }

    }


    check_if_email_is_taken(&new_email);




    client.execute("UPDATE users SET email=$1 WHERE email=$2", &[&new_email, &old_email])?;

    Ok(())
}


fn check_if_email_is_taken(email: &str) -> Result<(), Error>{
    // CHANGE ACCORDING TO YOUR POSTGRES USERNAME & DATABASE
    let mut client = Client::connect("postgresql://grimgram:grimgram@localhost/rust", NoTls)?;
    for row in client.query("SELECT email FROM users WHERE email=$1", &[&email])? {
        let email_match: &str = row.get(0);

        if email_match == email {
            println!("EMAIL {} IS TAKEN, PLEASE CHOOSE ANOTHER ONE.", email);
            update_email();
        }

    }
    Ok(())
}

pub(crate) fn update_password() -> Result<(), Error>{
    // CHANGE ACCORDING TO YOUR POSTGRES USERNAME & DATABASE
    let mut client = Client::connect("postgresql://grimgram:grimgram@localhost/rust", NoTls)?;

    let mut email = String::new();
    let mut username = String::new();
    let mut old_password = String::new();
    let mut new_password= String::new();


    println!("Enter Email: ");
    std::io::stdin().read_line(&mut email).unwrap();
    println!("Enter Username: ");
    std::io::stdin().read_line(&mut username).unwrap();
    println!("Enter Old Password: ");
    std::io::stdin().read_line(&mut old_password);
    println!("Enter *NEW* Password (min 8): ");
    std::io::stdin().read_line(&mut new_password).unwrap();

    if new_password.chars().count() > 126 {
        println!("PASSWORD IS TOO LONG (sorry)");
        update_password();
    }

    if new_password.chars().count() < 9 {
        println!("PASSWORD IS TOO SHORT");
       update_password();
    }

    let mcrypt = new_magic_crypt!("your_password_key");
    let password_en = mcrypt.encrypt_str_to_base64(old_password);

    for row in client.query("SELECT username, password, email FROM users WHERE email=$1", &[&email])? {
        let username_match: &str = row.get(0);
        let password_match: &str = row.get(1);
        let email_match: &str = row.get(2);

        if email_match != email {
            println!("Email DOES NOT MATCH\n============");
            update_password();
        }
        if username_match != username {
            println!("Username DOES NOT MATCH\n============");
            update_password();
        }
        if password_match != password_en {
            println!("Passwords DO NOT MATCH!\n===========");
            update_password();
        }

        if password_match == new_password {
            println!("OLD PASSWORD CANNOT BE YOUR NEW PASSWORD!\n============");
            update_password();
        }



    }

    let new_en_password = mcrypt.encrypt_str_to_base64(new_password);


    client.execute("UPDATE users SET password=$1 WHERE email=$2", &[&new_en_password, &email])?;

    Ok(())
}

