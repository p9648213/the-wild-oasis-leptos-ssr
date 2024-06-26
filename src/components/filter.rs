use leptos::*;
use leptos_router::{use_location, use_navigate, use_query_map};
use std::rc::Rc;

#[component]
pub fn Filter() -> impl IntoView {
    let search_query = use_query_map();
    let location = use_location();
    let navigate = use_navigate();

    let active_filter = move || {
        search_query
            .with(|query| query.get("capacity").cloned())
            .unwrap_or("all".to_owned())
    };

    let handle_filter = move |filter: &String| {
        let pathname = location.pathname.get();
        let navigate_url = format!("{}?capacity={}", pathname, filter);
        navigate(&navigate_url, Default::default());
    };

    view! {
        <div class="border border-primary-800 flex">
            <Button
                filter="all".to_owned().into()
                active_filter=active_filter
                handle_filter=handle_filter.clone()
            >
                "All cabins"
            </Button>
            <Button

                filter="small".to_owned().into()
                active_filter=active_filter
                handle_filter=handle_filter.clone()
            >
                "1-3 guests"
            </Button>
            <Button

                filter="medium".to_owned().into()
                active_filter=active_filter
                handle_filter=handle_filter.clone()
            >
                "4-7 guests"
            </Button>
            <Button

                filter="large".to_owned().into()
                active_filter=active_filter
                handle_filter=handle_filter.clone()
            >
                "8-12 guests"
            </Button>
        </div>
    }
}

#[component]
fn Button<T: Fn(&String) -> () + 'static, A: Fn() -> String + 'static>(
    children: Children,
    filter: Rc<String>,
    active_filter: A,
    handle_filter: T,
) -> impl IntoView {
    let filter_clone = Rc::clone(&filter);

    let active_class = move || match active_filter() == *filter_clone {
        true => "bg-primary-700 text-primary-50",
        false => "",
    };

    let filter_click = Rc::clone(&filter);

    view! {
        <button
            class=move || format!("px-5 py-2 hover:bg-primary-700 {}", active_class())
            on:click=move |_| handle_filter(&filter_click)
        >
            {children()}
        </button>
    }
}
