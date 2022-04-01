use postgres::{Client, Error, NoTls};
fn init_db() -> Result<(), Error>{
    /// CHANGE ACCORDING TO YOUR POSTGRES USERNAME & DATABASE
    let mut client = Client::connect("postgresql://grimgram:grimgram@localhost/rust", NoTls)?;

    println!("Setting up database...");
    client.batch_execute("
        CREATE DATABASE IF NOT EXISTS rust;
    ")?;
    println!("Creating tables...");
    client.batch_execute("
    CREATE TABLE rust(
        id serial primary key,
        username varchar not null unique,
        password varchar not null,
        email varchar not null unique,
        a_type varchar not null
    );
    ")?;
    println!("Done! you can start using now!");

    Ok(())
}