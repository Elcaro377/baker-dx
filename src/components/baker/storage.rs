use anyhow::{Context, anyhow};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use legacy::{LegacyAppState, LegacyMessage};
#[cfg(not(target_arch = "wasm32"))]
use std::fs;
use v1::{
    AppState as V1AppState, Contact as V1Contact, Message as V1Message,
    MessageKind as V1MessageKind, Operator as V1Operator, UserProfile as V1UserProfile,
};
use v2::{
    AppState, BackgroundMode, BackgroundSettings, ChatHeadStyle, Contact, Message, MessageKind,
    MessageReaction, Operator, UserProfile,
};

pub(crate) mod legacy;
pub(crate) mod v1;
pub(crate) mod v2;

const DEFAULT_STATE_JSON: &str = include_str!("../../../baker_dx_state_default.json");
const V1_STORAGE_KEY: &str = "baker_dx_state";
const V2_META_STORAGE_KEY: &str = "baker_dx_state_v2_meta";
const V2_DB_NAME: &str = "baker_dx_state_v2";
const V3_META_STORAGE_KEY: &str = "baker_dx_state_v3_meta";
const V3_DB_NAME: &str = "baker_dx_state_v3";
const MESSAGE_STORE_PREFIX: &str = "messages__";

const LOCAL_STORAGE_GET_SCRIPT: &str = r#"
    const key = await dioxus.recv();
    return window.localStorage.getItem(key);
"#;

const LOAD_V2_DB_SCRIPT: &str = r#"
    const dbName = await dioxus.recv();
    const messagePrefix = await dioxus.recv();

    function openExistingDb(name) {
        return new Promise((resolve, reject) => {
            const request = indexedDB.open(name);
            let settled = false;

            const resolveOnce = (value) => {
                if (!settled) {
                    settled = true;
                    resolve(value);
                }
            };
            const rejectOnce = (error) => {
                if (!settled) {
                    settled = true;
                    reject(error);
                }
            };

            request.onupgradeneeded = () => {
                const db = request.result;
                if (db) {
                    db.close();
                }
                if (request.transaction) {
                    request.transaction.abort();
                }
                resolveOnce(null);
            };

            request.onerror = () => {
                rejectOnce(request.error || new Error("Failed to open IndexedDB"));
            };

            request.onblocked = () => {
                rejectOnce(new Error("IndexedDB open blocked"));
            };

            request.onsuccess = () => {
                if (settled) {
                    request.result.close();
                    return;
                }
                resolveOnce(request.result);
            };
        });
    }

    function getAllFromStore(db, storeName) {
        return new Promise((resolve, reject) => {
            if (!db.objectStoreNames.contains(storeName)) {
                resolve(null);
                return;
            }
            const transaction = db.transaction(storeName, "readonly");
            const request = transaction.objectStore(storeName).getAll();

            request.onsuccess = () => resolve(request.result);
            request.onerror = () =>
                reject(request.error || new Error(`Failed to read ${storeName}`));
            transaction.onabort = () =>
                reject(transaction.error || new Error(`Read transaction aborted for ${storeName}`));
        });
    }

    const db = await openExistingDb(dbName);
    if (!db) {
        return null;
    }

    if (!db.objectStoreNames.contains("contacts") || !db.objectStoreNames.contains("images")) {
        db.close();
        return null;
    }

    const contacts = await getAllFromStore(db, "contacts");
    const images = await getAllFromStore(db, "images");
    if (!contacts || !images) {
        db.close();
        return null;
    }

    const messages = {};
    for (const contact of contacts) {
        const storeName = `${messagePrefix}${contact.id}`;
        const records = await getAllFromStore(db, storeName);
        if (records === null) {
            db.close();
            return null;
        }
        messages[contact.id] = records;
    }

    db.close();
    return JSON.stringify({ contacts, images, messages });
"#;

const LOAD_V3_DB_SCRIPT: &str = r#"
    const dbName = await dioxus.recv();

    function openExistingDb(name) {
        return new Promise((resolve, reject) => {
            const request = indexedDB.open(name);
            let settled = false;

            const resolveOnce = (value) => {
                if (!settled) {
                    settled = true;
                    resolve(value);
                }
            };
            const rejectOnce = (error) => {
                if (!settled) {
                    settled = true;
                    reject(error);
                }
            };

            request.onupgradeneeded = () => {
                const db = request.result;
                if (db) {
                    db.close();
                }
                if (request.transaction) {
                    request.transaction.abort();
                }
                resolveOnce(null);
            };

            request.onerror = () => {
                rejectOnce(request.error || new Error("Failed to open IndexedDB"));
            };

            request.onblocked = () => {
                rejectOnce(new Error("IndexedDB open blocked"));
            };

            request.onsuccess = () => {
                if (settled) {
                    request.result.close();
                    return;
                }
                resolveOnce(request.result);
            };
        });
    }

    function getStoreValue(db, storeName, key) {
        return new Promise((resolve, reject) => {
            if (!db.objectStoreNames.contains(storeName)) {
                resolve(null);
                return;
            }

            const transaction = db.transaction(storeName, "readonly");
            const request = transaction.objectStore(storeName).get(key);

            request.onsuccess = () => resolve(request.result ?? null);
            request.onerror = () =>
                reject(request.error || new Error(`Failed to read ${storeName}`));
            transaction.onabort = () =>
                reject(transaction.error || new Error(`Read transaction aborted for ${storeName}`));
        });
    }

    const db = await openExistingDb(dbName);
    if (!db) {
        return null;
    }

    const snapshot = await getStoreValue(db, "state", "snapshot");
    db.close();

    if (!snapshot || typeof snapshot.value !== "string") {
        return null;
    }

    return snapshot.value;
"#;

const SAVE_V3_DB_SCRIPT: &str = r#"
    const dbName = await dioxus.recv();
    const metaKey = await dioxus.recv();
    const metaJson = await dioxus.recv();
    const snapshotJson = await dioxus.recv();
    const meta = JSON.parse(metaJson);

    function openDb(name, version, onUpgrade) {
        return new Promise((resolve, reject) => {
            const request =
                version === null || version === undefined
                    ? indexedDB.open(name)
                    : indexedDB.open(name, version);

            request.onerror = () => reject(request.error || new Error("Failed to open IndexedDB"));
            request.onblocked = () => reject(new Error("IndexedDB open blocked"));
            request.onupgradeneeded = () => {
                if (onUpgrade) {
                    onUpgrade(request.result);
                }
            };
            request.onsuccess = () => resolve(request.result);
        });
    }

    function ensureStores(db) {
        if (!db.objectStoreNames.contains("meta")) {
            db.createObjectStore("meta", { keyPath: "key" });
        }
        if (!db.objectStoreNames.contains("state")) {
            db.createObjectStore("state", { keyPath: "key" });
        }
    }

    let db = await openDb(dbName, null, (upgradeDb) => {
        ensureStores(upgradeDb);
    });

    const missingStores = ["meta", "state"].filter(
        (storeName) => !db.objectStoreNames.contains(storeName)
    );

    if (missingStores.length > 0) {
        const nextVersion = db.version + 1;
        db.close();
        db = await openDb(dbName, nextVersion, (upgradeDb) => {
            ensureStores(upgradeDb);
        });
    }

    const result = await new Promise((resolve, reject) => {
        const transaction = db.transaction(["meta", "state"], "readwrite");
        const metaStore = transaction.objectStore("meta");
        const stateStore = transaction.objectStore("state");
        let skipped = false;

        const revisionRequest = metaStore.get("revision");
        revisionRequest.onerror = () => {
            reject(revisionRequest.error || new Error("Failed to read current revision"));
        };

        revisionRequest.onsuccess = () => {
            const currentRevision = Number(revisionRequest.result?.value ?? 0);
            const incomingRevision = Number(meta.revision ?? 0);

            if (currentRevision > incomingRevision) {
                skipped = true;
                transaction.abort();
                return;
            }

            metaStore.put({ key: "revision", value: incomingRevision });
            stateStore.put({ key: "snapshot", value: snapshotJson });
        };

        transaction.oncomplete = () => resolve({ skipped: false });
        transaction.onabort = () => {
            if (skipped) {
                resolve({ skipped: true });
            } else {
                reject(transaction.error || new Error("IndexedDB write transaction aborted"));
            }
        };
        transaction.onerror = () => {};
    });

    db.close();

    if (!result.skipped) {
        window.localStorage.setItem(metaKey, metaJson);
    }

    return JSON.stringify(result);
"#;

const DELETE_V2_STORAGE_SCRIPT: &str = r#"
    const dbName = await dioxus.recv();
    const metaKey = await dioxus.recv();

    function deleteDb(name) {
        return new Promise((resolve, reject) => {
            const request = indexedDB.deleteDatabase(name);
            request.onsuccess = () => resolve();
            request.onerror = () =>
                reject(request.error || new Error("Failed to delete IndexedDB"));
            request.onblocked = () => reject(new Error("IndexedDB delete blocked"));
        });
    }

    window.localStorage.removeItem(metaKey);
    await deleteDb(dbName);
    return "ok";
"#;

pub struct LoadedState {
    pub state: AppState,
    pub revision: u64,
    pub skip_initial_save: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct PersistedImageRecord {
    id: String,
    sha256: String,
    data_url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value")]
enum StoredImageRef {
    Indexed(String),
    Raw(String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct PersistedUserProfile {
    id: String,
    name: String,
    avatar: Option<StoredImageRef>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct PersistedOperator {
    id: String,
    name: String,
    avatar: Option<StoredImageRef>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct PersistedContact {
    id: String,
    order: u64,
    unread_count: usize,
    chat_head_style: ChatHeadStyle,
    name: String,
    avatar: Option<StoredImageRef>,
    participant_ids: Vec<String>,
    is_group: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value")]
enum PersistedMessageKind {
    Normal(String),
    Status(String),
    TopicEnded(String),
    Image(StoredImageRef),
    Sticker(StoredImageRef),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct PersistedMessage {
    order: u64,
    id: String,
    sender_id: String,
    kind: PersistedMessageKind,
    reactions: Vec<MessageReaction>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value")]
enum PersistedBackground {
    DotDark,
    DotLight,
    CustomColor(String),
    CustomImage(StoredImageRef),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct PersistedMeta {
    version: u8,
    revision: u64,
    user_profile: PersistedUserProfile,
    operators: Vec<PersistedOperator>,
    stickers: Vec<StoredImageRef>,
    background: PersistedBackground,
    update_snooze_date: Option<String>,
    hide_tutorial: bool,
    show_tip_saving_image_problem_on_web: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct PersistedDbSnapshot {
    contacts: Vec<PersistedContact>,
    images: Vec<PersistedImageRecord>,
    messages: HashMap<String, Vec<PersistedMessage>>,
}

#[derive(Clone, Debug, Serialize)]
struct PersistedV3SavePayload {
    meta_json: String,
    snapshot_json: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct PersistedV3Meta {
    version: u8,
    revision: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct PersistedV3Snapshot {
    state: AppState,
}

fn migrate_legacy_state_to_v1(legacy: LegacyAppState) -> V1AppState {
    let mut id_map: HashMap<usize, String> = HashMap::new();
    let user_id = Uuid::new_v4().to_string();

    let operators = legacy
        .operators
        .into_iter()
        .map(|op| {
            let new_id = Uuid::new_v4().to_string();
            id_map.insert(op.id, new_id.clone());
            V1Operator {
                id: new_id,
                name: op.name,
                avatar_url: op.avatar_url,
            }
        })
        .collect::<Vec<_>>();
    let operator_map = operators
        .iter()
        .map(|op| (op.id.clone(), (op.name.clone(), op.avatar_url.clone())))
        .collect::<HashMap<_, _>>();

    let contacts = legacy
        .contacts
        .into_iter()
        .map(|contact| {
            let new_id = id_map.get(&contact.id).cloned().unwrap_or_else(|| {
                let new_id = Uuid::new_v4().to_string();
                id_map.insert(contact.id, new_id.clone());
                new_id
            });
            let (name, avatar) = operator_map
                .get(&new_id)
                .cloned()
                .unwrap_or_else(|| ("".to_string(), "".to_string()));
            V1Contact {
                id: new_id.clone(),
                unread_count: contact.unread_count,
                chat_head_style: contact.chat_head_style,
                name,
                avatar_url: avatar,
                participant_ids: vec![new_id],
                is_group: false,
            }
        })
        .collect::<Vec<_>>();

    let mut messages: HashMap<String, Vec<V1Message>> = HashMap::new();
    for (legacy_contact_id, list) in legacy.messages {
        let contact_id = id_map.get(&legacy_contact_id).cloned().unwrap_or_else(|| {
            let new_id = Uuid::new_v4().to_string();
            id_map.insert(legacy_contact_id, new_id.clone());
            new_id
        });
        let converted = list
            .into_iter()
            .map(|msg| {
                let LegacyMessage {
                    id: _legacy_id,
                    sender_id,
                    content,
                    timestamp: _timestamp,
                    animate,
                } = msg;
                let sender_id = if sender_id == 0 {
                    user_id.clone()
                } else {
                    id_map.get(&sender_id).cloned().unwrap_or_else(|| {
                        let new_id = Uuid::new_v4().to_string();
                        id_map.insert(sender_id, new_id.clone());
                        new_id
                    })
                };
                V1Message {
                    id: Uuid::new_v4().to_string(),
                    sender_id,
                    content,
                    kind: V1MessageKind::Normal,
                    animate,
                    animate_reactions: false,
                    reactions: Vec::new(),
                }
            })
            .collect::<Vec<_>>();
        messages.insert(contact_id, converted);
    }

    V1AppState {
        user_profile: V1UserProfile {
            id: user_id,
            name: legacy.user_profile.name,
            avatar_url: legacy.user_profile.avatar_url,
        },
        contacts,
        messages,
        operators,
        stickers: Vec::new(),
        background: legacy.background,
        update_snooze_date: None,
        hide_tutorial: false,
        show_tip_saving_image_problem_on_web: false,
    }
}

fn migrate_v1_state_to_v2(state: V1AppState) -> AppState {
    AppState {
        user_profile: UserProfile {
            id: state.user_profile.id,
            name: state.user_profile.name,
            avatar_url: state.user_profile.avatar_url,
        },
        contacts: state
            .contacts
            .into_iter()
            .map(|contact| Contact {
                id: contact.id,
                unread_count: contact.unread_count,
                chat_head_style: match contact.chat_head_style {
                    v1::ChatHeadStyle::Default => ChatHeadStyle::Default,
                    v1::ChatHeadStyle::Alt => ChatHeadStyle::Alt,
                },
                name: contact.name,
                avatar_url: contact.avatar_url,
                participant_ids: contact.participant_ids,
                participants_selves_ids: vec![],
                is_group: contact.is_group,
            })
            .collect(),
        messages: state
            .messages
            .into_iter()
            .map(|(contact_id, messages)| {
                (
                    contact_id,
                    messages
                        .into_iter()
                        .map(|message| Message {
                            id: message.id,
                            sender_id: message.sender_id,
                            content: message.content,
                            kind: match message.kind {
                                V1MessageKind::Normal => MessageKind::Normal,
                                V1MessageKind::Status => MessageKind::Status,
                                V1MessageKind::TopicEnded => MessageKind::TopicEnded,
                                V1MessageKind::Image => MessageKind::Image,
                                V1MessageKind::Sticker => MessageKind::Sticker,
                            },
                            animate: message.animate,
                            animate_reactions: message.animate_reactions,
                            reactions: message
                                .reactions
                                .into_iter()
                                .map(|reaction| MessageReaction {
                                    content: reaction.content,
                                    sender_id: reaction.sender_id,
                                })
                                .collect(),
                        })
                        .collect(),
                )
            })
            .collect(),
        operators: state
            .operators
            .into_iter()
            .map(|operator| Operator {
                id: operator.id,
                name: operator.name,
                avatar_url: operator.avatar_url,
            })
            .collect(),
        stickers: state.stickers,
        background: BackgroundSettings {
            mode: match state.background.mode {
                v1::BackgroundMode::DotDark => BackgroundMode::DotDark,
                v1::BackgroundMode::DotLight => BackgroundMode::DotLight,
                v1::BackgroundMode::CustomColor => BackgroundMode::CustomColor,
                v1::BackgroundMode::CustomImage => BackgroundMode::CustomImage,
            },
            custom_color: state.background.custom_color,
            custom_image: state.background.custom_image,
        },
        update_snooze_date: state.update_snooze_date,
        hide_tutorial: state.hide_tutorial,
        show_tip_saving_image_problem_on_web: state.show_tip_saving_image_problem_on_web,
    }
}

fn parse_v1_state_from_str(raw: &str) -> Option<V1AppState> {
    serde_json::from_str::<V1AppState>(raw).ok()
}

fn parse_legacy_state_from_str(raw: &str) -> Option<LegacyAppState> {
    serde_json::from_str::<LegacyAppState>(raw).ok()
}

fn default_loaded_state() -> LoadedState {
    if let Some(v1_state) = parse_v1_state_from_str(DEFAULT_STATE_JSON) {
        return LoadedState {
            state: migrate_v1_state_to_v2(v1_state),
            revision: 0,
            skip_initial_save: false,
        };
    }
    if let Some(legacy_state) = parse_legacy_state_from_str(DEFAULT_STATE_JSON) {
        return LoadedState {
            state: migrate_v1_state_to_v2(migrate_legacy_state_to_v1(legacy_state)),
            revision: 0,
            skip_initial_save: false,
        };
    }

    LoadedState {
        state: AppState::default(),
        revision: 0,
        skip_initial_save: false,
    }
}

fn resolve_image_ref(
    reference: &Option<StoredImageRef>,
    images: &HashMap<String, String>,
) -> Option<String> {
    match reference {
        None => Some(String::new()),
        Some(StoredImageRef::Indexed(id)) => images.get(id).cloned(),
        Some(StoredImageRef::Raw(value)) => Some(value.clone()),
    }
}

fn encode_v3_state(state: &AppState, revision: u64) -> anyhow::Result<PersistedV3SavePayload> {
    let meta = PersistedV3Meta {
        version: 3,
        revision,
    };
    let snapshot = PersistedV3Snapshot {
        state: state.clone(),
    };

    Ok(PersistedV3SavePayload {
        meta_json: serde_json::to_string(&meta).context("failed to serialize v3 metadata")?,
        snapshot_json: serde_json::to_string(&snapshot)
            .context("failed to serialize v3 snapshot")?,
    })
}

fn decode_state(meta: PersistedMeta, snapshot: PersistedDbSnapshot) -> Option<AppState> {
    let image_map = snapshot
        .images
        .into_iter()
        .map(|image| (image.id, image.data_url))
        .collect::<HashMap<_, _>>();

    let contacts = {
        let mut contacts = snapshot.contacts;
        contacts.sort_by_key(|contact| contact.order);
        contacts
            .into_iter()
            .map(|contact| {
                Some(Contact {
                    id: contact.id,
                    unread_count: contact.unread_count,
                    chat_head_style: contact.chat_head_style,
                    name: contact.name,
                    avatar_url: resolve_image_ref(&contact.avatar, &image_map)?,
                    participant_ids: contact.participant_ids,
                    participants_selves_ids: vec![],
                    is_group: contact.is_group,
                })
            })
            .collect::<Option<Vec<_>>>()?
    };

    let messages = snapshot
        .messages
        .into_iter()
        .map(|(contact_id, list)| {
            let mut list = list;
            list.sort_by_key(|message| message.order);
            let list = list
                .into_iter()
                .map(|message| {
                    let (kind, content) = match message.kind {
                        PersistedMessageKind::Normal(content) => (MessageKind::Normal, content),
                        PersistedMessageKind::Status(content) => (MessageKind::Status, content),
                        PersistedMessageKind::TopicEnded(content) => {
                            (MessageKind::TopicEnded, content)
                        }
                        PersistedMessageKind::Image(image) => (
                            MessageKind::Image,
                            resolve_image_ref(&Some(image), &image_map)?,
                        ),
                        PersistedMessageKind::Sticker(image) => (
                            MessageKind::Sticker,
                            resolve_image_ref(&Some(image), &image_map)?,
                        ),
                    };

                    Some(Message {
                        id: message.id,
                        sender_id: message.sender_id,
                        content,
                        kind,
                        animate: false,
                        animate_reactions: false,
                        reactions: message.reactions,
                    })
                })
                .collect::<Option<Vec<_>>>()?;
            Some((contact_id, list))
        })
        .collect::<Option<HashMap<_, _>>>()?;

    let background = match meta.background {
        PersistedBackground::DotDark => BackgroundSettings {
            mode: BackgroundMode::DotDark,
            ..BackgroundSettings::default()
        },
        PersistedBackground::DotLight => BackgroundSettings {
            mode: BackgroundMode::DotLight,
            ..BackgroundSettings::default()
        },
        PersistedBackground::CustomColor(color) => BackgroundSettings {
            mode: BackgroundMode::CustomColor,
            custom_color: color,
            custom_image: String::new(),
        },
        PersistedBackground::CustomImage(image) => BackgroundSettings {
            mode: BackgroundMode::CustomImage,
            custom_color: BackgroundSettings::default().custom_color,
            custom_image: resolve_image_ref(&Some(image), &image_map)?,
        },
    };

    Some(AppState {
        user_profile: UserProfile {
            id: meta.user_profile.id,
            name: meta.user_profile.name,
            avatar_url: resolve_image_ref(&meta.user_profile.avatar, &image_map)?,
        },
        contacts,
        messages,
        operators: meta
            .operators
            .into_iter()
            .map(|operator| {
                Some(Operator {
                    id: operator.id,
                    name: operator.name,
                    avatar_url: resolve_image_ref(&operator.avatar, &image_map)?,
                })
            })
            .collect::<Option<Vec<_>>>()?,
        stickers: meta
            .stickers
            .into_iter()
            .map(|sticker| resolve_image_ref(&Some(sticker), &image_map))
            .collect::<Option<Vec<_>>>()?,
        background,
        update_snooze_date: meta.update_snooze_date,
        hide_tutorial: meta.hide_tutorial,
        show_tip_saving_image_problem_on_web: meta.show_tip_saving_image_problem_on_web,
    })
}

fn decode_v3_state(snapshot: PersistedV3Snapshot) -> AppState {
    snapshot.state
}

async fn eval_value(script: &str, inputs: &[String]) -> anyhow::Result<serde_json::Value> {
    let eval = document::eval(script);
    for input in inputs {
        eval.send(input.clone())
            .map_err(|err| anyhow!(err.to_string()))?;
    }

    eval.await.map_err(|err| anyhow!(err.to_string()))
}

async fn web_storage_get(key: &str) -> anyhow::Result<Option<String>> {
    let value = eval_value(LOCAL_STORAGE_GET_SCRIPT, &[key.to_string()]).await?;
    if value.is_null() {
        return Ok(None);
    }

    value
        .as_str()
        .map(|raw| Some(raw.to_string()))
        .ok_or_else(|| anyhow!("localStorage returned a non-string value"))
}

async fn load_v2_snapshot_from_web_storage()
-> anyhow::Result<Option<(PersistedMeta, PersistedDbSnapshot)>> {
    let Some(meta_raw) = web_storage_get(V2_META_STORAGE_KEY).await? else {
        return Ok(None);
    };

    let meta = match serde_json::from_str::<PersistedMeta>(&meta_raw) {
        Ok(meta) if meta.version == 2 => meta,
        _ => return Ok(None),
    };

    let snapshot_raw = eval_value(
        LOAD_V2_DB_SCRIPT,
        &[V2_DB_NAME.to_string(), MESSAGE_STORE_PREFIX.to_string()],
    )
    .await?;

    let Some(snapshot_json) = snapshot_raw.as_str() else {
        return Ok(None);
    };

    let snapshot = match serde_json::from_str::<PersistedDbSnapshot>(snapshot_json) {
        Ok(snapshot) => snapshot,
        Err(_) => return Ok(None),
    };

    Ok(Some((meta, snapshot)))
}

async fn load_v3_snapshot_from_web_storage()
-> anyhow::Result<Option<(PersistedV3Meta, PersistedV3Snapshot)>> {
    let Some(meta_raw) = web_storage_get(V3_META_STORAGE_KEY).await? else {
        return Ok(None);
    };

    let meta = match serde_json::from_str::<PersistedV3Meta>(&meta_raw) {
        Ok(meta) if meta.version == 3 => meta,
        _ => return Ok(None),
    };

    let snapshot_raw = eval_value(LOAD_V3_DB_SCRIPT, &[V3_DB_NAME.to_string()]).await?;

    let Some(snapshot_json) = snapshot_raw.as_str() else {
        return Ok(None);
    };

    let snapshot = match serde_json::from_str::<PersistedV3Snapshot>(snapshot_json) {
        Ok(snapshot) => snapshot,
        Err(_) => return Ok(None),
    };

    Ok(Some((meta, snapshot)))
}

async fn cleanup_v2_web_storage() -> anyhow::Result<()> {
    let _ = eval_value(
        DELETE_V2_STORAGE_SCRIPT,
        &[V2_DB_NAME.to_string(), V2_META_STORAGE_KEY.to_string()],
    )
    .await?;
    Ok(())
}

async fn try_load_v1_from_web_storage() -> anyhow::Result<Option<V1AppState>> {
    let Some(raw) = web_storage_get(V1_STORAGE_KEY).await? else {
        return Ok(None);
    };
    Ok(parse_v1_state_from_str(&raw))
}

async fn try_load_legacy_from_web_storage() -> anyhow::Result<Option<LegacyAppState>> {
    let Some(raw) = web_storage_get(V1_STORAGE_KEY).await? else {
        return Ok(None);
    };
    Ok(parse_legacy_state_from_str(&raw))
}

#[cfg(not(target_arch = "wasm32"))]
fn try_load_v1_from_desktop_file() -> Option<V1AppState> {
    fs::read_to_string("baker_dx_state.json")
        .ok()
        .and_then(|raw| parse_v1_state_from_str(&raw))
}

#[cfg(not(target_arch = "wasm32"))]
fn try_load_legacy_from_desktop_file() -> Option<LegacyAppState> {
    fs::read_to_string("baker_dx_state.json")
        .ok()
        .and_then(|raw| parse_legacy_state_from_str(&raw))
}

pub async fn load_state() -> LoadedState {
    if let Ok(Some((meta, snapshot))) = load_v3_snapshot_from_web_storage().await {
        return LoadedState {
            state: decode_v3_state(snapshot),
            revision: meta.revision,
            skip_initial_save: true,
        };
    }

    if let Ok(Some((meta, snapshot))) = load_v2_snapshot_from_web_storage().await
        && let Some(state) = decode_state(meta.clone(), snapshot)
    {
        return LoadedState {
            state,
            revision: meta.revision,
            skip_initial_save: false,
        };
    }

    if let Ok(Some(v1_state)) = try_load_v1_from_web_storage().await {
        return LoadedState {
            state: migrate_v1_state_to_v2(v1_state),
            revision: 0,
            skip_initial_save: false,
        };
    }

    #[cfg(not(target_arch = "wasm32"))]
    if let Some(v1_state) = try_load_v1_from_desktop_file() {
        return LoadedState {
            state: migrate_v1_state_to_v2(v1_state),
            revision: 0,
            skip_initial_save: false,
        };
    }

    if let Ok(Some(legacy_state)) = try_load_legacy_from_web_storage().await {
        return LoadedState {
            state: migrate_v1_state_to_v2(migrate_legacy_state_to_v1(legacy_state)),
            revision: 0,
            skip_initial_save: false,
        };
    }

    #[cfg(not(target_arch = "wasm32"))]
    if let Some(legacy_state) = try_load_legacy_from_desktop_file() {
        return LoadedState {
            state: migrate_v1_state_to_v2(migrate_legacy_state_to_v1(legacy_state)),
            revision: 0,
            skip_initial_save: false,
        };
    }

    default_loaded_state()
}

pub async fn save_state(state: &AppState, revision: u64) -> anyhow::Result<()> {
    let payload = encode_v3_state(state, revision)?;
    let result_value = eval_value(
        SAVE_V3_DB_SCRIPT,
        &[
            V3_DB_NAME.to_string(),
            V3_META_STORAGE_KEY.to_string(),
            payload.meta_json,
            payload.snapshot_json,
        ],
    )
    .await?;

    let result_json = result_value
        .as_str()
        .ok_or_else(|| anyhow!("save script returned a non-string value"))?;

    #[derive(Deserialize)]
    struct SaveResult {
        skipped: bool,
    }

    let parsed = serde_json::from_str::<SaveResult>(result_json)
        .context("failed to parse IndexedDB save result")?;
    if !parsed.skipped {
        let _ = cleanup_v2_web_storage().await;
    }

    Ok(())
}
