use icondata::{LuMoon, LuSun};
use leptos::{component, view, IntoView};
use leptos::prelude::*;
use crate::components::icon::{Gitlab, Icon};

#[component]
pub(crate) fn Header(is_dark: ReadSignal<bool>, set_is_dark: WriteSignal<bool>) -> impl IntoView {
    view! {
        <header class="bg-white dark:bg-gray-800 shadow-sm md:px-6 py-4 flex flex-col md:flex-row justify-between items-center gap-4">
            <h1 class="text-2xl font-bold text-gray-800 dark:text-gray-100">Rigz REPL</h1>
            <nav class="mx-auto md:ml-auto flex flex-wrap justify-center items-center gap-6 text-gray-800 dark:text-gray-100 md:mr-6">
                <a href="https://rigz-lang.org" class="hover:text-blue-500 transition-colors" rel="external">Rigz</a>
                <a href="https://docs.rigz-lang.org" class="hover:text-blue-500 transition-colors" rel="external">Docs</a>
                <Gitlab />
            </nav>
            <button
                class="px-3 py-1 text-sm font-semibold outline-none focus:outline-none dark:text-white hover:opacity-50"
                on:click=move |_| set_is_dark.set(!is_dark.get())
                aria-label=move || if is_dark.get() { "Switch to Light Mode" } else { "Switch to Dark Mode" }
            >
                { move || if is_dark.get() { view! { <Icon icon=LuSun height="1.5rem" width="1.5rem"/> } } else { view! { <Icon icon=LuMoon height="1.5rem" width="1.5rem"/> }  }}
            </button>
        </header>
    }
}