use std::{
    os::windows::process::CommandExt,
    path::{Path, PathBuf},
    process::Command,
};

pub fn open_path_in_explorer(file: &PathBuf) {
    if let Err(e) = (|| {
        #[cfg(target_os = "windows")]
        return Command::new("explorer").arg(file).spawn();

        #[cfg(target_os = "macos")]
        return Command::new("open").arg(path.to_string()).spawn();

        #[cfg(target_os = "linux")]
        return Command::new("xdg-open").arg(path.to_string()).spawn();

        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Unsupported OS",
        ))
    })() {
        tracing::error!("{}", e);
    }
}

pub fn highlight_path_in_explorer(file: &Path) {
    if let Err(e) = (|| {
        #[cfg(target_os = "windows")]
        {
            if let Some(s) = file.to_str() {
                let mut s = s.replace('/', r#"\"#);
                s.push('\"');
                let s = s.as_str();

                return Command::new("explorer")
                    .raw_arg("/select,\"".to_string() + s)
                    .spawn();
            } else {
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Can't convert PathBuf to_str",
                ))
            }
        }

        #[cfg(target_os = "macos")]
        return Command::new("open").arg("-R").arg(path.to_string()).spawn();

        #[cfg(target_os = "linux")]
        return Command::new("xdg-open")
            .arg("--select")
            .arg(path.to_string())
            .spawn();

        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Unsupported OS",
        ))
    })() {
        tracing::error!("{}", e);
    }
}
