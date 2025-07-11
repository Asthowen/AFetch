pub trait ToOptionString {
    fn to_option_string(self) -> Option<String>;
}

impl ToOptionString for Option<String> {
    fn to_option_string(self) -> Option<String> {
        self
    }
}

impl ToOptionString for Option<&str> {
    fn to_option_string(self) -> Option<String> {
        self.map(str::to_owned)
    }
}

impl ToOptionString for String {
    fn to_option_string(self) -> Option<String> {
        Some(self)
    }
}

impl ToOptionString for &str {
    fn to_option_string(self) -> Option<String> {
        Some(self.to_string())
    }
}

#[macro_export]
macro_rules! filtered_values {
    ($fields:expr, [ $( ($field:expr, $value_expr:expr) ),* $(,)? ]) => {{
        let mut info: Vec<InfoValue> = Vec::new();
        $(
            if $fields.contains(&$field) {
                if let Some(value) = $value_expr.to_option_string() {
                    info.push(InfoValue {
                        field: $field,
                        value,
                    });
                }
            }
        )*
        info
    }};
}
