use leptos::mount::mount_to_body;
use leptos::prelude::*;
mod components; 
use components::TodoApp; 
use components::Modal; 

#[component]
fn App() -> impl IntoView {
    view! {
    
        <TodoApp/>
        // <Modal/>
    }
}

fn main() {
    mount_to_body(App);
}