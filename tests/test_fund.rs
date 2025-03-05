
use kesulu::fund::*;
use tokio::test;
use serde_json::Value;

#[test]
async fn test_fund_list() -> Result<(), Box<dyn std::error::Error>> {
    let fund_list = fund_list().await;
    println!("{:?}", fund_list);
    assert_eq!(1, 1);

    Ok(())
}