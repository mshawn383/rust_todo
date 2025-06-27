use std::collections::{HashMap};

#[derive(Clone)]
pub struct TodoList {
  pub category:HashMap::<String, Vec<Todo>>,
}
#[derive(Clone)]
pub struct Todo {
    pub id: u32,
   pub  title: String
}