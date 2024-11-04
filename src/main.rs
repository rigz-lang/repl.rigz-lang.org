use leptos::*;
use leptos_meta::*;
use rigz_runtime::eval;
use rigz_runtime::runtime::test;

#[component]
fn App() -> impl IntoView {
    let (contents, set_contents) = create_signal("puts 2 + 2".to_string());
    let (results, set_result) = create_signal(String::new());

    // todo support syntax highlighting
    // todo support multiple files
    view! {
        <Html lang="en" dir="ltr" attr:data-theme="light"/>

        <Title text="rigz REPL"/>

        <Meta charset="UTF-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <section>
             <textarea
                on:change= move |ev| {
                    set_contents(event_target_value(&ev))
                }
            >
                { move || contents() }
            </textarea>
            <p>All print/log output is shown in JavaScript console</p>
            <p>Output</p>
            <pre>
                { move || results() }
            </pre>
            <button on:click = move |_| { set_result(eval(contents().as_str()).map_err(|e| format!("Error: {e:?}")).map(|v| v.to_string()).unwrap_or_else(|err| err)) }>
                Run
            </button>
            <button on:click = move |_| { set_result(test(contents().as_str()).map_err(|e| format!("Error: {e:?}")).map(|v| v.to_string()).unwrap_or_else(|err| err)) }>
                Test
            </button>
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