use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="grid grid-cols-3 size-full">
            <div class="col-span-2">Test</div>
            <div class="col-span-1">Test</div>
        </div>
    }
}
