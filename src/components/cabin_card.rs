use crate::model::cabin::Cabin;
use icondata::HiUserSolidLg;
use leptos::*;
use leptos_icons::Icon;
use leptos_router::A;

#[component]
pub fn CabinCard(cabin: Cabin) -> impl IntoView {
    let price = if cabin.discount > 0 {
        view! {
            <span class="text-3xl font-[350]">{cabin.regular_price - cabin.discount}</span>
            <span class="line-through font-semibold text-primary-600">{cabin.regular_price}</span>
        }
        .into_view()
    } else {
        view! { <span class="text-3xl font-[350]">{cabin.regular_price}</span> }.into_view()
    };

    view! {
        <div class="flex border-primary-800 border">
            // <img
            // src={image}
            // alt={`Cabin ${name}`}
            // class="flex-1 border-r border-primary-800"
            // />

            <div class="flex-grow">
                <div class="pt-5 pb-4 px-7 bg-primary-950">
                    <h3 class="text-accent-500 font-semibold text-2xl mb-3">
                        "Cabin" {cabin.name}
                    </h3>

                    <div class="flex gap-3 items-center mb-2">
                        <Icon icon=HiUserSolidLg class="h-5 w-5 text-primary-600"/>
                        <p class="text-lg text-primary-200">
                            "For up to" <span class="font-bold">{cabin.max_capacity}</span> "guests"
                        </p>
                    </div>

                    <p class="flex gap-3 justify-end items-baseline">{price}</p>
                </div>

                <div class="bg-primary-950 border-t border-t-primary-800 text-right">
                    <A
                        href=format!("/cabins/{}", cabin.id.unwrap_or_default())
                        class="border-l border-primary-800 py-4 px-6 inline-block hover:bg-accent-600 transition-all hover:text-primary-900"
                    >
                        "Details & reservation &rarr;"
                    </A>
                </div>
            </div>
        </div>
    }
}
