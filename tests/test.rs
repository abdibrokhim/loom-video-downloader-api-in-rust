use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct LoomRequest {
    url: String,
}

#[derive(Deserialize, Debug)]
struct LoomResponse {
    video_url: String,
}

#[derive(Deserialize, Debug)]
struct ErrorResponse {
    error: String,
}

#[tokio::test]
async fn test_loom_download() {
    // Initialize the HTTP client
    let client = Client::new();
    
    // The Loom video URL to test
    let loom_url = "https://www.loom.com/share/17c3b800367e47ebaf06151f6d45447a?sid=944b8ac3-4a7a-4faf-86be-4cc3f41249f4";
    
    // Create the request payload
    let payload = LoomRequest { url: loom_url.to_string() };
    
    // If testing locally, use this endpoint
    // let endpoint = "http://127.0.0.1:8000/api/loom-dl";
    
    // If testing against your deployed Shuttle instance, use this endpoint with your app's URL
    let endpoint = "https://loom-dl-pwtf.shuttle.app/api/loom-dl";
    
    // Send the request
    let response = client.post(endpoint)
        .json(&payload)
        .send()
        .await;
    
    match response {
        Ok(res) => {
            println!("Status: {}", res.status());
            
            if res.status().is_success() {
                match res.json::<LoomResponse>().await {
                    Ok(data) => {
                        println!("Successfully retrieved video URL: {}", data.video_url);
                        // You can download the video here if needed
                        // let video_bytes = client.get(&data.video_url).send().await?.bytes().await?;
                        // std::fs::write("loom_video.mp4", video_bytes)?;
                    },
                    Err(e) => println!("Failed to parse response: {}", e),
                }
            } else {
                match res.json::<ErrorResponse>().await {
                    Ok(error) => println!("API error: {}", error.error),
                    Err(e) => println!("Failed to parse error: {}", e),
                }
            }
        },
        Err(e) => println!("Request failed: {}", e),
    }
}

// To run this test, make sure to add tokio test feature in Cargo.toml:
// tokio = { version = "1.26.0", features = ["full", "test-util"] } 