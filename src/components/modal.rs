use leptos::*;
use leptos::prelude::{ClassAttribute, Get, OnAttribute, ReadSignal, Set};
use leptos::prelude::ElementChild;
use leptos::prelude::WriteSignal;
use std::collections::{HashMap};
use crate::components::models::{TodoList,Todo};
use leptos::prelude::OnTargetAttribute;
use leptos::prelude::signal;
use leptos::prelude::RwSignal;
use leptos::prelude::Update;


#[component]
pub fn Modal(
    show: ReadSignal<bool>,
    set_is_category_modal:WriteSignal<bool>,
    set_todo_list:WriteSignal<TodoList>
) -> impl IntoView {
  let (category,set_category)=signal::<String>("".to_string());
    view! {
   
      <div class=move || {
        let base = "fixed inset-0 flex items-center justify-center bg-black bg-opacity-50";
        if show.get() {
            format!("{base} block")
        } else {
            format!("{base} hidden")
        }
      } >
        <div class="bg-white w-full max-w-md rounded-lg shadow-lg p-6">
          
          <div class="flex justify-between items-center border-b pb-2 mb-4">
            <h2 class="text-xl font-semibold">Add Category</h2>
    
            // <span class="cursor-pointer text-gray-600 hover:text-gray-900 font-bold">&times;</span>
          </div>
      
        
          <input 
            type="text" 
            placeholder="Add Category Here" 
            class="w-full border border-gray-300 rounded px-3 py-2 mb-4 focus:outline-none focus:ring-2 focus:ring-blue-500"
            on:input:target=move |ev| {
              set_category.set(ev.target().value())
           
          }
          
          />
      
        
          <div class="flex justify-end space-x-2">
            <button class="px-4 py-2 bg-gray-200 text-gray-700 rounded hover:bg-gray-300" on:click=move |_| set_is_category_modal.set(false)>
              Close
            </button>
            <button class="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700" on:click=move |_| {
              set_todo_list.update(|todo_list| {
                let key = category.get();
                todo_list.category.entry(key).or_insert_with(|| RwSignal::new(Vec::new()));
            });
            set_is_category_modal.set(false)
            }>
              Add
            </button>
          </div>
          
        </div>
      </div>
        
 
    }
}