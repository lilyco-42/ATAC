pub mod en;
pub mod zh;

use parking_lot::RwLock;

pub static CURRENT_LANG: RwLock<Lang> = RwLock::new(Lang::En);

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Lang {
    En,
    Zh,
}

impl Lang {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "zh" | "zh-cn" | "zh_cn" | "chinese" | "中文" => Lang::Zh,
            _ => Lang::En,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Lang::En => "en",
            Lang::Zh => "zh",
        }
    }
}

pub fn set_lang(lang: Lang) {
    *CURRENT_LANG.write() = lang;
}

pub fn current_lang() -> Lang {
    *CURRENT_LANG.read()
}

/// Get localized string by English key.
/// Falls back to the key itself if no translation is found.
pub fn get(key: &'static str) -> &'static str {
    match *CURRENT_LANG.read() {
        Lang::En => key,
        Lang::Zh => zh::get(key),
    }
}

/// Macro: translate a string literal.
/// Usage: `t!("Hello")` returns `&str`.
/// For formatted strings: `format!(t!("Hello {}"), name)`
#[macro_export]
macro_rules! t {
    ($key:expr) => {
        $crate::locale::get($key)
    };
}
