use leptos::prelude::*;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <div class="w-full p-5 bg-purple-900 text-white text-xl">
            <a href="/">Map distance graph</a>
            <a class="text-base m-7 font-weight-400" href="map">
                Map
            </a>
        </div>
    }
}
