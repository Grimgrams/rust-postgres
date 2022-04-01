mod db_functions;
use postgres::{Client, Error, NoTls};
use crate::db_functions::check::check_user_details;



fn main() -> Result<(), Error> {
    /// CHANGE ACCORDING TO YOUR POSTGRES USERNAME & DATABASE
    let mut client = Client::connect("postgresql://grimgram:grimgram@localhost/rust", NoTls)?;

    // Returns all users in database (it can be deleted)
    for row in client.query("SELECT id, username, password, email, a_type FROM users", &[])? {
        let id: i32 = row.get(0);
        let username: &str = row.get(1);
        let password: &str = row.get(2);
        let email: &str = row.get(3);
        let a_type: &str = row.get(4);

        println!("found user: {}, {}, {}, {}, {}", id, username, password, email, a_type);

    }


    Ok(())
}
