use leptos::*;
use leptos_meta::*;
use rigz_runtime::eval;
use rigz_runtime::runtime::test;

#[component]
fn App() -> impl IntoView {
    let (contents, set_contents) = create_signal("puts 2 + 2".to_string());
    let (results, set_result) = create_signal(String::new());

    view! {
        <Html lang="en" dir="ltr" attr:data-theme="light"/>

        <Title text="rigz REPL"/>

        <Meta charset="UTF-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <section class="flex flex-col items-center gap-4 p-6 bg-gray-50 min-h-screen">
            <h1 class="text-2xl font-bold text-gray-800">Rigz REPL</h1>

            <textarea
                class="w-full max-w-2xl h-48 p-4 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 resize-none text-gray-800 font-mono"
                placeholder="Write your code here..."
                on:change=move |ev| set_contents(event_target_value(&ev))
            >
                { move || contents() }
            </textarea>

            <p class="text-gray-600">All print/log output is shown in JavaScript console</p>

            <div class="container mx-auto">
                <p class="text-lg font-semibold text-gray-800">Output</p>
                <p class="w-full p-4 mt-2 bg-gray-100 border border-gray-300 rounded-md text-gray-800 font-mono whitespace-pre-wrap">
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