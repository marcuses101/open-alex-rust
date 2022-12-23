use eyre::Result;
use std::env;
use std::process;

enum Mode {
    All,
    Level,
}

struct Config {
    mode: Mode,
    level: Option<u32>,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config> {
        args.next();
        let level = args.next();
        match level {
            Some(lev) => {
                let level_int: u32 = lev.parse()?;
                Ok(Config {
                    level: Some(level_int),
                    mode: Mode::Level,
                })
            }
            None => Ok(Config {
                level: None,
                mode: Mode::All,
            }),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::build(env::args()).unwrap_or_else(|er| {
        eprintln!("Problem parsing arguments: {er}");
        process::exit(1);
    });
    let client = reqwest::Client::new();
    let (concepts, filename) = match config.mode {
        Mode::All => (
            open_alex_rust::get_all_level_concepts(&client).await?,
            "all_concepts".to_string(),
        ),
        Mode::Level => {
            let level = &config.level.unwrap();
            (
                open_alex_rust::get_concepts_paged(&client, level).await?,
                format!("level_{level}_concepts"),
            )
        }
    };
    open_alex_rust::write_concepts_to_csv_file(concepts, filename)?;
    Ok(())
}
