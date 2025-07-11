use crate::components::models::{Todo, TodoList};
use leptos::leptos_dom::logging::console_log;
use leptos::prelude::signal;
use leptos::prelude::ClassAttribute;
use leptos::prelude::Effect;
use leptos::prelude::ElementChild;
use leptos::prelude::For;
use leptos::prelude::Get;
use leptos::prelude::OnAttribute;
use leptos::prelude::OnTargetAttribute;
use leptos::prelude::ReadSignal;
use leptos::prelude::RwSignal;
use leptos::prelude::Set;
use leptos::prelude::Update;
use leptos::prelude::WriteSignal;
use leptos::*;
use web_sys::console;

#[component]
pub fn OptionsModal(
    show: ReadSignal<bool>,
    set_is_todo_modal: WriteSignal<bool>,
    todo_list: ReadSignal<TodoList>,
    set_todo_list: WriteSignal<TodoList>,
) -> impl IntoView {
    let (select_value, set_select_value) = signal::<String>("".to_string());
    let (todo, set_todo) = signal::<String>("".to_string());
    Effect::new(move |_| {
        let current = todo_list.get();
        console_log(&format!("UI re-rendered: {:?}", current));
    });

    view! {
            <div class=move || {
                let base = "fixed inset-0 flex items-center justify-center bg-black bg-opacity-50";
                if show.get() {
                    format!("{base} block")
                } else {
                    format!("{base} hidden")
                }
            }>
                <div class="bg-white w-full max-w-md rounded-lg shadow-lg p-6">
                    <div class="flex justify-between items-center border-b pb-2 mb-4">
                        <h2 class="text-xl font-semibold">Add Category</h2>
                    </div>

                    <select class="w-full border border-gray-300 rounded px-3 py-2 mb-4 focus:outline-none focus:ring-2 focus:ring-blue-500"
                    on:input:target=move |ev|{
                        set_select_value.set(ev.target().value())
                    }
                    >
                    <option value="">Select a category</option>
                    <For
                    each=move || {
                        todo_list.get().category.keys().cloned().collect::<Vec<_>>()
                    }
                    key=|category| category.clone()
                    children=move |category| {
                        let category_name=category.clone();
                        let category_name_value=category.clone();
    view!{

        <option value={category_name_value}>{category_name}</option>

    }
                    }
                    />

                    </select>
                    <input
                    type="text"
                    placeholder="Add Todo"
                    class="w-full border border-gray-300 rounded px-3 py-2 mb-4 focus:outline-none focus:ring-2 focus:ring-blue-500"
                    on:input:target=move |ev|{
                        if ev.target().value() !=""{
                            set_todo.set(ev.target().value())
                        }

                    }

                  />
                    <div class="flex justify-end space-x-2">
                        <button class="px-4 py-2 bg-gray-200 text-gray-700 rounded hover:bg-gray-300" on:click=move |_| set_is_todo_modal.set(false)>
                            Close
                        </button>
                        <button class="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700" on:click=move |_|{
                            let category = select_value.get();
                            let todo_value = todo.get();

                            set_todo_list.update(|list| {
                                let category_signal = list
                                    .category
                                    .entry(category.clone())
                                    .or_insert_with(|| RwSignal::new(Vec::new()));
                                let id = category_signal.get().len() as u32 + 1;
                                category_signal.update(|todos| {
                                    todos.push(Todo { id, title: todo_value });
                                });
                            });
                  set_is_todo_modal.set(false)

                        }>
                            Add
                        </button>
                    </div>
                </div>
            </div>
        }
}
