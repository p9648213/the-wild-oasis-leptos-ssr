use crate::env::env;
use postgrest::Postgrest;

pub fn create_client() -> Postgrest {
    let supabase_url = env::SUPABASE_URL;
    let supabase_key = env::SUPABASE_KEY;

    Postgrest::new(supabase_url)
        .insert_header("apikey", supabase_key)
        .insert_header("Authorization", format!("{} {}", "Bearer ", supabase_key))
}
