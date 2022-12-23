use eyre::Result;
use open_alex_rust::Concept;
use std::env;
use std::process;
struct Config {
    level: u32,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config> {
        args.next();
        let level: u32 = args.next().unwrap().parse()?;
        Ok(Config { level })
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::build(env::args()).unwrap_or_else(|er| {
        eprintln!("Problem parsing arguments: {er}");
        process::exit(1);
    });
    let level = &config.level;
    let client = reqwest::Client::new();
    let concepts: Vec<Concept> = vec![];

    // let concepts = open_alex_rust::get_concepts(client, level).await?;
    open_alex_rust::write_concepts_to_csv_file(concepts, level)?;
    Ok(())
}
