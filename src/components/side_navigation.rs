use crate::components::signout_button::SignoutButton;
use icondata::{HiCalendarDaysSolidLg, HiHomeModernSolidLg, HiUserSolidLg};
use leptos::*;
use leptos_icons::Icon;
use leptos_router::A;

#[component]
pub fn SideNavigation() -> impl IntoView {
    view! {
        <nav class="border-r border-primary-900">
            <ul class="flex flex-col gap-2 h-full text-lg">
                <li>
                    <A
                        class="py-3 px-5 hover:bg-primary-900 hover:text-primary-100 transition-colors flex items-center gap-4 font-semibold text-primary-200 nav-icon"
                        href="/account"
                    >
                        <Icon icon=HiHomeModernSolidLg class="inline-block -translate-y-[3px]"/>
                        <span>"Home"</span>
                    </A>
                </li>

                <li>
                    <A
                        class="py-3 px-5 hover:bg-primary-900 hover:text-primary-100 transition-colors flex items-center gap-4 font-semibold text-primary-200 nav-icon"
                        href="/account/reservations"
                    >
                        <Icon icon=HiCalendarDaysSolidLg class="inline-block -translate-y-[3px]"/>
                        <span>"Reservations"</span>
                    </A>
                </li>

                <li>
                    <A
                        class="py-3 px-5 hover:bg-primary-900 hover:text-primary-100 transition-colors flex items-center gap-4 font-semibold text-primary-200 nav-icon"
                        href="/account/profile"
                    >
                        <Icon icon=HiUserSolidLg class="inline-block -translate-y-[3px]"/>
                        <span>"Guest profile"</span>
                    </A>
                </li>

                <li class="mt-auto">
                    <SignoutButton/>
                </li>
            </ul>
        </nav>
    }
}
