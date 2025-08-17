use leptos::{component, view, IntoView};
use leptos::prelude::*;
use crate::components::{Accordion};

#[component]
pub(crate) fn TextAccordion(
    #[prop(optional, default=true)]
    default_open: bool,
    title: &'static str,
    children: ChildrenFn,
) -> impl IntoView {
    let expand = signal(default_open);
    let title = Box::new(move || {
        view! {
            <h2 class="text-xl font-semibold text-gray-800 dark:text-gray-100">
                {title.to_string()}
            </h2>
        }.into_any()
    });

    view! {
        <Accordion
            expand={expand}
            trigger={title}
            class="w-full transition-all"
        >
            {children()}
        </Accordion>
    }
}