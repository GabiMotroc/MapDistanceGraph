mod components;
mod pages;

use crate::components::navbar::Navbar;
use crate::pages::home::Home;
use crate::pages::map::Map;
use leptos::prelude::*;
use leptos::{view, IntoView};
use leptos_meta::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/style/output.css" />
        <div class="flex flex-col h-full">
            <Navbar />
            <Router>
                <main class="bg-black text-white flex flex-grow justify-center items-center">
                    <Routes fallback=|| "Not found">
                        <Route path=path!("/") view=Home />
                        <Route path=path!("/map") view=Map />
                    </Routes>
                </main>
            </Router>
        </div>
    }
}
