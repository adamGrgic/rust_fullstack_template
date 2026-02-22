use leptos::*;
use crate::components::ui::*;

#[component]
pub fn ComponentShowcasePage() -> impl IntoView {
    let (page, set_page) = create_signal(0);

    view! {
        <div class="max-w-6xl mx-auto space-y-12">
            <div>
                <h1 class="text-4xl font-bold text-slate-800 dark:text-slate-100 mb-2">"Component Showcase"</h1>
                <p class="text-lg text-slate-600 dark:text-slate-400">"Complete UI component library - Bootstrap-like components"</p>
            </div>

            // Buttons
            <Card title="Buttons".to_string()>
                <div class="space-y-6">
                    <div>
                        <h4 class="text-sm font-semibold text-slate-700 dark:text-slate-300 mb-3">"Solid Variants"</h4>
                        <div class="flex flex-wrap gap-3">
                            <button style="background-color: var(--color-primary);" class="px-4 py-2 rounded-lg font-semibold text-white hover:opacity-90 transition-all">"Primary"</button>
                            <button style="background-color: var(--color-secondary);" class="px-4 py-2 rounded-lg font-semibold text-white hover:opacity-90 transition-all">"Secondary"</button>
                            <button style="background-color: var(--color-tertiary);" class="px-4 py-2 rounded-lg font-semibold text-white hover:opacity-90 transition-all">"Tertiary"</button>
                            <button class="px-4 py-2 rounded-lg font-semibold bg-green-600 text-white hover:bg-green-700 transition-all">"Success"</button>
                            <button class="px-4 py-2 rounded-lg font-semibold bg-amber-600 text-white hover:bg-amber-700 transition-all">"Warning"</button>
                            <button class="px-4 py-2 rounded-lg font-semibold bg-red-600 text-white hover:bg-red-700 transition-all">"Danger"</button>
                            <button class="px-4 py-2 rounded-lg font-semibold bg-cyan-600 text-white hover:bg-cyan-700 transition-all">"Info"</button>
                        </div>
                    </div>
                    <Divider/>
                    <div>
                        <h4 class="text-sm font-semibold text-slate-700 dark:text-slate-300 mb-3">"Outline Variants"</h4>
                        <div class="flex flex-wrap gap-3">
                            <button class="px-4 py-2 rounded-lg font-semibold border-2 border-blue-600 text-blue-600 hover:bg-blue-50 dark:hover:bg-blue-900/20 transition-all">"Outline Primary"</button>
                            <button class="px-4 py-2 rounded-lg font-semibold border-2 border-slate-600 text-slate-600 hover:bg-slate-50 dark:hover:bg-slate-800 transition-all">"Outline Secondary"</button>
                            <button class="px-4 py-2 rounded-lg font-semibold border-2 border-red-600 text-red-600 hover:bg-red-50 dark:hover:bg-red-900/20 transition-all">"Outline Danger"</button>
                        </div>
                    </div>
                    <Divider/>
                    <div>
                        <h4 class="text-sm font-semibold text-slate-700 dark:text-slate-300 mb-3">"Sizes"</h4>
                        <div class="flex flex-wrap gap-3 items-center">
                            <button style="background-color: var(--color-primary);" class="px-2.5 py-1.5 text-xs rounded font-semibold text-white hover:opacity-90 transition-all">"Extra Small"</button>
                            <button style="background-color: var(--color-primary);" class="px-3 py-2 text-sm rounded-md font-semibold text-white hover:opacity-90 transition-all">"Small"</button>
                            <button style="background-color: var(--color-primary);" class="px-4 py-2 text-base rounded-lg font-semibold text-white hover:opacity-90 transition-all">"Medium"</button>
                            <button style="background-color: var(--color-primary);" class="px-5 py-3 text-lg rounded-lg font-semibold text-white hover:opacity-90 transition-all">"Large"</button>
                            <button style="background-color: var(--color-primary);" class="px-6 py-4 text-xl rounded-xl font-semibold text-white hover:opacity-90 transition-all">"Extra Large"</button>
                        </div>
                    </div>
                    <Divider/>
                    <div>
                        <h4 class="text-sm font-semibold text-slate-700 dark:text-slate-300 mb-3">"States"</h4>
                        <div class="flex flex-wrap gap-3">
                            <button style="background-color: var(--color-primary);" class="px-4 py-2 rounded-lg font-semibold text-white hover:opacity-90 transition-all">"Normal"</button>
                            <button disabled style="background-color: var(--color-primary);" class="px-4 py-2 rounded-lg font-semibold text-white opacity-60 cursor-not-allowed">"Disabled"</button>
                        </div>
                    </div>
                    <Divider/>
                    <div>
                        <h4 class="text-sm font-semibold text-slate-700 dark:text-slate-300 mb-3">"Ghost & Link"</h4>
                        <div class="flex flex-wrap gap-3">
                            <button class="px-4 py-2 rounded-lg font-semibold text-slate-600 hover:bg-slate-100 dark:text-slate-400 dark:hover:bg-slate-800 transition-all">"Ghost"</button>
                            <button class="px-4 py-2 font-semibold text-blue-600 hover:underline dark:text-blue-400 transition-all">"Link Button"</button>
                        </div>
                    </div>
                </div>
            </Card>

            // Badges
            <Card title="Badges".to_string()>
                <div class="space-y-6">
                    <div>
                        <h4 class="text-sm font-semibold text-slate-700 dark:text-slate-300 mb-3">"Variants"</h4>
                        <div class="flex flex-wrap gap-3 items-center">
                            <Badge variant=BadgeVariant::Primary>"Primary"</Badge>
                            <Badge variant=BadgeVariant::Secondary>"Secondary"</Badge>
                            <Badge variant=BadgeVariant::Tertiary>"Tertiary"</Badge>
                            <Badge variant=BadgeVariant::Success>"Success"</Badge>
                            <Badge variant=BadgeVariant::Warning>"Warning"</Badge>
                            <Badge variant=BadgeVariant::Danger>"Danger"</Badge>
                            <Badge variant=BadgeVariant::Info>"Info"</Badge>
                        </div>
                    </div>
                    <Divider/>
                    <div>
                        <h4 class="text-sm font-semibold text-slate-700 dark:text-slate-300 mb-3">"Sizes & Pill Shape"</h4>
                        <div class="flex flex-wrap gap-3 items-center">
                            <Badge size=BadgeSize::Sm>"Small"</Badge>
                            <Badge size=BadgeSize::Md>"Medium"</Badge>
                            <Badge size=BadgeSize::Lg>"Large"</Badge>
                            <Badge pill=true>"Pill Badge"</Badge>
                            <Badge variant=BadgeVariant::Success pill=true>"Success Pill"</Badge>
                        </div>
                    </div>
                </div>
            </Card>

            // Form Components
            <Card title="Form Components".to_string()>
                <div class="space-y-6">
                    <FormGroup label="Email Address".to_string() required=true help_text="We'll never share your email.".to_string()>
                        <input
                            type="email"
                            placeholder="you@example.com"
                            class="w-full px-4 py-2 border-2 rounded-lg border-slate-200 dark:border-slate-600 bg-white dark:bg-slate-700 text-slate-900 dark:text-slate-100 focus:border-blue-500 focus:outline-none transition-colors"
                        />
                    </FormGroup>
                    
                    <FormGroup label="Message".to_string()>
                        <textarea
                            placeholder="Enter your message here..."
                            class="w-full px-4 py-2 border-2 rounded-lg border-slate-200 dark:border-slate-600 bg-white dark:bg-slate-700 text-slate-900 dark:text-slate-100 resize-y focus:border-blue-500 focus:outline-none transition-colors min-h-[100px]"
                        />
                    </FormGroup>
                    
                    <div class="grid grid-cols-3 gap-4">
                        <FormGroup label="Success".to_string()>
                            <input
                                type="text"
                                value="Validated!"
                                class="w-full px-4 py-2 border-2 rounded-lg border-green-500 bg-white dark:bg-slate-700 text-slate-900 dark:text-slate-100 focus:outline-none"
                            />
                        </FormGroup>
                        <FormGroup label="Warning".to_string()>
                            <input
                                type="text"
                                value="Check this"
                                class="w-full px-4 py-2 border-2 rounded-lg border-amber-500 bg-white dark:bg-slate-700 text-slate-900 dark:text-slate-100 focus:outline-none"
                            />
                        </FormGroup>
                        <FormGroup label="Error".to_string() error="This field has an error".to_string()>
                            <input
                                type="text"
                                value="Invalid!"
                                class="w-full px-4 py-2 border-2 rounded-lg border-red-500 bg-white dark:bg-slate-700 text-slate-900 dark:text-slate-100 focus:outline-none"
                            />
                        </FormGroup>
                    </div>
                </div>
            </Card>

            // Alerts
            <Card title="Alerts".to_string()>
                <div class="space-y-4">
                    <Alert variant=AlertVariant::Primary title="Primary Alert".to_string()>
                        "This is a primary alert with important information."
                    </Alert>
                    <Alert variant=AlertVariant::Success title="Success!".to_string()>
                        "Your action was completed successfully."
                    </Alert>
                    <Alert variant=AlertVariant::Warning title="Warning".to_string()>
                        "Please review this information before proceeding."
                    </Alert>
                    <Alert variant=AlertVariant::Danger title="Error".to_string()>
                        "Something went wrong. Please try again."
                    </Alert>
                    <Alert variant=AlertVariant::Info>
                        "Simple info alert without title."
                    </Alert>
                </div>
            </Card>

            // Progress
            <Card title="Progress Bars".to_string()>
                <div class="space-y-6">
                    <div>
                        <h4 class="text-sm font-semibold text-slate-700 dark:text-slate-300 mb-3">"Variants"</h4>
                        <div class="space-y-3">
                            <Progress value=Signal::derive(move || 65.0) variant=ProgressVariant::Primary show_label=true/>
                            <Progress value=Signal::derive(move || 85.0) variant=ProgressVariant::Success show_label=true/>
                            <Progress value=Signal::derive(move || 45.0) variant=ProgressVariant::Warning show_label=true/>
                            <Progress value=Signal::derive(move || 25.0) variant=ProgressVariant::Danger show_label=true/>
                        </div>
                    </div>
                    <Divider/>
                    <div>
                        <h4 class="text-sm font-semibold text-slate-700 dark:text-slate-300 mb-3">"Sizes"</h4>
                        <div class="space-y-3">
                            <Progress value=Signal::derive(move || 70.0) size=ProgressSize::Sm/>
                            <Progress value=Signal::derive(move || 70.0) size=ProgressSize::Md/>
                            <Progress value=Signal::derive(move || 70.0) size=ProgressSize::Lg/>
                        </div>
                    </div>
                </div>
            </Card>

            // Spinners
            <Card title="Spinners".to_string()>
                <div class="space-y-6">
                    <div>
                        <h4 class="text-sm font-semibold text-slate-700 dark:text-slate-300 mb-3">"Sizes"</h4>
                        <div class="flex items-center gap-6">
                            <Spinner size=SpinnerSize::Sm/>
                            <Spinner size=SpinnerSize::Md/>
                            <Spinner size=SpinnerSize::Lg/>
                            <Spinner size=SpinnerSize::Xl/>
                        </div>
                    </div>
                </div>
            </Card>

            // Avatars
            <Card title="Avatars".to_string()>
                <div class="space-y-6">
                    <div>
                        <h4 class="text-sm font-semibold text-slate-700 dark:text-slate-300 mb-3">"Sizes"</h4>
                        <div class="flex items-center gap-4">
                            <Avatar initials="XS".to_string() size=AvatarSize::Xs/>
                            <Avatar initials="SM".to_string() size=AvatarSize::Sm/>
                            <Avatar initials="MD".to_string() size=AvatarSize::Md/>
                            <Avatar initials="LG".to_string() size=AvatarSize::Lg/>
                            <Avatar initials="XL".to_string() size=AvatarSize::Xl/>
                        </div>
                    </div>
                    <Divider/>
                    <div>
                        <h4 class="text-sm font-semibold text-slate-700 dark:text-slate-300 mb-3">"Shapes"</h4>
                        <div class="flex items-center gap-4">
                            <Avatar initials="AB".to_string() size=AvatarSize::Lg/>
                            <Avatar initials="CD".to_string() size=AvatarSize::Lg rounded=true/>
                        </div>
                    </div>
                </div>
            </Card>

            // Pagination
            <Card title="Pagination".to_string()>
                <div class="space-y-6">
                    <Pagination
                        current_page=Signal::derive(move || page.get())
                        total_pages=10_usize
                        on_page_change=Callback::new(move |new_page| set_page.set(new_page))
                        max_visible=5_usize
                    />
                    <p class="text-sm text-slate-600 dark:text-slate-400">
                        "Current page: " {move || page.get() + 1}
                    </p>
                </div>
            </Card>

            // Breadcrumbs
            <Card title="Breadcrumbs".to_string()>
                <Breadcrumbs items=vec![
                    BreadcrumbItem { label: "Home".to_string(), href: Some("/".to_string()) },
                    BreadcrumbItem { label: "Settings".to_string(), href: Some("/settings/theme".to_string()) },
                    BreadcrumbItem { label: "Components".to_string(), href: None },
                ]/>
            </Card>

            // Dividers
            <Card title="Dividers".to_string()>
                <div class="space-y-6">
                    <div>
                        <p class="text-slate-700 dark:text-slate-300">"Content above"</p>
                        <Divider/>
                        <p class="text-slate-700 dark:text-slate-300">"Content below"</p>
                    </div>
                    <div>
                        <p class="text-slate-700 dark:text-slate-300">"Content above"</p>
                        <Divider label="OR".to_string()/>
                        <p class="text-slate-700 dark:text-slate-300">"Content below"</p>
                    </div>
                </div>
            </Card>

            // Dropdown
            <Card title="Dropdown Menus".to_string()>
                <div class="flex gap-4">
                    <Dropdown label="Actions".to_string()>
                        <DropdownItem>"Edit"</DropdownItem>
                        <DropdownItem>"Duplicate"</DropdownItem>
                        <DropdownDivider/>
                        <DropdownItem danger=true>"Delete"</DropdownItem>
                    </Dropdown>
                    
                    <Dropdown label="Options".to_string()>
                        <DropdownItem>"Profile"</DropdownItem>
                        <DropdownItem>"Settings"</DropdownItem>
                        <DropdownItem>"Sign Out"</DropdownItem>
                    </Dropdown>
                </div>
            </Card>

            // Summary
            <Card>
                <div class="text-center py-8">
                    <h3 class="text-2xl font-bold text-slate-800 dark:text-slate-100 mb-4">"Full Component Library"</h3>
                    <p class="text-slate-600 dark:text-slate-400 mb-6">
                        "20+ components with multiple variants, sizes, and states"
                    </p>
                    <div class="flex justify-center gap-4">
                        <Badge variant=BadgeVariant::Primary size=BadgeSize::Lg>"Production Ready"</Badge>
                        <Badge variant=BadgeVariant::Success size=BadgeSize::Lg>"Dark Mode"</Badge>
                        <Badge variant=BadgeVariant::Info size=BadgeSize::Lg>"Tailwind CSS"</Badge>
                    </div>
                </div>
            </Card>
        </div>
    }
}
