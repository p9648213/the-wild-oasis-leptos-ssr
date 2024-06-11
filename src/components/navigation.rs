use leptos::*;
use leptos_router::A;

#[component]
pub fn Navigation() -> impl IntoView {
    view! {
        <nav class="z-10 text-xl">
            <ul class="flex gap-16 items-center">
                <li>
                    <A href="/cabins" class="hover:text-accent-400 transition-colors">
                        Cabins
                    </A>
                </li>
                <li>
                    <A href="/about" class="hover:text-accent-400 transition-colors">
                        About
                    </A>
                </li>
                <li>
                    <A href="/account" class="hover:text-accent-400 transition-colors">
                        Guest area
                    </A>
                </li>
            </ul>
        </nav>
    }
}
