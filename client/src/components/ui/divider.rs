use leptos::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DividerOrientation {
    Horizontal,
    Vertical,
}

#[component]
pub fn Divider(
    #[prop(optional, default = DividerOrientation::Horizontal)] orientation: DividerOrientation,
    #[prop(optional, default = String::new())] label: String,
    #[prop(optional, default = String::new())] class: String,
) -> impl IntoView {
    let has_label = !label.is_empty();

    match orientation {
        DividerOrientation::Horizontal => {
            if has_label {
                view! {
                    <div class=format!("flex items-center gap-4 my-4 {}", class)>
                        <div class="flex-1 border-t border-slate-200 dark:border-slate-700"/>
                        <span class="text-sm text-slate-500 dark:text-slate-400 font-medium">
                            {label}
                        </span>
                        <div class="flex-1 border-t border-slate-200 dark:border-slate-700"/>
                    </div>
                }
                .into_view()
            } else {
                view! {
                    <div class=format!("border-t border-slate-200 dark:border-slate-700 my-4 {}", class)/>
                }
                .into_view()
            }
        }
        DividerOrientation::Vertical => view! {
            <div class=format!("border-l border-slate-200 dark:border-slate-700 h-full {}", class)/>
        }
        .into_view(),
    }
}
