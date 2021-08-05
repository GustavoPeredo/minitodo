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
num_lines = 8
text_format = "%H:%M"
min_line_length = 20
horizontal_divisor = '-'
vertical_divisor = '|'

[week]
starts_on_monday = true
horizontal_divisor = "-"
show_vertically = false
today_char = "*"
text_format = "%A"

[database]
path = "~/.local/minitodo"
filename = "todo.txt"
"#
}