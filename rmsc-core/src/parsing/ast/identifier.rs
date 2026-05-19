use std::fmt::{Display, Formatter};
use std::hash::{Hash};
use std::sync::LazyLock;
use regex::Regex;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Identifier(pub String);

static PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?:\d+_PLAYER_GAME|\d+_TEAM_GAME|PLAYER\d+_TEAM\d+|TEAM\d+_SIZE\d+)$").unwrap()
});

impl Identifier {
    pub fn new(name: &str) -> Self {
        Identifier(name.to_string())
    }

    pub fn is_default_name(&self) -> bool {
        match self.0.as_str() {
            /* game modes */
            "DEATH_MATCH"             => true,
            "REGICIDE"                => true,
            "CAPTURE_THE_RELIC"       => true,
            "CAPTURE_RELICS"          => true, // up-only, not available on DE
            "RANDOM_MAP"              => true,
            "TURBO_RANDOM_MAP"        => true,
            "KING_OT_HILL"            => true,
            "WONDER_RACE"             => true,
            "DEFEND_WONDER"           => true,
            "EMPIRE_WARS"             => true,
            "BATTLE_ROYALE"           => true,
            "SUDDEN_DEATH"            => true,

            /* legacy map sizes */
            "TINY_MAP"                => true,
            "SMALL_MAP"               => true,
            "MEDIUM_MAP"              => true,
            "LARGE_MAP"               => true,
            "HUGE_MAP"                => true,
            "GIGANTIC_MAP"            => true,
            "LUDIKRIS_MAP"            => true,

            /* map sizes */
            "MAPSIZE_MINI"            => true,
            "MAPSIZE_TINY"            => true,
            "MAPSIZE_SMALL"           => true,
            "MAPSIZE_MEDIUM"          => true,
            "MAPSIZE_NORMAL"          => true,
            "MAPSIZE_LARGE"           => true,
            "MAPSIZE_HUGE"            => true,
            "MAPSIZE_GIANT"           => true,
            "MAPSIZE_MASSIVE"         => true,
            "MAPSIZE_ENORMOUS"        => true,
            "MAPSIZE_COLOSSAL"        => true,
            "MAPSIZE_INCREDIBLE"      => true,
            "MAPSIZE_MONSTROUS"       => true,
            "MAPSIZE_LUDICROUS"       => true,

            /* starting resources */
            "HIGH_RESOURCES"          => true,
            "MEDIUM_RESOURCES"        => true,
            "LOW_RESOURCES"           => true,
            "DEFAULT_RESOURCES"       => true,
            "INFINITE_RESOURCES"      => true,
            "RANDOM_RESOURCES"        => true,

            /* age start */
            "DARK_AGE_START"          => true,
            "FEUDAL_AGE_START"        => true,
            "CASTLE_AGE_START"        => true,
            "IMPERIAL_AGE_START"      => true,
            "POST_IMPERIAL_AGE_START" => true,

            /* lobby settings */
            "FIXED_POSITIONS"         => true,
            "TURBO_MODE"              => true,
            "TEAM_POSITIONS"          => true,
            "FULL_TECH_TREE"          => true,
            "AI_PLAYERS"              => true,
            "SOLID_FARMS"             => true,
            "ANTIQUITY_MODE"          => true,

            "UP_AVAILABLE"            => true,
            "UP_EXTENSION"            => true,
            "DE_AVAILABLE"            => true,
            "DE_GAME_ROME"            => true,
            "DE_GAME_AGE2"            => true,

            // "%d_PLAYER_GAME"          => true,
            // "%d_TEAM_GAME"            => true,
            // "PLAYER%d_TEAM%d"         => true,
            // "TEAM%d_SIZE%d"           => true,
            name                 => PATTERN.is_match(name),
        }
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for Identifier {
    fn from(name: &str) -> Self {
        Identifier(name.to_string())
    }
}