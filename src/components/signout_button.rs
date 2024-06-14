use icondata::HiArrowRightOnRectangleSolidLg;
use leptos::*;
use leptos_icons::Icon;

#[component]
pub fn SignoutButton() -> impl IntoView {
    view! {
        <button class="py-3 px-5 hover:bg-primary-900 hover:text-primary-100 transition-colors flex items-center gap-4 font-semibold text-primary-200 w-full">
            <Icon icon=HiArrowRightOnRectangleSolidLg class="h-5 w-5 text-primary-600"/>
            <span>"Sign out"</span>
        </button>
    }
}
