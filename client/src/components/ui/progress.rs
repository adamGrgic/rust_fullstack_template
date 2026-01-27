use leptos::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProgressVariant {
    Primary,
    Success,
    Warning,
    Danger,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProgressSize {
    Sm,
    Md,
    Lg,
}

#[component]
pub fn Progress(
    #[prop(into)] value: Signal<f32>,
    #[prop(optional, default = 100.0)] max: f32,
    #[prop(optional, default = ProgressVariant::Primary)] variant: ProgressVariant,
    #[prop(optional, default = ProgressSize::Md)] size: ProgressSize,
    #[prop(optional, default = false)] show_label: bool,
    #[prop(optional, default = String::new())] class: String,
) -> impl IntoView {
    let size_classes = match size {
        ProgressSize::Sm => "h-1",
        ProgressSize::Md => "h-2",
        ProgressSize::Lg => "h-4",
    };

    let bar_classes = match variant {
        ProgressVariant::Primary => "bg-blue-600 dark:bg-blue-500",
        ProgressVariant::Success => "bg-green-600 dark:bg-green-500",
        ProgressVariant::Warning => "bg-amber-600 dark:bg-amber-500",
        ProgressVariant::Danger => "bg-red-600 dark:bg-red-500",
    };

    let percentage = move || ((value.get() / max) * 100.0).min(100.0).max(0.0);

    view! {
        <div class=format!("progress-container {}", class)>
            <div class=format!("w-full bg-slate-200 dark:bg-slate-700 rounded-full overflow-hidden {}", size_classes)>
                <div
                    class=format!("h-full transition-all duration-300 {}", bar_classes)
                    style=move || format!("width: {}%", percentage())
                    role="progressbar"
                    aria-valuenow=move || value.get()
                    aria-valuemin="0"
                    aria-valuemax=max
                />
            </div>
            {move || if show_label {
                view! {
                    <div class="text-sm text-slate-600 dark:text-slate-400 mt-1 text-right">
                        {format!("{:.0}%", percentage())}
                    </div>
                }.into_view()
            } else {
                view! { <div/> }.into_view()
            }}
        </div>
    }
}
