mod db_functions;
use postgres::{Client, Error, NoTls};
use crate::db_functions::check::check;



fn main() -> Result<(), Error> {
    let mut client = Client::connect("postgresql://grimgram:grimgram@localhost/rust", NoTls)?;

    let mut username= String::new();
    let mut password = String::new();
    let mut email = String::new();
    let mut a_type = String::new();
    //username = "Bobby201";
    //password = "SuperNotHashedPassword2";
    //email = "bobbyhill244@gmail.com";
    //a_type = "000";
        println!("Enter Username: ");
        std::io::stdin().read_line(&mut username).unwrap();
        println!("Enter Email: ");
        std::io::stdin().read_line(&mut email).unwrap();
        println!("Enter Password: ");
        std::io::stdin().read_line(&mut password).unwrap();
        a_type = "000".parse().unwrap();

        // check function checks db for accounts with the same input
        // !!goes before insert query so no error get executed
        check(&*username, &*email);

    client.execute(
        "INSERT INTO users (username, password, email, a_type) VALUES ($1, $2, $3, $4)",
       &[&username, &password, &email, &a_type],
    )?;



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
