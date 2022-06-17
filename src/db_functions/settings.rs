use mongodb::{Client, options::ClientOptions};
use mongodb::error::Error;
extern crate dotenv;

use dotenv::dotenv;
use std::env;

async fn init_settings_cache()  -> Result<(), Error>{
    dotenv().ok();
    let monkey = "MONGOHOST";
    let monvalue= dotenv::var(monkey).unwrap();
    // CHANGE ACCORDING TO YOUR POSTGRES USERNAME & DATABASE
    let mut client_options = ClientOptions::parse(monvalue).await?;

    Ok(())
}

fn create_settings_cache(){

}