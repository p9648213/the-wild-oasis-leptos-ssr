use leptos_query::{create_query, QueryOptions, QueryScope};
use crate::util::supabase::create_client;

use crate::model::country::Country;
use crate::model::cabin::Cabin;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct CountryKey;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct CabinKey;


pub async fn get_countries() -> Result<Vec<Country>, String> {
    let res = reqwest::get("https://restcountries.com/v2/all?fields=name,flag")
        .await
        .map_err(|e| e.to_string())?;

    let countries: Vec<Country> = res.json()
        .await
        .map_err(|e| e.to_string())?;

    Ok(countries)
}


pub async fn get_cabins() -> Result<Vec<Cabin>, String> {
    let client = create_client();

    let resp = client.from("cabins").select("*").execute().await.map_err(|e| e.to_string())?;

    let response = resp.error_for_status().map_err(|e| e.to_string())?;

    let body = response.text().await.map_err(|e| e.to_string())?;

    let cabins: Vec<Cabin> = serde_json::from_str(&body).map_err(|e| e.to_string())?;

    Ok(cabins)
}

pub fn all_country_query() -> QueryScope<CountryKey, Result<Vec<Country>, String>> {
    create_query(move |_| async move { get_countries().await }, QueryOptions::default())
}

pub fn all_cabins_query() -> QueryScope<CabinKey, Result<Vec<Cabin>, String>> {
    create_query(
        move |_| async move { get_cabins().await },
        QueryOptions::default(),
    )
}
