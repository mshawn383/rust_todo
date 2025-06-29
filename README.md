# 🗂️ Category-Based Todo App in Rust + Leptos

A reactive, client-side Todo application built with [Leptos](https://leptos.dev/) and powered by fine-grained signals. This app allows users to organize their tasks by categories, offering a clean and intuitive way to manage todos.

## ✨ Features

- ➕ Add and delete categories
- 📝 Add todos under specific categories
- ❌ Delete entire categories
- 🔁 Reactive UI updates using `Signal` and `RwSignal`
- ⚡ Built with client-side rendering (CSR) using Leptos
- 🎨 Styled with Tailwind CSS (optional, if used)

## 🛠️ Tech Stack

| Layer        | Technology            |
|--------------|------------------------|
| Frontend     | Leptos `v0.8.2` (CSR)  |
| Build Tool   | Trunk `v0.21.14`       |
| Web APIs     | `web-sys`              |
| Language     | Rust                   |

## 📦 Dependencies

```toml
[dependencies]
leptos = { version = "0.8.2", features = ["csr"] }
trunk = "0.21.14"
web-sys = "0.3.77"
