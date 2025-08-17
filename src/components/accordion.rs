use icondata::{LuChevronDown, LuChevronUp};
use leptos::children::ChildrenFn;
use leptos::{component, view, IntoView};
use leptos::prelude::*;
use crate::components::Icon;
// modified from https://github.com/opensass/accordion-rs/blob/main/src/leptos.rs

#[component]
pub fn Accordion(
    /// Signal to track if the accordion is expanded.
    ///
    /// This is a tuple containing a `ReadSignal` to observe the expanded state
    /// and a `WriteSignal` to update it. Use this to programmatically control or
    /// react to the accordion's expansion.
    expand: (ReadSignal<bool>, WriteSignal<bool>),

    /// Content show whether accordion is open or closed.
    ///
    /// This is a function returning an `AnyView` that will be rendered inside the accordion
    /// when it is in an expanded state.
    trigger: Box<dyn Fn() -> AnyView + Send + Sync>,

    /// Child elements inside the accordion.
    ///
    /// These are additional elements that are rendered as part of the accordion's body.
    children: ChildrenFn,

    /// ARIA controls attribute.
    ///
    /// Sets the value for the `aria-controls` attribute, which is used for accessibility
    /// purposes to associate the accordion header with its content. Defaults to an empty string.
    #[prop(default = "")]
    aria_controls: &'static str,

    /// CSS class for the accordion.
    ///
    /// Adds a CSS class to the accordion container for styling purposes. Defaults to an empty string.
    #[prop(default = "")]
    class: &'static str,

    /// CSS class for the content container.
    ///
    /// Adds a CSS class to the container that wraps the accordion's content. Defaults to an empty string.
    #[prop(default = "")]
    content_class: &'static str
) -> impl IntoView {
    let toggle_expansion = move || {
        expand.1.set(!expand.0.get())
    };

    view! {
        <div
            class=class
        >
            <div
                aria-expanded={move || Some(expand.0.get().to_string()) }
                aria-controls=aria_controls
                on:click=move |_| toggle_expansion()
                class="flex gap-x-2 items-center"
            >
                {trigger()}
                {
                    move || view! {
                        <Icon icon={if expand.0.get() { LuChevronUp } else { LuChevronDown }}/>
                    }
                }
            </div>
            <Show when=move || expand.0.get() clone:children>
                <div
                    id=aria_controls
                    class=content_class
                >
                    {children()}
                </div>
            </Show>
        </div>
    }
}