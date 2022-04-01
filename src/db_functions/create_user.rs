use regex::Regex;
use postgres::{Client, Error, NoTls};
use crate::db_functions::check::check_user_details;

pub(crate) fn create_user() -> Result<(), Error>{
    let mut client = Client::connect("postgresql://grimgram:grimgram@localhost/rust", NoTls)?;

    let mut username= String::new();
    let mut password = String::new();
    let mut email = String::new();
    let mut a_type = String::new();

    fn create(){
        println!("Enter Username: ");
        std::io::stdin().read_line(&mut username).unwrap();
        println!("Enter Email: ");
        std::io::stdin().read_line(&mut email).unwrap();
        println!("Enter Password: ");
        std::io::stdin().read_line(&mut password).unwrap();
        a_type = "000".parse().unwrap();

        if username.count() < 26 {
            println!("USERNAME IS TOO LONG (25 CHARS ONLY)");
            create();
        }

        let pwdre = Regex::new(r"");

    }


    // check function checks db for accounts with the same input
    // !!goes before insert query so no error gets executed
    check_user_details(&*username, &*email);

    client.execute(
        "INSERT INTO users (username, password, email, a_type) VALUES ($1, $2, $3, $4)",
        &[&username, &password, &email, &a_type],
    )?;

    Ok(())
}