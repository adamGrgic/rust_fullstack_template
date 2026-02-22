use leptos::*;
use crate::components::todo_list::TodoList;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="text-center mb-12">
            <h1 class="text-4xl font-bold mb-2" style="color: var(--color-primary);">"Atom Platform"</h1>
            <p class="text-lg text-slate-600">"Personal Data Management - Todo Application"</p>
            <TodoList/>
        </div>
    }
}
