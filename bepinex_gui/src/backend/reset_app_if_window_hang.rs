use std::process::exit;

use std::fs;

use std::process::Command;

use std::env;

use crate::config;

use super::process;

// Bad serialized app settings can sometimes make
// the gui window not respond
// bandaid fix that call winapi for checking if the window hung
// and reset the process with a cleaned settings file if so
pub fn spawn_thread() {
    process::spawn_thread_check_if_process_is_hung(|| {
        if let Some(app_ron_file_path) = config::get_app_ron_file_full_path() {
            let current_exe =
                env::current_exe().expect("Failed to retrieve current executable path");

            let args: Vec<String> = env::args().collect();

            let mut command = Command::new(current_exe);
            command.args(args[1..].iter());

            match fs::remove_file(app_ron_file_path) {
                Ok(_) => {}
                Err(err) => {
                    tracing::error!("{}", err);
                }
            }

            match command.spawn() {
                Ok(_) => {
                    exit(0);
                }
                Err(err) => {
                    tracing::error!("{}", err);
                }
            }
        }
    });
}
