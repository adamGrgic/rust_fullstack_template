use leptos::*;
use leptos_router::*;

#[derive(Clone, Debug)]
pub struct BreadcrumbItem {
    pub label: String,
    pub href: Option<String>,
}

#[component]
pub fn Breadcrumbs(
    items: Vec<BreadcrumbItem>,
    #[prop(optional, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <nav class=format!("flex items-center text-sm {}", class) aria-label="Breadcrumb">
            <ol class="flex items-center gap-2">
                {items.iter().enumerate().map(|(idx, item)| {
                    let is_last = idx == items.len() - 1;
                    let label = item.label.clone();
                    let href = item.href.clone();
                    
                    view! {
                        <li class="flex items-center gap-2">
                            {if let Some(path) = href {
                                view! {
                                    <A
                                        href=path
                                        class="text-blue-600 dark:text-blue-400 hover:underline"
                                    >
                                        {label}
                                    </A>
                                }.into_view()
                            } else {
                                view! {
                                    <span class="text-slate-600 dark:text-slate-400">{label}</span>
                                }.into_view()
                            }}
                            {if !is_last {
                                view! {
                                    <span class="text-slate-400 dark:text-slate-600">"/"</span>
                                }.into_view()
                            } else {
                                view! { <span/> }.into_view()
                            }}
                        </li>
                    }
                }).collect_view()}
            </ol>
        </nav>
    }
}
