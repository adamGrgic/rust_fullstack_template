use leptos::*;

#[component]
pub fn Table(
    children: Children,
    #[prop(optional, default = String::new())] class: String,
) -> impl IntoView {
    let base_classes = "w-full border-collapse bg-white rounded-xl shadow-md overflow-hidden";
    let combined_classes = format!("{} {}", base_classes, class);

    view! {
        <div class="overflow-x-auto">
            <table class=combined_classes>
                {children()}
            </table>
        </div>
    }
}

#[component]
pub fn TableHeader(
    children: Children,
) -> impl IntoView {
    view! {
        <thead class="bg-slate-100 border-b border-slate-200">
            {children()}
        </thead>
    }
}

#[component]
pub fn TableBody(
    children: Children,
) -> impl IntoView {
    view! {
        <tbody class="divide-y divide-slate-100">
            {children()}
        </tbody>
    }
}

#[component]
pub fn TableRow(
    children: Children,
    #[prop(optional, default = String::new())] class: String,
) -> impl IntoView {
    let base_classes = "hover:bg-slate-50 transition-colors";
    let combined_classes = format!("{} {}", base_classes, class);

    view! {
        <tr class=combined_classes>
            {children()}
        </tr>
    }
}

#[component]
pub fn TableHeaderCell(
    children: Children,
    #[prop(optional, default = String::new())] class: String,
) -> impl IntoView {
    let base_classes = "px-6 py-3 text-left text-xs font-semibold text-slate-700 uppercase tracking-wider";
    let combined_classes = format!("{} {}", base_classes, class);

    view! {
        <th class=combined_classes>
            {children()}
        </th>
    }
}

#[component]
pub fn TableCell(
    children: Children,
    #[prop(optional, default = String::new())] class: String,
) -> impl IntoView {
    let base_classes = "px-6 py-4 text-sm text-slate-800";
    let combined_classes = format!("{} {}", base_classes, class);

    view! {
        <td class=combined_classes>
            {children()}
        </td>
    }
}
