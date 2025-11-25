mod logger;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    logger::init_logger()?;

    Ok(())
}
