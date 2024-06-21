use crate::{
    components::{cabin_card::CabinCard, filter::Filter, spinner::Spinner},
    model::cabin::Cabin,
    util::data_service::{all_cabins_query, CabinKey},
};
use leptos::*;
use leptos_router::use_query_map;

#[component]
pub fn Cabins() -> impl IntoView {
    let query = all_cabins_query().use_query(|| CabinKey);
    let search_query = use_query_map();

    let active_filter = move || {
        search_query
            .with(|query| query.get("capacity").cloned())
            .unwrap_or("all".to_owned())
    };

    let data = query.data;

    let renderCabin = move || {
        data.get().map(|cabin| match cabin {
            Ok(cabin) => {
                let display_cabin = move || match active_filter().as_str() {
                    "all" => cabin,
                    "small" => {
                        let filter_cabin: Vec<Cabin> = cabin
                            .into_iter()
                            .filter(|cabin| cabin.max_capacity <= 3)
                            .collect();
                        filter_cabin
                    }
                    "medium" => {
                        let filter_cabin: Vec<Cabin> = cabin
                            .into_iter()
                            .filter(|cabin| cabin.max_capacity >= 4 && cabin.max_capacity <= 7)
                            .collect();
                        filter_cabin
                    }
                    "large" => {
                        let filter_cabin: Vec<Cabin> = cabin
                            .into_iter()
                            .filter(|cabin| cabin.max_capacity >= 8)
                            .collect();
                        filter_cabin
                    }
                    _ => cabin,
                };

                display_cabin()
                    .iter()
                    .map(|cabin| {
                        view! { <CabinCard cabin=cabin.clone()/> }
                    })
                    .collect_view()
            }
            Err(err) => view! { <div>{err}</div> }.into_view(),
        })
    };

    view! {
        <div>
            <h1 class="text-4xl mb-5 text-accent-400 font-medium">"Our Luxury Cabins"</h1>
            <p class="text-primary-200 text-lg mb-10">
                "Cozy yet luxurious cabins, located right in the heart of the Italian
                Dolomites. Imagine waking up to beautiful mountain views, spending your
                days exploring the dark forests around, or just relaxing in your private
                hot tub under the stars. Enjoy nature's beauty in your own little
                home away from home. The perfect spot for a peaceful, calm vacation.
                Welcome to paradise."
            </p>

            <div class="flex justify-end mb-8">
                <Filter/>
            </div>

            <Suspense fallback=move || view! { <Spinner/> }>
                <div class="grid sm:grid-cols-1 md:grid-cols-2 gap-8 lg:gap-12 xl:gap-14">
                    {renderCabin()}
                </div>
            </Suspense>

        </div>
    }
}
