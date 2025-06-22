use leptos::*;
use leptos::prelude::{ClassAttribute, Get, OnAttribute, ReadSignal, Set};
use leptos::prelude::ElementChild;

#[component]
pub fn Modal(
    show: ReadSignal<bool>,
) -> impl IntoView {
    view! {
   
        <button  class="bg-blue-600 text-white px-4 py-2 rounded">
          Open Modal
        </button>
        
    
        <div  class="fixed inset-0 bg-black bg-opacity-50 hidden items-center justify-center z-50">
          <div class="bg-white p-6 rounded shadow-lg w-1/3">
            <h2 class="text-xl font-bold mb-4">Modal Title</h2>
            <p class="mb-4">This is a simple modal using Tailwind CSS.</p>
            <button  class="bg-red-500 text-white px-4 py-2 rounded">Close</button>
          </div>
        </div>
        
 
    }
}