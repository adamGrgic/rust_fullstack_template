use leptos::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SpinnerSize {
    Sm,
    Md,
    Lg,
    Xl,
}

#[component]
pub fn Spinner(
    #[prop(optional, default = SpinnerSize::Md)] size: SpinnerSize,
    #[prop(optional, default = String::new())] class: String,
) -> impl IntoView {
    let size_classes = match size {
        SpinnerSize::Sm => "w-4 h-4 border-2",
        SpinnerSize::Md => "w-8 h-8 border-2",
        SpinnerSize::Lg => "w-12 h-12 border-3",
        SpinnerSize::Xl => "w-16 h-16 border-4",
    };

    let base_classes = "inline-block rounded-full border-blue-200 border-t-blue-600 animate-spin";
    let combined_classes = format!("{} {} {}", base_classes, size_classes, class);

    view! {
        <div class=combined_classes role="status">
            <span class="sr-only">"Loading..."</span>
        </div>
    }
}

#[component]
pub fn SpinnerOverlay(
    #[prop(optional, default = false)] show: bool,
    #[prop(optional, default = "Loading...".to_string())] message: String,
) -> impl IntoView {
    view! {
        <div
            class=move || if show {
                "fixed inset-0 z-50 flex flex-col items-center justify-center bg-black bg-opacity-50 transition-opacity"
            } else {
                "hidden"
            }
        >
            <div class="bg-white dark:bg-slate-800 p-6 rounded-xl shadow-2xl">
                <div class="flex flex-col items-center gap-4">
                    <Spinner size=SpinnerSize::Lg/>
                    <p class="text-slate-700 dark:text-slate-300 font-medium">{message}</p>
                </div>
            </div>
        </div>
    }
}
