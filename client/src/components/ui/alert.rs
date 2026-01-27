use leptos::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AlertVariant {
    Primary,
    Success,
    Warning,
    Danger,
    Info,
}

#[component]
pub fn Alert(
    children: Children,
    #[prop(optional, default = AlertVariant::Info)] variant: AlertVariant,
    #[prop(optional, default = String::new())] title: String,
    #[prop(optional)] on_close: Option<Callback<ev::MouseEvent>>,
    #[prop(optional, default = String::new())] class: String,
) -> impl IntoView {
    let base_classes = "px-4 py-3 rounded-lg border-l-4";
    
    let variant_classes = match variant {
        AlertVariant::Primary => "bg-blue-50 dark:bg-blue-900/20 border-blue-500 text-blue-900 dark:text-blue-100",
        AlertVariant::Success => "bg-green-50 dark:bg-green-900/20 border-green-500 text-green-900 dark:text-green-100",
        AlertVariant::Warning => "bg-amber-50 dark:bg-amber-900/20 border-amber-500 text-amber-900 dark:text-amber-100",
        AlertVariant::Danger => "bg-red-50 dark:bg-red-900/20 border-red-500 text-red-900 dark:text-red-100",
        AlertVariant::Info => "bg-cyan-50 dark:bg-cyan-900/20 border-cyan-500 text-cyan-900 dark:text-cyan-100",
    };

    let combined_classes = format!("{} {} {}", base_classes, variant_classes, class);
    let has_title = !title.is_empty();
    let is_dismissible = on_close.is_some();

    view! {
        <div class=combined_classes role="alert">
            <div class="flex items-start justify-between gap-3">
                <div class="flex-1">
                    {move || if has_title {
                        view! {
                            <h4 class="font-semibold mb-1">{title.clone()}</h4>
                        }.into_view()
                    } else {
                        view! { <div/> }.into_view()
                    }}
                    <div class="text-sm">
                        {children()}
                    </div>
                </div>
                {move || if is_dismissible {
                    view! {
                        <button
                            on:click=move |ev| {
                                if let Some(callback) = on_close {
                                    callback.call(ev);
                                }
                            }
                            class="text-current hover:opacity-75 transition-opacity"
                        >
                            "✕"
                        </button>
                    }.into_view()
                } else {
                    view! { <div/> }.into_view()
                }}
            </div>
        </div>
    }
}
