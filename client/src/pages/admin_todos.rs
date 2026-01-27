use leptos::*;
use platform_core::{Todo, TodoStatus};

use crate::api;
use crate::components::ui::{DataTable, Column};

#[component]
fn DeleteButton(todo_id: uuid::Uuid) -> impl IntoView {
    let (deleting, set_deleting) = create_signal(false);
    
    let handle_delete = move |_| {
        set_deleting.set(true);
        spawn_local(async move {
            match api::delete_todo(todo_id).await {
                Ok(_) => {
                    // Trigger a page reload to refresh the list
                    if let Some(window) = web_sys::window() {
                        let _ = window.location().reload();
                    }
                }
                Err(e) => {
                    log::error!("Failed to delete todo: {}", e);
                    set_deleting.set(false);
                }
            }
        });
    };
    
    view! {
        <button
            on:click=handle_delete
            disabled=deleting.get()
            class="px-3 py-1 bg-red-600 text-white rounded-lg text-xs font-semibold hover:bg-red-700 transition-colors disabled:opacity-60"
        >
            {move || if deleting.get() { "Deleting..." } else { "Delete" }}
        </button>
    }
}

#[component]
pub fn AdminTodosPage() -> impl IntoView {
    let (todos, set_todos) = create_signal(Vec::<Todo>::new());
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal(None::<String>);

    // Load todos on mount
    create_effect(move |_| {
        spawn_local(async move {
            set_loading.set(true);
            match api::fetch_todos().await {
                Ok(fetched_todos) => {
                    set_todos.set(fetched_todos);
                    set_error.set(None);
                }
                Err(e) => {
                    set_error.set(Some(e));
                }
            }
            set_loading.set(false);
        });
    });

    // Define table columns
    let columns = vec![
        Column::new("Title", |todo: &Todo| {
            view! {
                <span class="font-medium text-slate-900">{todo.title.clone()}</span>
            }
            .into_view()
        })
        .with_width("30%"),
        Column::new("Description", |todo: &Todo| {
            view! {
                <span class="text-slate-600">
                    {todo.description.clone().unwrap_or_else(|| "—".to_string())}
                </span>
            }
            .into_view()
        })
        .with_width("35%"),
        Column::new("Status", |todo: &Todo| {
            let (bg_color, text_color) = match todo.status {
                TodoStatus::Pending => ("bg-slate-100", "text-slate-700"),
                TodoStatus::InProgress => ("bg-amber-100", "text-amber-700"),
                TodoStatus::Completed => ("bg-green-100", "text-green-700"),
                TodoStatus::Cancelled => ("bg-red-100", "text-red-700"),
            };
            view! {
                <span class=format!("inline-block px-3 py-1 rounded-full text-xs font-semibold {} {}", bg_color, text_color)>
                    {format!("{:?}", todo.status)}
                </span>
            }
            .into_view()
        })
        .with_width("15%"),
        Column::new("Created", |todo: &Todo| {
            view! {
                <span class="text-slate-600 text-xs">
                    {todo.created_at.format("%Y-%m-%d %H:%M").to_string()}
                </span>
            }
            .into_view()
        })
        .with_width("12%"),
        Column::new("Actions", |todo: &Todo| {
            let todo_id = todo.id;
            view! {
                <DeleteButton todo_id=todo_id/>
            }
            .into_view()
        })
        .with_width("8%"),
    ];

    view! {
        <div>
            <div class="mb-6">
                <h1 class="text-3xl font-bold text-slate-800 mb-2">"Todos - Admin View"</h1>
                <p class="text-slate-600">"Professional table view with data management"</p>
            </div>

            {move || {
                if let Some(err) = error.get() {
                    view! { 
                        <div class="text-center p-8 bg-white rounded-xl shadow-md text-red-600">
                            "Error: " {err}
                        </div> 
                    }.into_view()
                } else {
                    view! {
                        <DataTable
                            data=Signal::derive(move || todos.get())
                            columns=columns.clone()
                            key_fn=|todo: &Todo| todo.id.to_string()
                            loading=loading.get()
                            empty_message="No todos yet. Create one from the Home page!".to_string()
                        />
                    }.into_view()
                }
            }}
        </div>
    }
}
