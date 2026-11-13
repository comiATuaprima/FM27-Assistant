//! Taalondersteuning: Nederlands, Engels en Portugees europeu (pt-PT).
//! Bestaande chamadas t!(nl, en) mantêm o comportamento original;
//! novas chamadas podem usar t!(nl, en, pt).

use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU8, Ordering};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Lang {
    Nl,
    En,
    Pt,
}

static LANG: AtomicU8 = AtomicU8::new(1);

pub fn set(lang: Lang) {
    let value = match lang {
        Lang::Nl => 0,
        Lang::En => 1,
        Lang::Pt => 2,
    };
    LANG.store(value, Ordering::Relaxed);
}

pub fn get() -> Lang {
    match LANG.load(Ordering::Relaxed) {
        0 => Lang::Nl,
        2 => Lang::Pt,
        _ => Lang::En,
    }
}

pub fn is_nl() -> bool { get() == Lang::Nl }
pub fn is_en() -> bool { get() == Lang::En }
pub fn is_pt() -> bool { get() == Lang::Pt }

/// Deteta neerlandês e português pela língua do sistema; inglês é a predefinição.
pub fn detect() -> Lang {
    match sys_locale::get_locale() {
        Some(locale) if locale.to_lowercase().starts_with("nl") => Lang::Nl,
        Some(locale) if locale.to_lowercase().starts_with("pt") => Lang::Pt,
        _ => Lang::En,
    }
}

/// Duas variantes preservam a compatibilidade; três variantes acrescentam pt-PT.
#[macro_export]
macro_rules! t {
    ($nl:expr, $en:expr $(,)?) => {
        if $crate::i18n::is_nl() { $nl } else { $en }
    };
    ($nl:expr, $en:expr, $pt:expr $(,)?) => {
        match $crate::i18n::get() {
            $crate::i18n::Lang::Nl => $nl,
            $crate::i18n::Lang::En => $en,
            $crate::i18n::Lang::Pt => $pt,
        }
    };
}

pub fn month_short(month: u32) -> &'static str {
    const NL: [&str; 12] = ["jan", "feb", "mrt", "apr", "mei", "jun", "jul", "aug", "sep", "okt", "nov", "dec"];
    const EN: [&str; 12] = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
    const PT: [&str; 12] = ["jan", "fev", "mar", "abr", "mai", "jun", "jul", "ago", "set", "out", "nov", "dez"];
    let index = (month.clamp(1, 12) - 1) as usize;
    match get() {
        Lang::Nl => NL[index],
        Lang::En => EN[index],
        Lang::Pt => PT[index],
    }
}

pub fn weekday_short(index: usize) -> &'static str {
    const NL: [&str; 7] = ["ma", "di", "wo", "do", "vr", "za", "zo"];
    const EN: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
    const PT: [&str; 7] = ["seg", "ter", "qua", "qui", "sex", "sáb", "dom"];
    match get() {
        Lang::Nl => NL[index % 7],
        Lang::En => EN[index % 7],
        Lang::Pt => PT[index % 7],
    }
}

pub fn date(date: chrono::NaiveDate) -> String {
    use chrono::Datelike;
    match get() {
        Lang::Nl => format!("{} {} {}", date.day(), month_short(date.month()), date.year()),
        Lang::En => format!("{} {} {}", month_short(date.month()), date.day(), date.year()),
        Lang::Pt => format!("{} de {} de {}", date.day(), month_short(date.month()), date.year()),
    }
}

pub fn date_short(date: chrono::NaiveDate) -> String {
    use chrono::Datelike;
    match get() {
        Lang::Nl => format!("{} {}", date.day(), month_short(date.month())),
        Lang::En => format!("{} {}", month_short(date.month()), date.day()),
        Lang::Pt => format!("{} {}", date.day(), month_short(date.month())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    #[test]
    fn portuguese_date_labels_are_european() {
        let _guard = TEST_LOCK.lock().unwrap();
        set(Lang::Pt);
        assert_eq!(month_short(3), "mar");
        assert_eq!(weekday_short(0), "seg");
        let value = chrono::NaiveDate::from_ymd_opt(2026, 3, 5).unwrap();
        assert_eq!(date(value), "5 de mar de 2026");
        assert_eq!(date_short(value), "5 mar");
    }

    #[test]
    fn two_language_macro_remains_compatible() {
        let _guard = TEST_LOCK.lock().unwrap();
        set(Lang::En);
        assert_eq!(crate::t!("Nederlands", "English"), "English");
        set(Lang::Nl);
        assert_eq!(crate::t!("Nederlands", "English"), "Nederlands");
    }
}
