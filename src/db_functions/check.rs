use std::process::exit;
use postgres::{Client, Error, NoTls};

pub(crate) fn check_user_details(username: &str, email: &str) -> Result<(), Error>{
    /// CHANGE ACCORDING TO YOUR POSTGRES USERNAME & DATABASE
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

//pub(crate) fn check_