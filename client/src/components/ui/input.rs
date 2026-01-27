use leptos::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputSize {
    Sm,
    Md,
    Lg,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputVariant {
    Default,
    Success,
    Warning,
    Error,
}

#[component]
pub fn Input(
    #[prop(optional, default = "text".to_string())] input_type: String,
    #[prop(optional, into)] value: Signal<String>,
    #[prop(optional)] on_input: Option<Callback<String>>,
    #[prop(optional, default = String::new())] placeholder: String,
    #[prop(optional, default = false)] disabled: bool,
    #[prop(optional, default = false)] required: bool,
    #[prop(optional, default = InputSize::Md)] size: InputSize,
    #[prop(optional, default = InputVariant::Default)] variant: InputVariant,
    #[prop(optional, default = String::new())] class: String,
) -> impl IntoView {
    let base_classes = "w-full border-2 rounded-lg transition-colors disabled:opacity-60 disabled:cursor-not-allowed bg-white dark:bg-slate-700 text-slate-900 dark:text-slate-100";
    
    let size_classes = match size {
        InputSize::Sm => "px-3 py-1.5 text-sm",
        InputSize::Md => "px-4 py-2 text-base",
        InputSize::Lg => "px-5 py-3 text-lg",
    };
    
    let variant_classes = match variant {
        InputVariant::Default => "border-slate-200 dark:border-slate-600 focus:border-blue-500 focus:outline-none",
        InputVariant::Success => "border-green-500 focus:border-green-600 focus:outline-none",
        InputVariant::Warning => "border-amber-500 focus:border-amber-600 focus:outline-none",
        InputVariant::Error => "border-red-500 focus:border-red-600 focus:outline-none",
    };

    let combined_classes = format!("{} {} {} {}", base_classes, size_classes, variant_classes, class);

    view! {
        <input
            type=input_type
            prop:value=move || value.get()
            on:input=move |ev| {
                let val = event_target_value(&ev);
                if let Some(callback) = on_input {
                    callback.call(val);
                }
            }
            placeholder=placeholder
            disabled=disabled
            required=required
            class=combined_classes
        />
    }
}

#[component]
pub fn Textarea(
    #[prop(optional, into)] value: Signal<String>,
    #[prop(optional)] on_input: Option<Callback<String>>,
    #[prop(optional, default = String::new())] placeholder: String,
    #[prop(optional, default = false)] disabled: bool,
    #[prop(optional, default = false)] required: bool,
    #[prop(optional, default = 3)] rows: u32,
    #[prop(optional, default = InputVariant::Default)] variant: InputVariant,
    #[prop(optional, default = String::new())] class: String,
) -> impl IntoView {
    let base_classes = "w-full border-2 rounded-lg px-4 py-2 transition-colors disabled:opacity-60 disabled:cursor-not-allowed resize-y bg-white dark:bg-slate-700 text-slate-900 dark:text-slate-100";
    
    let variant_classes = match variant {
        InputVariant::Default => "border-slate-200 dark:border-slate-600 focus:border-blue-500 focus:outline-none",
        InputVariant::Success => "border-green-500 focus:border-green-600 focus:outline-none",
        InputVariant::Warning => "border-amber-500 focus:border-amber-600 focus:outline-none",
        InputVariant::Error => "border-red-500 focus:border-red-600 focus:outline-none",
    };

    let combined_classes = format!("{} {} {}", base_classes, variant_classes, class);

    view! {
        <textarea
            prop:value=move || value.get()
            on:input=move |ev| {
                let val = event_target_value(&ev);
                if let Some(callback) = on_input {
                    callback.call(val);
                }
            }
            placeholder=placeholder
            disabled=disabled
            required=required
            rows=rows
            class=combined_classes
        />
    }
}

#[component]
pub fn FormGroup(
    children: Children,
    #[prop(optional, default = String::new())] label: String,
    #[prop(optional, default = String::new())] help_text: String,
    #[prop(optional, default = String::new())] error: String,
    #[prop(optional, default = false)] required: bool,
    #[prop(optional, default = String::new())] class: String,
) -> impl IntoView {
    let has_label = !label.is_empty();
    let has_help = !help_text.is_empty();
    let has_error = !error.is_empty();

    view! {
        <div class=format!("form-group {}", class)>
            {move || if has_label {
                view! {
                    <label class="block mb-2 text-sm font-semibold text-slate-700 dark:text-slate-300">
                        {label.clone()}
                        {if required {
                            view! { <span class="text-red-500">" *"</span> }.into_view()
                        } else {
                            view! { <span/> }.into_view()
                        }}
                    </label>
                }.into_view()
            } else {
                view! { <div/> }.into_view()
            }}
            
            {children()}
            
            {move || if has_error {
                view! {
                    <p class="mt-1 text-sm text-red-600 dark:text-red-400">{error.clone()}</p>
                }.into_view()
            } else if has_help {
                view! {
                    <p class="mt-1 text-sm text-slate-500 dark:text-slate-400">{help_text.clone()}</p>
                }.into_view()
            } else {
                view! { <div/> }.into_view()
            }}
        </div>
    }
}
