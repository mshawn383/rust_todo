use leptos::mount::mount_to_body;
use leptos::prelude::*;
mod components;
use components::TodoApp;

#[component]
fn App() -> impl IntoView {
    view! {

        <TodoApp/>
    }
}

fn main() {
    mount_to_body(App);
}
