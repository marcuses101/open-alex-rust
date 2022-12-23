#[allow(dead_code)]
use csv;
use eyre::Result;
use reqwest::{header::USER_AGENT, Client};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct ConceptCountByYear {
    year: u32,
    works_count: u32,
    cited_by_count: u32,
}
#[derive(Deserialize, Debug)]
pub struct OpenAlexMeta {
    count: u32,
    db_response_time_ms: u32,
    page: u32,
    per_page: u32,
}

#[derive(Deserialize, Debug)]
pub struct OpenAlexResponse<T> {
    meta: OpenAlexMeta,
    results: Vec<T>,
}

#[derive(Deserialize, Debug)]
pub struct IdsObject {
    openalex: String,
    wikidata: Option<String>,
    mag: Option<String>,
    wikipedia: Option<String>,
    umls_aui: Option<Vec<String>>,
    umls_cui: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
pub struct ConceptAncestor {
    id: String,
    wikidata: Option<String>,
    display_name: String,
    level: u32,
}

#[derive(Deserialize, Debug)]
pub struct RelatedConcept {
    id: String,
    wikidata: Option<String>,
    display_name: String,
    level: u32,
    score: f32,
}
#[derive(Deserialize, Debug)]
pub struct International {
    display_name: HashMap<String, String>,
}
#[derive(Deserialize, Debug)]
pub struct Concept {
    pub id: String,
    // wikidata: Option<String>,
    pub display_name: String,
    pub level: u32,
    pub description: Option<String>,
    // works_count: u32,
    // cited_by_count: u32,
    // ids: Ids_Object,
    // image_url: Option<String>,
    // image_thumbnail_url: Option<String>,
    // international: International,
    // ancestors: Vec<ConceptAncestor>,
    // related_concepts: Vec<RelatedConcept>,
    pub counts_by_year: Vec<ConceptCountByYear>,
    pub works_api_url: String,
    // updated_date: String,
    // created_date: String,
}

#[derive(Debug, Serialize)]
pub struct CSVRecord<'a> {
    id: &'a String,
    level: &'a u32,
    display_name: &'a String,
    description: &'a Option<String>,
    works_api_url: &'a String,
    year: u32,
    works_count: u32,
    cited_by_count: u32,
}

async fn get_concept_page(
    client: &Client,
    page: u32,
    level: &u32,
) -> Result<OpenAlexResponse<Concept>> {
    let per_page: u32 = 200;
    let path = format!(
        "https://api.openalex.org/concepts?filter=level:{level}&per-page={per_page}&page={page}"
    );
    let response = client
        .get(path)
        .header(USER_AGENT, "mailto:mnjconnolly@gmail.com")
        .send()
        .await?;
    let json = response.json::<OpenAlexResponse<Concept>>().await?;
    Ok(json)
}

async fn get_concept_cursor_result(
    client: &Client,
    cursor: String,
    level: &u32,
) -> Result<OpenAlexResponse<Concept>> {
    let per_page: u32 = 200;
    let path = format!(
        "https://api.openalex.org/concepts?filter=level:{level}&per-page={per_page}&cursor={cursor}"
    );
    let response = client
        .get(path)
        .header(USER_AGENT, "mailto:mnjconnolly@gmail.com")
        .send()
        .await?;
    let json = response.json::<OpenAlexResponse<Concept>>().await?;
    Ok(json)
}

pub async fn get_concepts_cursor(client: &Client, level: &u32) -> Result<Vec<Concept>> {
    println!("fetching page 1 of concept level: {level}");
    let mut next_cursor = Some("*".to_string());
    {
        let cursor = next_cursor.unwrap().to_string();
    }
    let first_response = get_concept_cursor_result(client, "*".to_string(), level).await?;
    Ok(first_response.results)
}

pub async fn get_concepts_paged(client: &Client, level: &u32) -> Result<Vec<Concept>> {
    println!("fetching page 1 of ??? of level: {level}");
    let first_response = get_concept_page(client, 1, &level).await?;
    let max_results = first_response.meta.count;
    let last_page = max_results / first_response.meta.per_page + 1;
    let mut results = first_response.results;
    for current_page in 2..=last_page {
        println!("fetching page {current_page} of {last_page}");
        let response_result = get_concept_page(client, current_page, &level).await;
        match response_result {
            Ok(res) => res
                .results
                .into_iter()
                .for_each(|concept| results.push(concept)),
            Err(_) => {
                println!("failed to fetch page {}", current_page)
            }
        }
    }
    Ok(results)
}

pub async fn get_all_level_concepts(client: &Client) -> Result<Vec<Concept>> {
    let mut concepts: Vec<Concept> = vec![];
    for level in 0..=5 {
        let level_concepts = get_concepts_paged(client, &level).await?;
        level_concepts
            .into_iter()
            .for_each(|concept| concepts.push(concept));
    }
    Ok(concepts)
}

pub fn write_concepts_to_csv_file(
    concepts: Vec<Concept>,
    filename: impl Into<String>,
) -> Result<()> {
    let filename_as_string = filename.into();
    let filename = format!("{filename_as_string}.csv");
    println!("writing to \"{filename}\"");
    let mut csv_writer = csv::Writer::from_path(filename)?;
    for concept in concepts.iter() {
        concept
            .counts_by_year
            .iter()
            .for_each(|count_per_year_entry| {
                let record = CSVRecord {
                    id: &concept.id,
                    level: &concept.level,
                    works_api_url: &concept.works_api_url,
                    display_name: &concept.display_name,
                    description: &concept.description,
                    cited_by_count: count_per_year_entry.cited_by_count,
                    works_count: count_per_year_entry.works_count,
                    year: count_per_year_entry.year,
                };
                csv_writer.serialize(record).expect("unable to serialize");
            })
    }
    Ok(())
}
