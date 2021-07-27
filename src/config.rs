pub fn get_config_path() -> &'static str {
    "~/.config/minitodo"
}

pub fn get_config_name() -> &'static str {
    "minitodo.conf"
}

pub fn get_defaults() -> &'static str {
r#"
[hours]
hours = 2
text_format = "%H:%M:%S"
line_length = 20

[week]
starts_on_monday = true
"#
}