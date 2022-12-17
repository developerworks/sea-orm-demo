pub fn format_datetime(dt: DateTimeUtc) -> String {
    format!("{}", dt.with_timezone(&chrono::Local).format("%Y-%m-%d %T"))
}
