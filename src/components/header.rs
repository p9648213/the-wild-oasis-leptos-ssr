use crate::components::{logo::Logo, navigation::Navigation};
use leptos::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="border-b border-primary-900 px-8 py-5">
            <div class="flex justify-between items-center max-w-7xl mx-auto">
                <Logo/>
                <Navigation/>
            </div>
        </header>
    }
}
