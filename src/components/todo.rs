use std::collections::{HashMap, HashSet};
use leptos::control_flow::Show;
use leptos::prelude::{ClassAttribute, Get, OnAttribute, Update};
use leptos::*;
use leptos::prelude::For;
use leptos::prelude::ElementChild;
use leptos::prelude::signal;
use crate::components::modal::Modal;
use web_sys::console;
use leptos::prelude::Set;
use crate::components::models::{TodoList,Todo};



#[component]
pub fn TodoApp()-> impl IntoView{
    let (is_category_modal,set_is_category_modal)=signal(false);
    let (open_categories, set_open_categories) = signal(HashSet::<String>::new());
    let (todo_list, set_todo_list) = signal(TodoList {
        category: HashMap::from([
            ("Work".to_string(), vec![
                Todo { id: 1, title: "Complete project report".to_string() },
                Todo { id: 2, title: "Attend team meeting".to_string() },
            ]),
            ("Personal".to_string(), vec![
                Todo { id: 3, title: "Buy groceries".to_string() },
                Todo { id: 4, title: "Call mom".to_string() },
            ]),
            ("Hobby".to_string(), vec![
                Todo { id: 5, title: "Read a book".to_string() },
                Todo { id: 6, title: "Practice guitar".to_string() },
            ]),
        ])
    });
    view!{
        <p class="text-lg font-medium flex items-center justify-center">Todo App</p>
        <div>
       <button class="bg-white hover:bg-gray-100 text-gray-800 font-semibold py-2 px-4 border border-gray-400 rounded shadow cursor-pointer">
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
        let todos = list.category.get(&category).cloned().unwrap_or_default();
        let category_name=category.clone();
          view! {
              <div class="mb-4">
                  <h2 class=" font-semibold flex items-center justify-center relative box-border bg-transparent cursor-pointer select-none align-middle appearance-none text-inherit w-full font-inter text-lg shadow-lg p-4 bg-white rounded"  
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
    
                }>{category.clone()}</h2>
            
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
                          each=move || todos.clone()
                          key=|todo| todo.id
                          children=move |todo| {
                              view! {
                                  <h2>{todo.title}</h2>
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
        
 
        

    }
}