use crate::components::{spinner::Spinner, text_expander::TextExpander};
use crate::util::data_service::{single_cabin_query, SingleCabinKey};
use icondata::{FaEyeSlashSolid, FaMapLocationSolid, FaUserGroupSolid};
use leptos::*;
use leptos_icons::Icon;
use leptos_router::use_params_map;

#[component]
pub fn Cabin() -> impl IntoView {
    let params = use_params_map();
    let id = move || {
        params
            .with(|param| param.get("id").cloned())
            .unwrap()
            .parse::<u32>()
            .unwrap()
    };

    let query = single_cabin_query(id()).use_query(move || SingleCabinKey(id()));
    let data = query.data;

    view! {
        <Suspense fallback=move || {
            view! { <Spinner/> }
        }>
            {move || {
                data.get()
                    .map(|cabin| match cabin {
                        Ok(cabin) => {
                            view! {
                                <div class="max-w-6xl mx-auto mt-8">
                                    <div class="grid grid-cols-[3fr_4fr] gap-20 border border-primary-800 py-3 px-10 mb-24">
                                        <div class="relative scale-[1.2] -translate-x-3">
                                            <img
                                                src=cabin.image
                                                fill
                                                alt=&cabin.name
                                                class="object-cover h-full w-full"
                                            />
                                        </div>

                                        <div>
                                            <h3 class="text-accent-100 font-black text-7xl mb-5 translate-x-[-254px] bg-primary-950 p-6 pb-1 w-[150%]">
                                                "Cabin " {&cabin.name}
                                            </h3>

                                            <p class="text-lg text-primary-300 mb-10">
                                                <TextExpander text=cabin.description.unwrap_or_default()/>
                                            </p>

                                            <ul class="flex flex-col gap-4 mb-7">
                                                <li class="flex gap-3 items-center">
                                                    <Icon
                                                        icon=FaUserGroupSolid
                                                        style="color: rgb(76 107 138);transform: translateY(-2px)"
                                                    />
                                                    <span class="text-lg">
                                                        "For up to "
                                                        <span class="font-bold">{cabin.max_capacity}</span>
                                                        " guests"
                                                    </span>
                                                </li>
                                                <li class="flex gap-3 items-center">
                                                    <Icon
                                                        icon=FaMapLocationSolid
                                                        style="color: rgb(76 107 138); transform: translateY(-2px)"
                                                    />
                                                    <span class="text-lg">
                                                        "Located in the heart of the "
                                                        <span class="font-bold">"Dolomites"</span> (Italy)
                                                    </span>
                                                </li>
                                                <li class="flex gap-3 items-center">
                                                    <Icon
                                                        icon=FaEyeSlashSolid
                                                        style="color: rgb(76 107 138); transform: translateY(-2px)"
                                                    />
                                                    <span class="text-lg">
                                                        "Privacy " <span class="font-bold">100%</span> " guaranteed"
                                                    </span>
                                                </li>
                                            </ul>
                                        </div>
                                    </div>

                                    <div>
                                        <h2 class="text-5xl font-semibold text-center">
                                            "Reserve today. Pay on arrival."
                                        </h2>
                                    </div>
                                </div>
                            }
                                .into_view()
                        }
                        Err(err) => view! { <div>{err}</div> }.into_view(),
                    })
            }}

        </Suspense>
    }
}
