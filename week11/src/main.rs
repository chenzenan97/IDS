use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use std::env;
use reqwest::header;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_json::json;
use std::fs::File;
use std::io::prelude::*;
use actix_files::Files;

#[derive(Debug, Serialize, Deserialize)]
struct keywordsRequest {
    text: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct KeywordsResponse {
    keywords: String,
}

#[get("/")]
async fn index() -> impl Responder {
    let mut file = File::open("ui/index.html").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    HttpResponse::Ok().content_type("text/html").body(contents)
}

#[post("/Keywords")]
async fn Keywords(info: web::Json<keywordsRequest>) -> impl Responder {
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    let url = "https://api.openai.com/v1/engines/text-davinci-003/completions";
    let prompt = format!("Extract keywords from this text::\n\n{}", info.text);
    let payload = json!({
        "prompt": prompt,
        "temperature": 0.7,
        "max_tokens": 256,
        "top_p": 1,
        "frequency_penalty": 0,
        "presence_penalty": 0
    });

    let client = reqwest::Client::new();
    let res = client.post(url)
        .header(header::AUTHORIZATION, format!("Bearer {}", api_key))
        .header(header::CONTENT_TYPE, "application/json")
        .json(&payload)
        .send()
        .await;

        match res {
            Ok(r) => {
                match r.json::<serde_json::Value>().await {
                    Ok(result) => {
                        println!("Result JSON: {:?}", result);
                        if let Some(keywords) = result["choices"][0]["text"].as_str() {
                            let keywords = keywords.to_owned();
                            let response = KeywordsResponse { keywords };
                            HttpResponse::Ok().json(response)
                        } else {
                            HttpResponse::InternalServerError().body("Error: Unable to retrieve keywords")
                        }
                    }
                    Err(e) => HttpResponse::InternalServerError().body(format!("Error parsing JSON: {:?}",
                }
            }
            Err(e) => HttpResponse::InternalServerError().body(format!("Error: {:?}", e)),
        }
        
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(Keywords)
            //.service(fs::Files::new("/", "./static").index_file("ui/index.html"))
            .service(Files::new("/static", "ui").index_file("index.html"))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}