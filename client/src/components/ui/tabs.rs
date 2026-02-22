use leptos::*;

#[component]
pub fn Tabs(
    children: Children,
    #[prop(optional, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div class=format!("tabs {}", class)>
            {children()}
        </div>
    }
}

#[component]
pub fn TabList(
    children: Children,
    #[prop(optional, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div class=format!("flex gap-2 border-b border-slate-200 dark:border-slate-700 mb-6 {}", class)>
            {children()}
        </div>
    }
}

#[component]
pub fn Tab(
    children: Children,
    #[prop(optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(optional, default = false)] active: bool,
    #[prop(optional, default = String::new())] class: String,
) -> impl IntoView {
    let base_classes = "px-4 py-2 font-medium text-sm transition-all cursor-pointer border-b-2";
    let active_classes = if active {
        "border-primary text-slate-900 dark:text-slate-100"
    } else {
        "border-transparent text-slate-600 dark:text-slate-400 hover:text-slate-900 dark:hover:text-slate-200 hover:border-slate-300 dark:hover:border-slate-600"
    };
    
    let combined_classes = format!("{} {} {}", base_classes, active_classes, class);

    view! {
        <button
            class=combined_classes
            on:click=move |ev| {
                if let Some(callback) = on_click {
                    callback.call(ev);
                }
            }
            style=if active { "border-color: var(--color-primary);" } else { "" }
        >
            {children()}
        </button>
    }
}

#[component]
pub fn TabPanel(
    children: Children,
    #[prop(optional, default = false)] active: bool,
) -> impl IntoView {
    view! {
        <div class=if active { "block" } else { "hidden" }>
            {children()}
        </div>
    }
}
