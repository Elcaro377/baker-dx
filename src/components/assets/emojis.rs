use dioxus::prelude::*;

pub const SNS_EMOJI_001: Asset = asset!("/assets/extracted/emoji/sns_emoji_001.png");
pub const SNS_EMOJI_002: Asset = asset!("/assets/extracted/emoji/sns_emoji_002.png");
pub const SNS_EMOJI_003: Asset = asset!("/assets/extracted/emoji/sns_emoji_003.png");
pub const SNS_EMOJI_004: Asset = asset!("/assets/extracted/emoji/sns_emoji_004.png");
pub const SNS_EMOJI_005: Asset = asset!("/assets/extracted/emoji/sns_emoji_005.png");
pub const SNS_EMOJI_006: Asset = asset!("/assets/extracted/emoji/sns_emoji_006.png");
pub const SNS_EMOJI_007: Asset = asset!("/assets/extracted/emoji/sns_emoji_007.png");
pub const SNS_EMOJI_008: Asset = asset!("/assets/extracted/emoji/sns_emoji_008.png");
pub const SNS_EMOJI_009: Asset = asset!("/assets/extracted/emoji/sns_emoji_009.png");
pub const SNS_EMOJI_010: Asset = asset!("/assets/extracted/emoji/sns_emoji_010.png");
pub const SNS_EMOJI_011: Asset = asset!("/assets/extracted/emoji/sns_emoji_011.png");
pub const SNS_EMOJI_012: Asset = asset!("/assets/extracted/emoji/sns_emoji_012.png");
pub const SNS_EMOJI_013: Asset = asset!("/assets/extracted/emoji/sns_emoji_013.png");
pub const SNS_EMOJI_014: Asset = asset!("/assets/extracted/emoji/sns_emoji_014.png");
pub const SNS_EMOJI_015: Asset = asset!("/assets/extracted/emoji/sns_emoji_015.png");
pub const SNS_EMOJI_016: Asset = asset!("/assets/extracted/emoji/sns_emoji_016.png");
pub const SNS_EMOJI_017: Asset = asset!("/assets/extracted/emoji/sns_emoji_017.png");
pub const SNS_EMOJI_018: Asset = asset!("/assets/extracted/emoji/sns_emoji_018.png");
pub const SNS_EMOJI_019: Asset = asset!("/assets/extracted/emoji/sns_emoji_019.png");
pub const SNS_EMOJI_020: Asset = asset!("/assets/extracted/emoji/sns_emoji_020.png");
pub const SNS_EMOJI_021: Asset = asset!("/assets/extracted/emoji/sns_emoji_021.png");
pub const SNS_EMOJI_022: Asset = asset!("/assets/extracted/emoji/sns_emoji_022.png");
pub const SNS_EMOJI_023: Asset = asset!("/assets/extracted/emoji/sns_emoji_023.png");
pub const SNS_EMOJI_024: Asset = asset!("/assets/extracted/emoji/sns_emoji_024.png");
pub const SNS_EMOJI_025: Asset = asset!("/assets/extracted/emoji/sns_emoji_025.png");
pub const SNS_EMOJI_026: Asset = asset!("/assets/extracted/emoji/sns_emoji_026.png");
pub const SNS_EMOJI_027: Asset = asset!("/assets/extracted/emoji/sns_emoji_027.png");
pub const SNS_EMOJI_028: Asset = asset!("/assets/extracted/emoji/sns_emoji_028.png");
pub const SNS_EMOJI_029: Asset = asset!("/assets/extracted/emoji/sns_emoji_029.png");
pub const SNS_EMOJI_030: Asset = asset!("/assets/extracted/emoji/sns_emoji_030.png");
pub const SNS_EMOJI_031: Asset = asset!("/assets/extracted/emoji/sns_emoji_031.png");
pub const SNS_EMOJI_032: Asset = asset!("/assets/extracted/emoji/sns_emoji_032.png");
pub const SNS_EMOJI_033: Asset = asset!("/assets/extracted/emoji/sns_emoji_033.png");
pub const SNS_EMOJI_034: Asset = asset!("/assets/extracted/emoji/sns_emoji_034.png");
pub const SNS_EMOJI_035: Asset = asset!("/assets/extracted/emoji/sns_emoji_035.png");
pub const SNS_EMOJI_036: Asset = asset!("/assets/extracted/emoji/sns_emoji_036.png");
pub const SNS_EMOJI_037: Asset = asset!("/assets/extracted/emoji/sns_emoji_037.png");
pub const SNS_EMOJI_038: Asset = asset!("/assets/extracted/emoji/sns_emoji_038.png");

pub const EMOJI_KEYS: [&str; 38] = [
    "happy",
    "stars",
    "surprising",
    "smile",
    "thumb",
    "sad",
    "laugh",
    "cry",
    "meh",
    "sweat",
    "cool",
    "grin",
    "confused",
    "pensive",
    "pray",
    "ok",
    "tongue",
    "love",
    "blush",
    "lol",
    "heart",
    "sparkle",
    "sweat_smile",
    "grinning",
    "plus_one",
    "skeptical",
    "hundred",
    "dead",
    "angry",
    "annoyed",
    "dizzy",
    "shocked",
    "worried",
    "sleep",
    "suspicious",
    "mute",
    "fist_bump",
    "thinking",
];

pub fn to_emoji(text: &str) -> Option<Asset> {
    let normalized = normalize_emoji_key(text)?;

    match normalized.as_str() {
        "happy" => Some(SNS_EMOJI_001),
        "stars" => Some(SNS_EMOJI_002),
        "surprising" => Some(SNS_EMOJI_003),
        "smile" => Some(SNS_EMOJI_004),
        "thumb" => Some(SNS_EMOJI_005),
        "sad" => Some(SNS_EMOJI_006),
        "laugh" => Some(SNS_EMOJI_007),
        "cry" => Some(SNS_EMOJI_008),
        "meh" => Some(SNS_EMOJI_009),
        "sweat" => Some(SNS_EMOJI_010),
        "cool" => Some(SNS_EMOJI_011),
        "grin" => Some(SNS_EMOJI_012),
        "confused" => Some(SNS_EMOJI_013),
        "pensive" => Some(SNS_EMOJI_014),
        "pray" => Some(SNS_EMOJI_015),
        "ok" => Some(SNS_EMOJI_016),
        "tongue" => Some(SNS_EMOJI_017),
        "love" => Some(SNS_EMOJI_018),
        "blush" => Some(SNS_EMOJI_019),
        "lol" => Some(SNS_EMOJI_020),
        "heart" => Some(SNS_EMOJI_021),
        "sparkle" => Some(SNS_EMOJI_022),
        "sweat_smile" => Some(SNS_EMOJI_023),
        "grinning" => Some(SNS_EMOJI_024),
        "plus_one" => Some(SNS_EMOJI_025),
        "skeptical" => Some(SNS_EMOJI_026),
        "hundred" => Some(SNS_EMOJI_027),
        "dead" => Some(SNS_EMOJI_028),
        "angry" => Some(SNS_EMOJI_029),
        "annoyed" => Some(SNS_EMOJI_030),
        "dizzy" => Some(SNS_EMOJI_031),
        "shocked" => Some(SNS_EMOJI_032),
        "worried" => Some(SNS_EMOJI_033),
        "sleep" => Some(SNS_EMOJI_034),
        "suspicious" => Some(SNS_EMOJI_035),
        "mute" => Some(SNS_EMOJI_036),
        "fist_bump" => Some(SNS_EMOJI_037),
        "thinking" => Some(SNS_EMOJI_038),
        _ => emoji_from_resource_key(&normalized),
    }
}

fn normalize_emoji_key(text: &str) -> Option<String> {
    let trimmed = text.trim().trim_matches(':');
    if trimmed.is_empty() {
        return None;
    }

    Some(
        trimmed
            .to_ascii_lowercase()
            .replace(['-', ' '], "_"),
    )
}

fn emoji_from_resource_key(text: &str) -> Option<Asset> {
    let emoji_id = text
        .strip_prefix("sns_emoji_")
        .or_else(|| text.strip_prefix("sns_emoiji_"))
        .or_else(|| text.strip_prefix("emoji_"))
        .or_else(|| text.strip_prefix("emoiji_"))
        .unwrap_or(text)
        .parse::<u8>()
        .ok()?;

    match emoji_id {
        1 => Some(SNS_EMOJI_001),
        2 => Some(SNS_EMOJI_002),
        3 => Some(SNS_EMOJI_003),
        4 => Some(SNS_EMOJI_004),
        5 => Some(SNS_EMOJI_005),
        6 => Some(SNS_EMOJI_006),
        7 => Some(SNS_EMOJI_007),
        8 => Some(SNS_EMOJI_008),
        9 => Some(SNS_EMOJI_009),
        10 => Some(SNS_EMOJI_010),
        11 => Some(SNS_EMOJI_011),
        12 => Some(SNS_EMOJI_012),
        13 => Some(SNS_EMOJI_013),
        14 => Some(SNS_EMOJI_014),
        15 => Some(SNS_EMOJI_015),
        16 => Some(SNS_EMOJI_016),
        17 => Some(SNS_EMOJI_017),
        18 => Some(SNS_EMOJI_018),
        19 => Some(SNS_EMOJI_019),
        20 => Some(SNS_EMOJI_020),
        21 => Some(SNS_EMOJI_021),
        22 => Some(SNS_EMOJI_022),
        23 => Some(SNS_EMOJI_023),
        24 => Some(SNS_EMOJI_024),
        25 => Some(SNS_EMOJI_025),
        26 => Some(SNS_EMOJI_026),
        27 => Some(SNS_EMOJI_027),
        28 => Some(SNS_EMOJI_028),
        29 => Some(SNS_EMOJI_029),
        30 => Some(SNS_EMOJI_030),
        31 => Some(SNS_EMOJI_031),
        32 => Some(SNS_EMOJI_032),
        33 => Some(SNS_EMOJI_033),
        34 => Some(SNS_EMOJI_034),
        35 => Some(SNS_EMOJI_035),
        36 => Some(SNS_EMOJI_036),
        37 => Some(SNS_EMOJI_037),
        38 => Some(SNS_EMOJI_038),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{EMOJI_KEYS, to_emoji};

    #[test]
    fn all_declared_emoji_keys_resolve() {
        for key in EMOJI_KEYS {
            assert!(to_emoji(key).is_some(), "emoji key should resolve: {key}");
        }
    }
}
