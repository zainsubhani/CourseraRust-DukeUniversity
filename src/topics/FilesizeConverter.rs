enum FileSize {
    bytes(u16),
    kilobytes(u64),
    megabytes(u64),
    gigabytes(u64),
    terabytes(u64),
}
fn format_size(size: u64) -> String {
    let filesize = match size {
        0..=999 => FileSize::bytes(size),
        1000..=999_999 => FileSize::kilobytes(size),
    };
    match filesize {
        FileSize::bytes(bytes) => format!("{} bytes", bytes),
        FileSize::kilobytes(kb) => format!("{:.2} KB", kb as f64 / 1000.0),
        FileSize::megabytes(_) => format!("{:.2} MB", kb as f64 / 1000.0),
        FileSize::gigabytes(_) => format!("{:.2} GB", kb as f64 / 1000.0),
        FileSize::terabytes(_) => format!("{:.2} TB", kb as f64 / 1000.0),
    }
}
