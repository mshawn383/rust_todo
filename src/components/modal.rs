use leptos::control_flow::Show;
use leptos::prelude::signal;
use leptos::prelude::ElementChild;
use leptos::prelude::Get;
use leptos::*;
use leptos::prelude::ClassAttribute;

pub fn Modal(// #[prop(default = false)]
    // isopen:ReadSignal<bool>, close: impl Fn() + 'static
) -> impl IntoView {
    let (is_open, _set_is_open) = signal(true);
    view! {

                    <Show when=move || is_open.get()
        fallback=|| ()>
         <div class="modal_overlay_style">
         <div class="modal_content">
         <h2>Category</h2><input type="text" class="modal_input_style" placeholder="Enter category name"/>
         </div>
         </div></Show>




    }
}
