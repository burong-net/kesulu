use crate::models::*;
use leptos_struct_table::{ColumnSort, PaginatedTableDataProvider};
use std::collections::VecDeque;
use crate::fund::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FundDataProvider {
    sorting: VecDeque<(usize, ColumnSort)>,
    funds: Vec<Fund>,
    total_count: usize,
}

impl Default for FundDataProvider {
    fn default() -> Self {
        Self {
            sorting: VecDeque::new(),
            funds: vec![],
            total_count: 0,
        }
    }
}

impl FundDataProvider {
    pub async fn new() -> Result<Self, String> {

        let (mut funds, total_count)= match fund_list(1).await  {
            Ok((funds, total)) => (funds, total),
            Err(e) => return Err(e.to_string()),
        };

        let pages = 5;
        for i in 1..=pages {
            let (resp, _) = match fund_list(i).await  {
                Ok((funds, _)) => (funds, total_count),
                Err(e) => return Err(e.to_string()),
            };
            funds.extend(resp);
        }

        Ok(Self {
            sorting: VecDeque::new(),
            funds,
            total_count: 150,
        })
        
    }
}

impl PaginatedTableDataProvider<Fund> for FundDataProvider {
    const PAGE_ROW_COUNT: usize = 30;

    async fn get_page(&self, page_index: usize) -> Result<Vec<Fund>, String> {
        if page_index > self.total_count / Self::PAGE_ROW_COUNT {
            return Ok(vec![]);
        }

        let start  = page_index * Self::PAGE_ROW_COUNT;
        let end    = start + Self::PAGE_ROW_COUNT;

        Ok(self.funds[start..end].to_vec())
    }

    async fn row_count(&self) -> Option<usize> {
        Some(self.total_count)
    }

    async fn page_count(&self) -> Option<usize> {
        Some((self.total_count / Self::PAGE_ROW_COUNT) + 1)
    }

    fn set_sorting(&mut self, sorting: &VecDeque<(usize, ColumnSort)>) {
        self.sorting = sorting.clone();
    }
}
