use reqwest::header::USER_AGENT;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
struct ConceptCountByYear {
    year: u32,
    works_count: u32,
    cited_by_count: u32,
}
#[derive(Deserialize, Debug)]
struct Open_Alex_Meta {
    count: u32,
    db_response_time_ms: u32,
    page: u32,
    per_page: u32,
}

#[derive(Deserialize, Debug)]
struct Open_Alex_Response<T> {
    meta: Open_Alex_Meta,
    results: Vec<T>,
}

#[derive(Deserialize, Debug)]
struct Ids_Object {
    openalex: String,
    wikidata: Option<String>,
    mag: Option<String>,
    wikipedia: Option<String>,
    umls_aui: Option<Vec<String>>,
    umls_cui: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
struct Concept_Ancestor {
    id: String,
    wikidata: Option<String>,
    display_name: String,
    level: u32,
}

#[derive(Deserialize, Debug)]
struct RelatedConcept {
    id: String,
    wikidata: Option<String>,
    display_name: String,
    level: u32,
    score: f32,
}
#[derive(Deserialize, Debug)]
struct International {
    display_name: HashMap<String, String>,
}
#[derive(Deserialize, Debug)]
struct Concept {
    id: String,
    // wikidata: Option<String>,
    display_name: String,
    // level: u32,
    description: Option<String>,
    // works_count: u32,
    // cited_by_count: u32,
    // ids: Ids_Object,
    // image_url: Option<String>,
    // image_thumbnail_url: Option<String>,
    // international: International,
    // ancestors: Vec<Concept_Ancestor>,
    // related_concepts: Vec<RelatedConcept>,
    counts_by_year: Vec<ConceptCountByYear>,
    // works_api_url: String,
    // updated_date: String,
    // created_date: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let open_alex_base_path = "https://api.openalex.org/concepts?filter=level:5";
    let client = reqwest::Client::new();
    let response = client
        .get(open_alex_base_path)
        .header(USER_AGENT, "mailto:mnjconnolly@gmail.com")
        .send()
        .await?;
    // println!("{:#?}", response);
    let json = response.json::<Open_Alex_Response<Concept>>().await?;
    //   println!("{:#?}", text);
    // let json: serde_json::Value = serde_json::from_str(&text)?;
    // let json: Open_Alex_Response<Concept> = serde_json::from_str(&text)?;
    // let body = response.json::<Open_Alex_Response<Concept>>().await?;
    println!("{:#?}", json);
    Ok(())
}
