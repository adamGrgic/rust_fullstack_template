use leptos::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BadgeVariant {
    Primary,
    Secondary,
    Tertiary,
    Success,
    Warning,
    Danger,
    Info,
    Light,
    Dark,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BadgeSize {
    Sm,
    Md,
    Lg,
}

#[component]
pub fn Badge(
    children: Children,
    #[prop(optional, default = BadgeVariant::Primary)] variant: BadgeVariant,
    #[prop(optional, default = BadgeSize::Md)] size: BadgeSize,
    #[prop(optional, default = false)] pill: bool,
    #[prop(optional, default = String::new())] class: String,
) -> impl IntoView {
    let base_classes = "inline-flex items-center justify-center font-semibold";
    
    let size_classes = match size {
        BadgeSize::Sm => "px-2 py-0.5 text-xs",
        BadgeSize::Md => "px-3 py-1 text-sm",
        BadgeSize::Lg => "px-4 py-1.5 text-base",
    };
    
    let shape_classes = if pill { "rounded-full" } else { "rounded" };
    
    let variant_classes = match variant {
        BadgeVariant::Primary => "bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200",
        BadgeVariant::Secondary => "bg-slate-100 text-slate-800 dark:bg-slate-700 dark:text-slate-200",
        BadgeVariant::Tertiary => "bg-emerald-100 text-emerald-800 dark:bg-emerald-900 dark:text-emerald-200",
        BadgeVariant::Success => "bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200",
        BadgeVariant::Warning => "bg-amber-100 text-amber-800 dark:bg-amber-900 dark:text-amber-200",
        BadgeVariant::Danger => "bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200",
        BadgeVariant::Info => "bg-cyan-100 text-cyan-800 dark:bg-cyan-900 dark:text-cyan-200",
        BadgeVariant::Light => "bg-slate-50 text-slate-700 dark:bg-slate-800 dark:text-slate-300",
        BadgeVariant::Dark => "bg-slate-800 text-slate-100 dark:bg-slate-200 dark:text-slate-900",
    };

    let combined_classes = format!("{} {} {} {} {}", base_classes, size_classes, shape_classes, variant_classes, class);

    view! {
        <span class=combined_classes>
            {children()}
        </span>
    }
}
