use crate::util::data_service::{all_country_query, CountryKey};
use leptos::*;

#[component]
pub fn SelectCountry(
    name: &'static str,
    id: &'static str,
    default_country: &'static str,
    class: &'static str,
) -> impl IntoView {
    let query = all_country_query().use_query(|| CountryKey);
    let data = query.data;

    let country = move || {
        data.with(|country| match country {
            Some(country) => match country {
                Ok(country) => {
                    if country.is_empty() {
                        view! { <p>"No country found"</p> }.into_view()
                    } else {
                        country
                            .into_iter()
                            .map(|country| {
                                view! {
                                    <option
                                        selected=default_country == country.clone().name
                                        value=format!("{}%{}", country.name, country.flag)
                                    >
                                        {country.clone().name}
                                    </option>
                                }
                            })
                            .collect_view()
                    }
                }
                Err(err) => view! { <p>{err.to_string()}</p> }.into_view(),
            },
            None => view! {}.into_view(),
        })
    };

    view! {
        <Suspense fallback=move || view! { <span>"Loading..."</span> }>
            <select name=name id=id class=class>
                <option value="">"Select country..."</option>
                {country}
            </select>
        </Suspense>
    }
}
