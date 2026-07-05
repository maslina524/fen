use alloc::borrow::ToOwned;
use alloc::collections::btree_map::BTreeMap;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

pub enum TomlValue {
    Number(f64),
    String(String),
    Bool(bool)
}

impl TomlValue {
    pub fn from(string: &str) -> Self {
        return if let Ok(v) = string.parse::<bool>() {
            Self::Bool(v)
        } else if let Ok(v) = string.parse::<f64>() {
            Self::Number(v)
        } else {
            Self::String(string.to_owned())
        };
    }
}

impl core::fmt::Display for TomlValue {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::String(v) => write!(f, "\"{}\"", v),
            Self::Bool(v) => write!(f, "{}", v),
            Self::Number(v) => write!(f, "{}", v)
        }
    }
}

pub type Name = String;
pub type Category<K, V> = BTreeMap<K, V>;
pub type Value<K, V> = BTreeMap<K, V>;
pub type TomlType = Category<Name, Value<Name, TomlValue>>;

pub struct Toml {
    pub map: TomlType
}

impl Toml {
    pub fn new() -> Self {
        Self { map: Category::new() }
    }

    pub fn from_map(map: TomlType) -> Self {
        Self { map }
    }

    pub fn from_str(string: &str) -> Option<Self> {
        let mut map = Category::new();

        let mut curr_name = String::new();
        let mut curr_cat = Category::new();
    
        for line in string.lines() {
            if line.starts_with('[') && line.ends_with(']') {
                if !curr_cat.is_empty() {
                    map.insert(curr_name, curr_cat);
                }
                curr_name = line[1..line.len() - 1].to_owned();
                curr_cat = Category::new();
            } else {
                let parts: Vec<&str> = line.split("=").collect();
                if parts.len() != 2 { continue; }

                let k = parts[0].trim().to_owned();
                let v = parts[1].trim();

                curr_cat.insert(k, TomlValue::from(v));
            }
        }

        Some(
            Self { map }
        )
    }
}

impl core::fmt::Display for Toml {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut lines = Vec::new();

        for (cat_name, cat) in &self.map {
            lines.push(format!("[{}]", cat_name));
            for (value_name, value) in cat {
                lines.push(format!("	{} = {}", value_name, value));
            }
        }

        write!(f, "{}", lines.join("\n"))
    }
}

#[macro_export]
macro_rules! toml_type {
    () => {
        $crate::toml::Category::new()
    };
    
    ($section:expr => { $($key:expr => $value:expr),* $(,)? }) => {{
        let mut map = $crate::toml::Category::new();
        let mut section = $crate::toml::Category::new();
        $(
            section.insert($key.to_string(), $crate::toml::TomlValue::from($value));
        )*
        map.insert($section.to_string(), section);
        map
    }};
    
    ($($section:expr => { $($key:expr => $value:expr),* $(,)? }),* $(,)?) => {{
        let mut map = $crate::toml::Category::new();
        $(
            let mut section = $crate::toml::Category::new();
            $(
                section.insert($key.to_string(), $crate::toml::TomlValue::from($value));
            )*
            map.insert($section.to_string(), section);
        )*
        map
    }};
}

#[macro_export]
macro_rules! toml {
    ($($section:expr => { $($key:expr => $value:expr),* $(,)? }),* $(,)?) => {{
        $crate::toml::Toml::from_map($crate::toml_type!($($section => { $($key => $value),* }),*))
    }};
}