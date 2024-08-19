use serde::{Deserialize, Serialize};

/// The finish the card can come in.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(not(feature = "unknown_variants"), derive(Copy))]
#[cfg_attr(
    all(
        not(feature = "unknown_variants"),
        not(feature = "unknown_variants_slim")
    ),
    non_exhaustive
)]
#[cfg_attr(test, serde(deny_unknown_fields))]
#[serde(rename_all = "lowercase")]
#[allow(missing_docs)]
pub enum PromoType {
    Alchemy,
    Arenaleague,
    Boosterfun,
    Boxtopper,
    Brawldeck,
    Bringafriend,
    Bundle,
    Buyabox,
    Commanderparty,
    Concept,
    Confettifoil,
    Convention,
    Datestamped,
    Dossier,
    Doubleexposure,
    Doublerainbow,
    Draculaseries,
    Draftweekend,
    Duels,
    Embossed,
    Event,
    Fnm,
    Fracturefoil,
    Galaxyfoil,
    Gameday,
    Giftbox,
    Gilded,
    Glossy,
    Godzillaseries,
    Halofoil,
    Imagine,
    Instore,
    Intropack,
    Invisibleink,
    Jpwalker,
    Judgegift,
    League,
    Magnified,
    Mediainsert,
    Moonlitland,
    Neonink,
    Oilslick,
    Openhouse,
    Planeswalkerdeck,
    Plastic,
    Playerrewards,
    Playpromo,
    Playtest,
    Portrait,
    Poster,
    Premiereshop,
    Prerelease,
    Promopack,
    Rainbowfoil,
    Raisedfoil,
    Ravnicacity,
    Rebalanced,
    Release,
    Ripplefoil,
    Schinesealtart,
    Scroll,
    Serialized,
    Setextension,
    Setpromo,
    Silverfoil,
    Stamped,
    Starterdeck,
    Stepandcompleat,
    Storechampionship,
    Surgefoil,
    Textured,
    Themepack,
    Thick,
    Tourney,
    UpsideDown,
    UpsideDownBack,
    Vault,
    Wizardsplaynetwork,
    #[cfg(feature = "unknown_variants")]
    #[serde(untagged)]
    /// Unknown variant
    Unknown(Box<str>),
    #[cfg(all(not(feature = "unknown_variants"), feature = "unknown_variants_slim"))]
    #[serde(other)]
    Unknown,
}