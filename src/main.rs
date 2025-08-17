mod code;
mod components;
pub(crate) mod utils;

use std::sync::Arc;
use leptos_meta::*;
use leptos::{mount::mount_to_body, prelude::*};
use rigz_runtime::{eval, CAPTURE};
use icondata::LuPlay;

static ERRORS_INPUT: &str = include_str!("examples/errors.rg");

static PROCESSES_INPUT: &str = include_str!("examples/processes.rg");

static LOOPS_INPUT: &str = include_str!("examples/loops.rg");

static TESTS_INPUT: &str = include_str!("examples/tests.rg");

static OBJECTS_INPUT: &str = include_str!("examples/objects.rg");
static BUILTIN_INPUT: &str = include_str!("examples/builtin.rg");

use rigz_runtime::runtime::test;
use crate::code::{register_rigz, CodeEditor};

use crate::components::{Footer, Header, Icon, LogLevelSelector, Results, RunResult, TextAccordion};
use crate::utils::log_capture;
use crate::utils::log_capture::{set_write_signal, CaptureSignal};

fn set_example_input(value: String, set_contents: WriteSignal<String>) {
    let input = match value.as_str() {
        "test" => TESTS_INPUT,
        "errors" => ERRORS_INPUT,
        "processes" => PROCESSES_INPUT,
        "loops" => LOOPS_INPUT,
        "objects" => OBJECTS_INPUT,
        "builtin" => BUILTIN_INPUT,
        _ => return
    };
    set_contents.set(input.to_string())
}

#[component]
fn Main() -> impl IntoView {
    let (contents, set_contents) = signal(BUILTIN_INPUT.trim().to_string());
    let (results, set_result) = signal(RunResult::default());
    let (logs, set_logs) = signal::<Vec<String>>(vec![]);
    let (print, set_print) = signal(String::new());
    let (err, set_err) = signal(String::new());

    let std_out: Arc<CaptureSignal> = Arc::new(set_print.into());
    let std_err: Arc<CaptureSignal> = Arc::new(set_err.into());
    let _ = CAPTURE.out.write().unwrap().insert(std_out);
    let _ = CAPTURE.err.write().unwrap().insert(std_err);
    set_write_signal(set_logs);

    view! {
        <main class="flex-1 w-full mx-auto md:px-4 py-6">
            <div class="flex h-full space-y-6">
                <div class="flex-1 md:rounded-lg shadow-sm p-4 flex flex-col min-h-[550px]">
                    <div class="md:flex items-center justify-between mb-2 gap-2">
                        <h2 class="text-2xl font-semibold text-gray-800 dark:text-gray-100">Editor</h2>
                        <div class="flex flex-wrap gap-3 justify-end max-sm:px-4">
                            <button
                                class="flex px-6 py-1 gap-2 items-center bg-green-500 text-white font-semibold rounded-md shadow hover:opacity-80 focus:outline-none focus:ring-2 focus:ring-green-400 transition-colors"
                                on:click=move |_| {
                                    set_err.set(String::new());
                                    set_print.set(String::new());
                                    set_logs.set(vec![]);
                                    set_result.set(eval(contents.get())
                                        .map_err(RunResult::Failure)
                                        .map(RunResult::Success)
                                        .unwrap_or_else(|err| err)
                                    )
                                }
                            >
                                Run
                                <Icon icon=LuPlay height="1rem" width="1rem" />
                            </button>
                            <button
                                class="flex-1 px-6 py-1 bg-yellow-500 text-gray-900 font-semibold rounded-md shadow hover:opacity-80 focus:outline-none focus:ring-2 focus:ring-yellow-400 transition-colors"
                                on:click=move |_| {
                                    set_err.set(String::new());
                                    set_print.set(String::new());
                                    set_logs.set(vec![]);
                                    set_result.set(test(contents.get())
                                        .map_err(RunResult::Failure)
                                        .map(RunResult::Test)
                                        .unwrap_or_else(|err| err)
                                    )
                                }
                            >
                                Test
                            </button>
                            <button
                                class="flex-1 sm:flex-none px-6 py-1 bg-gray-600 text-white font-semibold rounded-md shadow hover:opacity-80 focus:outline-none focus:ring-2 focus:ring-gray-900 transition-colors"
                                on:click=move |_| {
                                    set_contents.set(rigz_ast::format(contents.get()))
                                }
                            >
                                Format
                            </button>
                        </div>
                    </div>
                    <div class="flex flex-1 flex-col lg:flex-row w-full my-2 gap-4">
                        <div>
                            <div class="w-max flex gap-4 mb-2 items-center">
                                <h3 class="text-lg font-semibold">Examples</h3>
                                <select on:change=move |x| set_example_input(event_target_value(&x), set_contents) class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-md focus:ring-blue-500 focus:border-blue-500 block w-full p-1 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500">
                                    <option value="builtin">Core Features</option>
                                    <option value="test">Test</option>
                                    <option value="errors">Errors</option>
                                    <option value="loops">Loops</option>
                                    <option value="processes">Processes</option>
                                    <option value="objects">Objects / Imports</option>
                                </select>
                            </div>
                            <CodeEditor contents={contents} set_contents={set_contents} />
                        </div>
                        <div class="grid gap-y-2 w-full">
                            <TextAccordion title="Result">
                                <p class="text-sm text-gray-500 dark:text-gray-400">All print/log output is shown in JavaScript console</p>
                                <Results results={results}/>
                            </TextAccordion>
                            <TextAccordion title="Output">
                                <textarea
                                    class="w-full h-24 p-4 bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-700 rounded-md text-gray-800 dark:text-gray-100 font-mono text-sm whitespace-pre-wrap resize-none"
                                    readonly
                                >
                                    {
                                        print.get()
                                    }
                                </textarea>
                            </TextAccordion>
                            <TextAccordion title="Errors">
                                <textarea
                                    class="w-full h-24 p-4 bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-700 rounded-md text-gray-800 dark:text-gray-100 font-mono text-sm whitespace-pre-wrap resize-none"
                                    readonly
                                >
                                    {
                                        err.get()
                                    }
                                </textarea>
                            </TextAccordion>
                            <TextAccordion title="Logs">
                                <LogLevelSelector />
                                <textarea
                                    class="w-full h-48 overflow-y-auto p-4 bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-700 rounded-md text-gray-800 dark:text-gray-100 font-mono text-sm whitespace-pre-wrap resize-none"
                                    readonly
                                >
                                    {
                                        logs.get().join("\n")
                                    }
                                </textarea>
                            </TextAccordion>
                        </div>
                    </div>
                </div>
            </div>
        </main>
    }
}

#[component]
fn App() -> impl IntoView {
    provide_meta_context();
    let (is_dark, set_is_dark) = signal(true);

    view! {
        <Html
            {..}
            lang="en"
            dir="ltr"
            class=("dark", move || is_dark.get())
        />
        <Title text="Rigz REPL - Online Code Editor"/>
        <Meta charset="UTF-8"/>
        <Meta name="description" content="Online REPL for the Rigz programming language"/>

        <div class="flex flex-col min-h-screen bg-gray-50 dark:bg-gray-900 transition-colors">
            <Header is_dark={is_dark} set_is_dark={set_is_dark} />
            <Main />
            <Footer />
        </div>
    }
}

#[cfg(target_family = "wasm")]
unsafe extern "C" {
    fn __wasm_call_ctors();
}

fn main() {
    #[cfg(target_family = "wasm")]
    unsafe {
        __wasm_call_ctors();
    }
    log_capture::init(log::Level::Info);
    console_error_panic_hook::set_once();
    register_rigz();
    mount_to_body(App)
}