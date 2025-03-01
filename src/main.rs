mod entry;
mod password_manager;
mod helper;
use password_manager::PasswordManager;
use tower_http::{self, cors::CorsLayer, trace::TraceLayer};
use axum::{http::Response, routing::{get, post}, Json, Router};

#[tokio::main]
async fn main(){
    let app = Router::new()
        .route("/test", get(test))
        .layer(CorsLayer::permissive()).layer(TraceLayer::new_for_http())
        ;
    let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind("127.0.0.1:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap()
}

async fn test()->Response<String>{
     Response::new("{\"message\": \"Ok -- connected\", \"status\":200}".to_string())
    // Response::new("Ok -- connected".to_string())

}   

async fn getVaults(){
    
}