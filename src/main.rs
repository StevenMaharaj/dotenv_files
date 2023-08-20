use std::env;
use std::error::Error;
fn main() -> Result<(), Box<dyn Error>>{
    dotenvy::dotenv()?;

    let secret = env::var("SECRET")?;
    println!("SECRET: {}", secret);
    Ok(())
}
