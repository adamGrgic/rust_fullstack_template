use leptos::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ModalSize {
    Sm,
    Md,
    Lg,
    Xl,
    Full,
}

#[component]
pub fn Modal(
    children: Children,
    #[prop(into)] open: Signal<bool>,
    #[prop(optional)] on_close: Option<Callback<()>>,
    #[prop(optional, default = ModalSize::Md)] size: ModalSize,
    #[prop(optional, default = String::new())] title: String,
    #[prop(optional, default = String::new())] class: String,
) -> impl IntoView {
    let size_classes = match size {
        ModalSize::Sm => "max-w-sm",
        ModalSize::Md => "max-w-md",
        ModalSize::Lg => "max-w-lg",
        ModalSize::Xl => "max-w-xl",
        ModalSize::Full => "max-w-full mx-4",
    };

    let handle_backdrop_click = move |_| {
        if let Some(callback) = on_close {
            callback.call(());
        }
    };

    let handle_close_click = move |_| {
        if let Some(callback) = on_close {
            callback.call(());
        }
    };

    view! {
        <div
            class=move || if open.get() {
                "fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50 transition-opacity"
            } else {
                "hidden"
            }
            on:click=handle_backdrop_click
        >
            <div
                class=format!("relative w-full {} bg-white dark:bg-slate-800 rounded-xl shadow-2xl {}", size_classes, class)
                on:click=move |ev| ev.stop_propagation()
            >
                {move || if !title.is_empty() {
                    view! {
                        <div class="flex items-center justify-between p-6 border-b border-slate-200 dark:border-slate-700">
                            <h3 class="text-xl font-semibold text-slate-900 dark:text-slate-100">
                                {title.clone()}
                            </h3>
                            {move || if on_close.is_some() {
                                view! {
                                    <button
                                        on:click=handle_close_click
                                        class="text-slate-400 hover:text-slate-600 dark:hover:text-slate-200 transition-colors"
                                    >
                                        "✕"
                                    </button>
                                }.into_view()
                            } else {
                                view! { <div/> }.into_view()
                            }}
                        </div>
                    }.into_view()
                } else {
                    view! { <div/> }.into_view()
                }}
                
                <div class="p-6">
                    {children()}
                </div>
            </div>
        </div>
    }
}
