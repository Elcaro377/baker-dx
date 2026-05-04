use crate::components::assets::emojis::{EMOJI_KEYS, to_emoji};
use crate::components::baker::locale::use_locale_refresh;
use crate::components::baker::storage::v2::Operator;
use crate::components::baker::{data_url_from_bytes, mime_from_filename};
use crate::dioxus_elements::FileData;
use dioxus::prelude::*;
use rust_i18n::t;

///
/// 弹窗的模板。
///
/// # 参数
///
/// - title: 弹窗标题。
/// - content_confirmation_button: “确定”按钮的内容。典型例子就是“确定”。
/// - children: 弹窗内容。
/// - on_close: 处理关闭弹窗的事件。
/// - on_confirm: 处理按下“确定”按钮的事件。本组件在 call 这个事件的时候不会自动 call on_close 事件。
/// - max_width: （可选）弹窗中内容的最大宽度。
///
#[component]
pub(crate) fn Modal(
    title: String,
    content_confirmation_button: String,
    children: Element,
    on_close: EventHandler,
    on_confirm: EventHandler,
    max_width: Option<u32>,
) -> Element {
    let _current_locale = use_locale_refresh();
    let cancel_label = t!("common.cancel").to_string();
    let content_style = if let Some(mw) = max_width {
        format!("w-full max-w-[{}px] mx-auto", mw)
    } else {
        "w-full max-w-[340px] mx-auto".to_owned()
    };

    rsx! {
        div {
            class: "fixed inset-0 bg-black/50 flex items-center justify-center z-50 backdrop-blur-sm",
            onmousedown: move |_| on_close.call(()),

            div { class: "modal-mask w-screen",

                div { class: "modal-reveal",

                    div {
                        class: "modal-panel bg-[#f0f0f0] shadow-2xl overflow-hidden border border-gray-600",
                        style: "background-image: linear-gradient(rgba(0,0,0,0.06) 1px, transparent 1px), linear-gradient(90deg, rgba(0,0,0,0.06) 1px, transparent 1px); background-size: 6px 6px",
                        onclick: |e| e.stop_propagation(),
                        onmousedown: |e| e.stop_propagation(),

                        div { class: "px-5 py-3 flex justify-between items-center bg-[#fdfc00] border-b border-black/10",
                            h2 { class: "text-black text-xl font-semibold tracking-wide",
                                "{title}"
                            }
                            button {
                                class: "w-7 h-7 rounded flex items-center justify-center text-black hover:bg-black/10 transition-colors cursor-pointer",
                                onclick: move |_| on_close.call(()),
                                "✕"
                            }
                        }

                        div { class: content_style,
                            div { class: "p-4 space-y-4",

                                {children}

                                div { class: "flex justify-end gap-3",
                                    button {
                                        class: "px-4 py-2 text-black hover:text-gray-400 text-sm cursor-pointer",
                                        onclick: move |_| on_close.call(()),
                                        "{cancel_label}"
                                    }
                                    button {
                                        class: "px-4 py-2 bg-[#fdfc00] hover:bg-[#fdfc00]/60 text-black rounded text-sm font-medium cursor-pointer",
                                        onclick: move |_| {
                                            on_confirm.call(());
                                        },
                                        "{content_confirmation_button}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

///
/// 回放间隔模式。
///
#[derive(Clone, PartialEq)]
pub enum ReplayIntervalMode {
    /// 固定间隔
    Fixed,
    /// 按字数：当前消息字数 * 每个字的间隔。请注意，当消息为表情包和图片时仍按照固定间隔处理
    PerChar,
}

///
/// 回放设置。
///
#[derive(Clone, PartialEq)]
pub struct ReplaySettings {
    /// 回放间隔模式
    pub mode: ReplayIntervalMode,
    /// 当设为固定间隔时的间隔
    pub fixed_ms: u64,
    /// 当设为按字数时，每个字的间隔
    pub per_char_ms: u64,
    /// 发送后的间隔
    pub gap_ms: u64,
}

///
/// 回放设置的弹窗。
///
/// # 参数
///
/// - on_start: 处理开始回放的事件。
///
#[component]
pub fn ReplaySettingsModal(
    on_close: EventHandler<()>,
    on_start: EventHandler<ReplaySettings>,
) -> Element {
    let _current_locale = use_locale_refresh();
    let mut mode = use_signal(|| ReplayIntervalMode::Fixed);
    let mut fixed_ms = use_signal(|| "800".to_string());
    let mut per_char_ms = use_signal(|| "40".to_string());
    let mut gap_ms = use_signal(|| "200".to_string());

    let fixed_class = if matches!(mode(), ReplayIntervalMode::Fixed) {
        "bg-[#fdfc00] text-black"
    } else {
        "bg-[#fdfc00]/0 text-black"
    };
    let per_char_class = if matches!(mode(), ReplayIntervalMode::PerChar) {
        "bg-[#fdfc00] text-black"
    } else {
        "bg-[#fdfc00]/0 text-black"
    };
    let fixed_interval_label = t!("modals.replay.fixed_interval").to_string();
    let per_char_label = t!("modals.replay.per_char").to_string();
    let fixed_ms_label = t!("modals.replay.fixed_ms").to_string();
    let per_char_ms_label = t!("modals.replay.per_char_ms").to_string();
    let gap_ms_label = t!("modals.replay.gap_ms").to_string();

    rsx! {
        Modal {
            title: t!("modals.replay.title").to_string(),
            content_confirmation_button: t!("modals.replay.start").to_string(),
            on_confirm: move |_| {
                let fixed = fixed_ms().parse::<u64>().unwrap_or(800);
                let per_char = per_char_ms().parse::<u64>().unwrap_or(40);
                let gap = gap_ms().parse::<u64>().unwrap_or(200);
                on_start
                    .call(ReplaySettings {
                        mode: mode(),
                        fixed_ms: fixed,
                        per_char_ms: per_char,
                        gap_ms: gap,
                    });
                on_close.call(());
            },
            on_close,

            {
                rsx! {
                    div { class: "flex gap-2",
                        button {
                            class: "flex-1 px-3 py-2 rounded text-sm font-medium transition-colors {fixed_class}",
                            onclick: move |_| mode.set(ReplayIntervalMode::Fixed),
                            "{fixed_interval_label}"
                        }
                        button {
                            class: "flex-1 px-3 py-2 rounded text-sm font-medium transition-colors {per_char_class}",
                            onclick: move |_| mode.set(ReplayIntervalMode::PerChar),
                            "{per_char_label}"
                        }
                    }
                    div { class: "space-y-3",
                        div { class: "space-y-1",
                            label { class: "block text-black text-sm", "{fixed_ms_label}" }
                            input {
                                class: "w-full bg-[#e9e9e9] border border-black/10 rounded p-3 text-black text-sm focus:outline-none focus:border-black/30 resize-none",
                                r#type: "number",
                                min: "0",
                                value: "{fixed_ms}",
                                oninput: move |e| fixed_ms.set(e.value()),
                            }
                        }
                        div { class: "space-y-1",
                            label { class: "block text-black text-sm", "{per_char_ms_label}" }
                            input {
                                class: "w-full bg-[#e9e9e9] border border-black/10 rounded p-3 text-black text-sm focus:outline-none focus:border-black/30 resize-none",
                                r#type: "number",
                                min: "0",
                                value: "{per_char_ms}",
                                oninput: move |e| per_char_ms.set(e.value()),
                            }
                        }
                        div { class: "space-y-1",
                            label { class: "block text-black text-sm", "{gap_ms_label}" }
                            input {
                                class: "w-full bg-[#e9e9e9] border border-black/10 rounded p-3 text-black text-sm focus:outline-none focus:border-black/30 resize-none",
                                r#type: "number",
                                min: "0",
                                value: "{gap_ms}",
                                oninput: move |e| gap_ms.set(e.value()),
                            }
                        }
                    }
                }
            }
        }
    }
}

///
/// 个人资料设置的弹窗
///
/// TODO: 将其移动进设置页面
///
#[component]
pub fn ProfileModal(
    current_name: String,
    current_avatar: String,
    on_close: EventHandler<()>,
    on_save: EventHandler<(String, String)>,
) -> Element {
    let _current_locale = use_locale_refresh();
    let mut name = use_signal(|| current_name);
    let avatar_preview = use_signal(|| current_avatar);
    let title_label = t!("modals.profile.title").to_string();
    let name_label = t!("modals.profile.name").to_string();
    let avatar_file_label = t!("modals.profile.avatar_file").to_string();
    let cancel_label = t!("common.cancel").to_string();
    let save_label = t!("common.save").to_string();

    rsx! {
        div {
            class: "fixed inset-0 bg-black/50 flex items-center justify-center z-50 backdrop-blur-sm",
            onclick: move |_| on_close.call(()),
            div {
                class: "bg-[#2b2b2b] w-[400px] rounded-xl shadow-2xl overflow-hidden border border-gray-600",
                onclick: |e| e.stop_propagation(),

                div { class: "px-6 py-4 border-b border-gray-600 flex justify-between items-center bg-[#333]",
                    h2 { class: "text-white text-lg font-bold", "{title_label}" }
                    button {
                        class: "text-gray-400 hover:text-white transition-colors",
                        onclick: move |_| on_close.call(()),
                        "✕"
                    }
                }

                div { class: "p-6",
                    div { class: "space-y-4",
                        div {
                            label { class: "block text-gray-400 text-sm mb-1", "{name_label}" }
                            input {
                                class: "w-full bg-[#222] border border-gray-600 rounded px-3 py-2 text-white text-sm focus:outline-none focus:border-blue-500",
                                value: "{name}",
                                oninput: move |e| name.set(e.value()),
                            }
                        }
                        div {
                            label { class: "block text-gray-400 text-sm mb-1", "{avatar_file_label}" }
                            input {
                                class: "w-full bg-[#222] border border-gray-600 rounded px-3 py-2 text-white text-sm focus:outline-none focus:border-blue-500",
                                r#type: "file",
                                accept: "image/*",
                                onchange: move |evt| {
                                    let files: Vec<FileData> = evt.files();
                                    if let Some(file) = files.first().cloned() {
                                        let file_name: String = file.name();
                                        let mime = file
                                            .content_type()
                                            .unwrap_or_else(|| mime_from_filename(&file_name).to_string());
                                        let mut preview = avatar_preview;
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

                        // Preview
                        div { class: "flex justify-center mt-4",
                            div { class: "w-20 h-20 rounded bg-gray-600 flex items-center justify-center overflow-hidden border border-gray-500",
                                if !avatar_preview().is_empty() {
                                    img {
                                        src: "{avatar_preview}",
                                        class: "w-full h-full object-cover",
                                    }
                                } else {
                                    span { class: "text-white font-bold text-xl",
                                        "{name.read().chars().next().unwrap_or('?')}"
                                    }
                                }
                            }
                        }
                    }

                    div { class: "flex justify-end gap-3 mt-6",
                        button {
                            class: "px-4 py-2 text-gray-400 hover:text-white text-sm",
                            onclick: move |_| on_close.call(()),
                            "{cancel_label}"
                        }
                        button {
                            class: "px-4 py-2 bg-blue-600 hover:bg-blue-500 text-white rounded text-sm font-medium",
                            onclick: move |_| { on_save.call((name(), avatar_preview())) },
                            "{save_label}"
                        }
                    }
                }
            }
        }
    }
}

///
/// 编辑消息的弹窗。
///
/// # 参数
///
/// - initial_content: 初始内容
///
#[component]
pub fn EditMessageModal(
    initial_content: String,
    on_close: EventHandler<()>,
    on_save: EventHandler<String>,
) -> Element {
    let _current_locale = use_locale_refresh();
    let mut content = use_signal(|| initial_content);

    rsx! {
        Modal {
            title: t!("modals.edit_message.title").to_string(),
            content_confirmation_button: t!("common.save").to_string(),
            on_close,
            on_confirm: move |_| {
                on_save.call(content());
            },

            {
                rsx! {
                    textarea {
                        class: "w-full h-32 bg-[#e9e9e9] border border-black/10 rounded p-3 text-black text-sm focus:outline-none focus:border-black/30 resize-none",
                        value: "{content}",
                        oninput: move |e| content.set(e.value()),
                    }
                }
            }
        }
    }
}

///
/// 添加反应的弹窗。
///
#[component]
pub fn ReactionModal(on_close: EventHandler<()>, on_save: EventHandler<String>) -> Element {
    let _current_locale = use_locale_refresh();
    let mut reaction = use_signal(|| "".to_string());
    let placeholder_label = t!("modals.reaction.placeholder").to_string();

    rsx! {
        Modal {
            title: t!("modals.reaction.title").to_string(),
            content_confirmation_button: t!("modals.reaction.add").to_string(),
            on_close,
            on_confirm: move |_| {
                let val = reaction();
                if !val.trim().is_empty() {
                    on_save.call(val);
                }
            },

            {
                rsx! {
                    input {
                        class: "w-full bg-[#e9e9e9] border border-black/10 rounded p-3 text-black text-sm focus:outline-none focus:border-black/30 resize-none",
                        placeholder: "{placeholder_label}",
                        value: "{reaction}",
                        oninput: move |e| reaction.set(e.value()),
                        onkeydown: move |e| {
                            if e.key() == Key::Enter {
                                let val = reaction();
                                if !val.trim().is_empty() {
                                    on_save.call(val);
                                }
                            }
                        },
                    }
                    // 常用表情快捷按钮
                    div { class: "flex flex-wrap gap-2",
                        for emoji in ["😀", "😂", "😭", "👍", "❤️", "❗", "❓"] {
                            {
                                let emoji_str = emoji.to_string();
                                rsx! {
                                    button {
                                        class: "px-2 py-1 rounded bg-black/10 hover:bg-black/20 text-lg text-black",
                                        onclick: {
                                            let emoji_val = emoji_str.clone();
                                            move |_| {
                                                on_save.call(emoji_val.clone());
                                            }
                                        },
                                        "{emoji}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

///
/// 告知用户有可用更新的弹窗。
///
/// # 参数
///
/// - latest_version: 最新的版本
/// - release_url: 正式版的 url
///
#[component]
pub fn UpdateAvailableModal(
    latest_version: String,
    release_url: String,
    on_update_now: EventHandler<String>,
    on_close: EventHandler<()>,
    on_skip_today: EventHandler<()>,
) -> Element {
    let _current_locale = use_locale_refresh();
    let release_url = use_hook(|| release_url);
    let latest_version_label = t!("modals.update.latest_version").to_string();
    let skip_today_label = t!("modals.update.skip_today").to_string();
    rsx! {
        Modal {
            title: t!("modals.update.title").to_string(),
            content_confirmation_button: t!("modals.update.now").to_string(),
            on_close,
            on_confirm: move |_| {
                on_update_now.call(release_url.clone());
                on_close.call(());
            },

            {
                rsx! {
                    div { class: "mb-4 text-black",
                        "{latest_version_label}"
                        span { class: "font-semibold", "{latest_version}" }
                    }
                    a {
                        class: "text-blue-400 hover:underline hover:cursor-pointer",
                        onclick: move |_| {
                            on_skip_today.call(());
                            on_close.call(());
                        },
                        "{skip_today_label}"
                    }
                }
            }
        }
    }
}

///
/// 选择发送者的弹窗。
///
/// # 参数
///
/// - members: 干员列表
///
#[component]
pub fn PickSenderModal(
    members: Vec<Operator>,
    on_close: EventHandler<()>,
    on_send: EventHandler<String>,
) -> Element {
    let _current_locale = use_locale_refresh();
    let mut selected_id = use_signal(|| Option::<String>::None);
    let no_members_label = t!("modals.pick_sender.no_members").to_string();
    rsx! {
        Modal {
            title: t!("modals.pick_sender.title").to_string(),
            content_confirmation_button: t!("common.confirm").to_string(),
            on_close,
            on_confirm: move |_| {
                if let Some(id) = selected_id() {
                    on_send.call(id);
                }
                on_close.call(());
            },
            div { class: "max-h-[50vh] overflow-y-auto custom-scrollbar",
                if members.is_empty() {
                    div { class: "text-center text-black/60 py-6", "{no_members_label}" }
                } else {
                    div { class: "grid grid-cols-1 gap-2",
                        for member in members {
                            {
                                let member_id = member.id.clone();
                                let member_name = member.name.clone();
                                let member_avatar = member.avatar_url.clone();
                                let is_selected = selected_id() == Some(member_id.clone());
                                rsx! {
                                    button {
                                        class: if is_selected { "flex items-center gap-3 p-3 rounded bg-black/10 transition-colors text-left group cursor-pointer" } else { "flex items-center gap-3 p-3 rounded hover:bg-black/5 transition-colors text-left group cursor-pointer" },
                                        onclick: move |_| selected_id.set(Some(member_id.clone())),
                                        div { class: if is_selected { "w-10 h-10 rounded bg-gray-300 flex items-center justify-center overflow-hidden border border-black/40" } else { "w-10 h-10 rounded bg-gray-300 flex items-center justify-center overflow-hidden border border-black/10 group-hover:border-black/30" },
                                            if !member_avatar.is_empty() {
                                                img { src: "{member_avatar}", class: "w-full h-full object-cover" }
                                            } else {
                                                span { class: "text-black font-semibold", "{member_name.chars().next().unwrap_or('?')}" }
                                            }
                                        }
                                        span { class: if is_selected { "text-black font-semibold" } else { "text-black font-medium group-hover:text-black/70" },
                                            "{member_name}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

///
/// 在……前插入消息的弹窗。
///
/// # 参数
///
/// - members: 干员列表
///
#[component]
pub fn InsertMessageModal(
    members: Vec<Operator>,
    on_close: EventHandler<()>,
    on_save: EventHandler<(String, Option<String>)>,
) -> Element {
    let _current_locale = use_locale_refresh();
    let mut content = use_signal(String::new);
    let mut is_self = use_signal(|| true);
    // 群组模式下，选"对方"后弹出成员选择
    let mut pick_sender = use_signal(|| false);

    let is_group = members.len() > 1;

    let self_class = if is_self() {
        "bg-[#fdfc00] text-black"
    } else {
        "bg-[#fdfc00]/0 text-black"
    };
    let other_class = if !is_self() {
        "bg-[#fdfc00] text-black"
    } else {
        "bg-[#fdfc00]/0 text-black"
    };
    let self_label = t!("modals.insert.self").to_string();
    let other_label = t!("modals.insert.other").to_string();
    let placeholder_label = t!("modals.insert.placeholder").to_string();
    let on_close_safe = {
        let on_close = on_close;
        let pick_sender = pick_sender;
        move |_| {
            if !pick_sender() {
                on_close.call(());
            }
        }
    };

    if pick_sender() {
        return rsx! {
            PickSenderModal {
                members,
                on_close: move |_| pick_sender.set(false),
                on_send: move |sender_id: String| {
                    let val = content();
                    if !val.trim().is_empty() {
                        on_save.call((val, Some(sender_id)));
                    }
                    pick_sender.set(false);
                },
            }
        };
    }

    rsx! {
        Modal {
            title: t!("modals.insert.title").to_string(),
            content_confirmation_button: t!("modals.insert.confirm").to_string(),
            on_close: on_close_safe,
            on_confirm: move |_| {
                let val = content();
                if val.trim().is_empty() {
                    return;
                }
                if is_self() {
                    on_save.call((val, None));
                } else if is_group {
                    pick_sender.set(true);
                } else {
                    on_save.call((val, members.first().map(|op| op.id.clone())));
                }
            },
            div { class: "space-y-4",
                div { class: "flex gap-2",
                    button {
                        class: "flex-1 px-3 py-2 rounded text-sm font-medium transition-colors cursor-pointer {self_class}",
                        onclick: move |_| is_self.set(true),
                        "{self_label}"
                    }
                    button {
                        class: "flex-1 px-3 py-2 rounded text-sm font-medium transition-colors cursor-pointer {other_class}",
                        onclick: move |_| is_self.set(false),
                        "{other_label}"
                    }
                }
                textarea {
                    class: "w-full h-32 bg-[#e9e9e9] border border-black/10 rounded p-3 text-black text-sm focus:outline-none focus:border-black/30 resize-none",
                    placeholder: "{placeholder_label}",
                    value: "{content}",
                    oninput: move |e| content.set(e.value()),
                }
            }
        }
    }
}

///
/// 新会话的类别
///
#[derive(Clone, PartialEq)]
pub enum NewChatSelection {
    /// 单人
    Single(Operator),
    /// 群组
    Group {
        /// 群组名
        name: String,
        /// 群组头像 url
        avatar_url: String,
        /// 群员。不包括自己。
        member_ids: Vec<String>,
    },
}

///
/// 发起新会话的弹窗。
///
/// # 参数
///
/// operators: 干员列表
///
#[component]
pub fn NewChatModal(
    operators: Signal<Vec<Operator>>,
    on_close: EventHandler<()>,
    on_select: EventHandler<NewChatSelection>,
) -> Element {
    let _current_locale = use_locale_refresh();
    let mut selected_ids = use_signal(Vec::<String>::new);
    let mut group_name = use_signal(|| "".to_string());
    let group_avatar = use_signal(|| "".to_string());
    let mut error_text = use_signal(|| "".to_string());
    let selected_count = selected_ids().len();
    let no_operators_label = t!("modals.new_chat.no_operators").to_string();
    let group_name_placeholder = t!("modals.new_chat.group_name_placeholder").to_string();

    rsx! {
        Modal {
            title: t!("modals.new_chat.title").to_string(),
            content_confirmation_button: t!("modals.new_chat.start").to_string(),
            on_close,
            on_confirm: move |_| {
                if selected_count == 1 {
                    if let Some(op_id) = selected_ids().first().cloned()
                        && let Some(op) = operators
                            .read()
                            .iter()
                            .find(|op| op.id == op_id)
                            .cloned()
                    {
                        on_select.call(NewChatSelection::Single(op));
                        on_close.call(());
                    }
                } else if selected_count > 1 {
                    let name = group_name().trim().to_string();
                    if name.is_empty() {
                        error_text.set(t!("modals.new_chat.group_name_required").to_string());
                        return;
                    }
                    on_select
                        .call(NewChatSelection::Group {
                            name,
                            avatar_url: group_avatar(),
                            member_ids: selected_ids(),
                        });
                    on_close.call(());
                }
            },

            {
                rsx! {
                    div { class: "p-4 max-h-[60vh] overflow-y-auto custom-scrollbar",
                        if operators.read().is_empty() {
                            div { class: "text-center text-gray-500 py-8",
                                "{no_operators_label}"
                            }
                        } else {
                            div { class: "grid grid-cols-1 gap-2",
                                for op in operators.read().iter().cloned() {
                                    {
                                        let op_id = op.id.clone();
                                        let op_name = op.name.clone();
                                        let op_avatar = op.avatar_url.clone();
                                        let op_id_for_click = op_id.clone();
                                        rsx! {
                                            div {
                                                class: "flex items-center gap-3 p-3 rounded hover:bg-black/20 transition-colors text-left group cursor-pointer",
                                                onclick: move |_| {
                                                    error_text.set("".to_string());
                                                    selected_ids
                                                        .with_mut(|list| {
                                                            if let Some(pos) = list.iter().position(|id| id == &op_id_for_click) {
                                                                list.remove(pos);
                                                            } else {
                                                                list.push(op_id_for_click.clone());
                                                            }
                                                        });
                                                },
                                                input {
                                                    r#type: "checkbox",
                                                    class: "w-4 h-4 accent-blue-600 cursor-pointer",
                                                    checked: selected_ids().contains(&op_id),
                                                }
                                                div { class: "w-10 h-10 rounded bg-gray-600 flex items-center justify-center overflow-hidden border border-gray-500 group-hover:border-blue-500",
                                                    if !op_avatar.is_empty() {
                                                        img { src: "{op_avatar}", class: "w-full h-full object-cover" }
                                                    } else {
                                                        span { class: "text-white font-bold", "{op_name.chars().next().unwrap_or('?')}" }
                                                    }
                                                }
                                                span { class: "text-black font-medium", "{op_name}" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    if !operators.read().is_empty() {
                        div { class: "px-4 pb-4 space-y-3",
                            if selected_count > 1 {
                                div { class: "space-y-3",
                                    input {
                                        class: "w-full bg-[#e9e9e9] border border-black/10 rounded p-3 text-black text-sm focus:outline-none focus:border-black/30 resize-none",
                                        placeholder: "{group_name_placeholder}",
                                        value: "{group_name}",
                                        oninput: move |e| {
                                            group_name.set(e.value());
                                            error_text.set("".to_string());
                                        },
                                    }
                                    input {
                                        class: "w-full bg-[#e9e9e9] border border-black/10 rounded p-3 text-black text-sm focus:outline-none focus:border-black/30 resize-none cursor-pointer",
                                        r#type: "file",
                                        accept: "image/*",
                                        onchange: move |evt| {
                                            let files: Vec<FileData> = evt.files();
                                            if let Some(file) = files.first().cloned() {
                                                let file_name: String = file.name();
                                                let mime = file
                                                    .content_type()
                                                    .unwrap_or_else(|| mime_from_filename(&file_name).to_string());
                                                let mut preview = group_avatar;
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
                                    if !group_avatar().is_empty() {
                                        div { class: "flex justify-center",
                                            div { class: "w-14 h-14 rounded bg-gray-600 flex items-center justify-center overflow-hidden border border-gray-500",
                                                img {
                                                    src: "{group_avatar}",
                                                    class: "w-full h-full object-cover",
                                                }
                                            }
                                        }
                                    }
                                    if !error_text().is_empty() {
                                        div { class: "text-red-400 text-sm", "{error_text}" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

///
/// 用于 SetGroupOpsListModal 设置的干员列表。
///
#[derive(PartialEq, Clone)]
pub(crate) struct OpsSelection {
    pub ops: Vec<String>,
    pub name: String,
    pub avatar_url: String,
}

///
/// 用于设置特定群聊中各项信息的弹窗。
///
#[component]
pub fn EditGroupChatProps(
    on_close: EventHandler,
    on_select: EventHandler<OpsSelection>,
    selected_contact_id: String,
) -> Element {
    let _current_locale = use_locale_refresh();
    let app_state = use_context::<Signal<crate::components::baker::storage::v2::AppState>>();

    let app_state_read = app_state.read();
    let operators = app_state_read.operators.clone();
    let contact = app_state_read
        .contacts
        .iter()
        .find(|x| x.id == selected_contact_id)
        .cloned();
    let missing_roster_label = t!("modals.group.missing_roster").to_string();

    if contact.is_none() {
        return rsx! {
            Modal {
                title: t!("common.error").to_string(),
                content_confirmation_button: t!("common.confirm").to_string(),
                on_close,
                on_confirm: move |_| on_close.call(()),

                {
                    rsx! { "{missing_roster_label}" }
                }
            }
        };
    }

    let contact = contact.unwrap();
    let mut group_ops_list = use_signal(|| contact.participant_ids.clone());
    let mut group_name = use_signal(|| contact.name.clone());
    let mut group_avatar = use_signal(|| contact.avatar_url.clone());
    let mut avatar_file_input_key = use_signal(|| 0usize);
    let mut error_text = use_signal(|| "".to_string());
    let group_name_label = t!("modals.group.name").to_string();
    let group_name_placeholder = t!("modals.group.name_placeholder").to_string();
    let group_avatar_label = t!("modals.group.avatar").to_string();
    let clear_avatar_label = t!("modals.group.clear_avatar").to_string();
    let members_title_label = t!("modals.group.members_title").to_string();

    rsx! {
        Modal {
            title: t!("modals.group.settings_title").to_string(),
            content_confirmation_button: t!("common.ok").to_string(),
            on_close,
            on_confirm: move |_| {
                let name = group_name().trim().to_string();
                if name.is_empty() {
                    error_text.set(t!("modals.group.name_placeholder").to_string());
                    return;
                }
                on_select
                    .call(OpsSelection {
                        ops: group_ops_list(),
                        name,
                        avatar_url: group_avatar(),
                    })
            },

            {
                rsx! {
                    div { class: "space-y-4",
                        div {
                            label { class: "block text-black text-sm mb-1", "{group_name_label}" }
                            input {
                                class: "w-full bg-[#e9e9e9] border border-black/10 rounded p-3 text-black text-sm focus:outline-none focus:border-black/30 resize-none",
                                placeholder: "{group_name_placeholder}",
                                value: "{group_name}",
                                oninput: move |e| {
                                    group_name.set(e.value());
                                    error_text.set("".to_string());
                                },
                            }
                        }
                        div {
                            label { class: "block text-black text-sm mb-1", "{group_avatar_label}" }
                            div { class: "flex items-center gap-3",
                                div { class: "w-14 h-14 rounded bg-gray-600 flex items-center justify-center overflow-hidden border border-gray-500 shrink-0",
                                    if !group_avatar().is_empty() {
                                        img {
                                            src: "{group_avatar}",
                                            class: "w-full h-full object-cover",
                                        }
                                    } else {
                                        span { class: "text-white font-bold text-lg",
                                            "{group_name.read().chars().next().unwrap_or('?')}"
                                        }
                                    }
                                }
                                div { class: "flex-1 space-y-1",
                                    input {
                                        key: "{avatar_file_input_key}",
                                        class: "w-full bg-[#e9e9e9] border border-black/10 rounded p-3 text-black text-sm focus:outline-none focus:border-black/30 resize-none cursor-pointer",
                                        r#type: "file",
                                        accept: "image/*",
                                        onchange: move |evt| {
                                            let files: Vec<FileData> = evt.files();
                                            if let Some(file) = files.first().cloned() {
                                                let file_name: String = file.name();
                                                let mime = file
                                                    .content_type()
                                                    .unwrap_or_else(|| mime_from_filename(&file_name).to_string());
                                                let mut preview = group_avatar;
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
                                    button {
                                        class: "text-sm text-blue-600 hover:text-blue-700 underline cursor-pointer",
                                        onclick: move |_| {
                                            group_avatar.set("".to_string());
                                            avatar_file_input_key.set(avatar_file_input_key() + 1);
                                        },
                                        "{clear_avatar_label}"
                                    }
                                }
                            }
                        }
                        if !error_text().is_empty() {
                            div { class: "text-red-400 text-sm", "{error_text}" }
                        }
                    }

                    h2 { class: "text-2xl font-bold text-black", "{members_title_label}" }

                    div { class: "p-4 max-h-[60vh] overflow-y-auto custom-scrollbar",
                        div { class: "grid grid-cols-1 gap-2",
                            for op in operators.iter() {
                                {
                                    let op_id = op.id.clone();
                                    let op_name = op.name.clone();
                                    let op_avatar = op.avatar_url.clone();
                                    let op_id_for_click = op_id.clone();
                                    rsx! {
                                        div {
                                            class: "flex items-center gap-3 p-3 rounded hover:bg-black/20 transition-colors text-left group cursor-pointer",
                                            onclick: move |_| {
                                                group_ops_list
                                                    .with_mut(|list| {
                                                        if let Some(pos) = list.iter().position(|id| id == &op_id_for_click) {
                                                            list.remove(pos);
                                                        } else {
                                                            list.push(op_id_for_click.clone());
                                                        }
                                                    });
                                            },
                                            input {
                                                r#type: "checkbox",
                                                class: "w-4 h-4 accent-blue-600 cursor-pointer",
                                                checked: group_ops_list().contains(&op_id),
                                            }
                                            div { class: "w-10 h-10 rounded bg-gray-600 flex items-center justify-center overflow-hidden border border-gray-500 group-hover:border-blue-500",
                                                if !op_avatar.is_empty() {
                                                    img { src: "{op_avatar}", class: "w-full h-full object-cover" }
                                                } else {
                                                    span { class: "text-white font-bold", "{op_name.chars().next().unwrap_or('?')}" }
                                                }
                                            }
                                            span { class: "text-black font-medium", "{op_name}" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn EditParticipantsSelvesIds(on_close: EventHandler, selected_contact_id: String) -> Element {
    let _current_locale = use_locale_refresh();
    let app_state = use_context::<Signal<crate::components::baker::storage::v2::AppState>>();

    let app_state_read = app_state.read();
    let operators = app_state_read.operators.clone();
    let contact = app_state_read
        .contacts
        .iter()
        .find(|x| x.id == selected_contact_id)
        .cloned();
    let missing_roster_label = t!("modals.group.missing_roster").to_string();

    if contact.is_none() {
        return rsx! {
            Modal {
                title: t!("common.error").to_string(),
                content_confirmation_button: t!("common.confirm").to_string(),
                on_close,
                on_confirm: move |_| on_close.call(()),

                {
                    rsx! { "{missing_roster_label}" }
                }
            }
        };
    }

    let contact = contact.unwrap();
    let participants = contact.participant_ids;
    let mut participants_selves_ids = use_signal(|| contact.participants_selves_ids.clone());

    rsx! {
        Modal {
            title: t!("modals.participants.title").to_string(),
            content_confirmation_button: t!("common.confirm").to_string(),
            on_close: move |_| on_close.call(()),
            on_confirm: move |_| {
                let mut app_state = use_context::<
                    Signal<crate::components::baker::storage::v2::AppState>,
                >();
                let mut app_state_write = app_state.write();
                app_state_write
                    .contacts
                    .iter_mut()
                    .find(|x| x.id == selected_contact_id)
                    .unwrap()
                    .participants_selves_ids = participants_selves_ids();
                on_close.call(());
            },

            {
                rsx! {
                    div { class: "p-4 max-h-[60vh] overflow-y-auto custom-scrollbar",
                        div { class: "grid grid-cols-1 gap-2",
                            for op in operators.iter().filter(|x| participants.contains(&x.id)) {
                                {
                                    let op_id = op.id.clone();
                                    let op_name = op.name.clone();
                                    let op_avatar = op.avatar_url.clone();
                                    let op_id_for_click = op_id.clone();
                                    rsx! {
                                        div {
                                            class: "flex items-center gap-3 p-3 rounded hover:bg-black/20 transition-colors text-left group cursor-pointer",
                                            onclick: move |_| {
                                                participants_selves_ids
                                                    .with_mut(|list| {
                                                        if let Some(pos) = list.iter().position(|id| id == &op_id_for_click) {
                                                            list.remove(pos);
                                                        } else {
                                                            list.push(op_id_for_click.clone());
                                                        }
                                                    });
                                            },
                                            input {
                                                r#type: "checkbox",
                                                class: "w-4 h-4 accent-blue-600 cursor-pointer",
                                                checked: participants_selves_ids().contains(&op_id),
                                            }
                                            div { class: "w-10 h-10 rounded bg-gray-600 flex items-center justify-center overflow-hidden border border-gray-500 group-hover:border-blue-500",
                                                if !op_avatar.is_empty() {
                                                    img { src: "{op_avatar}", class: "w-full h-full object-cover" }
                                                } else {
                                                    span { class: "text-white font-bold", "{op_name.chars().next().unwrap_or('?')}" }
                                                }
                                            }
                                            span { class: "text-black font-medium", "{op_name}" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Notice(on_close: EventHandler) -> Element {
    let _current_locale = use_locale_refresh();
    let notice_p1 = t!("modals.notice.p1").to_string();
    let notice_p2 = t!("modals.notice.p2").to_string();
    let notice_p3 = t!("modals.notice.p3").to_string();
    let notice_thanks = t!("modals.notice.thanks").to_string();

    rsx! {
        Modal {
            title: t!("modals.notice.title").to_string(),
            content_confirmation_button: t!("modals.notice.confirm").to_string(),
            on_close: move |_| on_close.call(()),
            on_confirm: move |_| on_close.call(()),

            {
                rsx! {
                    p { class: "m-b-[5px] text-black", "{notice_p1}" }

                    p { class: "m-b-[5px] text-black", "{notice_p2}" }

                    p { class: "m-b-[5px] text-black", "{notice_p3}" }

                    p { class: "m-b-[5px] text-black", "{notice_thanks}" }
                }
            }
        }
    }
}

#[component]
pub fn EmojiSupportModal(on_close: EventHandler<()>) -> Element {
    let _current_locale = use_locale_refresh();
    let heading_label = t!("modals.emoji.heading").to_string();
    let syntax_prefix_label = t!("modals.emoji.syntax_prefix").to_string();
    let syntax_separator_label = t!("modals.emoji.syntax_separator").to_string();
    let syntax_suffix_label = t!("modals.emoji.syntax_suffix").to_string();
    let display_label = t!("modals.emoji.display").to_string();
    let example_label = t!("modals.emoji.example_label").to_string();
    let example_text = t!("modals.emoji.example_text").to_string();

    rsx! {
        Modal {
            title: t!("modals.emoji.title").to_string(),
            content_confirmation_button: t!("modals.emoji.confirm").to_string(),
            on_close: move |_| on_close.call(()),
            on_confirm: move |_| on_close.call(()),
            max_width: 960,

            {
                rsx! {
                    div { class: "p-4 max-h-[60vh] overflow-y-auto custom-scrollbar text-black text-base leading-relaxed space-y-4",
                        h1 { class: "text-2xl font-bold", "{heading_label}" }
                        p {
                            "{syntax_prefix_label}"
                            span { class: "px-1 py-0.5 rounded bg-black/10 font-mono text-sm", ":happy:" }
                            "{syntax_separator_label}"
                            span { class: "px-1 py-0.5 rounded bg-black/10 font-mono text-sm", ":thumb:" }
                            "{syntax_suffix_label}"
                        }
                        p { "{display_label}" }
                        div { class: "rounded border border-black/10 bg-black/5 px-3 py-2 text-sm",
                            span { class: "font-semibold", "{example_label}" }
                            span { class: "font-mono", "{example_text}" }
                        }
                        div { class: "grid grid-cols-2 md:grid-cols-3 gap-3",
                            for key in EMOJI_KEYS {
                                if let Some(asset) = to_emoji(key) {
                                    div {
                                        key: "{key}",
                                        class: "flex items-center gap-3 rounded border border-black/10 bg-white/50 px-3 py-2",
                                        img {
                                            src: asset,
                                            alt: ":{key}:",
                                            class: "w-6 h-6 shrink-0 object-contain",
                                        }
                                        div { class: "min-w-0",
                                            div { class: "font-mono text-sm break-all", ":{key}:" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

const IMAGE_TUTORIAL_1: Asset = asset!("/tutorial/1.png");
const IMAGE_TUTORIAL_2: Asset = asset!("/tutorial/2.png");
const IMAGE_TUTORIAL_3: Asset = asset!("/tutorial/3.png");
const IMAGE_TUTORIAL_4: Asset = asset!("/tutorial/4.png");
const IMAGE_TUTORIAL_5: Asset = asset!("/tutorial/5.png");

///
/// 教程弹窗。
///
#[component]
pub fn TutorialModal(on_close: EventHandler<()>, on_confirm: EventHandler<bool>) -> Element {
    let _current_locale = use_locale_refresh();
    let mut dont_show_again = use_signal(|| false);
    let close_label = t!("common.close").to_string();
    let heading_label = t!("modals.tutorial.heading").to_string();
    let add_operator_label = t!("modals.tutorial.add_operator").to_string();
    let alt_add_operator_label = t!("modals.tutorial.alt_add_operator").to_string();
    let open_settings_label = t!("modals.tutorial.open_settings").to_string();
    let operator_fields_label = t!("modals.tutorial.operator_fields").to_string();
    let finish_operator_label = t!("modals.tutorial.finish_operator").to_string();
    let session_heading_label = t!("modals.tutorial.session_heading").to_string();
    let alt_session_label = t!("modals.tutorial.alt_session").to_string();
    let default_session_label = t!("modals.tutorial.default_session").to_string();
    let switch_session_label = t!("modals.tutorial.switch_session").to_string();
    let chat_head_style_label = t!("modals.tutorial.chat_head_style").to_string();
    let menu_intro_label = t!("modals.tutorial.menu_intro").to_string();
    let send_other_label = t!("modals.tutorial.send_other").to_string();
    let send_other_shortcut_label = t!("modals.tutorial.send_other_shortcut").to_string();
    let send_status_label = t!("modals.tutorial.send_status").to_string();
    let status_line_label = t!("modals.tutorial.status_line").to_string();
    let replay_heading_label = t!("modals.tutorial.replay_heading").to_string();
    let alt_full_chat_label = t!("modals.tutorial.alt_full_chat").to_string();
    let alt_replay_label = t!("modals.tutorial.alt_replay").to_string();
    let dialogue_ready_label = t!("modals.tutorial.dialogue_ready").to_string();
    let start_replay_label = t!("modals.tutorial.start_replay").to_string();
    let replay_modes_label = t!("modals.tutorial.replay_modes").to_string();
    let fixed_interval_label = t!("modals.replay.fixed_interval").to_string();
    let per_char_mode_label = t!("modals.tutorial.per_char_mode").to_string();
    let interval_formula_label = t!("modals.tutorial.interval_formula").to_string();
    let recommended_settings_label = t!("modals.tutorial.recommended_settings").to_string();
    let after_replay_missing_label = t!("modals.tutorial.after_replay_missing").to_string();
    let share_label = t!("modals.tutorial.share").to_string();
    let dont_show_again_label = t!("modals.tutorial.dont_show_again").to_string();

    rsx! {
        Modal {
            title: t!("modals.tutorial.title").to_string(),
            content_confirmation_button: close_label,
            on_close,
            on_confirm: move |_| { on_confirm.call(dont_show_again()) },
            max_width: 1280,

            {
                rsx! {
                    div { class: "p-6 max-h-[60vh] overflow-y-auto custom-scrollbar text-black text-base leading-relaxed space-y-3",
                        h1 { class: "text-3xl font-bold", "{heading_label}" }
                        h2 { class: "text-2xl font-bold", "{add_operator_label}" }
                        p {
                            img {
                                class: "max-w-[600px]",
                                alt: "{alt_add_operator_label}",
                                src: IMAGE_TUTORIAL_1,
                            }
                        }
                        p {
                            img {
                                class: "max-w-[600px]",
                                alt: "{alt_add_operator_label}",
                                src: IMAGE_TUTORIAL_2,
                            }
                        }
                        p { "{open_settings_label}" }
                        p { "{operator_fields_label}" }
                        p { "{finish_operator_label}" }
                        h2 { class: "text-2xl font-bold mt-10", "{session_heading_label}" }
                        p {
                            img { class: "max-w-[600px]", alt: "{alt_session_label}", src: IMAGE_TUTORIAL_3 }
                        }
                        p { "{default_session_label}" }
                        p { "{switch_session_label}" }
                        ul { style: "list-style: circle inside",
                            li { "{chat_head_style_label}" }
                            li {
                                "{menu_intro_label}"
                                ul { class: "ml-10", style: "list-style: square inside",
                                    li { "{send_other_label}" }
                                    li {
                                        em { "{send_other_shortcut_label}" }
                                    }
                                    li {
                                        "{send_status_label}"
                                        ul { class: "ml-10", style: "list-style: inside",
                                            li { "{status_line_label}" }
                                        }
                                    }
                                }
                            }
                        }
                        h2 { class: "text-2xl font-bold mt-10", "{replay_heading_label}" }
                        p {
                            img {
                                class: "max-w-[600px]",
                                alt: "{alt_full_chat_label}",
                                src: IMAGE_TUTORIAL_4,
                            }
                            img {
                                class: "max-w-[600px]",
                                alt: "{alt_replay_label}",
                                src: IMAGE_TUTORIAL_5,
                            }
                        }
                        p { "{dialogue_ready_label}" }
                        p { "{start_replay_label}" }
                        p { "{replay_modes_label}" }
                        ul { style: "list-style: circle inside",
                            li { "{fixed_interval_label}" }
                            li { "{per_char_mode_label}" }
                        }
                        p { "{interval_formula_label}" }
                        p { white_space: "pre-line", "{recommended_settings_label}" }
                        p { "{after_replay_missing_label}" }
                        hr {}
                        p {
                            em { "{share_label}" }
                        }
                    }
                    label { class: "flex items-center gap-2 text-black text-base cursor-pointer select-none",
                        input {
                            r#type: "checkbox",
                            class: "w-4 h-4 accent-blue-600",
                            checked: dont_show_again(),
                            onclick: move |_| dont_show_again.set(!dont_show_again()),
                        }
                        span { "{dont_show_again_label}" }
                    }
                }
            }
        }
    }
}
