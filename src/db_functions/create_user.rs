use regex::Regex;
use postgres::{Client, Error, NoTls};
use crate::db_functions::check::check_user_details;



pub(crate) fn create_user() -> Result<(), Error>{
    let mut client = Client::connect("postgresql://grimgram:grimgram@localhost/rust", NoTls)?;

    let mut username= String::new();
    let mut password = String::new();
    let mut email = String::new();
    let mut a_type = String::new();



        println!("Enter Username (min 4): ");
        std::io::stdin().read_line(&mut username).unwrap();
        println!("Enter Email: ");
        std::io::stdin().read_line(&mut email).unwrap();
        println!("Enter Password (min 16): ");
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

        if password.chars().count() < 17 {
            println!("PASSWORD IS TOO SHORT");
            create_user();
        }

    //TODO fix email validation

        //let email_regex = Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.][a-z0-9]+)*\.[a-z]{2,6})").unwrap();

        //let check_email = email_regex.is_match(&*email);

    /*
        if check_email != true {
            println!("EMAIL IS INVALID");
            create_user();
        }
*/

    // check function checks db for accounts with the same input
    // !!goes before insert query so no error gets executed
    check_user_details(&*username, &*email);

    client.execute(
        "INSERT INTO users (username, password, email, a_type) VALUES ($1, $2, $3, $4)",
        &[&username, &password, &email, &a_type],
    )?;

    Ok(())
}