use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum NbtValue {
    String(String),
    Int(i32),
    Float(f32),
    Double(f64),
    Compound(HashMap<&'static str, NbtValue>),
    Array(Vec<NbtValue>),
    Bool(bool),
}

impl From<&'static str> for NbtValue {
    fn from(v: &'static str) -> Self { NbtValue::String(v.to_owned()) }
}
impl From<i32> for NbtValue {
    fn from(v: i32) -> Self { NbtValue::Int(v) }
}
impl From<HashMap<&'static str, NbtValue>> for NbtValue {
    fn from(v: HashMap<&'static str, NbtValue>) -> Self { NbtValue::Compound(v) }
}
impl From<Vec<NbtValue>> for NbtValue {
    fn from(v: Vec<NbtValue>) -> Self { NbtValue::Array(v) }
}
impl From<bool> for NbtValue {
    fn from(v: bool) -> Self { NbtValue::Bool(v) }
}
impl From<String> for NbtValue {
    fn from(value: String) -> Self { NbtValue::String(value) }
}
impl From<f32> for NbtValue {
    fn from(value: f32) -> Self { NbtValue::Float(value) }
}
impl From<f64> for NbtValue {
    fn from(value: f64) -> Self { NbtValue::Double(value) }
}


impl NbtValue {
    fn to_value(&self) -> Result<valence_nbt::Value, String> {
        match self {
            NbtValue::String(value) => Ok(valence_nbt::Value::String(value.clone())),
            NbtValue::Int(value) => Ok(valence_nbt::Value::Int(*value)),
            NbtValue::Float(value) => Ok(valence_nbt::Value::Float(*value)),
            NbtValue::Double(value) => Ok(valence_nbt::Value::Double(*value)),
            NbtValue::Bool(value) => Ok(valence_nbt::Value::Byte(i8::from(*value))),
            NbtValue::Compound(_values) => Ok(valence_nbt::Value::Compound(self.to_compound()?)),
            NbtValue::Array(values) => {
                let mut list = valence_nbt::List::new();

                for value in values {
                    let converted = value.to_value()?;
                    if !list.try_push(converted) {
                        return Err("NBT lists must contain values of one type".to_owned());
                    }
                }

                Ok(valence_nbt::Value::List(list))
            }
        }
    }

    pub fn to_compound(&self) -> Result<valence_nbt::Compound, String> {
        let NbtValue::Compound(values) = self else {
            return Err("the root NBT value must be a compound".to_owned());
        };

        let mut compound = valence_nbt::Compound::new();
        for (key, value) in values {
            compound.insert((*key).to_owned(), value.to_value()?);
        }

        Ok(compound)
    }
}

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

macro_rules! nbt_value {
    ( [ $( $value:tt ),* $(,)? ] ) => {{
        NbtValue::Array(vec![ $( nbt_value!($value) ),* ])
    }};
    ( { $( $key:tt : $value:tt ),* $(,)? } ) => {{
        NbtValue::Compound(hashmap! {$( $key => nbt_value!($value) ),* })
    }};
    ( $value:expr ) => {{
        NbtValue::from($value)
    }}
}

macro_rules! nbt {
    () => {
        Vec::new()
    };
    ( $( $key:tt : $val:tt ),* $(,)? ) => {{
        let nbt = nbt_value!( {$( $key: $val ),*} );
        let compound = nbt.to_compound().expect("invalid NBT data");
        let mut bytes = Vec::new();
        valence_nbt::to_binary(&compound, &mut bytes, "").unwrap();
        bytes.drain(1..3);
        bytes
    }}
}

macro_rules! damage_type {
    ($message_id:expr, $exhaustion:expr, $scaling:expr) => {
        nbt! {
            "message_id": $message_id,
            "exhaustion": $exhaustion,
            "scaling": $scaling
        }
    };
    ($message_id:expr, $exhaustion:expr, $scaling:expr, $effects:expr) => {
        nbt! {
            "message_id": $message_id,
            "exhaustion": $exhaustion,
            "scaling": $scaling,
            "effects": $effects
        }
    };
    ($message_id:expr, $exhaustion:expr, $scaling:expr, $effects:expr, $death_message_type:expr) => {
        nbt! {
            "message_id": $message_id,
            "exhaustion": $exhaustion,
            "scaling": $scaling,
            "effects": $effects,
            "death_message_type": $death_message_type
        }
    };
}

macro_rules! damage_type_with_death {
    ($message_id:expr, $exhaustion:expr, $scaling:expr, $death_message_type:expr) => {
        nbt! {
            "message_id": $message_id,
            "exhaustion": $exhaustion,
            "scaling": $scaling,
            "death_message_type": $death_message_type
        }
    };
}

macro_rules! biome {
    () => {
        nbt! {
            "has_precipitation": true,
            "temperature": 0.8_f32,
            "downfall": 0.4_f32,
            "effects": {
                "sky_color": 7907327,
                "water_fog_color": 329011,
                "fog_color": 12638463,
                "water_color": 4159204,
                "mood_sound": {
                    "sound": "minecraft:ambient.cave",
                    "tick_delay": 6000,
                    "block_search_extent": 8,
                    "offset": 2.0_f64
                }
            },
            "carvers": [],
            "features": [],
            "spawners": {},
            "spawn_costs": {}
        }
    };
}

macro_rules! trim_pattern {
    ($trim_name:expr) => {
        nbt! {
            "asset_id": (format!("minecraft:{}", $trim_name)),
            "description": {
                "translate": (format!("trim_pattern.minecraft.{}", $trim_name))
            },
            "decal": 0x00
        }
    };
}

macro_rules! trim_material {
    ($material_name:expr, $item_model_index:expr) => {
        nbt! {
            "asset_name": $material_name,
            "item_model_index": $item_model_index,
            "description": {
                "translate": (format!("trim_material.minecraft.{}", $material_name))
            },
        }
    };
}

macro_rules! wolf_variant {
    ($name:expr, $biome:expr) => {
        nbt! {
            "assets": {
                "wild": (format!("minecraft:entity/wolf/{}", $name)),
                "tame": (format!("minecraft:entity/wolf/{}_tamed", $name)),
                "angry": (format!("minecraft:entity/wolf/{}_angry", $name))
            },
            "biomes": $biome
        }
    };
}

macro_rules! wolf_sound_variant {
    ($name:expr) => {
        nbt! {
            "adult_sounds": {
                "ambient_sound": (format!("minecraft:entity.{}.ambient", $name)),
                "death_sound": (format!("minecraft:entity.{}.death", $name)),
                "growl_sound": (format!("minecraft:entity.{}.growl", $name)),
                "hurt_sound": (format!("minecraft:entity.{}.hurt", $name)),
                "pant_sound": (format!("minecraft:entity.{}.pant", $name)),
                "whine_sound": (format!("minecraft:entity.{}.whine", $name))
            },
            "baby_sounds": {
                "ambient_sound": "minecraft:entity.wolf.ambient",
                "death_sound": "minecraft:entity.wolf.death",
                "growl_sound": "minecraft:entity.wolf.growl",
                "hurt_sound": "minecraft:entity.wolf.hurt",
                "pant_sound": "minecraft:entity.wolf.pant",
                "whine_sound": "minecraft:entity.wolf.whine"
            }
        }
    };
}

macro_rules! pig_variant {
    ($name:expr, $model:expr) => {
        nbt! {
            "model": $model,
            "asset_id": (format!("minecraft:pig/{}", $name)),
        }
    };
}

macro_rules! frog_variant {
    ($name:expr) => {
        nbt! {
            "asset_id": (format!("minecraft:{}", $name)),
        }
    };
}

macro_rules! banner_pattern {
    ($name:expr) => {
        nbt! {
            "asset_id": (format!("minecraft:{}", $name)),
            "translation_key": (format!("block.minecraft.banner.{}", $name)),
        }
    };
}

pub fn get_configuration_data() -> HashMap<&'static str, HashMap<&'static str, Vec<u8>>> {
    hashmap! {
        "minecraft:banner_pattern" => hashmap! {
            "minecraft:base" => banner_pattern!("base"),
            "minecraft:border" => banner_pattern!("border"),
            "minecraft:bricks" => banner_pattern!("bricks"),
            "minecraft:circle" => banner_pattern!("circle"),
            "minecraft:creeper" => banner_pattern!("creeper"),
            "minecraft:cross" => banner_pattern!("cross"),
            "minecraft:curly_border" => banner_pattern!("curly_border"),
            "minecraft:diagonal_left" => banner_pattern!("diagonal_left"),
            "minecraft:diagonal_right" => banner_pattern!("diagonal_right"),
            "minecraft:diagonal_up_left" => banner_pattern!("diagonal_up_left"),
            "minecraft:diagonal_up_right" => banner_pattern!("diagonal_up_right"),
            "minecraft:flow" => banner_pattern!("flow"),
            "minecraft:flower" => banner_pattern!("flower"),
            "minecraft:globe" => banner_pattern!("globe"),
            "minecraft:gradient" => banner_pattern!("gradient"),
            "minecraft:gradient_up" => banner_pattern!("gradient_up"),
            "minecraft:guster" => banner_pattern!("guster"),
            "minecraft:half_horizontal" => banner_pattern!("half_horizontal"),
            "minecraft:half_horizontal_bottom" => banner_pattern!("half_horizontal_bottom"),
            "minecraft:half_vertical" => banner_pattern!("half_vertical"),
            "minecraft:half_vertical_right" => banner_pattern!("half_vertical_right"),
            "minecraft:mojang" => banner_pattern!("mojang"),
            "minecraft:piglin" => banner_pattern!("piglin"),
            "minecraft:rhombus" => banner_pattern!("rhombus"),
            "minecraft:skull" => banner_pattern!("skull"),
            "minecraft:small_stripes" => banner_pattern!("small_stripes"),
            "minecraft:square_bottom_left" => banner_pattern!("square_bottom_left"),
            "minecraft:square_bottom_right" => banner_pattern!("square_bottom_right"),
            "minecraft:square_top_left" => banner_pattern!("square_top_left"),
            "minecraft:square_top_right" => banner_pattern!("square_top_right"),
            "minecraft:straight_cross" => banner_pattern!("straight_cross"),
            "minecraft:stripe_bottom" => banner_pattern!("stripe_bottom"),
            "minecraft:stripe_center" => banner_pattern!("stripe_center"),
            "minecraft:stripe_downleft" => banner_pattern!("stripe_downleft"),
            "minecraft:stripe_downright" => banner_pattern!("stripe_downright"),
            "minecraft:stripe_left" => banner_pattern!("stripe_left"),
            "minecraft:stripe_middle" => banner_pattern!("stripe_middle"),
            "minecraft:stripe_right" => banner_pattern!("stripe_right"),
            "minecraft:stripe_top" => banner_pattern!("stripe_top"),
            "minecraft:triangle_bottom" => banner_pattern!("triangle_bottom"),
            "minecraft:triangle_top" => banner_pattern!("triangle_top"),
            "minecraft:triangles_bottom" => banner_pattern!("triangles_bottom"),
            "minecraft:triangles_top" => banner_pattern!("triangles_top")
        },
        "minecraft:chat_type" => hashmap! {
            "minecraft:chat" => nbt! {
                "chat": {
                    "translation_key": "chat.type.text",
                    "parameters": [
                        "sender",
                        "content"
                    ]
                },
                "narration": {
                    "translation_key": "chat.type.text.narrate",
                    "parameters": [
                        "sender",
                        "content"
                    ]
                }
            },
            "minecraft:emote_command" => nbt! {
                "chat": {
                    "translation_key": "chat.type.emote",
                    "parameters": [
                        "sender",
                        "content"
                    ]
                },
                "narration": {
                    "translation_key": "chat.type.emote",
                    "parameters": [
                        "sender",
                        "content"
                    ]
                }
            },
            "minecraft:msg_command_incoming" => nbt! {
                "chat": {
                    "translation_key": "commands.message.display.incoming",
                    "parameters": [
                        "sender",
                        "content"
                    ]
                },
                "narration": {
                    "translation_key": "chat.type.text.narrate",
                    "parameters": [
                        "sender",
                        "content"
                    ]
                }
            },
            "minecraft:msg_command_outgoing" => nbt! {
                "chat": {
                    "translation_key": "commands.message.display.outgoing",
                    "parameters": [
                        "target",
                        "content"
                    ]
                },
                "narration": {
                    "translation_key": "chat.type.text.narrate",
                    "parameters": [
                        "target",
                        "content"
                    ]
                }
            },
            "minecraft:say_command" => nbt! {
                "chat": {
                    "translation_key": "chat.type.announcement",
                    "parameters": [
                        "sender",
                        "content"
                    ]
                },
                "narration": {
                    "translation_key": "chat.type.text.narrate",
                    "parameters": [
                        "sender",
                        "content"
                    ]
                }
            },
            "minecraft:team_msg_command_incoming" => nbt! {
                "chat": {
                    "translation_key": "chat.type.team.text",
                    "parameters": [
                        "target",
                        "sender",
                        "content"
                    ]
                },
                "narration": {
                    "translation_key": "chat.type.text.narrate",
                    "parameters": [
                        "sender",
                        "content"
                    ]
                }
            },
            "minecraft:team_msg_command_outgoing" => nbt! {
                "chat": {
                    "translation_key": "chat.type.team.sent",
                    "parameters": [
                        "target",
                        "sender",
                        "content"
                    ]
                },
                "narration": {
                    "translation_key": "chat.type.text.narrate",
                    "parameters": [
                        "sender",
                        "content"
                    ]
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
                "asset_name": "resin",
                "description": {"translate": "trim_material.minecraft.resin"}
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
            "minecraft:all_black" => nbt! {"asset_id": "minecraft:all_black"},
            "minecraft:black" => nbt! {"asset_id": "minecraft:black"},
            "minecraft:british_shorthair" => nbt! {"asset_id": "minecraft:british_shorthair"},
            "minecraft:calico" => nbt! {"asset_id": "minecraft:calico"},
            "minecraft:jellie" => nbt! {"asset_id": "minecraft:jellie"},
            "minecraft:persian" => nbt! {"asset_id": "minecraft:persian"},
            "minecraft:ragdoll" => nbt! {"asset_id": "minecraft:ragdoll"},
            "minecraft:red" => nbt! {"asset_id": "minecraft:red"},
            "minecraft:siamese" => nbt! {"asset_id": "minecraft:siamese"},
            "minecraft:tabby" => nbt! {"asset_id": "minecraft:tabby"},
            "minecraft:white" => nbt! {"asset_id": "minecraft:white"},
        },
        "minecraft:cow_variant" => hashmap! {
            "minecraft:temperate" => nbt! {
                "model": "normal",
                "asset_id": "minecraft:temperate_cow",
                "spawn_conditions": [
                    {
                        "priority": 0
                    }
                ]
            },
            "minecraft:cold" => nbt! {
                "model": "cold",
                "asset_id": "minecraft:cold_cow",
                "spawn_conditions": [
                    {
                        "priority": 1,
                        "condition": {
                            "biomes": [
                                "minecraft:taiga",
                                "minecraft:snowy_taiga",
                                "minecraft:old_growth_pine_taiga",
                                "minecraft:old_growth_spruce_taiga",
                                "minecraft:windswept_hills",
                                "minecraft:windswept_gravelly_hills",
                                "minecraft:windswept_forest"
                            ]
                        }
                    }
                ]
            },
            "minecraft:warm" => nbt! {
                "model": "warm",
                "asset_id": "minecraft:warm_cow",
                "spawn_conditions": [
                    {
                        "priority": 1,
                        "condition": {
                            "biomes": [
                                "minecraft:savanna",
                                "minecraft:savanna_plateau",
                                "minecraft:windswept_savanna",
                                "minecraft:jungle",
                                "minecraft:sparse_jungle",
                                "minecraft:bamboo_jungle",
                                "minecraft:badlands",
                                "minecraft:eroded_badlands",
                                "minecraft:wooded_badlands"
                            ]
                        }
                    }
                ]
            }
        },
    }
}
