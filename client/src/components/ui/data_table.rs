use leptos::*;
use std::rc::Rc;

/// Column definition for DataTable
pub struct Column<T>
where
    T: Clone + 'static,
{
    pub header: String,
    pub render: Rc<dyn Fn(&T) -> View>,
    pub width: Option<String>,
}

impl<T> Clone for Column<T>
where
    T: Clone + 'static,
{
    fn clone(&self) -> Self {
        Self {
            header: self.header.clone(),
            render: self.render.clone(),
            width: self.width.clone(),
        }
    }
}

impl<T> Column<T>
where
    T: Clone + 'static,
{
    pub fn new(header: impl Into<String>, render: impl Fn(&T) -> View + 'static) -> Self {
        Self {
            header: header.into(),
            render: Rc::new(render),
            width: None,
        }
    }

    pub fn with_width(mut self, width: impl Into<String>) -> Self {
        self.width = Some(width.into());
        self
    }
}

/// DataTable component - A feature-rich table for displaying data
#[component]
pub fn DataTable<T, K>(
    /// The data to display in the table
    data: Signal<Vec<T>>,
    /// Column definitions
    columns: Vec<Column<T>>,
    /// Function to extract a unique key from each row
    key_fn: K,
    #[prop(optional, default = false)] loading: bool,
    #[prop(optional, default = "No data to display".to_string())] empty_message: String,
    #[prop(optional, default = String::new())] class: String,
) -> impl IntoView
where
    T: Clone + 'static,
    K: Fn(&T) -> String + 'static + Copy,
{
    let base_classes = "w-full bg-white rounded-xl shadow-md overflow-hidden";
    let combined_classes = format!("{} {}", base_classes, class);
    
    // Store columns in a signal so they can be accessed in the reactive closure
    let columns_signal = create_rw_signal(columns);

    view! {
        <div class="overflow-x-auto">
            {move || {
                let columns = columns_signal.get();
                if loading {
                    view! {
                        <div class="text-center p-8 bg-white rounded-xl shadow-md">
                            <div class="animate-pulse">"Loading..."</div>
                        </div>
                    }
                        .into_view()
                } else if data.get().is_empty() {
                    view! {
                        <div class="text-center p-8 bg-white rounded-xl shadow-md text-slate-600">
                            {empty_message.clone()}
                        </div>
                    }
                        .into_view()
                } else {
                    view! {
                        <table class=combined_classes.clone()>
                            <thead class="bg-slate-100 border-b border-slate-200">
                                <tr>
                                    {columns
                                        .iter()
                                        .map(|col| {
                                            let width_style = col
                                                .width
                                                .as_ref()
                                                .map(|w| format!("width: {};", w))
                                                .unwrap_or_default();
                                            view! {
                                                <th
                                                    class="px-6 py-3 text-left text-xs font-semibold text-slate-700 uppercase tracking-wider"
                                                    style=width_style
                                                >
                                                    {col.header.clone()}
                                                </th>
                                            }
                                        })
                                        .collect_view()}

                                </tr>
                            </thead>
                            <tbody class="divide-y divide-slate-100">
                                <For
                                    each=move || data.get()
                                    key=key_fn
                                    children=move |item: T| {
                                        view! {
                                            <tr class="hover:bg-slate-50 transition-colors">
                                                {columns
                                                    .iter()
                                                    .map(|col| {
                                                        let cell_view = (col.render)(&item);
                                                        view! {
                                                            <td class="px-6 py-4 text-sm text-slate-800">{cell_view}</td>
                                                        }
                                                    })
                                                    .collect_view()}

                                            </tr>
                                        }
                                    }
                                />

                            </tbody>
                        </table>
                    }
                        .into_view()
                }
            }}

        </div>
    }
}
