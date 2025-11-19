use leptos::{view, IntoView};
use leptos::prelude::*;
use leptos_meta::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/style/output.css" />
        <p class="bg-blue-500 text-black">"Hello, world!"</p>
    }
}
