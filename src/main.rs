mod logger;

use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    logger::init_logger()?;
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        error!("Too few arguements")
    } else if args.len() > 2 {
        error!("Too many arguements")
    }

    Ok(())
}
