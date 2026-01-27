use leptos::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Tertiary,
    Success,
    Warning,
    Danger,
    Info,
    Light,
    Dark,
    OutlinePrimary,
    OutlineSecondary,
    OutlineDanger,
    Ghost,
    Link,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ButtonSize {
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
}

#[component]
pub fn Button(
    children: Children,
    #[prop(optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(optional, default = false)] disabled: bool,
    #[prop(optional, default = ButtonVariant::Primary)] variant: ButtonVariant,
    #[prop(optional, default = ButtonSize::Md)] size: ButtonSize,
    #[prop(optional, default = false)] full_width: bool,
    #[prop(optional, default = String::new())] class: String,
) -> impl IntoView {
    let base_classes = "inline-flex items-center justify-center font-semibold transition-all focus:outline-none focus:ring-2 focus:ring-offset-2 disabled:opacity-60 disabled:cursor-not-allowed";
    
    let size_classes = match size {
        ButtonSize::Xs => "px-2.5 py-1.5 text-xs rounded",
        ButtonSize::Sm => "px-3 py-2 text-sm rounded-md",
        ButtonSize::Md => "px-4 py-2 text-base rounded-lg",
        ButtonSize::Lg => "px-5 py-3 text-lg rounded-lg",
        ButtonSize::Xl => "px-6 py-4 text-xl rounded-xl",
    };
    
    let width_class = if full_width { "w-full" } else { "" };
    
    let (variant_classes, bg_style) = match variant {
        ButtonVariant::Primary => ("text-white hover:opacity-90 focus:ring-blue-500", "background-color: var(--color-primary);"),
        ButtonVariant::Secondary => ("text-white hover:opacity-90 focus:ring-slate-500", "background-color: var(--color-secondary);"),
        ButtonVariant::Tertiary => ("text-white hover:opacity-90 focus:ring-emerald-500", "background-color: var(--color-tertiary);"),
        ButtonVariant::Success => ("bg-green-600 text-white hover:bg-green-700 focus:ring-green-500", ""),
        ButtonVariant::Warning => ("bg-amber-600 text-white hover:bg-amber-700 focus:ring-amber-500", ""),
        ButtonVariant::Danger => ("bg-red-600 text-white hover:bg-red-700 focus:ring-red-500", ""),
        ButtonVariant::Info => ("bg-cyan-600 text-white hover:bg-cyan-700 focus:ring-cyan-500", ""),
        ButtonVariant::Light => ("bg-slate-100 text-slate-900 hover:bg-slate-200 focus:ring-slate-400 dark:bg-slate-700 dark:text-slate-100", ""),
        ButtonVariant::Dark => ("bg-slate-800 text-white hover:bg-slate-900 focus:ring-slate-600", ""),
        ButtonVariant::OutlinePrimary => ("border-2 border-blue-600 text-blue-600 hover:bg-blue-50 focus:ring-blue-500 dark:border-blue-400 dark:text-blue-400 dark:hover:bg-blue-900/20", ""),
        ButtonVariant::OutlineSecondary => ("border-2 border-slate-600 text-slate-600 hover:bg-slate-50 focus:ring-slate-500 dark:border-slate-400 dark:text-slate-400 dark:hover:bg-slate-800", ""),
        ButtonVariant::OutlineDanger => ("border-2 border-red-600 text-red-600 hover:bg-red-50 focus:ring-red-500 dark:border-red-400 dark:text-red-400 dark:hover:bg-red-900/20", ""),
        ButtonVariant::Ghost => ("text-slate-600 hover:bg-slate-100 focus:ring-slate-400 dark:text-slate-400 dark:hover:bg-slate-800", ""),
        ButtonVariant::Link => ("text-blue-600 hover:underline focus:ring-blue-500 dark:text-blue-400", ""),
    };

    let combined_classes = format!("{} {} {} {} {}", base_classes, size_classes, width_class, variant_classes, class);

    view! {
        <button
            class=combined_classes
            style=bg_style
            disabled=disabled
            on:click=move |ev| {
                if let Some(callback) = on_click {
                    callback.call(ev);
                }
            }
        >
            {children()}
        </button>
    }
}

/// Button Group - renders buttons side-by-side with connected borders
#[component]
pub fn ButtonGroup(
    children: Children,
    #[prop(optional, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div class=format!("inline-flex rounded-lg shadow-sm {}", class) role="group">
            {children()}
        </div>
    }
}

