mod code;

use leptos_meta::*;
use leptos::{mount::mount_to_body, prelude::*, svg};
use rigz_runtime::{eval, RuntimeError};
use icondata::{LuSun, LuMoon, SiGitlab, LuPlay};
use itertools::Itertools;

static ERRORS_INPUT: &str = r#"fn foo = raise "foo failed"

bar = foo catch = "hello"

baz = foo catch
    1 + 2
end

bar + baz
"#;

static PROCESSES_INPUT: &str = r#"a = spawn do
    "first"
end

b = spawn do
    "second"
end

receive [a, b]
"#;

static LOOPS_INPUT: &str = r#"
lists = [for a in [1, 2, 3, 4]: a * a]

maps = {for k, v in {a = 1, b = 2, c = 3, d = 4}: v, k * v}

mut a = 0
loop
    a += 1
    break if a == 10
end

mut res = []

for v in lists
    next unless v % 2 == 0

    res.push v
end

{ a, lists, maps, res }
"#;

static TESTS_INPUT: &str = r#"mut a = 1
bar = do
    a += 1
    21 * a
end

fn foo = bar

@test
fn test_foo
  mut a = 1 # variables in main scope are not available for tests, this will be fixed in a later version
  bar = do
    a += 1
    21 * a
  end

  # assert_eq returns error if false
  # try is required because there are no panics
  try assert_eq foo, 42
  # scopes are only processed once
  # try not required because last line is return value
  assert_eq foo, 42
end

foo
"#;

use rigz_runtime::runtime::test;
use crate::code::{highlight, register_rigz, CodeEditor};

/// The Icon component, modified from https://github.com/carloskiki/leptos-icons/blob/main/src/lib.rs
#[component]
pub fn Icon(
    /// The icon to render.
    #[prop(into)]
    icon: Signal<icondata::Icon>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(into, optional)] width: MaybeProp<String>,
    #[prop(into, optional)] height: MaybeProp<String>,
    #[prop(into, optional)] class: MaybeProp<String>,
) -> impl IntoView {
    move || {
        let icon = icon.get();
        svg::svg()
            .style(match (style.get(), icon.style) {
                (Some(a), Some(b)) => Some(format!("{b} {a}")),
                (Some(a), None) => Some(a),
                (None, Some(b)) => Some(b.to_string()),
                _ => None,
            })
            .attr("x", icon.x)
            .attr("y", icon.y)
            .attr("width", width.get().unwrap_or_else(|| "1em".to_string()))
            .attr("height", height.get().unwrap_or_else(|| "1em".to_string()))
            .attr("viewBox", icon.view_box)
            .attr("stroke-linecap", icon.stroke_linecap)
            .attr("stroke-linejoin", icon.stroke_linejoin)
            .attr("stroke-width", icon.stroke_width)
            .attr("stroke", icon.stroke)
            .attr("fill", icon.fill.unwrap_or("currentColor"))
            .attr("role", "graphics-symbol")
            .class(class.get().unwrap_or_default())
            .inner_html(icon.data)
    }
}

use rigz_core::{ObjectValue, PrimitiveValue, TestResults};

#[derive(Clone, Default)]
enum RunResult {
    #[default]
    None,
    Success(ObjectValue),
    Test(TestResults),
    Failure(RuntimeError)
}

#[component]
fn Results(results: ReadSignal<RunResult>) -> impl IntoView {
    move || match results.get() {
        RunResult::None => {
            view! {
                <textarea
                    class="w-full h-32 p-4 bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-700 rounded-md text-gray-800 dark:text-gray-100 font-mono text-sm whitespace-pre-wrap resize-none"
                    readonly 
                />
            }.into_any()
        }
        RunResult::Failure(v) => {
            view! {
                <textarea
                    class="w-full h-32 p-4 bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-700 rounded-md text-gray-800 dark:text-gray-100 font-mono text-sm whitespace-pre-wrap resize-none"
                    readonly
                >
                    { move || v.to_string() }
                </textarea>
            }.into_any()
        }
        RunResult::Success(v) => {
            view! {
                <div
                    class="w-full h-32 p-4 bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-700 rounded-md text-gray-800 dark:text-gray-100 font-mono text-sm whitespace-pre-wrap resize-none"
                >
                    <pre aria_hidden={"true"} class="language-rigz font-mono text-wrap">
                        <code inner_html={move || highlight(if matches!(v, ObjectValue::Primitive(PrimitiveValue::String(_))) { format!("'{v}'")} else { v.to_string() }, "rigz".to_string()).into_render()} />
                        <br />
                    </pre>
                </div>
            }.into_any()
        }
        RunResult::Test(v) => {
            let success = v.success();
            view! {
                <pre
                    class="text-wrap w-full h-32 p-4 bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-700 rounded-md text-gray-800 dark:text-gray-100 font-mono text-sm whitespace-pre-wrap"
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

#[component]
fn Header(is_dark: ReadSignal<bool>, set_is_dark: WriteSignal<bool>) -> impl IntoView {
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

#[component]
fn Gitlab() -> impl IntoView {
    view! {
        <a href="https://gitlab.com/rigz_lang/repl.rigz-lang.org" class="hover:text-[#FC6D26] transition-colors" rel="external">
            <Icon icon=SiGitlab height="1.5rem" width="1.5rem" />
        </a>
    }
}

#[component]
fn Footer() -> impl IntoView {
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

fn set_example_input(value: String, set_contents: WriteSignal<String>) {
    let input = match value.as_str() {
        "test" => TESTS_INPUT,
        "errors" => ERRORS_INPUT,
        "processes" => PROCESSES_INPUT,
        "loops" => LOOPS_INPUT,
        _ => return
    };
    set_contents.set(input.to_string())
}

#[component]
fn Main() -> impl IntoView {
    let (contents, set_contents) = signal(TESTS_INPUT.trim().to_string());
    let (results, set_result) = signal(RunResult::default());

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
                                    <option value="test">Test</option>
                                    <option value="errors">Errors</option>
                                    <option value="loops">Loops</option>
                                    <option value="processes">Processes</option>
                                </select>
                            </div>
                            <CodeEditor contents={contents} set_contents={set_contents} />
                        </div>
                        <div class="md:rounded-lg flex-grow">
                            <div class="flex items-center justify-between mb-2">
                                <h2 class="text-xl font-semibold text-gray-800 dark:text-gray-100">Result</h2>
                                <p class="text-sm text-gray-500 dark:text-gray-400">All print/log output is shown in JavaScript console</p>
                            </div>
                            <Results results={results}/>
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

    console_error_panic_hook::set_once();
    let _ = console_log::init_with_level(log::Level::Info);
    register_rigz();
    mount_to_body(App)
}