use std::fmt::{self, Display};

use serde::Serialize;
use urlencoding::encode;

use crate::card::{Attribute, LinkMarker, MonsterRace};

#[derive(Debug, Default)]
pub struct Request<'a> {
    names: Vec<&'a str>,
    fname: Option<&'a str>,
    atk: Option<i32>,
    def: Option<i32>,
    level: Option<u8>,
    card_types: Vec<CardType>,
    races: Vec<MonsterRace>,
    attributes: Vec<Attribute>,
    link: Option<u8>,
    link_markers: Vec<LinkMarker>,
    scale: Option<u8>,
    cardset: Option<&'a str>,
}

impl<'a> Request<'a> {
    pub fn to_url_params(&self) -> String {
        let mut params = Vec::new();

        if !self.names.is_empty() {
            params.push(format!("name={}", encode(&self.names.join("|"))));
        }

        if let Some(fname) = self.fname {
            params.push(format!("fname={}", encode(fname)));
        }

        if let Some(atk) = self.atk {
            params.push(format!("atk={}", atk));
        }

        if let Some(def) = self.def {
            params.push(format!("def={}", def));
        }

        if let Some(level) = self.level {
            params.push(format!("level={}", level));
        }

        if !self.card_types.is_empty() {
            let joined = self
                .card_types
                .iter()
                .map(|t| format!("{}", t))
                .collect::<Vec<_>>()
                .join(",");
            params.push(format!("type={}", encode(&joined)));
        }

        if !self.races.is_empty() {
            let joined = self
                .races
                .iter()
                .map(|r| format!("{}", r))
                .collect::<Vec<_>>()
                .join(",");
            params.push(format!("race={}", encode(&joined)));
        }

        if !self.attributes.is_empty() {
            let joined = self
                .attributes
                .iter()
                .map(|a| format!("{}", a))
                .collect::<Vec<_>>()
                .join(",");
            params.push(format!("attribute={}", encode(&joined)));
        }

        if let Some(link) = self.link {
            params.push(format!("link={}", link));
        }

        if !self.link_markers.is_empty() {
            let joined = self
                .link_markers
                .iter()
                .map(|m| format!("{}", m))
                .collect::<Vec<_>>()
                .join(",");
            params.push(format!("linkmarker={}", encode(&joined)));
        }

        if let Some(scale) = self.scale {
            params.push(format!("scale={}", scale));
        }

        if let Some(cardset) = self.cardset {
            params.push(format!("cardset={}", encode(cardset)));
        }

        params.join("&")
    }
}

pub struct RequestBuilder<'a> {
    request: Request<'a>,
}

impl<'a> RequestBuilder<'a> {
    pub fn new() -> Self {
        Self {
            request: Request::default(),
        }
    }

    pub fn build(self) -> Request<'a> {
        self.request
    }

    pub fn with_name(mut self, name: &'a str) -> Self {
        self.request.names.push(name);
        self
    }

    pub fn with_fname(mut self, fname: &'a str) -> Self {
        self.request.fname = Some(fname);
        self
    }

    pub fn with_atk(mut self, atk: i32) -> Self {
        self.request.atk = Some(atk);
        self
    }

    pub fn with_def(mut self, def: i32) -> Self {
        self.request.def = Some(def);
        self
    }

    pub fn with_level(mut self, level: u8) -> Self {
        self.request.level = Some(level);
        self
    }

    pub fn with_type(mut self, card_type: CardType) -> Self {
        self.request.card_types.push(card_type);
        self
    }

    pub fn with_race(mut self, race: MonsterRace) -> Self {
        self.request.races.push(race);
        self
    }

    pub fn with_attribute(mut self, attribute: Attribute) -> Self {
        self.request.attributes.push(attribute);
        self
    }

    pub fn with_link(mut self, link: u8) -> Self {
        self.request.link = Some(link);
        self
    }

    pub fn with_link_marker(mut self, link_marker: LinkMarker) -> Self {
        self.request.link_markers.push(link_marker);
        self
    }

    pub fn with_scale(mut self, scale: u8) -> Self {
        self.request.scale = Some(scale);
        self
    }

    pub fn with_cardset(mut self, cardset: &'a str) -> Self {
        self.request.cardset = Some(cardset);
        self
    }
}

#[derive(Debug, Serialize, PartialEq, Eq)]
pub enum CardType {
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
    #[serde(rename = "Spell Card")]
    Spell,
    #[serde(rename = "Trap Card")]
    Trap,
    #[serde(rename = "Skil Card")]
    Skill,
}

impl Display for CardType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CardType::EffectMonster => write!(f, "Effect Monster"),
            CardType::FlipEffectMonster => write!(f, "Flip Effect Monster"),
            CardType::FlipTunerEffectMonster => write!(f, "Flip Tuner Effect Monster"),
            CardType::GeminiMonster => write!(f, "Gemini Monster"),
            CardType::NormalMonster => write!(f, "Normal Monster"),
            CardType::NormalTunerMonster => write!(f, "Normal Tuner Monster"),
            CardType::PendulumEffectMonster => write!(f, "Pendulum Effect Monster"),
            CardType::PendulumEffectRitualMonster => write!(f, "Pendulum Effect Ritual Monster"),
            CardType::PendulumFlipEffectMonster => write!(f, "Pendulum Flip Effect Monster"),
            CardType::PendulumNormalMonster => write!(f, "Pendulum Normal Monster"),
            CardType::PendulumTunerEffectMonster => write!(f, "Pendulum Tuner Effect Monster"),
            CardType::RitualEffectMonster => write!(f, "Ritual Effect Monster"),
            CardType::RitualMonster => write!(f, "Ritual Monster"),
            CardType::SpiritMonster => write!(f, "Spirit Monster"),
            CardType::ToonMonster => write!(f, "Toon Monster"),
            CardType::TunerMonster => write!(f, "Tuner Monster"),
            CardType::UnionEffectMonster => write!(f, "Union Effect Monster"),
            CardType::FusionMonster => write!(f, "Fusion Monster"),
            CardType::LinkMonster => write!(f, "Link Monster"),
            CardType::PendulumEffectFusionMonster => write!(f, "Pendulum Effect Fusion Monster"),
            CardType::SynchroMonster => write!(f, "Synchro Monster"),
            CardType::SynchroPendulumEffectMonster => write!(f, "Synchro Pendulum Effect Monster"),
            CardType::SynchroTunerMonster => write!(f, "Synchro Tuner Monster"),
            CardType::XYZMonster => write!(f, "XYZ Monster"),
            CardType::XYZPendulumEffectMonster => write!(f, "XYZ Pendulum Effect Monster"),
            CardType::Token => write!(f, "Token"),
            CardType::Spell => write!(f, "Spell Card"),
            CardType::Trap => write!(f, "Trap Card"),
            CardType::Skill => write!(f, "Skill Card"),
        }
    }
}
