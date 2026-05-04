use anyhow::anyhow;
use dioxus::prelude::*;

pub const DEFAULT_LOCALE: &str = "zh-CN";
pub const ZH_CN_LOCALE: &str = "zh-CN";
pub const EN_US_LOCALE: &str = "en-US";

const LOCALE_STORAGE_KEY: &str = "baker_dx_locale";

const LOCAL_STORAGE_GET_LOCALE_SCRIPT: &str = r#"
    const key = await dioxus.recv();
    try {
        return window.localStorage.getItem(key);
    } catch {
        return null;
    }
"#;

const LOCAL_STORAGE_SET_LOCALE_SCRIPT: &str = r#"
    const key = await dioxus.recv();
    const locale = await dioxus.recv();
    try {
        window.localStorage.setItem(key, locale);
        return true;
    } catch (err) {
        return String(err);
    }
"#;

#[derive(Clone, Copy)]
pub struct LocaleContext {
    pub locale: Signal<String>,
}

impl LocaleContext {
    pub fn current(&self) -> String {
        (self.locale)()
    }
}

pub fn use_locale_refresh() -> String {
    use_context::<LocaleContext>().current()
}

pub fn normalize_locale(locale: &str) -> &'static str {
    match locale.trim() {
        ZH_CN_LOCALE => ZH_CN_LOCALE,
        EN_US_LOCALE => EN_US_LOCALE,
        _ => DEFAULT_LOCALE,
    }
}

pub fn apply_locale(locale: &str) -> &'static str {
    let locale = normalize_locale(locale);
    rust_i18n::set_locale(locale);
    locale
}

pub async fn load_locale_from_local_storage() -> anyhow::Result<&'static str> {
    let eval = document::eval(LOCAL_STORAGE_GET_LOCALE_SCRIPT);
    eval.send(LOCALE_STORAGE_KEY.to_string())
        .map_err(|err| anyhow!(err.to_string()))?;

    let value = eval.await.map_err(|err| anyhow!(err.to_string()))?;
    let locale = value
        .as_str()
        .map(normalize_locale)
        .unwrap_or(DEFAULT_LOCALE);
    Ok(locale)
}

pub async fn save_locale_to_local_storage(locale: &str) -> anyhow::Result<()> {
    let locale = normalize_locale(locale);
    let eval = document::eval(LOCAL_STORAGE_SET_LOCALE_SCRIPT);
    eval.send(LOCALE_STORAGE_KEY.to_string())
        .map_err(|err| anyhow!(err.to_string()))?;
    eval.send(locale.to_string())
        .map_err(|err| anyhow!(err.to_string()))?;

    let value = eval.await.map_err(|err| anyhow!(err.to_string()))?;
    if value.as_bool() == Some(true) {
        Ok(())
    } else {
        Err(anyhow!(
            "{}",
            value
                .as_str()
                .unwrap_or("failed to save locale to localStorage")
        ))
    }
}
