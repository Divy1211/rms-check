use chumsky::prelude::*;

use crate::parsing::ast::Identifier;
use crate::parsing::lexer::token::Token;
use crate::parsing::Span;

pub fn keyword<'src>() -> impl Parser<
    'src, &'src str, Token, extra::Err<Rich<'src, char, Span>>
> {
    let ident = one_of("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_")
        .repeated()
        .at_least(1)
        .collect::<String>();
    just("#").or_not().ignore_then(ident).map(|ident| match ident.as_str() {
        "const"                               => Token::Const,
        "define"                              => Token::Define,
        "undefine"                            => Token::Undefine,

        "if"                                  => Token::If,
        "elseif"                              => Token::ElseIf,
        "else"                                => Token::Else,
        "endif"                               => Token::EndIf,

        "start_random"                        => Token::StartRandom,
        "percent_chance"                      => Token::PercentChance,
        "end_random"                          => Token::EndRandom,

        "rnd"                                 => Token::Rnd,

        "include_drs"                         => Token::IncludeDrs,
        "include_xs"                          => Token::IncludeXs,

        /* Player Setup */
        "random_placement"
        | "direct_placement"
        | "grouped_by_team"
        | "nomad_resources"
        | "force_nomad_treaty"
        | "behavior_version"
        | "override_map_size"
        | "set_gaia_civilization"
        | "ai_info_map_type"
        | "effect_amount"
        | "effect_percent"
        | "guard_state"
        | "terrain_state"
        | "weather_type"
        | "water_definition"

        /* Land Generation */
        | "base_terrain"
        | "base_layer"
        | "enable_waves"
        | "create_player_lands"
        | "create_land"

        | "terrain_type"
        | "land_percent"
        | "number_of_tiles"
        | "base_size"
        | "set_circular_base"
        | "generate_mode"
        | "land_position"
        | "circle_radius"
        | "left_border"
        | "right_border"
        | "top_border"
        | "bottom_border"
        | "border_fuzziness"
        | "clumping_factor"
        | "land_conformity"
        | "base_elevation"
        | "assign_to_player"
        | "assign_to"
        | "zone"
        | "set_zone_by_team"
        | "set_zone_randomly"
        | "other_zone_avoidance_distance"
        | "min_placement_distance"
        | "land_id"

        /* Elevation Generation */
        | "create_elevation"

        | "number_of_clumps"
        | "set_scale_by_size"
        | "set_scale_by_groups"
        | "spacing"
        | "enable_balanced_elevation"
        | "cliff_type"
        | "min_number_of_cliffs"
        | "max_number_of_cliffs"
        | "min_length_of_cliff"
        | "max_length_of_cliff"
        | "cliff_curliness"
        | "min_distance_cliffs"
        | "min_terrain_distance"

        /* Terrain Generation */
        | "color_correction"
        | "create_terrain"

        | "beach_terrain"
        | "terrain_mask"
        | "spacing_to_other_terrain_types"
        | "spacing_to_specific_terrain"
        | "set_flat_terrain_only"
        | "set_avoid_player_start_areas"
        | "height_limits"

        /* Connection Generation */
        | "accumulate_connections"

        | "create_connect_all_players_land"
        | "create_connect_teams_lands"
        | "create_connect_all_lands"
        | "create_connect_same_land_zones"
        | "create_connect_land_zones"
        | "create_connect_to_nonplayer_land"

        | "default_terrain_replacement"
        | "replace_terrain"
        | "terrain_cost"
        | "terrain_size"

        /* Object Generation */
        | "create_object"
        | "create_object_group"
        | "create_actor_area"
        | "add_object"

        | "number_of_objects"
        | "number_of_groups"
        | "group_variance"
        | "group_placement_radius"
        | "set_tight_grouping"
        | "set_loose_grouping"
        | "min_connected_tiles"
        | "resource_delta"
        | "second_object"
        | "set_scaling_to_map_size"
        | "set_scaling_to_player_number"
        | "set_place_for_every_player"
        | "place_on_specific_land_id"
        | "avoid_other_land_zones"
        | "generate_for_first_land_only"
        | "set_gaia_object_only"
        | "set_gaia_unconvertible"
        | "set_building_capturable"
        | "make_indestructible"
        | "min_distance_to_players"
        | "max_distance_to_players"
        | "set_circular_placement"
        | "terrain_to_place_on"
        | "layer_to_place_on"
        | "ignore_terrain_restrictions"
        | "max_distance_to_other_zones"
        | "place_on_forest_zone"
        | "avoid_forest_zone"
        | "avoid_cliff_zone"
        | "min_distance_to_map_edge"
        | "min_distance_group_placement"
        | "temp_min_distance_group_placement"
        | "find_closest"
        | "find_closest_to_map_center"
        | "find_closest_to_map_edge"
        | "enable_tile_shuffling"
        | "require_path"
        | "force_placement"
        | "actor_area"
        | "actor_area_radius"
        | "override_actor_radius_if_required"
        | "actor_area_to_place_in"
        | "avoid_actor_area"
        | "avoid_all_actor_areas"
        | "set_facet"
        | "match_player_civ"                  => Token::Command(Identifier(ident)),

        _                                     => Token::Identifier(Identifier(ident))
    })
}
