use crate::{
    components::cabin_card::CabinCard,
    model::cabin::Cabin,
    util::data_service::{all_cabins_query, CabinKey},
};
use leptos::*;

#[component]
pub fn Cabins() -> impl IntoView {
    let cabins: Vec<Cabin> = vec![];
    let query = all_cabins_query().use_query(|| CabinKey);
    let data = query.data;

    create_effect(move |_| {
        logging::log!("{:#?}", data.get());
    });

    let renderCabin = cabins
        .iter()
        .map(|cabin| {
            view! {
                <div class="grid sm:grid-cols-1 md:grid-cols-2 gap-8 lg:gap-12 xl:gap-14">
                    <CabinCard cabin=cabin.clone()/>
                </div>
            }
        })
        .collect_view();

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
            <Show when=move || { cabins.len() > 0 }>{renderCabin.clone()}</Show>
        </div>
    }
}
