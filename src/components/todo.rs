use std::collections::{HashMap, HashSet};
use leptos::control_flow::Show;
use leptos::prelude::{ClassAttribute, Get, OnAttribute, Update};
use leptos::*;
use leptos::prelude::For;
use leptos::prelude::ElementChild;
use leptos::prelude::signal;
use crate::components::modal::Modal;
use crate::components::optionmodal::OptionsModal;
use web_sys::console;
use leptos::prelude::RwSignal;
use leptos::prelude::Set;
use crate::components::models::{TodoList,Todo};



#[component]
pub fn TodoApp()-> impl IntoView{
    let (is_category_modal,set_is_category_modal)=signal(false);
    let (is_todo_modal,set_is_todo_modal)=signal(false);
    let (open_categories, set_open_categories) = signal(HashSet::<String>::new());
    let (todo_list, set_todo_list) = signal(TodoList {
        category: HashMap::from([
            ("Work".to_string(), RwSignal::new(vec![
                Todo { id: 1, title: "Complete project report".to_string() },
                Todo { id: 2, title: "Attend team meeting".to_string() },
            ])),
            ("Personal".to_string(),RwSignal::new(vec![
                Todo { id: 1, title: "Buy groceries".to_string() },
                Todo { id: 2, title: "Call mom".to_string() },
            ])),
            ("Hobby".to_string(), RwSignal::new(vec![
                Todo { id: 1, title: "Read a book".to_string() },
                Todo { id: 2, title: "Practice guitar".to_string() },
            ])),
        ])
    });
    view!{
        <p class="text-5xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-blue-500 via-purple-500 to-pink-500 flex items-center justify-center">
  Todo App
</p>
        <div>
       <button class="bg-white hover:bg-gray-100 text-gray-800 font-semibold py-2 px-4 border border-gray-400 rounded shadow cursor-pointer" on:click=move |_|set_is_todo_modal.update(|val| *val= !*val)>
   + Add Todo
</button>
 <button class="bg-white hover:bg-gray-100 text-gray-800 font-semibold py-2 px-4 border border-gray-400 rounded shadow cursor-pointer " on:click =move |_| set_is_category_modal.update(|val| *val=!*val) >
   + Add Category
</button>
        </div>


<br/>
<br/>
    <For
      each=move ||{let list = todo_list.get();
                todo_list.get().category.keys().cloned().collect::<Vec<_>>()
            }
      key=|category| category.clone()
      children=move |category| {
           let list = todo_list.get();
        let todos = list.category.get(&category).map(|s| s.get()).unwrap_or_default();
        let category_name=category.clone();
        let category_for_delete = category.clone();
        let selected_category=category.clone();

          view! {
              <div class="mb-4">
                  <div class="flex items-center justify-between p-4 bg-white rounded shadow-lg"  
                  on:click=move |_| {
                    let category = category.clone();
                    set_open_categories.set({
                        let mut new_set = open_categories.get().clone();
                        if new_set.contains(&category) {
                            new_set.remove(&category);
                        } else {
                            new_set.insert(category.clone());
                        }
                        new_set
                    });
    
                }><h2 class="text-lg font-semibold mx-auto cursor-pointer">{category.clone()}</h2>    <button
                class="ml-2 text-red-500 hover:text-red-700"
                on:click=move |_| {
                  set_todo_list.update(|list| {
                    list.category.remove(&category_for_delete);
                  });
            
                }
              >
              Delete
              </button></div>
             
            
                <div class=move || {
                    let base = "border-[2px] border-[#ecedee] p-[22px] rounded-[5px]";
                    if open_categories.get().contains(&category_name) {
                        format!("{base} block")
                    } else {
                        format!("{base} hidden")
                    }
                }
            >
            <For
            each=move || {
                todo_list
                    .get()
                    .category
                    .get(&selected_category)
                    .map(|signal| signal.get())
                    .unwrap_or_default()
            }
            key=|todo| todo.id
            children=move |todo| {
                view! {
                    <li class="list-decimal list-inside p-2">{todo.title.clone()}</li>
                }
            }
        />
                      </div>

      
              </div>
          }
            }
       />
        <Modal 
        show=is_category_modal
        set_is_category_modal=set_is_category_modal
        set_todo_list=set_todo_list
        />
        
 <OptionsModal
 show=is_todo_modal
 set_is_todo_modal=set_is_todo_modal
 todo_list=todo_list
 set_todo_list=set_todo_list
 />
        

        

    }
}