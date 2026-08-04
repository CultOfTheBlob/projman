use colored::Colorize;
use notify_rust::{Notification, Timeout};
use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex, PoisonError},
    time::{Duration, Instant},
};

static LAST_LOGS: LazyLock<Mutex<HashMap<String, Instant>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

const LOG_COOLDOWN: Duration = Duration::from_secs(5);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogType {
    Info,
    Error,
}

pub fn log(message: &str, log_type: LogType) {
    {
        let mut logs = LAST_LOGS.lock().unwrap_or_else(PoisonError::into_inner);

        let now = Instant::now();

        logs.retain(|_, t| now.duration_since(*t) < Duration::from_mins(1));

        match logs.get(message) {
            Some(last) if now.duration_since(*last) < LOG_COOLDOWN => return,
            _ => {
                logs.insert(message.to_owned(), now);
            }
        }
    }

    let mut notification = Notification::new();
    notification
        .summary("ProjMan:")
        .body(message)
        .timeout(Timeout::Milliseconds(4000));

    let result = match log_type {
        LogType::Error => notification.icon("dialog-error").show(),
        LogType::Info => notification.icon("dialog-information").show(),
    };

    if result.is_err() {
        match log_type {
            LogType::Error => {
                eprintln!("{} {}", "ERROR:".red().bold(), message.bold());
            }
            LogType::Info => {
                eprintln!("{} {}", "INFO:".blue().bold(), message.bold());
            }
        }
    }
}
