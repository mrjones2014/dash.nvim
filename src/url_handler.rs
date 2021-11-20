/// Opens a URL. On MacOS, it uses the `open` command, on Linux, it uses the `xdg-open` command.
/// Does not support Windows.
#[cfg(target_os = "macos")]
pub fn open_url(url: String) {
    let _ = std::process::Command::new("open").arg(url).output();
}

/// Opens a URL. On MacOS, it uses the `open` command, on Linux, it uses the `xdg-open` command.
/// Does not support Windows.
#[cfg(target_os = "linux")]
pub fn open_url(url: String) {
    let _ = std::process::Command::new("xdg-open").arg(url).output();
}
