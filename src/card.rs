use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

/// Represents any Yu-Gi-Oh! card.
///
/// This is an enum tagged by the card’s `frameType` field, which indicates
/// the card’s overall category (e.g. `"effect"`, `"spell"`, `"trap"`).
///
/// Each variant wraps a specific struct with fields that match the
/// YGOProDeck API response for that card type.
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "frameType")]
pub enum Card {
    /// A standard non-effect monster.
    #[serde(rename = "normal")]
    Normal(NormalMonster),
    /// A monster with a continuous or triggered effect.
    #[serde(rename = "effect")]
    Effect(EffectMonster),
    #[serde(rename = "ritual")]
    Ritual(RitualMonster),
    /// A monster that is fusion summoned.
    #[serde(rename = "fusion")]
    Fusion(FusionMonster),
    /// A monster that is synchro summoned.
    #[serde(rename = "synchro")]
    Synchro(SynchroMonster),
    /// A monster that is XYZ summoned.
    #[serde(rename = "xyz")]
    Xyz(XyzMonster),
    /// A monster that is link summoned.
    #[serde(rename = "link")]
    Link(LinkMonster),
    /// A spell card.
    #[serde(rename = "spell")]
    Spell(SpellCard),
    /// A trap card.
    #[serde(rename = "trap")]
    Trap(TrapCard),
    #[serde(rename = "skill")]
    Skill,
    #[serde(rename = "token")]
    Token,
    /// A Pendulum Monster
    ///
    /// Covers all pendulum-based frame types from the API:
    /// - `normal_pendulum`
    /// - `effect_pendulum`
    /// - `ritual_pendulum`
    /// - `fusion_pendulum`
    /// - `synchro_pendulum`
    /// - `xyz_pendulum`
    #[serde(
        rename = "normal_pendulum",
        alias = "effect_pendulum",
        alias = "ritual_pendulum",
        alias = "fusion_pendulum",
        alias = "synchro_pendulum",
        alias = "xyz_pendulum"
    )]
    Pendulum(PendulumMonster),
}

/// Shared metadata for all Yu-Gi-Oh! cards.
///
/// This struct is flattened into the other card structs so their
/// base information (name, description, ID, etc.) is directly accessible.
#[derive(Debug, Serialize, Deserialize)]
pub struct CardInfo {
    /// The unique ID of the card.
    pub id: CardId,
    /// The name of the card.
    pub name: String,
    /// The description or effect text of the card.
    pub desc: String,
    /// A human-readable version of the card’s type (e.g. `"Effect Monster"`).
    #[serde(rename = "humanReadableCardType")]
    pub human_readable_card_type: String,
    /// The official YGOProDeck card page URL.
    pub ygoprodeck_url: String,
    /// Card set data, if available.
    #[serde(rename = "card_sets", default)]
    pub sets: Vec<CardSet>,
    /// Image data for the card.
    #[serde(rename = "card_images")]
    pub images: Vec<CardImage>,
    /// Market price data from multiple vendors.
    #[serde(rename = "card_prices", default)]
    pub prices: Vec<CardPrices>,
}

/// Represents a Normal Monster card.
#[derive(Debug, Serialize, Deserialize)]
pub struct NormalMonster {
    /// Common card metadata.
    #[serde(flatten)]
    pub info: CardInfo,
    pub race: MonsterRace,
    pub attribute: Attribute,
    pub level: u8,
    pub atk: i32,
    pub def: i32,
    #[serde(rename = "type")]
    pub card_type: MonsterType,
}

/// Represents an Effect Monster card.
#[derive(Debug, Serialize, Deserialize)]
pub struct EffectMonster {
    #[serde(flatten)]
    pub info: CardInfo,
    pub race: MonsterRace,
    pub attribute: Attribute,
    pub atk: i32,
    pub def: i32,
    pub level: u8,
    #[serde(rename = "type")]
    pub card_type: MonsterType,
}

/// Represents a Ritual Monster card.
#[derive(Debug, Serialize, Deserialize)]
pub struct RitualMonster {
    #[serde(flatten)]
    pub info: CardInfo,
    pub race: MonsterRace,
    pub attribute: Attribute,
    pub atk: i32,
    pub def: i32,
    pub level: u8,
    #[serde(rename = "type")]
    pub card_type: MonsterType,
}

/// Represents a Fusion Monster card.
#[derive(Debug, Serialize, Deserialize)]
pub struct FusionMonster {
    #[serde(flatten)]
    pub info: CardInfo,
    pub race: MonsterRace,
    pub attribute: Attribute,
    pub atk: i32,
    pub def: i32,
    pub level: u8,
    #[serde(rename = "type")]
    pub card_type: MonsterType,
}

/// Represents a Synchro Monster card.
#[derive(Debug, Serialize, Deserialize)]
pub struct SynchroMonster {
    #[serde(flatten)]
    pub info: CardInfo,
    pub race: MonsterRace,
    pub attribute: Attribute,
    pub atk: i32,
    pub def: i32,
    pub level: u8,
    #[serde(rename = "type")]
    pub card_type: MonsterType,
}

/// Represents an XYZ Monster card.
///
/// The `rank` field corresponds to the “level” key in the API.
#[derive(Debug, Serialize, Deserialize)]
pub struct XyzMonster {
    #[serde(flatten)]
    pub info: CardInfo,
    pub race: MonsterRace,
    pub attribute: Attribute,
    pub atk: i32,
    pub def: i32,
    #[serde(rename = "level")]
    pub rank: u8,
    #[serde(rename = "type")]
    pub card_type: MonsterType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PendulumMonster {
    #[serde(flatten)]
    pub info: CardInfo,
    pub race: MonsterRace,
    pub attribute: Attribute,
    pub atk: i32,
    pub def: i32,
    pub level: u8,
    #[serde(rename = "type")]
    pub card_type: MonsterType,
    pub scale: u8,
}

/// Represents a Link Monster card.
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkMonster {
    #[serde(flatten)]
    pub info: CardInfo,
    pub race: MonsterRace,
    pub attribute: Attribute,
    pub atk: i32,
    pub linkval: u8,
    #[serde(rename = "type")]
    pub card_type: MonsterType,
    #[serde(rename = "linkmarkers")]
    pub link_markers: Vec<LinkMarker>,
}

/// Represents a Spell Card.
#[derive(Debug, Serialize, Deserialize)]
pub struct SpellCard {
    #[serde(flatten)]
    pub info: CardInfo,
    pub race: SpellRace,
}

/// Represents a Trap Card.
#[derive(Debug, Serialize, Deserialize)]
pub struct TrapCard {
    #[serde(flatten)]
    pub info: CardInfo,
    pub race: TrapRace,
}

/// Enum describing all possible frame types returned by the API.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FrameType {
    Normal,
    Effect,
    Ritual,
    Fusion,
    Synchro,
    Xyz,
    Link,
    NormalPendulum,
    EffectPendulum,
    RitualPendulum,
    FusionPendulum,
    SynchroPendulum,
    Spell,
    Trap,
    Token,
    Skill,
}

/// All supported monster races (e.g., Dragon, Warrior, etc.).
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum MonsterRace {
    Aqua,
    Beast,
    #[serde(rename = "Beast-Warrior")]
    BeastWarrior,
    #[serde(rename = "Creator-God")]
    CreatorGod,
    Cyberse,
    Dinosaur,
    #[serde(rename = "Divine-Beast")]
    DivineBeast,
    Dragon,
    Fairy,
    Fiend,
    Fish,
    Illusion,
    Insect,
    Machine,
    Plant,
    Psychic,
    Pyro,
    Reptile,
    Rock,
    #[serde(rename = "Sea Serpent")]
    SeaSerpent,
    Spellcaster,
    Thunder,
    Warrior,
    #[serde(rename = "Winged Beast")]
    WingedBeast,
    Wyrm,
    Zombie,
}

/// Spell card subtypes (e.g., Equip, Field, Ritual).
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum SpellRace {
    Normal,
    Field,
    Equip,
    Continuous,
    #[serde(rename = "Quick-Play")]
    QuickPlay,
    Ritual,
}

/// Trap card subtypes (e.g., Continuous, Counter).
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum TrapRace {
    Normal,
    Continuous,
    Counter,
}

/// All monster type variants, such as “Fusion Monster” or “Effect Monster”.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum MonsterType {
    #[serde(rename = "Effect Monster")]
    EffectMonster,
    #[serde(rename = "Flip Effect Monster")]
    FlipEffectMonster,
    #[serde(rename = "Flip Tuner Effect Monster")]
    FlipTunerEffectMonster,
    #[serde(rename = "Gemini Monster")]
    GeminiMonster,
    #[serde(rename = "Normal Monster")]
    NormalMonster,
    #[serde(rename = "Normal Tuner Monster")]
    NormalTunerMonster,
    #[serde(rename = "Pendulum Effect Monster")]
    PendulumEffectMonster,
    #[serde(rename = "Pendulum Effect Ritual Monster")]
    PendulumEffectRitualMonster,
    #[serde(rename = "Pendulum Flip Effect Monster")]
    PendulumFlipEffectMonster,
    #[serde(rename = "Pendulum Normal Monster")]
    PendulumNormalMonster,
    #[serde(rename = "Pendulum Tuner Effect Monster")]
    PendulumTunerEffectMonster,
    #[serde(rename = "Ritual Effect Monster")]
    RitualEffectMonster,
    #[serde(rename = "Ritual Monster")]
    RitualMonster,
    #[serde(rename = "Spirit Monster")]
    SpiritMonster,
    #[serde(rename = "Toon Monster")]
    ToonMonster,
    #[serde(rename = "Tuner Monster")]
    TunerMonster,
    #[serde(rename = "Union Effect Monster")]
    UnionEffectMonster,
    #[serde(rename = "Fusion Monster")]
    FusionMonster,
    #[serde(rename = "Link Monster")]
    LinkMonster,
    #[serde(rename = "Pendulum Effect Fusion Monster")]
    PendulumEffectFusionMonster,
    #[serde(rename = "Synchro Monster")]
    SynchroMonster,
    #[serde(rename = "Synchro Pendulum Effect Monster")]
    SynchroPendulumEffectMonster,
    #[serde(rename = "Synchro Tuner Monster")]
    SynchroTunerMonster,
    #[serde(rename = "XYZ Monster")]
    XYZMonster,
    #[serde(rename = "XYZ Pendulum Effect Monster")]
    XYZPendulumEffectMonster,
    #[serde(rename = "Token")]
    Token,
}

/// Card attributes (LIGHT, DARK, FIRE, etc.).
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum Attribute {
    Light,
    Dark,
    Water,
    Fire,
    Earth,
    Wind,
    Divine,
}

/// Indicates the direction of a Link Monster’s markers.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum LinkMarker {
    Top,
    #[serde(rename = "Top-Left")]
    TopLeft,
    #[serde(rename = "Top-Right")]
    TopRight,
    Left,
    Right,
    Bottom,
    #[serde(rename = "Bottom-Left")]
    BottomLeft,
    #[serde(rename = "Bottom-Right")]
    BottomRight,
}

/// Represents a set (printing) the card belongs to.
#[derive(Debug, Serialize, Deserialize)]
pub struct CardSet {
    #[serde(rename = "set_name")]
    pub name: String,
    #[serde(rename = "set_code")]
    pub code: String,
    #[serde(rename = "set_rarity")]
    pub rarity: String,
    #[serde(rename = "set_rarity_code")]
    pub rarity_code: String,
    #[serde(rename = "set_price")]
    pub price: String,
}

/// Unique identifier for a card.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct CardId(pub u64);

/// Image URLs for a card in various resolutions.
#[derive(Debug, Serialize, Deserialize)]
pub struct CardImage {
    pub id: u64,
    #[serde(rename = "image_url")]
    pub url: String,
    #[serde(rename = "image_url_small")]
    pub url_small: String,
    #[serde(rename = "image_url_cropped")]
    pub url_cropped: String,
}

/// Market price information for a card across multiple vendors.
#[derive(Debug, Serialize, Deserialize)]
pub struct CardPrices {
    #[serde(rename = "cardmarket_price")]
    pub cardmarket: String,
    #[serde(rename = "tcgplayer_price")]
    pub tcgplayer: String,
    #[serde(rename = "ebay_price")]
    pub ebay: String,
    #[serde(rename = "amazon_price")]
    pub amazon: String,
    #[serde(rename = "coolstuffinc_price")]
    pub coolstuffinc: String,
}

impl Display for MonsterRace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            MonsterRace::Aqua => "Aqua",
            MonsterRace::Beast => "Beast",
            MonsterRace::BeastWarrior => "Beast-Warrior",
            MonsterRace::CreatorGod => "Creator-God",
            MonsterRace::Cyberse => "Cyberse",
            MonsterRace::Dinosaur => "Dinosaur",
            MonsterRace::DivineBeast => "Divine-Beast",
            MonsterRace::Dragon => "Dragon",
            MonsterRace::Fairy => "Fairy",
            MonsterRace::Fiend => "Fiend",
            MonsterRace::Fish => "Fish",
            MonsterRace::Illusion => "Illusion",
            MonsterRace::Insect => "Insect",
            MonsterRace::Machine => "Machine",
            MonsterRace::Plant => "Plant",
            MonsterRace::Psychic => "Psychic",
            MonsterRace::Pyro => "Pyro",
            MonsterRace::Reptile => "Reptile",
            MonsterRace::Rock => "Rock",
            MonsterRace::SeaSerpent => "Sea Serpent",
            MonsterRace::Spellcaster => "Spellcaster",
            MonsterRace::Thunder => "Thunder",
            MonsterRace::Warrior => "Warrior",
            MonsterRace::WingedBeast => "Winged Beast",
            MonsterRace::Wyrm => "Wyrm",
            MonsterRace::Zombie => "Zombie",
        };
        write!(f, "{}", text)
    }
}

impl Display for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Attribute::Light => "LIGHT",
            Attribute::Dark => "DARK",
            Attribute::Water => "WATER",
            Attribute::Fire => "FIRE",
            Attribute::Earth => "EARTH",
            Attribute::Wind => "WIND",
            Attribute::Divine => "DIVINE",
        };
        write!(f, "{}", text)
    }
}

impl Display for LinkMarker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            LinkMarker::Top => "Top",
            LinkMarker::TopLeft => "Top-Left",
            LinkMarker::TopRight => "Top-Right",
            LinkMarker::Left => "Left",
            LinkMarker::Right => "Right",
            LinkMarker::Bottom => "Bottom",
            LinkMarker::BottomLeft => "Bottom-Left",
            LinkMarker::BottomRight => "Bottom-Right",
        };
        write!(f, "{}", text)
    }
}
