use leptos::*;

#[component]
pub fn Pagination(
    #[prop(into)] current_page: Signal<usize>,
    #[prop(into)] total_pages: usize,
    on_page_change: Callback<usize>,
    #[prop(optional, default = 5)] max_visible: usize,
    #[prop(optional, default = String::new())] class: String,
) -> impl IntoView {
    let handle_page_change = move |page: usize| {
        on_page_change.call(page);
    };

    let visible_pages = move || {
        let current = current_page.get();
        let half = max_visible / 2;
        let mut start = current.saturating_sub(half);
        let mut end = (start + max_visible).min(total_pages);
        
        if end - start < max_visible {
            start = end.saturating_sub(max_visible);
        }
        
        (start..end).collect::<Vec<_>>()
    };

    view! {
        <nav class=format!("flex items-center justify-center gap-1 {}", class) aria-label="Pagination">
            <button
                on:click=move |_| {
                    let page = current_page.get();
                    if page > 0 {
                        handle_page_change(page - 1);
                    }
                }
                disabled=move || current_page.get() == 0
                class="px-3 py-2 rounded-lg text-sm font-medium text-slate-700 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-800 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
            >
                "←"
            </button>
            
            {move || if visible_pages().first() != Some(&0) {
                view! {
                    <>
                        <button
                            on:click=move |_| handle_page_change(0)
                            class="px-3 py-2 rounded-lg text-sm font-medium text-slate-700 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-800 transition-colors"
                        >
                            "1"
                        </button>
                        <span class="text-slate-400">"..."</span>
                    </>
                }.into_view()
            } else {
                view! { <></> }.into_view()
            }}
            
            <For
                each=visible_pages
                key=|page| *page
                children=move |page| {
                    let is_current = move || current_page.get() == page;
                    view! {
                        <button
                            on:click=move |_| handle_page_change(page)
                            class=move || if is_current() {
                                "px-3 py-2 rounded-lg text-sm font-medium bg-blue-600 text-white"
                            } else {
                                "px-3 py-2 rounded-lg text-sm font-medium text-slate-700 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-800 transition-colors"
                            }
                        >
                            {page + 1}
                        </button>
                    }
                }
            />
            
            {move || if visible_pages().last() != Some(&(total_pages - 1)) && total_pages > 0 {
                view! {
                    <>
                        <span class="text-slate-400">"..."</span>
                        <button
                            on:click=move |_| handle_page_change(total_pages - 1)
                            class="px-3 py-2 rounded-lg text-sm font-medium text-slate-700 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-800 transition-colors"
                        >
                            {total_pages}
                        </button>
                    </>
                }.into_view()
            } else {
                view! { <></> }.into_view()
            }}
            
            <button
                on:click=move |_| {
                    let page = current_page.get();
                    if page < total_pages - 1 {
                        handle_page_change(page + 1);
                    }
                }
                disabled=move || current_page.get() >= total_pages - 1
                class="px-3 py-2 rounded-lg text-sm font-medium text-slate-700 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-800 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
            >
                "→"
            </button>
        </nav>
    }
}
