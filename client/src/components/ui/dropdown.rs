use leptos::*;

#[component]
pub fn Dropdown(
    children: Children,
    #[prop(into)] label: String,
    #[prop(optional, default = false)] open: bool,
    #[prop(optional, default = String::new())] class: String,
) -> impl IntoView {
    let (is_open, set_is_open) = create_signal(open);

    view! {
        <div class=format!("relative inline-block {}", class)>
            <button
                on:click=move |_| set_is_open.update(|open| *open = !*open)
                class="px-4 py-2 bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-lg hover:bg-slate-50 dark:hover:bg-slate-700 transition-colors flex items-center gap-2 text-slate-700 dark:text-slate-300"
            >
                {label}
                <span class="text-xs">
                    {move || if is_open.get() { "▲" } else { "▼" }}
                </span>
            </button>
            
            <div
                class=move || if is_open.get() {
                    "absolute right-0 mt-2 w-56 bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-lg shadow-lg z-10"
                } else {
                    "hidden"
                }
            >
                {children()}
            </div>
        </div>
    }
}

#[component]
pub fn DropdownItem(
    children: Children,
    #[prop(optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(optional, default = false)] danger: bool,
    #[prop(optional, default = String::new())] class: String,
) -> impl IntoView {
    let color_classes = if danger {
        "text-red-600 dark:text-red-400 hover:bg-red-50 dark:hover:bg-red-900/20"
    } else {
        "text-slate-700 dark:text-slate-300 hover:bg-slate-50 dark:hover:bg-slate-700"
    };

    view! {
        <button
            on:click=move |ev| {
                if let Some(callback) = on_click {
                    callback.call(ev);
                }
            }
            class=format!("w-full text-left px-4 py-2 text-sm transition-colors first:rounded-t-lg last:rounded-b-lg {} {}", color_classes, class)
        >
            {children()}
        </button>
    }
}

#[component]
pub fn DropdownDivider() -> impl IntoView {
    view! {
        <div class="border-t border-slate-200 dark:border-slate-700 my-1"/>
    }
}
