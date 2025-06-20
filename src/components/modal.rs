use leptos::*;
use leptos::prelude::{ClassAttribute, Get, OnAttribute, ReadSignal, Set};

#[component]
pub fn Modal(
    show: ReadSignal<bool>,
) -> impl IntoView {
    view! {
        <div class=move || {
            format!(
                "fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center {}",
                if show.get() { "" } else { "hidden" }
            )
        }>
            <div class="bg-white rounded-lg p-6 max-w-md w-full mx-4 relative">
                // Close button (top-right)
                <button
                    on:click=move |_| show.set(false)
                    class="
                        absolute top-2 right-2
                        p-1
                        rounded-full
                        text-gray-400
                        hover:text-gray-600
                        hover:bg-gray-100
                        focus:outline-none
                        focus:ring-2
                        focus:ring-gray-400
                        transition-all
                        duration-200
                    "
                >
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        class="h-6 w-6"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke="currentColor"
                    >
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M6 18L18 6M6 6l12 12"
                        />
                    </svg>
                </button>

                // Modal content
                <h2 class="text-xl font-bold mb-4">Category</h2>
                <input
                    type="text"
                    class="w-full p-2 border border-gray-300 rounded mb-4"
                    placeholder="Enter category name"
                />

                // Action buttons
                <div class="flex justify-end space-x-2">
                    <button
                        on:click=move |_| show.set(false)
                        class="px-4 py-2 text-gray-600 hover:bg-gray-100 rounded"
                    >
                        Cancel
                    </button>
                    <button
                        class="px-4 py-2 bg-blue-500 text-white hover:bg-blue-600 rounded"
                    >
                        Save
                    </button>
                </div>
            </div>
        </div>
    }
}