use crate::components::select_country::SelectCountry;
use leptos::*;

#[component]
pub fn Profile() -> impl IntoView {
    let countryFlag = "pt.jpg";
    let nationality = "Portugal";
    view! {
        <div>
            <h2 class="font-semibold text-2xl text-accent-400 mb-4">"Update your guest profile"</h2>

            <p class="text-lg mb-8 text-primary-200">
                "Providing the following information will make your check-in process
                faster and smoother. See you soon!"
            </p>

            <form class="bg-primary-900 py-8 px-12 text-lg flex gap-6 flex-col">
                <div class="space-y-2">
                    <label>"Full name"</label>
                    <input
                        disabled
                        class="px-5 py-3 bg-primary-200 text-primary-800 w-full shadow-sm rounded-sm disabled:cursor-not-allowed disabled:bg-gray-600 disabled:text-gray-400"
                    />
                </div>

                <div class="space-y-2">
                    <label>"Email address"</label>
                    <input
                        disabled
                        class="px-5 py-3 bg-primary-200 text-primary-800 w-full shadow-sm rounded-sm disabled:cursor-not-allowed disabled:bg-gray-600 disabled:text-gray-400"
                    />
                </div>

                <div class="space-y-2">
                    <div class="flex items-center justify-between">
                        <label htmlFor="nationality">"Where are you from?"</label>
                        <img src=countryFlag alt="Country flag" class="h-5 rounded-sm"/>
                    </div>

                    <SelectCountry
                        name="nationality"
                        id="nationality"
                        class="px-5 py-3 bg-primary-200 text-primary-800 w-full shadow-sm rounded-sm"
                        default_country=nationality
                    />
                </div>

                <div class="space-y-2">
                    <label htmlFor="nationalID">"National ID number"</label>
                    <input
                        name="nationalID"
                        class="px-5 py-3 bg-primary-200 text-primary-800 w-full shadow-sm rounded-sm"
                    />
                </div>

                <div class="flex justify-end items-center gap-6">
                    <button class="bg-accent-500 px-8 py-4 text-primary-800 font-semibold hover:bg-accent-600 transition-all disabled:cursor-not-allowed disabled:bg-gray-500 disabled:text-gray-300">
                        "Update profile"
                    </button>
                </div>
            </form>
        </div>
    }
}
