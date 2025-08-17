use leptos::prelude::*;
use crate::components::icon::Gitlab;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="bg-white dark:bg-gray-800 shadow-sm">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4">
                <div class="flex flex-wrap justify-center items-center gap-6 text-sm text-gray-600 dark:text-gray-400">
                    <a href="https://rigz-lang.org" class="hover:text-blue-500 transition-colors" rel="external">Rigz</a>
                    <a href="https://docs.rigz-lang.org" class="hover:text-blue-500 transition-colors" rel="external">Documentation</a>
                    <Gitlab />
                </div>
            </div>
        </footer>
    }
}