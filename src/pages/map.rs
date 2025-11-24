use leptos::prelude::*;
use leptos_leaflet::prelude::*;

use std::time::Duration;
use crate::components::base::button::UiButton;

/// Documentation
/// https://github.com/headless-studio/leptos-leaflet/blob/main/examples/simple-map/src/app.rs
#[component]
pub fn Map() -> impl IntoView {
    let (marker_position, set_marker_position) =
        JsRwSignal::new_local(Position::new(51.49, -0.08)).split();

    Effect::new(move |_| {
        set_interval_with_handle(
            move || {
                set_marker_position.update(|pos| {
                    pos.lat += 0.001;
                    pos.lng += 0.001;
                });
            },
            Duration::from_millis(200),
        )
        .ok()
    });

    view! {
        <div class="flex size-full">
            <MapContainer
                class="size-full w-2/3 flex-auto"
                center=Position::new(44.32, 23.80)
                zoom=12.0
                set_view=true
            >
                <TileLayer
                    url="https://tile.openstreetmap.org/{z}/{x}/{y}.png"
                    attribution="&copy; <a href=\"https://www.openstreetmap.org/copyright\">OpenStreetMap</a> contributors"
                />
                <MapMenu />
                <Polygon
                    color="purple"
                    positions=positions(&[(51.515, -0.09), (51.52, -0.1), (51.52, -0.12)])
                >
                    <Tooltip sticky=true direction="top">
                        <strong>{"I'm a polygon"}</strong>
                    </Tooltip>
                </Polygon>
            </MapContainer>
            <div class="flex flex-col min-w-1/6">
                <UiButton on:click=|_| {} text="as".to_string()></UiButton>
                <UiButton on:click=|_| {} text="as".to_string()></UiButton>
            </div>
        </div>
    }
}

#[component]
pub fn MapMenu() -> impl IntoView {
    view! { <button class="z-5000 bg-black/75 w-50 h-50 leaflet-bottom leaflet-right">test</button> }
}
