use leptos::prelude::*;
use log::Level;
use crate::utils::title_case;

#[component]
pub fn LogLevelSelector() -> impl IntoView {
    let (level, set_level) = signal(log::Level::Info);
    let levels = vec![
        Level::Error,
        Level::Warn,
        Level::Info,
        Level::Debug,
        Level::Trace,
    ];

    Effect::new(move || {
        level.with(|level| {
            log::set_max_level(level.to_level_filter());
        })
    });

    view! {
        <div class="flex flex-col md:flex-row gap-2 p-4">
            <h3 class="text-lg font-semibold">Log Level:</h3>
            <div class="flex flex-wrap gap-4">
                {
                    levels.iter().copied().map(|l| {
                        view! {
                            <label class="inline-flex items-center cursor-pointer">
                                <input
                                    type="radio"
                                    name="log_level"
                                    value={l.to_string()}
                                    checked={move || level.get() == l}
                                    on:change=move |_| set_level.set(l)
                                    class="form-radio h-4 w-4 focus:ring-blue-500 border-gray-300"
                                />
                                <span class="ml-2">{title_case(l)}</span>
                            </label>
                        }
                    }).collect_view()
                }
            </div>
        </div>
    }
}