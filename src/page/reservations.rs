use icondata::HiArrowSmallRightOutlineLg;
use leptos::*;
use leptos_icons::Icon;
use leptos_router::A;

#[component]
pub fn Reservations() -> impl IntoView {
    let bookings: Vec<u32> = vec![];
    view! {
        <div>
            <h2 class="font-semibold text-2xl text-accent-400 mb-7">Your reservations</h2>

            <Show
                when=move || bookings.len() != 0
                fallback=move || {
                    view! {
                        <p class="text-lg">
                            "You have no reservations yet. Check out our" " "
                            <A class="text-accent-500 reservation-icon" href="/cabins">
                                <span>
                                    "luxury cabins "
                                    <Icon
                                        class="inline-block -translate-y-[1px]"
                                        icon=HiArrowSmallRightOutlineLg
                                    />
                                </span>
                            </A>
                        </p>
                    }
                }
            >

                <ul class="space-y-6">
                    <li>"Todo"</li>
                </ul>
            </Show>
        </div>
    }
}
