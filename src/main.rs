// main.rs

use axum::{
    routing::get,
    Router,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

// Define a Cat struct
#[derive(Serialize, Deserialize, Clone)]
struct Cat {
    name: String,
    gender: Gender,
    age: u8,
}

#[derive(Serialize, Deserialize, Clone)]
enum Gender {
    Male,
    Female,
}

#[tokio::main]
async fn main() {
    // Create a router with one route that responds with a greeting based on the cat's gender
    let app = Router::new().route("/", get(greet_chloe));

    // Define the address and run the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Handler function that returns a greeting for Chloe
async fn greet_chloe() -> impl IntoResponse {
    let chloe = Cat {
        name: "Chloe".to_string(),
        gender: Gender::Female,
        age: 5,
    };

    greet_cat(&chloe).await
}

// The greet_cat function accepts a &Cat reference and returns a greeting
async fn greet_cat(cat: &Cat) -> impl IntoResponse {
    let greeting = match cat.gender {
        Gender::Male => format!("Meow, {}!", cat.name),
        Gender::Female => format!("Purr, {}!", cat.name),
    };

    Json(greeting)
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_greet_chloe() {
        let chloe = Cat {
            name: "Chloe".to_string(),
            gender: Gender::Female,
            age: 5,
        };

        let response = greet_cat(&chloe).await.into_response();

        assert_eq!(response.into_body().to_string(), "Purr, Chloe!");
    }

    #[tokio::test]
    async fn test_greet_margaret() {
        let margaret = Cat {
            name: "Margaret".to_string(),
            gender: Gender::Female,
            age: 3,
        };

        let response = greet_cat(&margaret).await.into_response();

        assert_eq!(response.into_body().to_string(), "Purr, Margaret!");
    }

    #[tokio::test]
    async fn test_greet_rocky() {
        let rocky = Cat {
            name: "Rocky".to_string(),
            gender: Gender::Male,
            age: 4,
        };

        let response = greet_cat(&rocky).await.into_response();

        assert_eq!(response.into_body().to_string(), "Meow, Rocky!");
    }
}
