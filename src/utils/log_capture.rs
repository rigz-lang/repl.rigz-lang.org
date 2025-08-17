use std::sync::{RwLock};
use console_log::log;
use leptos::prelude::{Update, WriteSignal};
use log::{Level, Log, Metadata, Record};
use rigz_runtime::StdOutCapture;


pub struct CaptureSignal {
    write: WriteSignal<String>
}

impl From<WriteSignal<String>> for CaptureSignal {
    fn from(write: WriteSignal<String>) -> Self {
        CaptureSignal { write }
    }
}

impl StdOutCapture for CaptureSignal {
    fn applied(&self, value: String) {
        self.write.update(|s| s.push_str(&value));
    }
}

#[derive(Default)]
pub struct WebLogger {
    write_signal: RwLock<Option<WriteSignal<Vec<String>>>>
}

impl Log for WebLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        match self.write_signal.read() {
            Ok(w) => {
                if let Some(w) = w.as_ref() {
                    w.update(|values|
                    values.push(format!("[{}] {}", record.metadata().level(), record.args()))
                    )
                }
            }
            Err(_) => {
                // todo notify user logging is broken and needs to be refreshed
                return;
            }
        }
        log(record);
    }

    fn flush(&self) {}
}

static LOG: WebLogger = WebLogger {
    write_signal: RwLock::new(None)
};

pub fn init(level: Level) {
    log::set_logger(&LOG).expect("Failed to set logger");
    log::set_max_level(level.to_level_filter());
}

pub fn set_write_signal(write_signal: WriteSignal<Vec<String>>) {
    let _ = LOG.write_signal.write().expect("Failed to set signal").insert(write_signal);
}