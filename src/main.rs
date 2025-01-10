use leptos::*;
use leptos_meta::*;
use rigz_runtime::eval;
static DEFAULT_INPUT: &str = r#"
mut a = 1
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

  assert_eq foo, 42
  # scopes are only processed once
  assert_eq foo, 42
end

foo
"#;

use rigz_runtime::runtime::test;

#[component]
fn App() -> impl IntoView {
    let (contents, set_contents) = create_signal(DEFAULT_INPUT.trim().to_string());
    let (results, set_result) = create_signal(String::new());
    let (is_dark, set_is_dark) = create_signal(true); // Manual toggle for dark mode

    view! {
        <Html lang="en" dir="ltr" attr:data-theme=move || if is_dark() { "dark" } else { "light" } class=move || if is_dark() { "dark" } else { "" } />

        <Title text="rigz REPL"/>

        <Meta charset="UTF-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>

        <section class="flex flex-col items-center gap-4 p-6 min-h-screen bg-gray-50 dark:bg-gray-900">
            <h1 class="text-2xl font-bold text-gray-800 dark:text-gray-100">Rigz REPL</h1>

            <textarea
                class="w-full max-w-2xl h-48 p-4 border border-gray-300 dark:border-gray-700 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 dark:focus:ring-blue-400 resize-none bg-gray-100 dark:bg-gray-800 text-gray-800 dark:text-gray-100 font-mono"
                placeholder="Write your code here..."
                on:change=move |ev| set_contents(event_target_value(&ev))
            >
                { move || contents() }
            </textarea>

            <p class="text-gray-600 dark:text-gray-400">All print/log output is shown in JavaScript console</p>

            <div class="container mx-auto">
                <p class="text-lg font-semibold text-gray-800 dark:text-gray-100">Output</p>
                <p class="w-full p-4 mt-2 bg-gray-100 dark:bg-gray-800 border border-gray-300 dark:border-gray-700 rounded-md text-gray-800 dark:text-gray-100 font-mono whitespace-pre-wrap">
                    { move || results() }
                </p>
            </div>

            <div class="flex space-x-4 mt-4">
                <button
                    class="px-4 py-2 bg-green-500 text-white font-semibold rounded-md shadow hover:bg-green-600 focus:outline-none focus:ring-2 focus:ring-green-400"
                    on:click=move |_| {
                        set_result(eval(contents().as_str())
                            .map_err(|e| format!("Error: {e:?}"))
                            .map(|v| v.to_string())
                            .unwrap_or_else(|err| err)
                        )
                    }
                >
                    Run
                </button>

                <button
                    class="px-4 py-2 bg-yellow-500 text-white font-semibold rounded-md shadow hover:bg-yellow-600 focus:outline-none focus:ring-2 focus:ring-yellow-400"
                    on:click=move |_| {
                        set_result(test(contents().as_str())
                            .map_err(|e| format!("Error: {e:?}"))
                            .map(|v| v.to_string())
                            .unwrap_or_else(|err| err)
                        )
                    }
                >
                    Test
                </button>
            </div>
            <footer class="w-full max-w-2xl mt-8 border-t border-gray-300 dark:border-gray-700 pt-4 flex justify-between text-gray-600 dark:text-gray-400">
                <button
                    class="mb-4 px-3 py-1 text-sm font-semibold bg-blue-500 text-white rounded-md shadow hover:bg-blue-600 focus:outline-none"
                    on:click=move |_| set_is_dark(!is_dark())
                    aria-label=move || if is_dark() { "Switch to Light Mode" } else { "Switch to Dark Mode" }
                >
                    { move || if is_dark() {
                        "Light"
                    } else {
                        "Dark"
                    }}
                </button>
                <a href="//rigz-lang.org" class="hover:text-blue-500" rel="external">Rigz</a>
                <a href="//docs.rigz-lang.org" class="hover:text-blue-500" rel="external">Docs</a>
            </footer>
        </section>
    }
}


fn main() {
    // min level must be info to ensure print output is displayed
    let _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|| view! {
        <App/>
    })
}