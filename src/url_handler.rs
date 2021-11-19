/// Opens a URL on MacOS. Uses MacOS `open` command.
#[cfg(target_os = "macos")]
pub fn open_url(url: String) {
    let _ = std::process::Command::new("open").arg(url).output();
}

/// Opens a URL on Linux. Uses Linux `xdg-open` command.
#[cfg(target_os = "linux")]
pub fn open_url(url: String) {
    let _ = std::process::Command::new("xdg-open").arg(url).output();
}
