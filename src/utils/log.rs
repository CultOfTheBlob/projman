use colored::Colorize;
use notify_rust::{Notification, Timeout};

#[derive(Debug, Clone, Copy)]
pub enum LogType {
    Info,
    Error,
}

pub fn log(message: &str, log_type: LogType) {
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
                eprintln!("{} {}", "ERROR:".red().bold(), message.to_string().bold());
            }
            LogType::Info => {
                eprintln!("{} {}", "INFO:".blue().bold(), message.to_string().bold());
            }
        }
    }
}
