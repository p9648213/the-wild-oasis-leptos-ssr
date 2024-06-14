use crate::components::side_navigation::SideNavigation;
use leptos::*;
use leptos_router::Outlet;

#[component]
pub fn AccountLayout() -> impl IntoView {
    view! {
        <div class="grid grid-cols-[16rem_1fr] h-full gap-12">
            <SideNavigation/>
            <div class="py-1">
                <Outlet/>
            </div>
        </div>
    }
}
