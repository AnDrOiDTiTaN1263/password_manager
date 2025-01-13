mod entry;
mod password_manager;
mod helper;
use password_manager::PasswordManager;
use axum::{routing::{get, post}, Router};

#[tokio::main]
async fn main(){
    let app = Router::new().route("/test", get(test));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap()
}

async fn test()->String{
    return  "Ok -- Connected".to_string();
}