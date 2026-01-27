use leptos::*;

#[component]
pub fn Card(
    children: Children,
    #[prop(optional, default = String::new())] title: String,
    #[prop(optional, default = String::new())] class: String,
) -> impl IntoView {
    let base_classes = "bg-white p-6 rounded-xl shadow-md";
    let combined_classes = format!("{} {}", base_classes, class);
    let has_title = !title.is_empty();

    view! {
        <div class=combined_classes>
            {move || if has_title {
                view! {
                    <h3 class="text-xl font-semibold text-slate-800 mb-4">{title.clone()}</h3>
                }.into_view()
            } else {
                view! { <div/> }.into_view()
            }}
            {children()}
        </div>
    }
}
