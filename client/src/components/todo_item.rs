use leptos::*;
use platform_core::{Todo, TodoStatus, TodoUpdate};
use uuid::Uuid;

use crate::api;

#[component]
pub fn TodoItem(
    todo: Todo,
    on_updated: Callback<Todo>,
    on_deleted: Callback<Uuid>,
) -> impl IntoView {
    let (is_editing, set_is_editing) = create_signal(false);
    let (title, set_title) = create_signal(todo.title.clone());
    let (description, set_description) = create_signal(todo.description.clone());
    let (status, set_status) = create_signal(todo.status);
    let (is_updating, set_is_updating) = create_signal(false);

    let todo_id = todo.id;

    let handle_status_change = move |_| {
        let on_updated = on_updated.clone();
        let new_status = match status.get() {
            TodoStatus::Pending => TodoStatus::InProgress,
            TodoStatus::InProgress => TodoStatus::Completed,
            TodoStatus::Completed => TodoStatus::Pending,
            TodoStatus::Cancelled => TodoStatus::Pending,
        };

        set_is_updating.set(true);
        spawn_local(async move {
            let update = TodoUpdate {
                title: None,
                description: None,
                status: Some(new_status),
            };

            match api::update_todo(todo_id, update).await {
                Ok(updated_todo) => {
                    set_status.set(updated_todo.status);
                    on_updated.call(updated_todo);
                }
                Err(e) => {
                    log::error!("Failed to update todo: {}", e);
                }
            }
            set_is_updating.set(false);
        });
    };

    let handle_save = move |_| {
        let on_updated = on_updated.clone();
        set_is_updating.set(true);
        spawn_local(async move {
            let update = TodoUpdate {
                title: Some(title.get()),
                description: Some(description.get().unwrap_or_default()),
                status: None,
            };

            match api::update_todo(todo_id, update).await {
                Ok(updated_todo) => {
                    set_title.set(updated_todo.title.clone());
                    set_description.set(updated_todo.description.clone());
                    on_updated.call(updated_todo);
                    set_is_editing.set(false);
                }
                Err(e) => {
                    log::error!("Failed to update todo: {}", e);
                }
            }
            set_is_updating.set(false);
        });
    };

    let handle_delete = move |_| {
        let on_deleted = on_deleted.clone();
        set_is_updating.set(true);
        spawn_local(async move {
            match api::delete_todo(todo_id).await {
                Ok(_) => {
                    on_deleted.call(todo_id);
                }
                Err(e) => {
                    log::error!("Failed to delete todo: {}", e);
                    set_is_updating.set(false);
                }
            }
        });
    };

    let border_color = move || match status.get() {
        TodoStatus::Pending => "border-slate-400",
        TodoStatus::InProgress => "border-amber-500",
        TodoStatus::Completed => "border-green-500",
        TodoStatus::Cancelled => "border-red-500",
    };

    let status_badge_color = move || match status.get() {
        TodoStatus::Pending => "bg-slate-100 text-slate-700",
        TodoStatus::InProgress => "bg-amber-100 text-amber-700",
        TodoStatus::Completed => "bg-green-100 text-green-700",
        TodoStatus::Cancelled => "bg-red-100 text-red-700",
    };

    view! {
        <div class=move || format!("bg-white p-6 rounded-xl shadow-md border-l-4 {} transition-all hover:-translate-y-1 hover:shadow-lg", border_color())>
            {move || {
                if is_editing.get() {
                    view! {
                        <div class="space-y-4">
                            <input
                                type="text"
                                prop:value=title
                                on:input=move |ev| set_title.set(event_target_value(&ev))
                                placeholder="Title"
                                class="w-full px-4 py-2 border-2 border-slate-200 rounded-lg focus:outline-none focus:border-blue-500 transition-colors"
                            />
                            <textarea
                                prop:value=move || description.get().unwrap_or_default()
                                on:input=move |ev| set_description.set(Some(event_target_value(&ev)))
                                placeholder="Description (optional)"
                                class="w-full px-4 py-2 border-2 border-slate-200 rounded-lg focus:outline-none focus:border-blue-500 transition-colors min-h-[80px] resize-y"
                            />
                            <div class="flex gap-2 flex-wrap">
                                <button 
                                    on:click=handle_save 
                                    disabled=is_updating
                                    class="px-4 py-2 bg-blue-600 text-white rounded-lg font-semibold hover:bg-blue-700 transition-colors disabled:opacity-60 disabled:cursor-not-allowed"
                                >
                                    "Save"
                                </button>
                                <button 
                                    on:click=move |_| set_is_editing.set(false)
                                    class="px-4 py-2 bg-slate-600 text-white rounded-lg font-semibold hover:bg-slate-700 transition-colors"
                                >
                                    "Cancel"
                                </button>
                            </div>
                        </div>
                    }
                        .into_view()
                } else {
                    view! {
                        <div class="space-y-4">
                            <div>
                                <h3 class="text-xl font-semibold text-slate-800 mb-2">{title}</h3>
                                {move || {
                                    description
                                        .get()
                                        .map(|desc| {
                                            view! { <p class="text-slate-600">{desc}</p> }
                                        })
                                }}
                            </div>
                            <div class="flex items-center gap-3">
                                <span class=move || format!("inline-block px-3 py-1 rounded-full text-sm font-semibold {}", status_badge_color())>
                                    {move || format!("{:?}", status.get())}
                                </span>
                            </div>
                            <div class="flex gap-2 flex-wrap">
                                <button
                                    on:click=handle_status_change
                                    disabled=is_updating
                                    class="px-4 py-2 bg-blue-600 text-white rounded-lg text-sm font-semibold hover:bg-blue-700 transition-colors disabled:opacity-60 disabled:cursor-not-allowed"
                                >
                                    "Toggle Status"
                                </button>
                                <button 
                                    on:click=move |_| set_is_editing.set(true)
                                    class="px-4 py-2 bg-slate-600 text-white rounded-lg text-sm font-semibold hover:bg-slate-700 transition-colors"
                                >
                                    "Edit"
                                </button>
                                <button
                                    on:click=handle_delete
                                    disabled=is_updating
                                    class="px-4 py-2 bg-red-600 text-white rounded-lg text-sm font-semibold hover:bg-red-700 transition-colors disabled:opacity-60 disabled:cursor-not-allowed"
                                >
                                    "Delete"
                                </button>
                            </div>
                        </div>
                    }
                        .into_view()
                }
            }}

        </div>
    }
}

