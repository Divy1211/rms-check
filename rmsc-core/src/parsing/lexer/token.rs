use std::fmt::{Display, Formatter};

use crate::parsing::ast::{Identifier, Literal};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Token {
    Error,
    
    Plus,
    Minus,
    Star,
    FSlash,
    PCent,
    DPlus,
    DMinus,
    Lt,
    Gt,
    Le,
    Ge,
    Deq,
    Neq,
    DAmp,
    DPipe,
    Excl,

    BSlash,
    Eq,
    LBrace,
    RBrace,
    LParen,
    RParen,
    SColon,
    Colon,
    Comma,
    Dot,
    
    Literal(Literal),
    Identifier(Identifier),
    
    Comment(String),

    Const,
    Define,
    Undefine,

    If,
    ElseIf,
    Else,
    EndIf,

    StartRandom,
    PercentChance,
    EndRandom,

    Rnd,

    IncludeDrs,
    IncludeXs,

    Command(Identifier),

    // /* Player Setup */
    //
    // RandomPlacement,
    // DirectPlacement,
    // GroupedByTeam,
    // NomadResources,
    // ForceNomadTreaty,
    // BehaviorVersion,
    // OverrideMapSize,
    // SetGaiaCivilization,
    // AiInfoMapType,
    // EffectAmount,
    // EffectPercent,
    // GuardState,
    // TerrainState,
    // WeatherType,
    // WaterDefinition,
    //
    // /* Land Generation */
    //
    // BaseTerrain,
    // BaseLayer,
    // EnableWaves,
    // CreatePlayerLands,
    // CreateLand,
    //
    // TerrainType,
    // LandPercent,
    // NumberOfTiles,
    // BaseSize,
    // SetCircularBase,
    // GenerateMode,
    // LandPosition,
    // CircleRadius,
    // LeftBorder,
    // RightBorder,
    // TopBorder,
    // BottomBorder,
    // BorderFuzziness,
    // ClumpingFactor,
    // LandConformity,
    // BaseElevation,
    // AssignToPlayer,
    // AssignTo,
    // Zone,
    // SetZoneByTeam,
    // SetZoneRandomly,
    // OtherZoneAvoidanceDistance,
    // MinPlacementDistance,
    // LandId,
    //
    //
    // /* Elevation Generation */
    //
    // CreateElevation,
    //
    // // BaseTerrain,
    // // BaseLayer,
    // // NumberOfTiles,
    // NumberOfClumps,
    // SetScaleBySize,
    // SetScaleByGroups,
    // Spacing,
    // EnableBalancedElevation,
    //
    // /* Cliff Generation */
    //
    // CliffType,
    // MinNumberOfCliffs,
    // MaxNumberOfCliffs,
    // MinLengthOfCliff,
    // MaxLengthOfCliff,
    // CliffCurliness,
    // MinDistanceCliffs,
    // MinTerrainDistance,
    //
    // /* Terrain Generation */
    //
    // ColorCorrection,
    // CreateTerrain,
    //
    // // BaseTerrain,
    // // BaseLayer,
    // BeachTerrain,
    // TerrainMask,
    // SpacingToOtherTerrainTypes,
    // SpacingToSpecificTerrain,
    // SetFlatTerrainOnly,
    // // LandPercent,
    // // NumberOfTiles,
    // // NumberOfClumps,
    // // ClumpingFactor,
    // // SetScaleBySize,
    // // SetScaleByGroups,
    // SetAvoidPlayerStartAreas,
    // HeightLimits,
    //
    // /* Connection Generation */
    //
    // AccumulateConnections,
    //
    // CreateConnectAllPlayersLand,
    // CreateConnectTeamsLands,
    // CreateConnectAllLands,
    // CreateConnectSameLandZones,
    // CreateConnectLandZones,
    // CreateConnectToNonplayerLand,
    //
    // DefaultTerrainReplacement,
    // ReplaceTerrain,
    // TerrainCost,
    // TerrainSize,
    //
    // /* Object Generation */
    //
    // CreateObject,
    // CreateObjectGroup,
    // CreateActorArea,
    // AddObject,
    //
    // NumberOfObjects,
    // NumberOfGroups,
    // GroupVariance,
    // GroupPlacementRadius,
    // SetTightGrouping,
    // SetLooseGrouping,
    // MinConnectedTiles,
    // ResourceDelta,
    // SecondObject,
    // SetScalingToMapSize,
    // SetScalingToPlayerNumber,
    // SetPlaceForEveryPlayer,
    // PlaceOnSpecificLandId,
    // AvoidOtherLandZones,
    // GenerateForFirstLandOnly,
    // SetGaiaObjectOnly,
    // SetGaiaUnconvertible,
    // SetBuildingCapturable,
    // MakeIndestructible,
    // MinDistanceToPlayers,
    // MaxDistanceToPlayers,
    // SetCircularPlacement,
    // TerrainToPlaceOn,
    // LayerToPlaceOn,
    // IgnoreTerrainRestrictions,
    // MaxDistanceToOtherZones,
    // PlaceOnForestZone,
    // AvoidForestZone,
    // AvoidCliffZone,
    // MinDistanceToMapEdge,
    // MinDistanceGroupPlacement,
    // TempMinDistanceGroupPlacement,
    // FindClosest,
    // FindClosestToMapCenter,
    // FindClosestToMapEdge,
    // EnableTileShuffling,
    // RequirePath,
    // ForcePlacement,
    // ActorArea,
    // ActorAreaRadius,
    // OverrideActorRadiusIfRequired,
    // ActorAreaToPlaceIn,
    // AvoidActorArea,
    // AvoidAllActorAreas,
    // SetFacet,
    // MatchPlayerCiv,
}

impl Token {
    pub fn is_comment(&self) -> bool {
        matches!(self, Token::Comment(_))
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Error => { write!(f, "ERROR") }
            
            Token::Plus => { write!(f, "+") }
            Token::Minus => { write!(f, "-") }
            Token::Star => { write!(f, "*") }
            Token::FSlash => { write!(f, "/") }
            Token::PCent => { write!(f, "%") }
            Token::DPlus => { write!(f, "++") }
            Token::DMinus => { write!(f, "--") }
            Token::Lt => { write!(f, "<") }
            Token::Gt => { write!(f, ">") }
            Token::Le => { write!(f, "<=") }
            Token::Ge => { write!(f, ">=") }
            Token::Deq => { write!(f, "==") }
            Token::Neq => { write!(f, "!=") }
            Token::DAmp => { write!(f, "&&") }
            Token::DPipe => { write!(f, "||") }
            Token::Excl => { write!(f, "!") }

            Token::BSlash => { write!(f, "\\") }
            Token::Eq => { write!(f, "=") }
            Token::LBrace => { write!(f, "{{") }
            Token::RBrace => { write!(f, "}}") }
            Token::LParen => { write!(f, "(") }
            Token::RParen => { write!(f, ")") }
            Token::SColon => { write!(f, ";") }
            Token::Colon => { write!(f, ":") }
            Token::Comma => { write!(f, ",") }
            Token::Dot => { write!(f, ".") }
            
            Token::Literal(lit) => { write!(f, "{}", lit) }
            Token::Identifier(id) => { write!(f, "{}", id) }
            Token::Command(id) => { write!(f, "{}", id) }

            Token::Comment(_) => { write!(f, "comment") }

            Token::Const => write!(f, "#const"),
            Token::Define => write!(f, "#define"),
            Token::Undefine => write!(f, "#undefine"),

            Token::If => write!(f, "if"),
            Token::ElseIf => write!(f, "elseif"),
            Token::Else => write!(f, "else"),
            Token::EndIf => write!(f, "endif"),

            Token::StartRandom => write!(f, "start_random"),
            Token::PercentChance => write!(f, "percent_chance"),
            Token::EndRandom => write!(f, "end_random"),

            Token::Rnd => write!(f, "rnd"),

            Token::IncludeDrs => write!(f, "#include_drs"),
            Token::IncludeXs => write!(f, "#includeXS"),

            // /* Player Setup */
            // Token::RandomPlacement => write!(f, "random_placement"),
            // Token::DirectPlacement => write!(f, "direct_placement"),
            // Token::GroupedByTeam => write!(f, "grouped_by_team"),
            // Token::NomadResources => write!(f, "nomad_resources"),
            // Token::ForceNomadTreaty => write!(f, "force_nomad_treaty"),
            // Token::BehaviorVersion => write!(f, "behavior_version"),
            // Token::OverrideMapSize => write!(f, "override_map_size"),
            // Token::SetGaiaCivilization => write!(f, "set_gaia_civilization"),
            // Token::AiInfoMapType => write!(f, "ai_info_map_type"),
            // Token::EffectAmount => write!(f, "effect_amount"),
            // Token::EffectPercent => write!(f, "effect_percent"),
            // Token::GuardState => write!(f, "guard_state"),
            // Token::TerrainState => write!(f, "terrain_state"),
            // Token::WeatherType => write!(f, "weather_type"),
            // Token::WaterDefinition => write!(f, "water_definition"),
            //
            // /* Land Generation */
            // Token::BaseTerrain => write!(f, "base_terrain"),
            // Token::BaseLayer => write!(f, "base_layer"),
            // Token::EnableWaves => write!(f, "enable_waves"),
            // Token::CreatePlayerLands => write!(f, "create_player_lands"),
            // Token::CreateLands => write!(f, "create_lands"),
            //
            // Token::TerrainType => write!(f, "terrain_type"),
            // Token::LandPercent => write!(f, "land_percent"),
            // Token::NumberOfTiles => write!(f, "number_of_tiles"),
            // Token::BaseSize => write!(f, "base_size"),
            // Token::SetCircularBase => write!(f, "set_circular_base"),
            // Token::GenerateMode => write!(f, "generate_mode"),
            // Token::LandPosition => write!(f, "land_position"),
            // Token::CircleRadius => write!(f, "circle_radius"),
            // Token::LeftBorder => write!(f, "left_border"),
            // Token::RightBorder => write!(f, "right_border"),
            // Token::TopBorder => write!(f, "top_border"),
            // Token::BottomBorder => write!(f, "bottom_border"),
            // Token::BorderFuzziness => write!(f, "border_fuzziness"),
            // Token::ClumpingFactor => write!(f, "clumping_factor"),
            // Token::LandConformity => write!(f, "land_conformity"),
            // Token::BaseElevation => write!(f, "base_elevation"),
            // Token::AssignToPlayer => write!(f, "assign_to_player"),
            // Token::AssignTo => write!(f, "assign_to"),
            // Token::Zone => write!(f, "zone"),
            // Token::SetZoneByTeam => write!(f, "set_zone_by_team"),
            // Token::SetZoneRandomly => write!(f, "set_zone_randomly"),
            // Token::OtherZoneAvoidanceDistance => write!(f, "other_zone_avoidance_distance"),
            // Token::MinPlacementDistance => write!(f, "min_placement_distance"),
            // Token::LandId => write!(f, "land_id"),
            //
            // /* Elevation Generation */
            // Token::CreateElevation => write!(f, "create_elevation"),
            //
            // Token::NumberOfClumps => write!(f, "number_of_clumps"),
            // Token::SetScaleBySize => write!(f, "set_scale_by_size"),
            // Token::SetScaleByGroups => write!(f, "set_scale_by_groups"),
            // Token::Spacing => write!(f, "spacing"),
            // Token::EnableBalancedElevation => write!(f, "enable_balanced_elevation"),
            //
            // /* Cliff Generation */
            // Token::CliffType => write!(f, "cliff_type"),
            // Token::MinNumberOfCliffs => write!(f, "min_number_of_cliffs"),
            // Token::MaxNumberOfCliffs => write!(f, "max_number_of_cliffs"),
            // Token::MinLengthOfCliff => write!(f, "min_length_of_cliff"),
            // Token::MaxLengthOfCliff => write!(f, "max_length_of_cliff"),
            // Token::CliffCurliness => write!(f, "cliff_curliness"),
            // Token::MinDistanceCliffs => write!(f, "min_distance_cliffs"),
            // Token::MinTerrainDistance => write!(f, "min_terrain_distance"),
            //
            // /* Terrain Generation */
            // Token::ColorCorrection => write!(f, "color_correction"),
            // Token::CreateTerrain => write!(f, "create_terrain"),
            //
            // Token::BeachTerrain => write!(f, "beach_terrain"),
            // Token::TerrainMask => write!(f, "terrain_mask"),
            // Token::SpacingToOtherTerrainTypes => write!(f, "spacing_to_other_terrain_types"),
            // Token::SpacingToSpecificTerrain => write!(f, "spacing_to_specific_terrain"),
            // Token::SetFlatTerrainOnly => write!(f, "set_flat_terrain_only"),
            // Token::SetAvoidPlayerStartAreas => write!(f, "set_avoid_player_start_areas"),
            // Token::HeightLimits => write!(f, "height_limits"),
            //
            // /* Connection Generation */
            // Token::AccumulateConnections => write!(f, "accumulate_connections"),
            //
            // Token::CreateConnectAllPlayersLand => write!(f, "create_connect_all_players_land"),
            // Token::CreateConnectTeamsLands => write!(f, "create_connect_teams_lands"),
            // Token::CreateConnectAllLands => write!(f, "create_connect_all_lands"),
            // Token::CreateConnectSameLandZones => write!(f, "create_connect_same_land_zones"),
            // Token::CreateConnectLandZones => write!(f, "create_connect_land_zones"),
            // Token::CreateConnectToNonplayerLand => write!(f, "create_connect_to_nonplayer_land"),
            //
            // Token::DefaultTerrainReplacement => write!(f, "default_terrain_replacement"),
            // Token::ReplaceTerrain => write!(f, "replace_terrain"),
            // Token::TerrainCost => write!(f, "terrain_cost"),
            // Token::TerrainSize => write!(f, "terrain_size"),
            //
            // /* Object Generation */
            // Token::CreateObject => write!(f, "create_object"),
            // Token::CreateObjectGroup => write!(f, "create_object_group"),
            // Token::CreateActorArea => write!(f, "create_actor_area"),
            // Token::AddObject => write!(f, "add_object"),
            //
            // Token::NumberOfObjects => write!(f, "number_of_objects"),
            // Token::NumberOfGroups => write!(f, "number_of_groups"),
            // Token::GroupVariance => write!(f, "group_variance"),
            // Token::GroupPlacementRadius => write!(f, "group_placement_radius"),
            // Token::SetTightGrouping => write!(f, "set_tight_grouping"),
            // Token::SetLooseGrouping => write!(f, "set_loose_grouping"),
            // Token::MinConnectedTiles => write!(f, "min_connected_tiles"),
            // Token::ResourceDelta => write!(f, "resource_delta"),
            // Token::SecondObject => write!(f, "second_object"),
            // Token::SetScalingToMapSize => write!(f, "set_scaling_to_map_size"),
            // Token::SetScalingToPlayerNumber => write!(f, "set_scaling_to_player_number"),
            // Token::SetPlaceForEveryPlayer => write!(f, "set_place_for_every_player"),
            // Token::PlaceOnSpecificLandId => write!(f, "place_on_specific_land_id"),
            // Token::AvoidOtherLandZones => write!(f, "avoid_other_land_zones"),
            // Token::GenerateForFirstLandOnly => write!(f, "generate_for_first_land_only"),
            // Token::SetGaiaObjectOnly => write!(f, "set_gaia_object_only"),
            // Token::SetGaiaUnconvertible => write!(f, "set_gaia_unconvertible"),
            // Token::SetBuildingCapturable => write!(f, "set_building_capturable"),
            // Token::MakeIndestructible => write!(f, "make_indestructible"),
            // Token::MinDistanceToPlayers => write!(f, "min_distance_to_players"),
            // Token::MaxDistanceToPlayers => write!(f, "max_distance_to_players"),
            // Token::SetCircularPlacement => write!(f, "set_circular_placement"),
            // Token::TerrainToPlaceOn => write!(f, "terrain_to_place_on"),
            // Token::LayerToPlaceOn => write!(f, "layer_to_place_on"),
            // Token::IgnoreTerrainRestrictions => write!(f, "ignore_terrain_restrictions"),
            // Token::MaxDistanceToOtherZones => write!(f, "max_distance_to_other_zones"),
            // Token::PlaceOnForestZone => write!(f, "place_on_forest_zone"),
            // Token::AvoidForestZone => write!(f, "avoid_forest_zone"),
            // Token::AvoidCliffZone => write!(f, "avoid_cliff_zone"),
            // Token::MinDistanceToMapEdge => write!(f, "min_distance_to_map_edge"),
            // Token::MinDistanceGroupPlacement => write!(f, "min_distance_group_placement"),
            // Token::TempMinDistanceGroupPlacement => write!(f, "temp_min_distance_group_placement"),
            // Token::FindClosest => write!(f, "find_closest"),
            // Token::FindClosestToMapCenter => write!(f, "find_closest_to_map_center"),
            // Token::FindClosestToMapEdge => write!(f, "find_closest_to_map_edge"),
            // Token::EnableTileShuffling => write!(f, "enable_tile_shuffling"),
            // Token::RequirePath => write!(f, "require_path"),
            // Token::ForcePlacement => write!(f, "force_placement"),
            // Token::ActorArea => write!(f, "actor_area"),
            // Token::ActorAreaRadius => write!(f, "actor_area_radius"),
            // Token::OverrideActorRadiusIfRequired => write!(f, "override_actor_radius_if_required"),
            // Token::ActorAreaToPlaceIn => write!(f, "actor_area_to_place_in"),
            // Token::AvoidActorArea => write!(f, "avoid_actor_area"),
            // Token::AvoidAllActorAreas => write!(f, "avoid_all_actor_areas"),
            // Token::SetFacet => write!(f, "set_facet"),
            // Token::MatchPlayerCiv => write!(f, "match_player_civ"),
        }
    }
}