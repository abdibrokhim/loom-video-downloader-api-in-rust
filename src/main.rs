use actix_web::{get, post, web, HttpResponse, Error};
use shuttle_actix_web::ShuttleActixWeb;
use serde::{Deserialize, Serialize};
use reqwest::Client;

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

#[derive(Deserialize)]
struct LoomRequest {
    url: String,
}

#[derive(Serialize)]
struct LoomResponse {
    video_url: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

#[post("/api/loom-dl")]
async fn loom_dl(request: web::Json<LoomRequest>) -> Result<HttpResponse, Error> {
    // Extract the Loom video ID from the URL
    let url = &request.url;
    let id = match url.split('/').last()
        .and_then(|s| s.split('?').next()) {
            Some(id) => id,
            None => return Ok(HttpResponse::BadRequest()
                .json(ErrorResponse { error: "Invalid Loom URL".to_string() })),
    };

    // Create a client for making HTTP requests
    let client = Client::new();
    
    // Fetch the transcoded URL from Loom API
    let loom_api_url = format!("https://www.loom.com/api/campaigns/sessions/{}/transcoded-url", id);
    
    match client.post(&loom_api_url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<serde_json::Value>().await {
                    Ok(data) => {
                        if let Some(video_url) = data.get("url").and_then(|v| v.as_str()) {
                            Ok(HttpResponse::Ok()
                                .json(LoomResponse { video_url: video_url.to_string() }))
                        } else {
                            Ok(HttpResponse::InternalServerError()
                                .json(ErrorResponse { error: "URL field not found in response".to_string() }))
                        }
                    },
                    Err(_) => {
                        Ok(HttpResponse::InternalServerError()
                            .json(ErrorResponse { error: "Failed to parse response data".to_string() }))
                    }
                }
            } else {
                Ok(HttpResponse::BadGateway()
                    .json(ErrorResponse { error: "Failed to fetch the video URL from Loom".to_string() }))
            }
        },
        Err(_) => {
            Ok(HttpResponse::InternalServerError()
                .json(ErrorResponse { error: "Failed to send request to Loom API".to_string() }))
        }
    }
}

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut web::ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut web::ServiceConfig| {
        cfg.service(hello_world)
           .service(loom_dl);
    };

    Ok(config.into())
}
