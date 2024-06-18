use crate::components::{account_layout::AccountLayout, app_layout::AppLayout};
use crate::error_template::{AppError, ErrorTemplate};
use crate::page::{
    about::About, account::Account, cabin::Cabin, cabins::Cabins, profile::Profile,
    reservations::Reservations,
};
use leptos::*;
use leptos_meta::*;
use leptos_query::provide_query_client;
use leptos_router::A;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    // Provides leptos query client
    provide_query_client();

    view! {
        <Stylesheet id="leptos" href="/pkg/the-wild-oasis-leptos-ssr.css"/>

        // sets the document title
        <Title text="The Wild Oasis"/>

        // sets the icon
        <Link rel="icon" type_="image/x-icon" href="/logo.png"/>

        // sets style for body
        <Body class="bg-primary-950 text-primary-100 min-h-screen flex flex-col antialiased"/>

        // sets font
        <Link rel="preconnect" href="https://fonts.googleapis.com"/>
        <Link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous"/>
        <Link
            href="https://fonts.googleapis.com/css2?family=Josefin+Sans:ital,wght@0,100..700;1,100..700&display=swap"
            rel="stylesheet"
        />

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <Routes>
                <Route path="/" view=AppLayout>
                    <Route path="/" view=HomePage/>
                    <Route path="/about" view=About/>
                    <Route path="/account" view=AccountLayout>
                        <Route path="/" view=Account/>
                        <Route path="/profile" view=Profile/>
                        <Route path="/reservations" view=Reservations/>
                    </Route>
                    <Route path="/cabins" view=Cabins/>
                    <Route path="/cabins/:id" view=Cabin/>
                </Route>
            </Routes>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <main class="mt-24">
            <img
                src="/bg.png"
                alt="Mountains and forests with two cabins"
                class="object-cover object-top w-full h-full absolute inset-0 text-transparent"
                sizes="100vw"
            />

            <div class="relative z-10 text-center">
                <h1 class="text-8xl text-primary-50 mb-10 tracking-tight font-normal">
                    "Welcome to paradise."
                </h1>
                <A
                    href="/cabins"
                    class="bg-accent-500 px-8 py-6 text-primary-800 text-lg font-semibold hover:bg-accent-600 transition-all"
                >
                    "Explore luxury cabins"
                </A>
            </div>
        </main>
    }
}
