use leptos_query::{create_query, QueryOptions, QueryScope};

use crate::model::country::Country;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct CountryKey;

pub async fn get_countries() -> Result<Vec<Country>, String> {
  let res = reqwest::get("https://restcountries.com/v2/all?fields=name,flag").await;
  match res {
    Ok(res) => {
      let countries: Result<Vec<Country>, reqwest::Error> = res.json().await;
      match countries {
        Ok(countries) => Ok(countries),
        Err(err) => Err(err.to_string()),
      }
    },
    Err(err) => Err(err.to_string()),
  }
}

pub fn all_country_query() -> QueryScope<CountryKey, Result<Vec<Country>, String>> {
  create_query(move |_| async move {get_countries().await}, QueryOptions::default())
}