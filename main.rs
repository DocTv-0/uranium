use valence_nbt::{compound, to_binary, List};
use std::collections::HashMap;

macro_rules! hashmap {
    () => {
        HashMap::new()
    };
    ( $( $key:expr => $value:expr ),* $(,)? ) => {
        {
            let mut _map = HashMap::with_capacity(
                <[()]>::len(&[ $( hashmap!(@replace $key) ),* ])
            );
            $(
                _map.insert($key, $value);
            )*
            _map
        }
    };

    (@replace $x:expr) => { () };
}

macro_rules! nbt {
    () => {
        Vec::new()
    };

    ( $( $key:expr => $value:expr ),* $(,)? ) => {
        {
            let compound = compound! {
                $( $key => $value ),*
            };
            let mut bytes = Vec::new();
            to_binary(&compound, &mut bytes, "").unwrap();
            bytes.drain(1..3);
            bytes
        }
    };

    (@replace $x:expr) => { () };
}

macro_rules! damage_type {
    ($message_id:expr, $exhaustion:expr, $scaling:expr) => {
        nbt! {
            "message_id" => $message_id,
            "exhaustion" => $exhaustion,
            "scaling" => $scaling
        }
    };
    ($message_id:expr, $exhaustion:expr, $scaling:expr, $effects:expr) => {
        nbt! {
            "message_id" => $message_id,
            "exhaustion" => $exhaustion,
            "scaling" => $scaling,
            "effects" => $effects
        }
    };
    ($message_id:expr, $exhaustion:expr, $scaling:expr, $effects:expr, $death_message_type:expr) => {
        nbt! {
            "message_id" => $message_id,
            "exhaustion" => $exhaustion,
            "scaling" => $scaling,
            "effects" => $effects,
            "death_message_type" => $death_message_type
        }
    };
}

macro_rules! damage_type_with_death {
    ($message_id:expr, $exhaustion:expr, $scaling:expr, $death_message_type:expr) => {
        nbt! {
            "message_id" => $message_id,
            "exhaustion" => $exhaustion,
            "scaling" => $scaling,
            "death_message_type" => $death_message_type
        }
    };
}

macro_rules! biome {
    () => {
        nbt! {
            "has_precipitation" => true,
            "temperature" => 0.8_f32,
            "downfall" => 0.4_f32,
            "effects" => compound! {
                "sky_color" => 7907327,
                "water_fog_color" => 329011,
                "fog_color" => 12638463,
                "water_color" => 4159204,
                "mood_sound" => compound! {
                    "sound" => "minecraft:ambient.cave",
                    "tick_delay" => 6000,
                    "block_search_extent" => 8,
                    "offset" => 2.0_f64
                }
            },
            "carvers" => List::List(Vec::new()),
            "features" => List::List(Vec::new()),
            "spawners" => compound! {},
            "spawn_costs" => compound! {}
        }
    };
}

macro_rules! trim_pattern {
    ($trim_name:expr) => {
        nbt! {
            "asset_id" => format!("minecraft:{}", $trim_name),
            "description" => compound! {"translate" => format!("trim_pattern.minecraft.{}", $trim_name)},
            "decal" => 0x00
        }
    };
}

macro_rules! trim_material {
    ($material_name:expr, $item_model_index:expr) => {
        nbt! {
            "asset_name" => $material_name,
            "item_model_index" => $item_model_index,
            "description" => compound! {"translate" => format!("trim_material.minecraft.{}", $material_name)},
        }
    };
}

macro_rules! wolf_variant {
    ($name:expr, $biome:expr) => {
        nbt! {
            "assets" => compound! {
                "wild" => format!("minecraft:entity/wolf/{}", $name),
                "tame" => format!("minecraft:entity/wolf/{}_tamed", $name),
                "angry" => format!("minecraft:entity/wolf/{}_angry", $name)
            },
            "biomes" => $biome
        }
    };
}

macro_rules! wolf_sound_variant {
    ($name:expr) => {
        nbt! {
            "adult_sounds" => compound! {
                "ambient_sound" => format!("minecraft:entity.{}.ambient", $name),
                "death_sound" => format!("minecraft:entity.{}.death", $name),
                "growl_sound" => format!("minecraft:entity.{}.growl", $name),
                "hurt_sound" => format!("minecraft:entity.{}.hurt", $name),
                "pant_sound" => format!("minecraft:entity.{}.pant", $name),
                "whine_sound" => format!("minecraft:entity.{}.whine", $name),
            },
            "baby_sounds" => compound! {
                "ambient_sound" => "minecraft:entity.wolf.ambient",
                "death_sound" => "minecraft:entity.wolf.death",
                "growl_sound" => "minecraft:entity.wolf.growl",
                "hurt_sound" => "minecraft:entity.wolf.hurt",
                "pant_sound" => "minecraft:entity.wolf.pant",
                "whine_sound" => "minecraft:entity.wolf.whine"
            }
        }
    };
}

macro_rules! pig_variant {
    ($name:expr, $model:expr) => {
        nbt! {
            "model" => $model,
            "asset_id" => format!("minecraft:pig/{}", $name),
        }
    };
}

macro_rules! frog_variant {
    ($name:expr) => {
        nbt! {
            "asset_id" => format!("minecraft:{}", $name),
        }
    };
}

pub fn get_configuration_data() -> HashMap<&'static str, HashMap<&'static str, Vec<u8>>> {
    hashmap! {
        "minecraft:banner_pattern" => hashmap! {
            "minecraft:base" => nbt! {
                "asset_id" => "minecraft:base",
                "translation_key" => "block.minecraft.banner.base"
            },
            "minecraft:border" => nbt! {
                "asset_id" => "minecraft:border",
                "translation_key" => "block.minecraft.banner.border"
            },
            "minecraft:bricks" => nbt! {
                "asset_id" => "minecraft:bricks",
                "translation_key" => "block.minecraft.banner.brick"
            },
            "minecraft:circle" => nbt! {
                "asset_id" => "minecraft:circle",
                "translation_key" => "block.minecraft.banner.circle"
            },
            "minecraft:creeper" => nbt! {
                "asset_id" => "minecraft:creeper",
                "translation_key" => "block.minecraft.banner.creeper"
            },
            "minecraft:cross" => nbt! {
                "asset_id" => "minecraft:cross",
                "translation_key" => "block.minecraft.banner.cross"
            },
            "minecraft:curly_border" => nbt! {
                "asset_id" => "minecraft:curly_border",
                "translation_key" => "block.minecraft.banner.curly_border"
            },
            "minecraft:diagonal_left" => nbt! {
                "asset_id" => "minecraft:diagonal_left",
                "translation_key" => "block.minecraft.banner.diagonal_left"
            },
            "minecraft:diagonal_right" => nbt! {
                "asset_id" => "minecraft:diagonal_right",
                "translation_key" => "block.minecraft.banner.diagonal_right"
            },
            "minecraft:diagonal_up_left" => nbt! {
                "asset_id" => "minecraft:diagonal_up_left",
                "translation_key" => "block.minecraft.banner.diagonal_up_left"
            },
            "minecraft:diagonal_up_right" => nbt! {
                "asset_id" => "minecraft:diagonal_up_right",
                "translation_key" => "block.minecraft.banner.diagonal_up_right"
            },
            "minecraft:flow" => nbt! {
                "asset_id" => "minecraft:flow",
                "translation_key" => "block.minecraft.banner.flow"
            },
            "minecraft:flower" => nbt! {
                "asset_id" => "minecraft:flower",
                "translation_key" => "block.minecraft.banner.flower"
            },
            "minecraft:globe" => nbt! {
                "asset_id" => "minecraft:globe",
                "translation_key" => "block.minecraft.banner.globe"
            },
            "minecraft:gradient" => nbt! {
                "asset_id" => "minecraft:gradient",
                "translation_key" => "block.minecraft.banner.gradient"
            },
            "minecraft:gradient_up" => nbt! {
                "asset_id" => "minecraft:gradient_up",
                "translation_key" => "block.minecraft.banner.gradient_up"
            },
            "minecraft:guster" => nbt! {
                "asset_id" => "minecraft:guster",
                "translation_key" => "block.minecraft.banner.guster"
            },
            "minecraft:half_horizontal" => nbt! {
                "asset_id" => "minecraft:half_horizontal",
                "translation_key" => "block.minecraft.banner.half_horizontal"
            },
            "minecraft:half_horizontal_bottom" => nbt! {
                "asset_id" => "minecraft:half_horizontal_bottom",
                "translation_key" => "block.minecraft.banner.half_horizontal_bottom"
            },
            "minecraft:half_vertical" => nbt! {
                "asset_id" => "minecraft:half_vertical",
                "translation_key" => "block.minecraft.banner.half_vertical"
            },
            "minecraft:half_vertical_right" => nbt! {
                "asset_id" => "minecraft:half_vertical_right",
                "translation_key" => "block.minecraft.banner.half_vertical_right"
            },
            "minecraft:mojang" => nbt! {
                "asset_id" => "minecraft:mojang",
                "translation_key" => "block.minecraft.banner.mojang"
            },
            "minecraft:piglin" => nbt! {
                "asset_id" => "minecraft:piglin",
                "translation_key" => "block.minecraft.banner.piglin"
            },
            "minecraft:rhombus" => nbt! {
                "asset_id" => "minecraft:rhombus",
                "translation_key" => "block.minecraft.banner.rhombus"
            },
            "minecraft:skull" => nbt! {
                "asset_id" => "minecraft:skull",
                "translation_key" => "block.minecraft.banner.skull"
            },
            "minecraft:small_stripes" => nbt! {
                "asset_id" => "minecraft:small_stripes",
                "translation_key" => "block.minecraft.banner.small_stripes"
            },
            "minecraft:square_bottom_left" => nbt! {
                "asset_id" => "minecraft:square_bottom_left",
                "translation_key" => "block.minecraft.banner.square_bottom_left"
            },
            "minecraft:square_bottom_right" => nbt! {
                "asset_id" => "minecraft:square_bottom_right",
                "translation_key" => "block.minecraft.banner.square_bottom_right"
            },
            "minecraft:square_top_left" => nbt! {
                "asset_id" => "minecraft:square_top_left",
                "translation_key" => "block.minecraft.banner.square_top_left"
            },
            "minecraft:square_top_right" => nbt! {
                "asset_id" => "minecraft:square_top_right",
                "translation_key" => "block.minecraft.banner.square_top_right"
            },
            "minecraft:straight_cross" => nbt! {
                "asset_id" => "minecraft:straight_cross",
                "translation_key" => "block.minecraft.banner.square_cross"
            },
            "minecraft:stripe_bottom" => nbt! {
                "asset_id" => "minecraft:stripe_bottom",
                "translation_key" => "block.minecraft.banner.stripe.bottom"
            },
            "minecraft:stripe_center" => nbt! {
                "asset_id" => "minecraft:stripe_center",
                "translation_key" => "block.minecraft.banner.stripe.center"
            },
            "minecraft:stripe_downleft" => nbt! {
                "asset_id" => "minecraft:stripe_downleft",
                "translation_key" => "block.minecraft.banner.stripe.downleft"
            },
            "minecraft:stripe_downright" => nbt! {
                "asset_id" => "minecraft:stripe_downright",
                "translation_key" => "block.minecraft.banner.stripe.downright"
            },
            "minecraft:stripe_left" => nbt! {
                "asset_id" => "minecraft:stripe_left",
                "translation_key" => "block.minecraft.banner.stripe.left"
            },
            "minecraft:stripe_middle" => nbt! {
                "asset_id" => "minecraft:stripe_middle",
                "translation_key" => "block.minecraft.banner.stripe.middle"
            },
            "minecraft:stripe_right" => nbt! {
                "asset_id" => "minecraft:stripe_right",
                "translation_key" => "block.minecraft.banner.stripe.right"
            },
            "minecraft:stripe_top" => nbt! {
                "asset_id" => "minecraft:stripe_top",
                "translation_key" => "block.minecraft.banner.stripe.top"
            },
            "minecraft:triangle_bottom" => nbt! {
                "asset_id" => "minecraft:triangle_bottom",
                "translation_key" => "block.minecraft.banner.triangle.bottom"
            },
            "minecraft:triangle_top" => nbt! {
                "asset_id" => "minecraft:triangle_top",
                "translation_key" => "block.minecraft.banner.triangle.top"
            },
            "minecraft:triangles_bottom" => nbt! {
                "asset_id" => "minecraft:triangles_bottom",
                "translation_key" => "block.minecraft.banner.triangles.bottom"
            },
            "minecraft:triangles_top" => nbt! {
                "asset_id" => "minecraft:triangles_top",
                "translation_key" => "block.minecraft.banner.triangles.top"
            }
        },
        "minecraft:chat_type" => hashmap! {
            "minecraft:chat" => nbt! {
                "chat" => compound! {
                    "translation_key" => "chat.type.text",
                    "parameters" => List::String(vec![
                        "sender".to_owned(),
                        "content".to_owned()
                    ])
                },
                "narration" => compound! {
                    "translation_key" => "chat.type.text.narrate",
                    "parameters" => List::String(vec![
                        "sender".to_owned(),
                        "content".to_owned()
                    ])
                }
            },
            "minecraft:emote_command" => nbt! {
                "chat" => compound! {
                    "translation_key" => "chat.type.emote",
                    "parameters" => List::String(vec![
                        "sender".to_owned(),
                        "content".to_owned()
                    ])
                },
                "narration" => compound! {
                    "translation_key" => "chat.type.emote",
                    "parameters" => List::String(vec![
                        "sender".to_owned(),
                        "content".to_owned()
                    ])
                }
            },
            "minecraft:msg_command_incoming" => nbt! {
                "chat" => compound! {
                    "translation_key" => "commands.message.display.incoming",
                    "parameters" => List::String(vec![
                        "sender".to_owned(),
                        "content".to_owned()
                    ])
                },
                "narration" => compound! {
                    "translation_key" => "chat.type.text.narrate",
                    "parameters" => List::String(vec![
                        "sender".to_owned(),
                        "content".to_owned()
                    ])
                }
            },
            "minecraft:msg_command_outgoing" => nbt! {
                "chat" => compound! {
                    "translation_key" => "commands.message.display.outgoing",
                    "parameters" => List::String(vec![
                        "target".to_owned(),
                        "content".to_owned()
                    ])
                },
                "narration" => compound! {
                    "translation_key" => "chat.type.text.narrate",
                    "parameters" => List::String(vec![
                        "target".to_owned(),
                        "content".to_owned()
                    ])
                }
            },
            "minecraft:say_command" => nbt! {
                "chat" => compound! {
                    "translation_key" => "chat.type.announcement",
                    "parameters" => List::String(vec![
                        "sender".to_owned(),
                        "content".to_owned()
                    ])
                },
                "narration" => compound! {
                    "translation_key" => "chat.type.text.narrate",
                    "parameters" => List::String(vec![
                        "sender".to_owned(),
                        "content".to_owned()
                    ])
                }
            },
            "minecraft:team_msg_command_incoming" => nbt! {
                "chat" => compound! {
                    "translation_key" => "chat.type.team.text",
                    "parameters" => List::String(vec![
                        "target".to_owned(),
                        "sender".to_owned(),
                        "content".to_owned()
                    ])
                },
                "narration" => compound! {
                    "translation_key" => "chat.type.text.narrate",
                    "parameters" => List::String(vec![
                        "sender".to_owned(),
                        "content".to_owned()
                    ])
                }
            },
            "minecraft:team_msg_command_outgoing" => nbt! {
                "chat" => compound! {
                    "translation_key" => "chat.type.team.sent",
                    "parameters" => List::String(vec![
                        "target".to_owned(),
                        "sender".to_owned(),
                        "content".to_owned()
                    ])
                },
                "narration" => compound! {
                    "translation_key" => "chat.type.text.narrate",
                    "parameters" => List::String(vec![
                        "sender".to_owned(),
                        "content".to_owned()
                    ])
                }
            }
        },
        "minecraft:damage_type" => hashmap! {
            "minecraft:arrow" => damage_type!("arrow", 0.1_f32, "when_caused_by_living_non_player"),
            "minecraft:bad_respawn_point" => damage_type!("badRespawnPoint", 0.1_f32, "always", "intentional_game_design"),
            "minecraft:cactus" => damage_type!("cactus", 0.1_f32, "when_caused_by_living_non_player"),
            "minecraft:campfire" => damage_type!("inFire", 0.1_f32, "when_caused_by_living_non_player", "burning"),
            "minecraft:cramming" => damage_type!("cramming", 0.0_f32, "when_caused_by_living_non_player"),
            "minecraft:dragon_breath" => damage_type!("dragonBreath", 0.0_f32, "when_caused_by_living_non_player"),
            "minecraft:drown" => damage_type!("drown", 0.0_f32, "when_caused_by_living_non_player", "drowning"),
            "minecraft:dry_out" => damage_type!("dryout", 0.1_f32, "when_caused_by_living_non_player"),
            "minecraft:ender_pearl" => damage_type_with_death!("fall", 0.0_f32, "when_caused_by_living_non_player", "fall_variants"),
            "minecraft:explosion" => damage_type!("explosion", 0.1_f32, "always"),
            "minecraft:fall" => damage_type_with_death!("fall", 0.0_f32, "when_caused_by_living_non_player", "fall_variants"),
            "minecraft:falling_anvil" => damage_type!("anvil", 0.1_f32, "when_caused_by_living_non_player"),
            "minecraft:falling_block" => damage_type!("fallingBlock", 0.1_f32, "when_caused_by_living_non_player"),
            "minecraft:falling_stalactite" => damage_type!("fallingStalactite", 0.1_f32, "when_caused_by_living_non_player"),
            "minecraft:fireball" => damage_type!("fireball", 0.1_f32, "when_caused_by_living_non_player", "burning"),
            "minecraft:fireworks" => damage_type!("fireworks", 0.1_f32, "when_caused_by_living_non_player"),
            "minecraft:fly_into_wall" => damage_type!("flyIntoWall", 0.0_f32, "when_caused_by_living_non_player"),
            "minecraft:freeze" => damage_type!("freeze", 0.0_f32, "when_caused_by_living_non_player", "freezing"),
            "minecraft:generic" => damage_type!("generic", 0.0_f32, "when_caused_by_living_non_player"),
            "minecraft:generic_kill" => damage_type!("genericKill", 0.0_f32, "when_caused_by_living_non_player"),
            "minecraft:hot_floor" => damage_type!("hotFloor", 0.1_f32, "when_caused_by_living_non_player", "burning"),
            "minecraft:in_fire" => damage_type!("inFire", 0.1_f32, "when_caused_by_living_non_player", "burning"),
            "minecraft:in_wall" => damage_type!("inWall", 0.0_f32, "when_caused_by_living_non_player"),
            "minecraft:indirect_magic" => damage_type!("indirectMagic", 0.0_f32, "when_caused_by_living_non_player"),
            "minecraft:lava" => damage_type!("lava", 0.1_f32, "when_caused_by_living_non_player", "burning"),
            "minecraft:lightning_bolt" => damage_type!("lightningBolt", 0.1_f32, "when_caused_by_living_non_player"),
            "minecraft:mace_smash" => damage_type!("mace_smash", 0.1_f32, "when_caused_by_living_non_player"),
            "minecraft:magic" => damage_type!("magic", 0.0_f32, "when_caused_by_living_non_player"),
            "minecraft:mob_attack" => damage_type!("mob", 0.1_f32, "when_caused_by_living_non_player"),
            "minecraft:mob_attack_no_aggro" => damage_type!("mob", 0.1_f32, "when_caused_by_living_non_player"),
            "minecraft:mob_projectile" => damage_type!("mob", 0.1_f32, "when_caused_by_living_non_player"),
            "minecraft:on_fire" => damage_type!("onFire", 0.0_f32, "when_caused_by_living_non_player", "burning"),
            "minecraft:out_of_world" => damage_type!("outOfWorld", 0.0_f32, "when_caused_by_living_non_player"),
            "minecraft:outside_border" => damage_type!("outsideBorder", 0.0_f32, "when_caused_by_living_non_player"),
            "minecraft:player_attack" => damage_type!("player", 0.1_f32, "when_caused_by_living_non_player"),
            "minecraft:player_explosion" => damage_type!("explosion.player", 0.1_f32, "always"),
            "minecraft:sonic_boom" => damage_type!("sonic_boom", 0.0_f32, "always"),
            "minecraft:spear" => damage_type!("spear", 0.1_f32, "when_caused_by_living_non_player"),
            "minecraft:spit" => damage_type!("mob", 0.1_f32, "when_caused_by_living_non_player"),
            "minecraft:stalagmite" => damage_type!("stalagmite", 0.0_f32, "when_caused_by_living_non_player"),
            "minecraft:starve" => damage_type!("starve", 0.0_f32, "when_caused_by_living_non_player"),
            "minecraft:sting" => damage_type!("sting", 0.1_f32, "when_caused_by_living_non_player"),
            "minecraft:sulfur_cube_hot" => damage_type!("sulfurCubeHot", 0.1_f32, "when_caused_by_living_non_player", "burning"),
            "minecraft:sweet_berry_bush" => damage_type!("sweetBerryBush", 0.1_f32, "when_caused_by_living_non_player", "poking"),
            "minecraft:thorns" => damage_type!("thorns", 0.1_f32, "when_caused_by_living_non_player", "thorns"),
            "minecraft:thrown" => damage_type!("thrown", 0.1_f32, "when_caused_by_living_non_player"),
            "minecraft:trident" => damage_type!("trident", 0.1_f32, "when_caused_by_living_non_player"),
            "minecraft:unattributed_fireball" => damage_type!("onFire", 0.1_f32, "when_caused_by_living_non_player", "burning"),
            "minecraft:wind_charge" => damage_type!("mob", 0.1_f32, "when_caused_by_living_non_player"),
            "minecraft:wither" => damage_type!("wither", 0.0_f32, "when_caused_by_living_non_player"),
            "minecraft:wither_skull" => damage_type!("witherSkull", 0.1_f32, "when_caused_by_living_non_player")
        },
        "minecraft:worldgen/biome" => hashmap! {
            "minecraft:badlands" => biome!(),
            "minecraft:bamboo_jungle" => biome!(),
            "minecraft:basalt_deltas" => biome!(),
            "minecraft:beach" => biome!(),
            "minecraft:birch_forest" => biome!(),
            "minecraft:cherry_grove" => biome!(),
            "minecraft:cold_ocean" => biome!(),
            "minecraft:crimson_forest" => biome!(),
            "minecraft:dappled_forest" => biome!(),
            "minecraft:dark_forest" => biome!(),
            "minecraft:deep_cold_ocean" => biome!(),
            "minecraft:deep_dark" => biome!(),
            "minecraft:deep_frozen_ocean" => biome!(),
            "minecraft:deep_lukewarm_ocean" => biome!(),
            "minecraft:deep_ocean" => biome!(),
            "minecraft:desert" => biome!(),
            "minecraft:dripstone_caves" => biome!(),
            "minecraft:end_barrens" => biome!(),
            "minecraft:end_highlands" => biome!(),
            "minecraft:end_midlands" => biome!(),
            "minecraft:eroded_badlands" => biome!(),
            "minecraft:flower_forest" => biome!(),
            "minecraft:forest" => biome!(),
            "minecraft:frozen_ocean" => biome!(),
            "minecraft:frozen_peaks" => biome!(),
            "minecraft:frozen_river" => biome!(),
            "minecraft:grove" => biome!(),
            "minecraft:ice_spikes" => biome!(),
            "minecraft:jagged_peaks" => biome!(),
            "minecraft:jungle" => biome!(),
            "minecraft:lukewarm_ocean" => biome!(),
            "minecraft:lush_caves" => biome!(),
            "minecraft:mangrove_swamp" => biome!(),
            "minecraft:meadow" => biome!(),
            "minecraft:mushroom_fields" => biome!(),
            "minecraft:nether_wastes" => biome!(),
            "minecraft:ocean" => biome!(),
            "minecraft:old_growth_birch_forest" => biome!(),
            "minecraft:old_growth_pine_taiga" => biome!(),
            "minecraft:old_growth_spruce_taiga" => biome!(),
            "minecraft:pale_garden" => biome!(),
            "minecraft:plains" => biome!(),
            "minecraft:river" => biome!(),
            "minecraft:savanna" => biome!(),
            "minecraft:savanna_plateau" => biome!(),
            "minecraft:small_end_islands" => biome!(),
            "minecraft:snowy_beach" => biome!(),
            "minecraft:snowy_plains" => biome!(),
            "minecraft:snowy_slopes" => biome!(),
            "minecraft:snowy_taiga" => biome!(),
            "minecraft:soul_sand_valley" => biome!(),
            "minecraft:sparse_jungle" => biome!(),
            "minecraft:stony_peaks" => biome!(),
            "minecraft:stony_shore" => biome!(),
            "minecraft:sulfur_caves" => biome!(),
            "minecraft:sunflower_plains" => biome!(),
            "minecraft:swamp" => biome!(),
            "minecraft:taiga" => biome!(),
            "minecraft:the_end" => biome!(),
            "minecraft:the_void" => biome!(),
            "minecraft:warm_ocean" => biome!(),
            "minecraft:warped_forest" => biome!(),
            "minecraft:windswept_forest" => biome!(),
            "minecraft:windswept_gravelly_hills" => biome!(),
            "minecraft:windswept_hills" => biome!(),
            "minecraft:windswept_savanna" => biome!(),
            "minecraft:wooded_badlands" => biome!()
        },
        "minecraft:trim_pattern" => hashmap! {
            "minecraft:bolt" => trim_pattern!("bolt"),
            "minecraft:coast" => trim_pattern!("coast"),
            "minecraft:dune" => trim_pattern!("dune"),
            "minecraft:eye" => trim_pattern!("eye"),
            "minecraft:flow" => trim_pattern!("flow"),
            "minecraft:host" => trim_pattern!("host"),
            "minecraft:raiser" => trim_pattern!("raiser"),
            "minecraft:rib" => trim_pattern!("rib"),
            "minecraft:sentry" => trim_pattern!("sentry"),
            "minecraft:shaper" => trim_pattern!("shaper"),
            "minecraft:silence" => trim_pattern!("silence"),
            "minecraft:snout" => trim_pattern!("snout"),
            "minecraft:spire" => trim_pattern!("spire"),
            "minecraft:tide" => trim_pattern!("tide"),
            "minecraft:vex" => trim_pattern!("vex"),
            "minecraft:ward" => trim_pattern!("ward"),
            "minecraft:wayfinder" => trim_pattern!("wayfinder"),
            "minecraft:wild" => trim_pattern!("wild")
        },
        "minecraft:trim_material" => hashmap! {
            "minecraft:amethyst" => trim_material!("amethyst", 0.1),
            "minecraft:copper" => trim_material!("copper", 0.2),
            "minecraft:diamond" => trim_material!("copper", 0.5),
            "minecraft:emerald" => trim_material!("emerald", 0.9),
            "minecraft:gold" => trim_material!("gold", 0.3),
            "minecraft:iron" => trim_material!("iron", 0.2),
            "minecraft:lapis" => trim_material!("lapis", 1.0),
            "minecraft:netherite" => trim_material!("netherite", 0.6),
            "minecraft:quartz" => trim_material!("quartz", 0.8),
            "minecraft:redstone" => trim_material!("redstone", 0.7),
            "minecraft:resin" => nbt! {
                "asset_name" => "resin",
                "description" => compound! {"translate" => "trim_material.minecraft.resin"}
            }
        },
        "minecraft:wolf_variant" => hashmap! {
            "minecraft:pale" => wolf_variant!("wolf", "#minecraft:is_taiga"),
            "minecraft:woods" => wolf_variant!("wolf_woods", "minecraft:forest"),
            "minecraft:ashen" => wolf_variant!("wolf_ashen", "minecraft:snowy_taiga"),
            "minecraft:black" => wolf_variant!("wolf_black", "minecraft:old_growth_pine_taiga"),
            "minecraft:chestnut" => wolf_variant!("wolf_chestnut", "minecraft:old_growth_spruce_taiga"),
            "minecraft:rusty" => wolf_variant!("wolf_rusty", "minecraft:sparse_jungle"),
            "minecraft:spotted" => wolf_variant!("wolf_spotted", "minecraft:savanna_plateau"),
            "minecraft:striped" => wolf_variant!("wolf_striped", "minecraft:wooded_badlands"),
            "minecraft:snowy" => wolf_variant!("wolf_snowy", "minecraft:grove")
        },
        "minecraft:wolf_sound_variant" => hashmap! {
            "minecraft:angry" => wolf_sound_variant!("wolf_angry"),
            "minecraft:big" => wolf_sound_variant!("wolf_big"),
            "minecraft:classic" => wolf_sound_variant!("wolf"),
            "minecraft:cute" => wolf_sound_variant!("wolf_cute"),
            "minecraft:grumpy" => wolf_sound_variant!("wolf_grumpy"),
            "minecraft:puglin" => wolf_sound_variant!("wolf_puglin"),
            "minecraft:sad" => wolf_sound_variant!("wolf_sad")
        },
        "minecraft:pig_variant" => hashmap! {
            "minecraft:cold" => pig_variant!("cold", "cold"),
            "minecraft:temperate" => pig_variant!("temperate", "normal"),
            "minecraft:warm" => pig_variant!("warm", "normal")
        },
        "minecraft:frog_variant" => hashmap! {
            "minecraft:cold" => frog_variant!("cold"),
            "minecraft:temperate" => frog_variant!("temperate"),
            "minecraft:warm" => frog_variant!("warm")
        },
        "minecraft:cat_variant" => hashmap! {
            "minecraft:all_black" => nbt! {"asset_id" => "minecraft:all_black"},
            "minecraft:black" => nbt! {"asset_id" => "minecraft:black"},
            "minecraft:british_shorthair" => nbt! {"asset_id" => "minecraft:british_shorthair"},
            "minecraft:calico" => nbt! {"asset_id" => "minecraft:calico"},
            "minecraft:jellie" => nbt! {"asset_id" => "minecraft:jellie"},
            "minecraft:persian" => nbt! {"asset_id" => "minecraft:persian"},
            "minecraft:ragdoll" => nbt! {"asset_id" => "minecraft:ragdoll"},
            "minecraft:red" => nbt! {"asset_id" => "minecraft:red"},
            "minecraft:siamese" => nbt! {"asset_id" => "minecraft:siamese"},
            "minecraft:tabby" => nbt! {"asset_id" => "minecraft:tabby"},
            "minecraft:white" => nbt! {"asset_id" => "minecraft:white"},
        },
        "minecraft:cow_variant" => hashmap! {
            "minecraft:temperate" => nbt! {
                "model" => "normal",
                "asset_id" => "minecraft:temperate_cow",
                "spawn_conditions" => List::Compound(vec![
                    compound! {
                        "priority" => 0
                    }
                ])
            },
            "minecraft:cold" => nbt! {
                "model" => "cold",
                "asset_id" => "minecraft:cold_cow",
                "spawn_conditions" => List::Compound(vec![
                    compound! {
                        "priority" => 1,
                        "condition" => compound! {
                            "biomes" => List::String(vec![
                                "minecraft:taiga".to_string(),
                                "minecraft:snowy_taiga".to_string(),
                                "minecraft:old_growth_pine_taiga".to_string(),
                                "minecraft:old_growth_spruce_taiga".to_string(),
                                "minecraft:windswept_hills".to_string(),
                                "minecraft:windswept_gravelly_hills".to_string(),
                                "minecraft:windswept_forest".to_string()
                            ])
                        }
                    }
                ])
            },
            "minecraft:warm" => nbt! {
                "model" => "warm",
                "asset_id" => "minecraft:warm_cow",
                "spawn_conditions" => List::Compound(vec![
                    compound! {
                        "priority" => 1,
                        "condition" => compound! {
                            "biomes" => List::String(vec![
                                "minecraft:savanna".to_string(),
                                "minecraft:savanna_plateau".to_string(),
                                "minecraft:windswept_savanna".to_string(),
                                "minecraft:jungle".to_string(),
                                "minecraft:sparse_jungle".to_string(),
                                "minecraft:bamboo_jungle".to_string(),
                                "minecraft:badlands".to_string(),
                                "minecraft:eroded_badlands".to_string(),
                                "minecraft:wooded_badlands".to_string()
                            ])
                        }
                    }
                ])
            }
        }
    }
}
