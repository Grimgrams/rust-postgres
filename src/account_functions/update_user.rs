use postgres::{Client, Error, NoTls};

fn update_username() -> Result<(), Error>{
    /// CHANGE ACCORDING TO YOUR POSTGRES USERNAME & DATABASE
    let mut client = Client::connect("postgresql://grimgram:grimgram@localhost/rust", NoTls)?;

    let mut username= String::new();
    let mut email = String::new();

    println!("Enter Email: ");
    std::io::stdin().read_line(&mut email).unwrap();
    println!("Enter *NEW* Username (min 4): ");
    std::io::stdin().read_line(&mut username).unwrap();


    client.execute("UPDATE users SET username = $1 WHERE email $2", &[&username, &email])?;

    Ok(())
}