use postgres::{Client, Error, NoTls};
use std::io::Write;
extern crate dotenv;

use dotenv::dotenv;
use std::env;
pub(crate) fn init_db() -> Result<(), Error>{
    dotenv().ok();
    let pskey = "PSQLHOST";
    let psvalue= dotenv::var(pskey).unwrap();
    // CHANGE ACCORDING TO YOUR POSTGRES USERNAME & DATABASE
    let mut client = Client::connect(&*psvalue, NoTls)?;

    println!("Setting up database...");
    client.batch_execute("
        CREATE DATABASE IF NOT EXISTS rust;
    ")?;
    println!("Creating tables...");
    client.batch_execute("
    CREATE TABLE users(
        id serial primary key,
        username varchar not null unique,
        password varchar not null,
        email varchar not null unique,
        a_type varchar not null
    );
    ")?;
    //let mut init_check = std::fs::File::create(".db_init_success_file_rustps");
    //init_check.write_all("init_successful, 100".as_bytes()).expect("Write Failed");
    println!("Done! you can start using now!");

    Ok(())
}