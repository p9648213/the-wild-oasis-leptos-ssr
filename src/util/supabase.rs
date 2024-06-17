use postgrest::Postgrest;

pub fn create_client() -> Postgrest {
    let supabase_url = std::env::var("SUPABASE_URL").expect("cannot find SUPABASE_URL env");
    let supabase_key = std::env::var("SUPABASE_KEY").expect("cannot find SUPABASE_KEY env");

    Postgrest::new(&supabase_url)
        .insert_header("apikey", &supabase_key)
        .insert_header("Authorization", format!("{} {}", "Bearer ", &supabase_key))
}
