use crate::components::header::Header;
use leptos::*;
use leptos_router::Outlet;

#[component]
pub fn AppLayout() -> impl IntoView {
    view! {
        <Header/>
        <div class="flex-1 px-8 py-12 grid">
            <main class="max-w-7xl mx-auto w-full">
                <Outlet/>
            </main>
        </div>
    }
}
