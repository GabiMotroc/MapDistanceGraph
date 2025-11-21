use leptos::prelude::*;
use leptos_leaflet::prelude::*;

use std::time::Duration;

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
        <MapContainer class="size-full" center=Position::new(44.32, 23.80) zoom=12.0 set_view=true>
            <TileLayer
                url="https://tile.openstreetmap.org/{z}/{x}/{y}.png"
                attribution="&copy; <a href=\"https://www.openstreetmap.org/copyright\">OpenStreetMap</a> contributors"
            />
            <Polygon
                color="purple"
                positions=positions(&[(51.515, -0.09), (51.52, -0.1), (51.52, -0.12)])
            >
                <Tooltip sticky=true direction="top">
                    <strong>{"I'm a polygon"}</strong>
                </Tooltip>
            </Polygon>
        </MapContainer>
    }
}
