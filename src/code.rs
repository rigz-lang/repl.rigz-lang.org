use gloo_utils::format::JsValueSerdeExt;
use js_sys::{
    Object,
    Reflect::{get, set},
    Function
};
use leptos::{component, view, IntoView};
use leptos::html::InnerHtml;
use leptos::prelude::{ReadSignal, WriteSignal};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use wasm_bindgen::closure::WasmClosureFnOnce;
// modified from https://github.com/leptos-rs/leptos/blob/main/examples/axum_js_ssr/src/hljs.rs

// node_module has default export, need to rename to defaultMod
// todo there should be a way to use default namespace
#[wasm_bindgen(
    module = "/public/js/highlight.min.js"
)]
extern "C" {
    type HighlightOptions;

    #[wasm_bindgen(catch, js_namespace = defaultMod, js_name = highlight)]
    fn highlight_lang(
        code: String,
        highlighter: Object,
    ) -> Result<Object, JsValue>;

    #[wasm_bindgen(catch, js_namespace = defaultMod, js_name = registerLanguage)]
    fn register(
        language: String,
        options: JsValue,
    ) -> Result<(), JsValue>;
}

#[wasm_bindgen(
    module = "/node_modules/@rigz-lang/highlight.js/index.js"
)]
extern "C" {
    #[wasm_bindgen(catch, js_name = rigz)]
    fn rigz_lang(
        highlighter: Object,
    ) -> Result<JsValue, JsValue>;
}

pub fn register_rigz() {
    register("rigz".to_string(), rigz_lang.into_js_function())
        .expect("failed to register language")
}

pub fn highlight(code: String, lang: String) -> Option<String> {
    let options = Object::new();
    set(&options, &"language".into(), &lang.into())
        .expect("failed to assign lang to options");
    highlight_lang(code, options)
        .map(|result| {
            let value = get(&result, &"value".into())
                .expect("HighlightResult failed to contain the value key");
            value.into_serde().expect("Value should have been a string")
        })
        .ok()
}

use leptos::prelude::*;

// modified from https://github.com/uiwjs/react-textarea-code-editor/blob/main/core/src/Editor.tsx
#[component]
pub fn CodeEditor(
    contents: ReadSignal<String>,
    #[prop(optional)]
    set_contents: Option<WriteSignal<String>>
) -> impl IntoView {
    view! {
        <div class="container min-h-[450px] w-full lg:w-[60vw] border border-gray-300 dark:border-gray-700 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 dark:focus:ring-blue-400 bg-gray-100 dark:bg-gray-800 text-gray-800 dark:text-gray-100 ">
            <Show when={move || set_contents.is_some()}>
                <textarea
                    class="resize-none p-4"
                    placeholder="Write your Rigz code here..."
                    spellcheck="false"
                    on:input=move |ev| set_contents.unwrap().set(event_target_value(&ev))
                >
                    { move || contents.get() }
                </textarea>
            </Show>

            <div class="p-4">
                <pre aria_hidden={"true"} class="language-rigz font-mono">
                    <code inner_html={move || highlight(contents.get(), "rigz".to_string()).into_render()} />
                    <br />
                </pre>
            </div>
        </div>
    }
}

