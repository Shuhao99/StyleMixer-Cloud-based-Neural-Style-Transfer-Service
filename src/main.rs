use actix_multipart::Multipart;
use actix_web::{get, post, App, HttpResponse, HttpServer, HttpRequest, Responder, Result};
use base64::{encode_config, URL_SAFE_NO_PAD};
use futures::{StreamExt, TryStreamExt};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::time::Duration;
use std::str;

#[derive(Serialize, Deserialize, Debug)]
struct ApiResponse {
    result_url: String,
}

#[derive(Serialize)]
struct ApiRequest {
    data: Vec<String>,
}

#[get("/")]
async fn index(_req: HttpRequest) -> Result<HttpResponse> {
    let html = include_str!("index.html");
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

#[post("/process_images")]
async fn process_images(mut payload: Multipart) -> impl Responder {
    let api_key = "YOUR_API_KEY";
    let mut images = Vec::new();
    let mut style_id = String::new();

    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_disposition = field.content_disposition();
        let name = content_disposition.get_name().unwrap();

        if name == "style_id" {
            let style_id_bytes = field.next().await.unwrap().unwrap();
            style_id = str::from_utf8(&style_id_bytes).unwrap().to_string();
        } else {
            let mut data = Vec::new();
            while let Some(chunk) = field.next().await {
                let chunk = chunk.unwrap();
                data.extend_from_slice(&chunk);
            }

            let base64_str = format!(
                "data:image/png;base64,{}",
                encode_config(&data, URL_SAFE_NO_PAD)
            );
            images.push(base64_str);
        }
    }

    if images.len() != 2 {
        return HttpResponse::BadRequest().body("Two images are required.");
    }

    let client = Client::new();
    let upload_url = "https://neuralstyle.art/api.json";

    let response = client
        .post(upload_url)
        .header("Content-Type", "multipart/form-data")
        .header("Accept", "application/json")
        .form(&[
            ("photo", images[0].clone()),
            ("photo", images[1].clone()),
            ("api_key", api_key.into()),
            ("style_id", style_id.clone()),
        ])
        .send()
        .await
        .unwrap();

    let response_json: Value = response.json().await.unwrap();
    let filterjob_id = response_json["filterjob_id"].as_str().unwrap();

    let mut progress = 0;
    let mut result_url = String::new();

    while progress < 100 {
        let status_url = format!(
            "https://neuralstyle.art/api/{}.json?api_key={}",
            filterjob_id, api_key
        );
        let status_response = client.get(&status_url).send().await.unwrap();
        let status_json: Value = status_response.json().await.unwrap();

        progress = status_json["progress"].as_i64().unwrap_or(0);
        result_url = status_json["url"].as_str().unwrap_or("").to_string();

        tokio::time::sleep(Duration::from_secs(5)).await;
    }

    HttpResponse::Ok().json(ApiResponse { result_url })
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(process_images))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}