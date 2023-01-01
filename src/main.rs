use crate::surreal_client::root_client;
use eyre::Result;
use open_alex_rust::Concept;
use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;
use std::env;
use std::process;
use surrealdb_rs::net::WsClient;
use surrealdb_rs::Surreal;

pub mod surreal_client;
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
#[derive(Deserialize, Debug, Serialize)]
struct Person {
    id: Option<String>,
    name: Cow<'static, str>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::build(env::args()).unwrap_or_else(|er| {
        eprintln!("Problem parsing arguments: {er}");
        process::exit(1);
    });
    let db_client = root_client("test", "test").await?;
    let client = reqwest::Client::new();
    let (concepts, _filename) = match config.mode {
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
    async fn create_concept_record(
        concept: Concept,
        db_client: &Surreal<WsClient>,
    ) -> Result<Concept> {
        let record: Concept = db_client
            .create(("concept", concept.id.clone()))
            .content(concept)
            .await?;

        Ok(record)
    }
    for concept in concepts {
        let res = create_concept_record(concept, &db_client).await;
        match res {
            Err(e) => {
                println!("{:#?}", e);
            }
            Ok(record) => {
                println!("{} ({}) created", record.display_name, record.level)
            }
        }
    }
    // open_alex_rust::write_concepts_to_csv_file(concepts, filename)?;
    Ok(())
}
