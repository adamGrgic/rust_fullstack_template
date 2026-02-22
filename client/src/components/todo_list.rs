use leptos::*;
use platform_core::Todo;

use crate::api;
use crate::components::todo_form::TodoForm;
use crate::components::todo_item::TodoItem;

#[component]
pub fn TodoList() -> impl IntoView {
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

    let on_todo_created = Callback::new(move |new_todo: Todo| {
        set_todos.update(|todos| {
            todos.insert(0, new_todo);
        });
    });

    let on_todo_updated = Callback::new(move |updated_todo: Todo| {
        set_todos.update(|todos| {
            if let Some(todo) = todos.iter_mut().find(|t| t.id == updated_todo.id) {
                *todo = updated_todo;
            }
        });
    });

    let on_todo_deleted = Callback::new(move |id: uuid::Uuid| {
        set_todos.update(|todos| {
            todos.retain(|t| t.id != id);
        });
    });

    view! {
        <div class="max-w-3xl mx-auto">
            <TodoForm on_created=on_todo_created/>

            {move || {
                if loading.get() {
                    view! { 
                        <div class="text-center p-8 bg-white rounded-xl shadow-md">
                            "Loading todos..."
                        </div> 
                    }.into_view()
                } else if let Some(err) = error.get() {
                    view! { 
                        <div class="text-center p-8 bg-white rounded-xl shadow-md text-red-600">
                            "Error: " {err}
                        </div> 
                    }.into_view()
                } else {
                    let todos_list = todos.get();
                    if todos_list.is_empty() {
                        view! { 
                            <div class="text-center p-8 bg-white rounded-xl shadow-md text-slate-600">
                                "No todos yet. Create one above!"
                            </div> 
                        }
                            .into_view()
                    } else {
                        view! {
                            <div class="space-y-4">
                                <For
                                    each=move || todos.get()
                                    key=|todo| todo.id
                                    children=move |todo| {
                                        view! {
                                            <TodoItem
                                                todo=todo
                                                on_updated=on_todo_updated
                                                on_deleted=on_todo_deleted
                                            />
                                        }
                                    }
                                />
                            </div>
                        }
                            .into_view()
                    }
                }
            }}

        </div>
    }
}

