use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
// use futures::executor::block_on;

// 定义一个结构体来表示基金信息
#[derive(Debug, Deserialize)]
struct Fund {
    code: String,
    name: String,
    // 添加其他字段...
}

// 构造查询参数
fn build_params() -> HashMap<&'static str, &'static str> {
    let mut params = HashMap::new();
    params.insert("product", "EFund");
    params.insert("deviceid", "874C427C-7C24-4980-A835-66FD40B67605");
    params.insert("MobileKey", "874C427C-7C24-4980-A835-66FD40B67605");
    params.insert("plat", "Iphone");
    params.insert("PhoneType", "IOS15.1.0");
    params.insert("OSVersion", "15.5");
    params.insert("version", "6.5.5");
    params.insert("ServerVersion", "6.5.5");
    params.insert("Version", "6.5.5");
    params.insert("appVersion", "6.5.5");
    params.insert("m", "1");
    params.insert("key", "110003");
    params
}

// 构造请求头
fn build_headers() -> Result<reqwest::header::HeaderMap, Box<dyn std::error::Error>> {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "validmark",
        reqwest::header::HeaderValue::from_str("aKVEnBbJF9Nip2Wjf4de/fSvA8W3X3iB4L6vT0Y5cxvZbEfEm17udZKUD2qy37dLRY3bzzHLDv+up/Yn3OTo5Q==")?,
    );
    Ok(headers)
}

// 搜索基金信息，改成异步函数
pub async fn search() -> Result<Value, Box<dyn std::error::Error>> {
    // 创建一个新的异步HTTP客户端
    let client = reqwest::Client::new();

    // 定义请求的URL
    let url = "https://fundsuggest.eastmoney.com/FundSearch/api/FundSearchAPI.ashx";

    // 构造查询参数
    let params = build_params();

    // 构造请求头
    let headers = build_headers()?;

    // 发送异步GET请求并包含查询参数和请求头
    let response = client
       .get(url)
       .query(&params) // 添加查询参数
       .headers(headers) // 设置请求头
       .send()
       .await?; // 等待请求完成

    // 检查响应状态码是否为成功（200 - 299）
    if response.status().is_success() {
        // 处理成功的响应体
        let body = response.text().await?;
        // 解析响应体为JSON
        let json: Value = serde_json::from_str(&body)?;
        Ok(json)
    } else {
        Err(format!("Request failed with status code: {}", response.status()).into())
    }
}