use leptos::prelude::*;

#[component]
pub fn UiButton(text: String) -> impl IntoView {
    view! {
        <button class="shadow-md bg-purple-500 hover:bg-purple-700 py-2 px-4 rounded m-1">
            {{ text }}
        </button>
    }
}
