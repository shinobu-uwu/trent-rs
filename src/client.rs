use std::fmt::Display;

use serde::Deserialize;

use crate::{card::Card, request::Request};

#[derive(Debug)]
pub struct Client {
    client: reqwest::Client,
}

impl Client {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub async fn get<'a>(&self, request: Request<'a>) -> Result<Vec<Card>, Error> {
        let response = self
            .client
            .get(format!(
                "https://db.ygoprodeck.com/api/v7/cardinfo.php?{}",
                request.to_url_params()
            ))
            .send()
            .await
            .map_err(|e| Error::Network(e))?;

        if response.status() == 400 {
            return Err(Error::NotFound);
        }

        let json = response
            .json::<ApiResponse>()
            .await
            .map_err(|_| Error::Deserialization)?;

        Ok(json.data)
    }

    pub async fn get_by_name(&self, name: &str) -> Result<Card, Error> {
        let response = self
            .client
            .get(format!(
                "https://db.ygoprodeck.com/api/v7/cardinfo.php?name={}",
                urlencoding::encode(name),
            ))
            .send()
            .await
            .map_err(|e| Error::Network(e))?;

        if response.status() == 400 {
            return Err(Error::NotFound);
        }

        let json = response.json::<ApiResponse>().await.map_err(|e| {
            dbg!(&e);
            Error::Deserialization
        })?;

        match json.data.into_iter().next() {
            Some(c) => Ok(c),
            None => Err(Error::NotFound),
        }
    }
}

#[derive(Deserialize)]
struct ApiResponse {
    pub data: Vec<Card>,
}

#[derive(Debug)]
pub enum Error {
    Network(reqwest::Error),
    NotFound,
    Serialization,
    Deserialization,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Network(error) => write!(f, "Network error: {error}"),
            Error::NotFound => write!(f, "Card not found"),
            Error::Serialization => write!(f, "Failed to serialize request"),
            Error::Deserialization => write!(f, "Failed to deserialize response payload"),
        }
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        card::{Attribute, CardId, LinkMarker, MonsterRace, MonsterType, SpellRace, TrapRace},
        request::{CardType, RequestBuilder},
    };

    #[tokio::test]
    async fn get() {
        let client = Client::new();
        let request = RequestBuilder::new()
            .with_type(CardType::LinkMonster)
            .with_attribute(Attribute::Wind)
            .with_link_marker(LinkMarker::Top)
            .with_link_marker(LinkMarker::Bottom)
            .with_link_marker(LinkMarker::BottomRight)
            .with_link_marker(LinkMarker::BottomLeft)
            .build();
        let result = client.get(request).await;
        assert!(result.is_ok());
        let cards = result.unwrap();
        assert_eq!(cards.len(), 2);
    }

    #[tokio::test]
    async fn list_all_cards() {
        let client = Client::new();
        let result = client.get(Request::default()).await;
        assert!(result.is_ok());
        let cards = result.unwrap();
        assert!(cards.len() > 200);
    }

    #[tokio::test]
    async fn get_normal_monsters_with_1800_atk() {
        let client = Client::new();
        let request = RequestBuilder::new()
            .with_atk(1800)
            .with_attribute(Attribute::Dark)
            .with_type(CardType::NormalMonster)
            .build();
        let result = client.get(request).await;
        assert!(result.is_ok());
        let cards = result.unwrap();
        assert!(cards.iter().any(|c| matches!(c, Card::Normal(_))));
    }

    #[tokio::test]
    async fn get_normal_monster() {
        let client = Client::new();
        let result = client.get_by_name("Trent").await;
        assert!(result.is_ok());
        let card = result.unwrap();

        match card {
            Card::Normal(m) => {
                assert_eq!(m.info.id, CardId(78780140));
                assert_eq!(m.info.name, "Trent");
                assert_eq!(
                    m.info.desc,
                    "A guardian of the woods, this massive tree is believed to be immortal."
                );
                assert_eq!(m.race, MonsterRace::Plant);
                assert_eq!(m.attribute, Attribute::Earth);
                assert_eq!(m.level, 5);
                assert_eq!(m.atk, 1500);
                assert_eq!(m.def, 1800);
                assert_eq!(m.card_type, MonsterType::NormalMonster);
                assert_eq!(m.info.human_readable_card_type, "Normal Monster");
                assert_eq!(
                    m.info.ygoprodeck_url,
                    "https://ygoprodeck.com/card/trent-6617"
                );
            }
            _ => panic!("Unexpected variant"),
        }
    }

    #[tokio::test]
    async fn get_link_monster() {
        let client = Client::new();
        let result = client.get_by_name("Apollousa, Bow of the Goddess").await;
        assert!(result.is_ok());
        let card = result.unwrap();

        match card {
            Card::Link(m) => {
                assert_eq!(m.info.id, CardId(4280258));
                assert_eq!(m.info.name, "Apollousa, Bow of the Goddess");
                assert_eq!(
                    m.info.desc,
                    "2+ monsters with different names, except Tokens\r\nYou can only control 1 \"Apollousa, Bow of the Goddess\". The original ATK of this card becomes 800 x the number of Link Materials used for its Link Summon. Once per Chain, when your opponent activates a monster effect (Quick Effect): You can make this card lose exactly 800 ATK, and if you do, negate the activation."
                );
                assert_eq!(m.race, MonsterRace::Fairy);
                assert_eq!(m.attribute, Attribute::Wind);
                assert_eq!(m.atk, -1); // ? atk
                assert_eq!(m.linkval, 4);
                assert_eq!(m.card_type, MonsterType::LinkMonster);
                assert_eq!(
                    m.link_markers,
                    vec![
                        LinkMarker::Top,
                        LinkMarker::BottomLeft,
                        LinkMarker::Bottom,
                        LinkMarker::BottomRight
                    ]
                );
                assert_eq!(m.info.human_readable_card_type, "Link Effect Monster");
                assert_eq!(
                    m.info.ygoprodeck_url,
                    "https://ygoprodeck.com/card/apollousa-bow-of-the-goddess-10242"
                );
            }
            _ => panic!("Unexpected variant"),
        }
    }

    #[tokio::test]
    async fn get_effect_monster() {
        let client = Client::new();
        let result = client.get_by_name("Man-eater Bug").await;
        assert!(result.is_ok());
        let card = result.unwrap();

        match card {
            Card::Effect(m) => {
                assert_eq!(m.info.id, CardId(54652250));
                assert_eq!(m.info.name, "Man-Eater Bug");
                assert_eq!(
                    m.info.desc,
                    "FLIP: Target 1 monster on the field; destroy it."
                );
                assert_eq!(m.race, MonsterRace::Insect);
                assert_eq!(m.attribute, Attribute::Earth);
                assert_eq!(m.level, 2);
                assert_eq!(m.atk, 450);
                assert_eq!(m.def, 600);
                assert_eq!(m.card_type, MonsterType::FlipEffectMonster);
                assert_eq!(m.info.human_readable_card_type, "Flip Effect Monster");
                assert_eq!(
                    m.info.ygoprodeck_url,
                    "https://ygoprodeck.com/card/man-eater-bug-4659"
                );
            }
            _ => panic!("Unexpected monster variant"),
        }
    }

    #[tokio::test]
    async fn get_normal_spell() {
        let client = Client::new();
        let result = client.get_by_name("Pot of Greed").await;
        assert!(result.is_ok());
        let card = result.unwrap();

        match card {
            Card::Spell(s) => {
                assert_eq!(s.info.id, CardId(55144522));
                assert_eq!(s.info.name, "Pot of Greed");
                assert_eq!(s.info.desc, "Draw 2 cards.");
                assert_eq!(s.race, SpellRace::Normal);
                assert_eq!(s.info.human_readable_card_type, "Normal Spell");
                assert_eq!(
                    s.info.ygoprodeck_url,
                    "https://ygoprodeck.com/card/pot-of-greed-4698"
                );
            }
            _ => panic!("Unexpected monster variant"),
        }
    }

    #[tokio::test]
    async fn get_normal_trap() {
        let client = Client::new();
        let result = client.get_by_name("Reckless Greed").await;
        assert!(result.is_ok());
        let card = result.unwrap();

        match card {
            Card::Trap(t) => {
                assert_eq!(t.info.id, CardId(37576645));
                assert_eq!(t.info.name, "Reckless Greed");
                assert_eq!(
                    t.info.desc,
                    "Draw 2 cards and skip your next 2 Draw Phases."
                );
                assert_eq!(t.race, TrapRace::Normal);
                assert_eq!(t.info.human_readable_card_type, "Normal Trap");
                assert_eq!(
                    t.info.ygoprodeck_url,
                    "https://ygoprodeck.com/card/reckless-greed-3180"
                );
            }
            _ => panic!("Unexpected monster variant"),
        }
    }

    #[tokio::test]
    async fn get_field_spell() {
        let client = Client::new();
        let result = client.get_by_name("Necrovalley").await;
        assert!(result.is_ok());
        let card = result.unwrap();

        match card {
            Card::Spell(s) => {
                assert_eq!(s.info.id, CardId(47355498));
                assert_eq!(s.info.name, "Necrovalley");
                assert_eq!(s.race, SpellRace::Field);
                assert_eq!(s.info.human_readable_card_type, "Field Spell");
            }
            _ => panic!("Unexpected card variant"),
        }
    }

    #[tokio::test]
    async fn get_equip_spell() {
        let client = Client::new();
        let result = client.get_by_name("Axe of Despair").await;
        assert!(result.is_ok());
        let card = result.unwrap();

        match card {
            Card::Spell(s) => {
                assert_eq!(s.info.id, CardId(40619825));
                assert_eq!(s.info.name, "Axe of Despair");
                assert_eq!(s.race, SpellRace::Equip);
                assert_eq!(s.info.human_readable_card_type, "Equip Spell");
            }
            _ => panic!("Unexpected card variant"),
        }
    }

    #[tokio::test]
    async fn get_continuous_spell() {
        let client = Client::new();
        let result = client.get_by_name("Burning Land").await;
        assert!(result.is_ok());
        let card = result.unwrap();

        match card {
            Card::Spell(s) => {
                assert_eq!(s.info.id, CardId(24294108));
                assert_eq!(s.info.name, "Burning Land");
                assert_eq!(s.race, SpellRace::Continuous);
                assert_eq!(s.info.human_readable_card_type, "Continuous Spell");
            }
            _ => panic!("Unexpected card variant"),
        }
    }

    #[tokio::test]
    async fn get_quick_play_spell() {
        let client = Client::new();
        let result = client.get_by_name("Mystical Space Typhoon").await;
        assert!(result.is_ok());
        let card = result.unwrap();

        match card {
            Card::Spell(s) => {
                assert_eq!(s.info.id, CardId(5318639));
                assert_eq!(s.info.name, "Mystical Space Typhoon");
                assert_eq!(s.race, SpellRace::QuickPlay);
                assert_eq!(s.info.human_readable_card_type, "Quick-Play Spell");
            }
            _ => panic!("Unexpected card variant"),
        }
    }

    #[tokio::test]
    async fn get_ritual_spell() {
        let client = Client::new();
        let result = client.get_by_name("Black Luster Ritual").await;
        assert!(result.is_ok());
        let card = result.unwrap();

        match card {
            Card::Spell(s) => {
                assert_eq!(s.info.id, CardId(55761792));
                assert_eq!(s.info.name, "Black Luster Ritual");
                assert_eq!(s.race, SpellRace::Ritual);
                assert_eq!(s.info.human_readable_card_type, "Ritual Spell");
            }
            _ => panic!("Unexpected card variant"),
        }
    }

    #[tokio::test]
    async fn get_continuous_trap() {
        let client = Client::new();
        let result = client.get_by_name("Call of the Haunted").await;
        assert!(result.is_ok());
        let card = result.unwrap();

        match card {
            Card::Trap(t) => {
                assert_eq!(t.info.id, CardId(97077563));
                assert_eq!(t.info.name, "Call of the Haunted");
                assert_eq!(t.race, TrapRace::Continuous);
                assert_eq!(t.info.human_readable_card_type, "Continuous Trap");
            }
            _ => panic!("Unexpected card variant"),
        }
    }

    #[tokio::test]
    async fn get_counter_trap() {
        let client = Client::new();
        let result = client.get_by_name("Solemn Judgment").await;
        assert!(result.is_ok());
        let card = result.unwrap();

        match card {
            Card::Trap(t) => {
                assert_eq!(t.info.id, CardId(41420027));
                assert_eq!(t.info.name, "Solemn Judgment");
                assert_eq!(t.race, TrapRace::Counter);
                assert_eq!(t.info.human_readable_card_type, "Counter Trap");
            }
            _ => panic!("Unexpected card variant"),
        }
    }

    #[tokio::test]
    async fn get_card_not_found() {
        let client = Client::new();
        match client.get_by_name("Trnet").await {
            Ok(_) => panic!("Expected error, but got card"),
            Err(e) => assert!(matches!(e, Error::NotFound)),
        }
    }
}
