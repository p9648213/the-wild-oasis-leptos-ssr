use leptos::*;
use leptos_image::Image;
use leptos_router::A;

#[component]
pub fn Logo() -> impl IntoView {
    view! {
        <A href="/" class="flex items-center gap-4 z-10">
            <Image src="/logo.png" height=60 width=60 quality=100 alt="The Wild Oasis logo"/>
            <span class="text-xl font-semibold text-primary-100">"The Wild Oasis"</span>
        </A>
    }
}
