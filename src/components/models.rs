use leptos::prelude::RwSignal;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct TodoList {
    pub category: HashMap<String, RwSignal<Vec<Todo>>>,
}
#[derive(Clone, Debug)]
pub struct Todo {
    pub id: u32,
    pub title: String,
}
