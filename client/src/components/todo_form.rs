use leptos::*;
use platform_core::{Todo, TodoCreate};

use crate::api;

#[component]
pub fn TodoForm(on_created: Callback<Todo>) -> impl IntoView {
    let (title, set_title) = create_signal(String::new());
    let (description, set_description) = create_signal(String::new());
    let (is_creating, set_is_creating) = create_signal(false);
    let (error, set_error) = create_signal(None::<String>);

    let handle_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();

        let title_value = title.get();
        if title_value.trim().is_empty() {
            set_error.set(Some("Title is required".to_string()));
            return;
        }

        let on_created = on_created.clone();
        set_is_creating.set(true);
        set_error.set(None);

        spawn_local(async move {
            let new_todo = TodoCreate {
                title: title_value,
                description: if description.get().is_empty() {
                    None
                } else {
                    Some(description.get())
                },
                status: None,
            };

            match api::create_todo(new_todo).await {
                Ok(created_todo) => {
                    on_created.call(created_todo);
                    set_title.set(String::new());
                    set_description.set(String::new());
                }
                Err(e) => {
                    set_error.set(Some(e));
                }
            }
            set_is_creating.set(false);
        });
    };

    view! {
        <div class="bg-white p-6 rounded-xl shadow-md mb-8">
            <h2 class="text-2xl font-semibold mb-6 text-slate-800">"Create New Todo"</h2>
            <form on:submit=handle_submit class="space-y-4">
                <div>
                    <input
                        type="text"
                        prop:value=title
                        on:input=move |ev| set_title.set(event_target_value(&ev))
                        placeholder="Todo title"
                        disabled=is_creating
                        style="border-color: #e2e8f0;"
                        class="w-full px-4 py-3 border-2 rounded-lg focus:outline-none transition-colors disabled:opacity-60 disabled:cursor-not-allowed focus:border-primary"
                    />
                </div>
                <div>
                    <textarea
                        prop:value=description
                        on:input=move |ev| set_description.set(event_target_value(&ev))
                        placeholder="Description (optional)"
                        disabled=is_creating
                        style="border-color: #e2e8f0;"
                        class="w-full px-4 py-3 border-2 rounded-lg focus:outline-none transition-colors min-h-[100px] resize-y disabled:opacity-60 disabled:cursor-not-allowed focus:border-primary"
                    />
                </div>
                {move || {
                    error
                        .get()
                        .map(|err| {
                            view! { <div class="text-red-600 text-sm">{err}</div> }
                        })
                }}

                <button 
                    type="submit" 
                    disabled=is_creating
                    style="background-color: var(--color-primary);"
                    class="w-full py-3 text-white rounded-lg font-semibold hover:opacity-90 transition-all disabled:opacity-60 disabled:cursor-not-allowed"
                >
                    {move || if is_creating.get() { "Creating..." } else { "Create Todo" }}
                </button>
            </form>
        </div>
    }
}

