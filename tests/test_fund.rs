
use axum::routing::connect;
use kesulu::fund::*;
use tokio::test;
use serde_json::Value;
use kesulu::database::*;

#[tokio::test]
async fn test_fund_list() {
    assert!(1 == 1);
}

#[tokio::test]
async fn test_db_connect() {
    db_connect().await.unwrap();  
}