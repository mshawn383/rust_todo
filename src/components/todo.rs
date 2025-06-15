use std::collections::HashMap;

use leptos::prelude::ClassAttribute;
use leptos::*;
use leptos::prelude::For;
use leptos::prelude::ElementChild;
use leptos::prelude::signal;

struct TodoList {
   category:HashMap::<String, Vec<Todo>>,
}

struct Todo {
    id: u32,
    title: String
}
#[component]
pub fn TodoApp()-> impl IntoView{
   
    let (todo_list, set_todo_list) = signal(TodoList {
        category: HashMap::from([
            ("work".to_string(), vec![
                Todo { id: 1, title: "Complete project report".to_string() },
                Todo { id: 2, title: "Attend team meeting".to_string() },
            ]),
            ("personal".to_string(), vec![
                Todo { id: 3, title: "Buy groceries".to_string() },
                Todo { id: 4, title: "Call mom".to_string() },
            ]),
            ("hobby".to_string(), vec![
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
 <button class="bg-white hover:bg-gray-100 text-gray-800 font-semibold py-2 px-4 border border-gray-400 rounded shadow cursor-pointer">
   + Add Category
</button>
        </div>
       
      {
            
    <For
      each=move || todo_list().category.keys();
      key=|category| category.clone()
      children=move |category| {
          let todos = todo_list.get().category.get(&category).unwrap_or(&vec![]);
          view! {
              <div class="mb-4">
                  <h2 class="text-xl font-semibold mb-2">{category}</h2>
                  <ul class="list-disc pl-5">
                      <For
                          each=move || todos.clone()
                          key=|todo| todo.id
                          children=move |todo| {
                              view! {
                                  <li>{todo.title}</li>
                              }
                          }
                      />
                  </ul>
              </div>
          } 
       />
    
       
       
       
    }
}
      
    }
}