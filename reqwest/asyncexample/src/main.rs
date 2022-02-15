use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Post {
    id: Option<i32>,
    title: String,
    body: String,
    #[serde(rename = "userId")]
    user_id: i32,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if false {
        let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
        println!("{:#?}", resp);
    }

    if false {
        // Some simple CLI args requirements...
        let url = match std::env::args().nth(1) {
            Some(url) => url,
            None => {
                println!("No CLI URL provided, using default.");
                "https://httpbin.org/get".into()
            }
        };

        eprintln!("Fetching {:?}...", url);

        // reqwest::get() is a convenience function.
        //
        // In most cases, you should create/build a reqwest::Client and reuse
        // it for all requests.
        let res = reqwest::get(url).await?;

        eprintln!("Response: {:?} {}", res.version(), res.status());
        eprintln!("Headers: {:#?}\n", res.headers());

        let body = res.text().await?;

        println!("Body: {}", body);
    }

    if false {
        // typed json
        let new_post = Post {
            id: None,
            title: "Reqwest.rs".into(),
            body: "https://docs.rs/reqwest".into(),
            user_id: 1,
        };
        let new_post: Post = reqwest::Client::new()
        .post("https://jsonplaceholder.typicode.com/posts")
        .json(&new_post)
        .send()
        .await?
        .json()
        .await?;

        println!("{:#?}", new_post);
    }

    {
        // dynamic json
        let echo_json: serde_json::Value = reqwest::Client::new()
        .post("https://jsonplaceholder.typicode.com/posts")
        .json(&serde_json::json!({
            "title": "Reqwest.rs",
            "body": "https://docs.rs/reqwest",
            "userId": 1
        }))
        .send()
        .await?
        .json()
        .await?;

    println!("{:#?}", echo_json);
    }

    Ok(())
}