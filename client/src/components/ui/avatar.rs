use leptos::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AvatarSize {
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
}

#[component]
pub fn Avatar(
    #[prop(optional, default = String::new())] src: String,
    #[prop(optional, default = String::new())] alt: String,
    #[prop(optional, default = String::new())] initials: String,
    #[prop(optional, default = AvatarSize::Md)] size: AvatarSize,
    #[prop(optional, default = false)] rounded: bool,
    #[prop(optional, default = String::new())] class: String,
) -> impl IntoView {
    let size_classes = match size {
        AvatarSize::Xs => "w-6 h-6 text-xs",
        AvatarSize::Sm => "w-8 h-8 text-sm",
        AvatarSize::Md => "w-10 h-10 text-base",
        AvatarSize::Lg => "w-12 h-12 text-lg",
        AvatarSize::Xl => "w-16 h-16 text-xl",
    };

    let shape_classes = if rounded { "rounded" } else { "rounded-full" };
    let has_image = !src.is_empty();
    let has_initials = !initials.is_empty();

    view! {
        <div class=format!("avatar inline-flex items-center justify-center bg-slate-200 dark:bg-slate-700 text-slate-700 dark:text-slate-300 font-semibold overflow-hidden {} {} {}", size_classes, shape_classes, class)>
            {move || if has_image {
                view! {
                    <img
                        src=src.clone()
                        alt=alt.clone()
                        class="w-full h-full object-cover"
                    />
                }.into_view()
            } else if has_initials {
                view! {
                    <span>{initials.clone()}</span>
                }.into_view()
            } else {
                view! {
                    <span>"?"</span>
                }.into_view()
            }}
        </div>
    }
}

#[component]
pub fn AvatarGroup(
    children: Children,
    #[prop(optional, default = 4)] max: usize,
    #[prop(optional, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div class=format!("flex -space-x-2 {}", class)>
            {children()}
            <div class="flex items-center justify-center w-10 h-10 rounded-full bg-slate-300 dark:bg-slate-600 text-slate-700 dark:text-slate-300 font-semibold text-sm border-2 border-white dark:border-slate-800">
                {format!("+{}", max)}
            </div>
        </div>
    }
}
