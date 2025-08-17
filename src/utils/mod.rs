pub mod log_capture;

use std::fmt::Display;

pub fn title_case<V: Display>(v: V) -> String {
    let t = v.to_string();
    if t.is_empty() {
        return t
    }

    let mut c = t.chars();
    format!("{}{}", c.next().unwrap().to_uppercase(), String::from_iter(c).to_lowercase())
}