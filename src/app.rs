use crate::components::app_layout::AppLayout;
use crate::error_template::{AppError, ErrorTemplate};
use crate::page::{about::About, account::Account, cabins::Cabins};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    leptos_image::provide_image_context();

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
                    <Route path="/account" view=Account/>
                    <Route path="/cabins" view=Cabins/>
                </Route>
            </Routes>

        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <div>
            <h1>"The Wild Oasis. Welcome to paradise"</h1>
        </div>
    }
}
