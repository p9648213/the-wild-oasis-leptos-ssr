use leptos::*;

#[component]
pub fn TextExpander(text: String) -> impl IntoView {
    let (is_expanded, set_is_expanded) = create_signal(false);
    let text = Signal::derive(move || {
        if is_expanded.get() == true {
            text.clone()
        } else {
            text.split_whitespace()
                .take(40)
                .collect::<Vec<&str>>()
                .join(" ")
                + "..."
        }
    });

    view! {
        {move || text}
        {" "}
        <button
            class="text-primary-700 border-b border-primary-700 leading-3 pb-1"
            on:click=move |_| set_is_expanded.update(|expanded| *expanded = !*expanded)
        >
            <Show when=move || { is_expanded.get() == true } fallback=move || view! { "Show more" }>
                "Show less"
            </Show>
        </button>
    }
}
