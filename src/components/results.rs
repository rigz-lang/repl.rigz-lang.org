use itertools::Itertools;
use leptos::{component, view, IntoView};
use leptos::prelude::*;
use rigz_core::{DevPrint, ObjectValue, TestResults};
use rigz_runtime::RuntimeError;
use crate::code::highlight;

#[derive(Clone)]
pub enum RunResult {
    Success(ObjectValue),
    Test(TestResults),
    Failure(RuntimeError)
}

impl Default for RunResult {
    fn default() -> Self {
        Self::Success(ObjectValue::default())
    }
}

#[component]
pub fn Results(results: ReadSignal<RunResult>) -> impl IntoView {
    move || match results.get() {
        RunResult::Failure(v) => {
            view! {
                <textarea
                    class="w-full h-24 p-4 bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-700 rounded-md text-gray-800 dark:text-gray-100 font-mono text-sm whitespace-pre-wrap resize-none"
                    readonly
                >
                    { move || v.to_string() }
                </textarea>
            }.into_any()
        }
        RunResult::Success(v) => {
            view! {
                <div
                    class="w-full h-24 p-4 bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-700 rounded-md text-gray-800 dark:text-gray-100 font-mono text-sm whitespace-pre-wrap resize-none"
                >
                    <pre aria_hidden={"true"} class="language-rigz font-mono text-wrap">
                        <code inner_html={move || highlight(v.dev_print(), "rigz".to_string()).into_render()} />
                        <br />
                    </pre>
                </div>
            }.into_any()
        }
        RunResult::Test(v) => {
            let success = v.success();
            view! {
                <pre
                    class="text-wrap w-full h-24 p-4 bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-700 rounded-md text-gray-800 dark:text-gray-100 font-mono text-sm whitespace-pre-wrap"
                >{"test result: "}<Show
                    when=move || success
                    fallback=|| view! { <strong class="text-red-500">failed</strong> }
                ><strong class="text-green-500">ok</strong></Show>.{
                    format!(" passed: {}, failed: {}, finished in {:?}\n{}",
                        v.passed, v.failed, v.duration,
                        v.failure_messages.into_iter()
                            .map(|(name, reason)| format!("\t{name}: {reason}"))
                            .join("\n"))
                }</pre>
            }.into_any()
        }
    }
}