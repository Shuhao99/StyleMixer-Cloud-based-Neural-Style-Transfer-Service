use actix_multipart::{Field, Multipart};
use futures::{StreamExt, TryStreamExt};
use reqwest::multipart::{Form, Part};
use actix_web::{post, web, HttpResponse, Result};
use reqwest::{Client};
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Write;
use std::io::Read;
use uuid::Uuid;
use tokio_util::codec::{BytesCodec, FramedRead};

#[derive(Serialize, Deserialize)]
pub struct NeuralStyleResponse {
    result: String,
    photo_id: String,
    filterjob_id: String,
}

#[post("/upload")]
pub async fn upload(
    mut payload: Multipart,
    client: web::Data<Client>,
) -> Result<HttpResponse> {
    let mut file_path = None;
    let mut style_id = None;

    while let Some(item) = payload.next().await {
        let mut field = item.unwrap();
        let content_disposition = field.content_disposition();
        let name = content_disposition.get_name().unwrap();

        if name == "image" {
            let filename = format!("{}.png", Uuid::new_v4().to_string());
            let filepath = format!("./images/{}", &filename);

            //fix image receive 
            let mut file = File::create(&filepath).unwrap();
            // let mut f = web::block(move || fs::File::create(&filepath)).await.unwrap();
            
           
            while let Some(chunk) = field.next().await {
                let data = chunk.unwrap();
                // f = web::block(move || f.write_all(&data).map(|_| f)).await.unwrap();
                file.write_all(&data).unwrap();
            }
            // end 

            file_path = Some(filepath);
        } else if name == "style" {
            let data = field.next().await.unwrap().unwrap();
            let style = String::from_utf8(data.to_vec()).unwrap();
            style_id = Some(style);
        }
    }

    if let (Some(file_path), Some(style_id)) = (file_path, style_id) {
        let response = call_neural_style_api(&client, &file_path, &style_id).await;
        match response {
            Ok(neural_style_response) => {
                Ok(HttpResponse::Ok().json(neural_style_response))
            }
            Err(_) => Ok(HttpResponse::InternalServerError().finish()),
        }
    } else {
        Ok(HttpResponse::BadRequest().finish())
    }
}

async fn call_neural_style_api(
    client: &Client,
    file_path: &str,
    style_id: &str,
) -> Result<NeuralStyleResponse, Box<dyn std::error::Error>> {
    let mut file = File::open(file_path).unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;
   
    let form = reqwest::multipart::Form::new()
        .part("photo", reqwest::multipart::Part::bytes(contents))
        .text("api_key", "VCGKSUTDMUCYMOHTEPAYYBGGKTCWBXCQGCTNGPIADSKRSRYJ")
        .text("style_id", style_id);

    let res = client
        .post("https://neuralstyle.art/api.json")
        .multipart(form)
        .send()
        .await?;

    let neural_style_response: NeuralStyleResponse = res.json().await?;
    Ok(neural_style_response)
}

#[post("/progress/{filterjob_id}")]
pub async fn get_progress(
    filterjob_id: web::Path<String>,
    client: web::Data<Client>,
) -> Result<HttpResponse> {
    let res = client
        .get(&format!(
            "https://neuralstyle.art/api/{}.json?api_key=VCGKSUTDMUCYMOHTEPAYYBGGKTCWBXCQGCTNGPIADSKRSRYJ",
            filterjob_id
        ))
        .send()
        .await
        .unwrap();
    
    let progress_data = res.text().await.unwrap();
    Ok(HttpResponse::Ok().json(progress_data))

}
