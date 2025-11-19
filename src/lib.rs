mod components;
mod pages;

use leptos::prelude::*;
use leptos::svg::path;
use leptos::{view, IntoView};
use leptos_meta::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::*;
use leptos::prelude::ElementChild;
use crate::pages::Page::Home;


#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/style/output.css" />

        <Navbar />
        <Router>
            <Routes fallback=|| "Not found">
                <Route path=path!("/") view=Home />
            </Routes>

        </Router>
    }
}
