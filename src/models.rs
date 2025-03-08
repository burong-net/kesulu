use crate::tailwind::TailwindClassesPreset;
use leptos_struct_table::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, TableRow, Serialize, Clone)]
#[table(sortable, classes_provider = "TailwindClassesPreset")]
pub struct Fund {
    #[serde(rename = "FCODE")]
    fcode: String,
    #[serde(rename = "SHORTNAME")]
    fname: String,
    #[serde(rename = "DWJZ")]
    fvalue: String,
}

#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    #[serde(rename = "Datas")]
    pub datas: Vec<Fund>,
    #[serde(rename = "ErrCode")]
    pub err_code: i32,
    #[serde(rename = "ErrMsg")]
    pub err_msg: Option<String>,
    #[serde(rename = "Expansion")]
    pub expansion: Option<Value>,
    #[serde(rename = "TotalCount")]
    pub total_count: usize,
}