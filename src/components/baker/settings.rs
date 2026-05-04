use crate::components::baker::layout::load_repo_config;
use crate::components::baker::locale::{
    EN_US_LOCALE, LocaleContext, ZH_CN_LOCALE, apply_locale, save_locale_to_local_storage,
};
use crate::components::baker::storage::v2::{BackgroundMode, Operator};
use crate::components::baker::{Route, data_url_from_bytes, mime_from_filename, use_synced_field};
use crate::dioxus_elements::FileData;
use dioxus::prelude::*;
use rust_i18n::t;
use uuid::Uuid;

#[component]
pub fn SettingsPage() -> Element {
    let mut app_state = use_context::<Signal<crate::components::baker::storage::v2::AppState>>();
    let locale_context = use_context::<LocaleContext>();
    let mut locale_signal = locale_context.locale;
    let current_locale = locale_context.current();

    let repo_config = load_repo_config().unwrap();
    let repo_url = format!(
        "https://github.com/{}/{}",
        repo_config.owner, repo_config.repo
    );

    let mut operators =
        use_synced_field(app_state, |s| s.operators.clone(), |s, v| s.operators = v);
    let mut background =
        use_synced_field(app_state, |s| s.background.clone(), |s, v| s.background = v);

    let mut new_name = use_signal(|| "".to_string());
    let mut new_avatar_preview = use_signal(|| "".to_string());
    let mut editing_operator_id = use_signal(|| Option::<String>::None);
    let mut edit_name = use_signal(|| "".to_string());
    let mut edit_avatar_preview = use_signal(|| "".to_string());

    #[derive(Clone, PartialEq)]
    enum SettingsSection {
        Operators,
        Background,
        Language,
        About,
    }

    let mut section = use_signal(|| SettingsSection::Operators);

    let handle_add = move |_| {
        let name = new_name();
        let avatar = new_avatar_preview();
        if !name.is_empty() {
            let id = Uuid::new_v4().to_string();
            operators.write().push(Operator {
                id,
                name,
                avatar_url: avatar,
            });
            new_name.set("".to_string());
            new_avatar_preview.set("".to_string());
        }
    };

    let mut handle_delete = move |id: String| {
        operators.write().retain(|op| op.id != id);
    };
    let mut handle_edit_start = move |op: Operator| {
        editing_operator_id.set(Some(op.id.clone()));
        edit_name.set(op.name);
        edit_avatar_preview.set(op.avatar_url);
    };
    let mut handle_edit_cancel = move |_| {
        editing_operator_id.set(None);
    };
    let mut handle_edit_save = move |id: String| {
        let name = edit_name();
        let avatar = edit_avatar_preview();
        if let Some(op) = operators.write().iter_mut().find(|op| op.id == id) {
            op.name = name.clone();
            op.avatar_url = avatar.clone();
        }
        let mut state = app_state.write();
        if let Some(contact) = state.contacts.iter_mut().find(|c| c.id == id) {
            contact.name = name;
            contact.avatar_url = avatar;
        }
        editing_operator_id.set(None);
    };

    let ops_list = operators.read().clone();
    let current_background = background.read().clone();
    let background_mode_value = match current_background.mode {
        BackgroundMode::DotDark => "dot_dark",
        BackgroundMode::DotLight => "dot_light",
        BackgroundMode::CustomColor => "custom_color",
        BackgroundMode::CustomImage => "custom_image",
    };
    let operators_tab_class = if matches!(section(), SettingsSection::Operators) {
        "bg-[#2b2b2b] text-white"
    } else {
        "text-gray-400 hover:text-white hover:bg-white/5"
    };
    let background_tab_class = if matches!(section(), SettingsSection::Background) {
        "bg-[#2b2b2b] text-white"
    } else {
        "text-gray-400 hover:text-white hover:bg-white/5"
    };
    let language_tab_class = if matches!(section(), SettingsSection::Language) {
        "bg-[#2b2b2b] text-white"
    } else {
        "text-gray-400 hover:text-white hover:bg-white/5"
    };
    let about_tab_class = if matches!(section(), SettingsSection::About) {
        "bg-[#2b2b2b] text-white"
    } else {
        "text-gray-400 hover:text-white hover:bg-white/5"
    };
    let settings_title_label = t!("settings.title").to_string();
    let operators_tab_label = t!("settings.tabs.operators").to_string();
    let background_tab_label = t!("settings.tabs.background").to_string();
    let language_tab_label = t!("settings.tabs.language").to_string();
    let about_tab_label = t!("settings.tabs.about").to_string();
    let language_title_label = t!("settings.language.title").to_string();
    let language_description_label = t!("settings.language.description").to_string();
    let language_accuracy_notice_label = t!("settings.language.accuracy_notice").to_string();
    let language_current_label = t!("settings.language.current").to_string();
    let zh_cn_label = t!("locales.zh_cn").to_string();
    let en_us_label = t!("locales.en_us").to_string();
    let zh_cn_selected = current_locale == ZH_CN_LOCALE;
    let en_us_selected = current_locale == EN_US_LOCALE;
    let add_operator_title_label = t!("settings.operators.add_title").to_string();
    let operator_name_placeholder = t!("settings.operators.name_placeholder").to_string();
    let add_operator_button_label = t!("settings.operators.add_button").to_string();
    let cancel_label = t!("common.cancel").to_string();
    let save_label = t!("common.save").to_string();
    let edit_label = t!("common.edit").to_string();
    let delete_label = t!("common.delete").to_string();
    let background_title_label = t!("settings.background.title").to_string();
    let dot_dark_label = t!("settings.background.dot_dark").to_string();
    let dot_light_label = t!("settings.background.dot_light").to_string();
    let custom_color_label = t!("settings.background.custom_color").to_string();
    let custom_image_todo_label = t!("settings.background.custom_image_todo").to_string();
    let about_description_label = t!("settings.about.description").to_string();
    let about_author_label = t!("settings.about.author").to_string();
    let about_open_source_at_label = t!("settings.about.open_source_at").to_string();
    let about_license_heading_label = t!("settings.about.license_heading").to_string();
    let about_license_summary_label = t!("settings.about.license_summary").to_string();

    let background_style = use_memo(move || {
        let bg = background.read().clone();
        match bg.mode {
            BackgroundMode::DotDark => {
                "background-color: #1a1a1a; background-image: radial-gradient(#2a2a2a 1px, transparent 1px); background-size: 20px 20px;".to_string()
            }
            BackgroundMode::DotLight => {
                "background-color: #f2f2f2; background-image: radial-gradient(#d0d0d0 1px, transparent 1px); background-size: 20px 20px;".to_string()
            }
            BackgroundMode::CustomColor => format!("background-color: {};", bg.custom_color),
            BackgroundMode::CustomImage => {
                if bg.custom_image.is_empty() {
                    format!("background-color: {};", bg.custom_color)
                } else {
                    format!("background-image: url({}); background-size: cover; background-position: center; background-repeat: no-repeat; background-color: #1a1a1a;", bg.custom_image)
                }
            }
        }
    });

    let navigator = use_navigator();

    rsx! {
        div {
            class: "w-full h-screen bg-cover bg-center flex flex-col overflow-hidden text-sans",
            style: "{background_style}",
            div { class: "h-14 flex items-center gap-3 px-6 border-b border-gray-600 bg-[#1f1f1f]/80 backdrop-blur-sm",
                button {
                    class: "text-gray-300 hover:text-white text-lg px-2 py-1 rounded-lg hover:bg-white/5 transition-colors cursor-pointer",
                    onclick: move |_| {
                        navigator.push(Route::BakerLayout {});
                    },
                    "←"
                }
                h1 { class: "text-white text-lg font-bold", "{settings_title_label}" }
            }
            div { class: "flex-1 flex min-h-0",
                div { class: "w-64 shrink-0 border-r border-gray-700 bg-[#1f1f1f]/70 p-4",
                    div { class: "space-y-2",
                        button {
                            class: "w-full text-left px-3 py-2 rounded-lg text-sm transition-colors cursor-pointer {operators_tab_class}",
                            onclick: move |_| section.set(SettingsSection::Operators),
                            "{operators_tab_label}"
                        }
                        button {
                            class: "w-full text-left px-3 py-2 rounded-lg text-sm transition-colors cursor-pointer {background_tab_class}",
                            onclick: move |_| section.set(SettingsSection::Background),
                            "{background_tab_label}"
                        }
                        button {
                            class: "w-full text-left px-3 py-2 rounded-lg text-sm transition-colors cursor-pointer {language_tab_class}",
                            onclick: move |_| section.set(SettingsSection::Language),
                            "{language_tab_label}"
                        }
                        button {
                            class: "w-full text-left px-3 py-2 rounded-lg text-sm transition-colors cursor-pointer {about_tab_class}",
                            onclick: move |_| section.set(SettingsSection::About),
                            "{about_tab_label}"
                        }
                    }
                }
                div { class: "flex-1 min-h-0 overflow-y-auto p-8",
                    if matches!(section(), SettingsSection::Operators) {
                        div { class: "max-w-[820px] space-y-6",
                            div { class: "p-4 bg-[#2b2b2b] rounded-xl border border-gray-600",
                                h2 { class: "text-white text-base font-bold mb-3",
                                    "{add_operator_title_label}"
                                }
                                div { class: "space-y-3 mb-3",
                                    input {
                                        class: "w-full bg-[#222] border border-gray-600 rounded px-3 py-2 text-white text-sm focus:outline-none focus:border-blue-500",
                                        placeholder: "{operator_name_placeholder}",
                                        value: "{new_name}",
                                        oninput: move |e| new_name.set(e.value()),
                                    }
                                    input {
                                        class: "w-full bg-[#222] border border-gray-600 rounded px-3 py-2 text-white text-sm focus:outline-none focus:border-blue-500 cursor-pointer",
                                        r#type: "file",
                                        accept: "image/*",
                                        onchange: move |evt| {
                                            let files: Vec<FileData> = evt.files();
                                            if let Some(file) = files.first().cloned() {
                                                let file_name: String = file.name();
                                                let mime = file
                                                    .content_type()
                                                    .unwrap_or_else(|| mime_from_filename(&file_name).to_string());
                                                let mut preview = new_avatar_preview;
                                                spawn(async move {
                                                    if let Ok(bytes) = file.read_bytes().await {
                                                        let bytes_vec = bytes.to_vec();
                                                        let data_url = data_url_from_bytes(&mime, bytes_vec);
                                                        preview.set(data_url);
                                                    }
                                                });
                                            }
                                        },
                                    }
                                }
                                button {
                                    class: "w-full bg-blue-600 hover:bg-blue-500 text-white py-2 rounded text-sm font-medium transition-colors cursor-pointer",
                                    onclick: handle_add,
                                    "{add_operator_button_label}"
                                }
                            }

                            div { class: "space-y-2",
                                for op in ops_list {
                                    {
                                        let op_id = op.id.clone();
                                        let op_clone = op.clone();
                                        if editing_operator_id() == Some(op_id.clone()) {
                                            rsx! {
                                                div { class: "p-4 bg-[#2b2b2b] rounded border border-gray-700 space-y-3",
                                                    div { class: "flex items-center gap-3",
                                                        div { class: "w-12 h-12 rounded bg-gray-600 flex items-center justify-center overflow-hidden",
                                                            if !edit_avatar_preview().is_empty() {
                                                                img {
                                                                    src: "{edit_avatar_preview}",
                                                                    class: "w-full h-full object-cover",
                                                                }
                                                            } else {
                                                                span { class: "text-white font-bold", "{edit_name.read().chars().next().unwrap_or('?')}" }
                                                            }
                                                        }
                                                        input {
                                                            class: "flex-1 bg-[#222] border border-gray-600 rounded px-3 py-2 text-white text-sm focus:outline-none focus:border-blue-500",
                                                            value: "{edit_name}",
                                                            oninput: move |e| edit_name.set(e.value()),
                                                        }
                                                    }
                                                    input {
                                                        class: "w-full bg-[#222] border border-gray-600 rounded px-3 py-2 text-white text-sm focus:outline-none focus:border-blue-500 cursor-pointer",
                                                        r#type: "file",
                                                        accept: "image/*",
                                                        onchange: move |evt| {
                                                            let files: Vec<FileData> = evt.files();
                                                            if let Some(file) = files.first().cloned() {
                                                                let file_name: String = file.name();
                                                                let mime = file
                                                                    .content_type()
                                                                    .unwrap_or_else(|| mime_from_filename(&file_name).to_string());
                                                                let mut preview = edit_avatar_preview;
                                                                spawn(async move {
                                                                    if let Ok(bytes) = file.read_bytes().await {
                                                                        let bytes_vec = bytes.to_vec();
                                                                        let data_url = data_url_from_bytes(&mime, bytes_vec);
                                                                        preview.set(data_url);
                                                                    }
                                                                });
                                                            }
                                                        },
                                                    }
                                                    div { class: "flex justify-end gap-3",
                                                        button {
                                                            class: "px-3 py-1 text-gray-400 hover:text-white text-sm cursor-pointer",
                                                            onclick: move |_| handle_edit_cancel(()),
                                                            "{cancel_label}"
                                                        }
                                                        button {
                                                            class: "px-3 py-1 bg-blue-600 hover:bg-blue-500 text-white rounded text-sm font-medium cursor-pointer",
                                                            onclick: move |_| handle_edit_save(op_id.clone()),
                                                            "{save_label}"
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            rsx! {
                                                div { class: "flex items-center justify-between p-3 bg-[#2b2b2b] rounded border border-gray-700",
                                                    div { class: "flex items-center gap-3",
                                                        div { class: "w-10 h-10 rounded bg-gray-600 flex items-center justify-center overflow-hidden",
                                                            if !op.avatar_url.is_empty() {
                                                                img {
                                                                    src: "{op.avatar_url}",
                                                                    class: "w-full h-full object-cover",
                                                                }
                                                            } else {
                                                                span { class: "text-white font-bold", "{op.name.chars().next().unwrap_or('?')}" }
                                                            }
                                                        }
                                                        span { class: "text-white font-medium", "{op.name}" }
                                                    }
                                                    div { class: "flex items-center gap-3",
                                                        button {
                                                            class: "text-gray-300 hover:text-white text-sm px-2 py-1 cursor-pointer",
                                                            onclick: move |_| handle_edit_start(op_clone.clone()),
                                                            "{edit_label}"
                                                        }
                                                        button {
                                                            class: "text-red-400 hover:text-red-300 text-sm px-2 py-1 cursor-pointer",
                                                            onclick: move |_| handle_delete(op_id.clone()),
                                                            "{delete_label}"
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    } else if matches!(section(), SettingsSection::Background) {
                        div { class: "max-w-[820px] space-y-6",
                            h2 { class: "text-white text-base font-bold", "{background_title_label}" }
                            div { class: "space-y-3",
                                select {
                                    class: "w-full bg-[#222] border border-gray-600 rounded px-3 py-2 text-white text-sm focus:outline-none focus:border-blue-500",
                                    value: "{background_mode_value}",
                                    oninput: move |e| {
                                        let mut bg = background.write();
                                        bg.mode = match e.value().as_str() {
                                            "dot_light" => BackgroundMode::DotLight,
                                            "custom_color" => BackgroundMode::CustomColor,
                                            "custom_image" => BackgroundMode::CustomImage,
                                            _ => BackgroundMode::DotDark,
                                        };
                                    },
                                    option { value: "dot_dark", "{dot_dark_label}" }
                                    option { value: "dot_light", "{dot_light_label}" }
                                    option { value: "custom_color", "{custom_color_label}" }
                                    option { disabled: true, value: "custom_image",
                                        "{custom_image_todo_label}"
                                    }
                                }
                                if matches!(current_background.mode, BackgroundMode::CustomColor) {
                                    div { class: "flex items-center gap-3",
                                        input {
                                            class: "w-24 h-10 bg-transparent border border-gray-600 rounded",
                                            r#type: "color",
                                            value: "{current_background.custom_color}",
                                            oninput: move |e| {
                                                let mut bg = background.write();
                                                bg.custom_color = e.value();
                                                bg.mode = BackgroundMode::CustomColor;
                                            },
                                        }
                                    }
                                }
                                if matches!(current_background.mode, BackgroundMode::CustomImage) {
                                    div { class: "flex items-center gap-3",
                                        input {
                                            class: "flex-1 bg-[#222] border border-gray-600 rounded px-3 py-2 text-white text-sm focus:outline-none focus:border-blue-500 cursor-pointer",
                                            r#type: "file",
                                            accept: "image/*",
                                            onchange: move |evt| {
                                                let files: Vec<FileData> = evt.files();
                                                if let Some(file) = files.first().cloned() {
                                                    let file_name: String = file.name();
                                                    let mime = file
                                                        .content_type()
                                                        .unwrap_or_else(|| mime_from_filename(&file_name).to_string());
                                                    let mut bg = background;
                                                    spawn(async move {
                                                        if let Ok(bytes) = file.read_bytes().await {
                                                            let bytes_vec = bytes.to_vec();
                                                            let data_url = data_url_from_bytes(&mime, bytes_vec);
                                                            let mut settings = bg.write();
                                                            settings.custom_image = data_url;
                                                            settings.mode = BackgroundMode::CustomImage;
                                                        }
                                                    });
                                                }
                                            },
                                        }
                                    }
                                }
                            }
                        }
                    } else if matches!(section(), SettingsSection::Language) {
                        div { class: "max-w-[820px] space-y-6",
                            div { class: "space-y-2",
                                h2 { class: "text-white text-base font-bold", "{language_title_label}" }
                                p { class: "text-gray-400 text-sm", "{language_description_label}" }
                                p { class: "text-yellow-300 text-sm", "{language_accuracy_notice_label}" }
                            }
                            div { class: "max-w-sm space-y-3",
                                label { class: "block text-gray-300 text-sm", "{language_current_label}" }
                                select {
                                    class: "w-full bg-[#222] border border-gray-600 rounded px-3 py-2 text-white text-sm focus:outline-none focus:border-blue-500",
                                    value: "{current_locale}",
                                    oninput: move |e| {
                                        let selected_locale = apply_locale(&e.value()).to_string();
                                        locale_signal.set(selected_locale.clone());
                                        spawn(async move {
                                            if let Err(err) = save_locale_to_local_storage(&selected_locale).await {
                                                error!("failed to save locale: {}", err);
                                            }
                                        });
                                    },
                                    option { value: ZH_CN_LOCALE, selected: zh_cn_selected, "{zh_cn_label}" }
                                    option { value: EN_US_LOCALE, selected: en_us_selected, "{en_us_label}" }
                                }
                            }
                        }
                    } else if matches!(section(), SettingsSection::About) {
                        div {
                            h1 { class: "text-4xl font-bold", "Baker" }
                            h2 { class: "text-xl mt-2 mb-10",
                                "{about_description_label}"
                            }

                            p { "{about_author_label}" }
                            div {
                                "{about_open_source_at_label}"
                                a {
                                    class: "text-blue-500 hover:underline",
                                    href: repo_url,
                                    {repo_url.clone()}
                                }
                            }

                            h2 { class: "text-xl mt-10 mb-2", "{about_license_heading_label}" }
                            p { "{about_license_summary_label}" }
                            p { class: "font-mono", "MIT License" }
                            p { class: "font-mono", "Copyright (c) 2026 Wanye_7300" }
                            p { class: "font-mono",
                                "Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the \"Software\"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:"
                            }
                            p { class: "font-mono",
                                "The above copyright notice and this permission notice (including the next paragraph) shall be included in all copies or substantial portions of the Software."
                            }
                            p { class: "font-mono",
                                "THE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE."
                            }
                        }
                    }
                }
            }
        }
    }
}
