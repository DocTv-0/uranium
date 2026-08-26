use valence_nbt::{compound, to_binary};
use strum::IntoEnumIterator;
use strum_macros::{EnumCount, EnumIter};

#[derive(Clone, Copy, EnumCount, EnumIter)]
pub enum Dimension {
    Overworld,
    Nether,
    End
}

impl Dimension {
    pub const fn name(self) -> &'static str {
        match self {
            Dimension::Overworld => "minecraft:overworld",
            Dimension::Nether => "minecraft:nether",
            Dimension::End => "minecraft:end"
        }
    }
    pub fn get_dimension_nbt(self) -> Vec<u8> {
        match self {
            Dimension::Overworld => {
                let dimension_properties = compound! {
                    "ambient_light" => 0.0f32,
                    "bed_works" => 1i8,
                    "coordinate_scale" => 1.0f64,
                    "effects" => "minecraft:overworld",
                    "has_ceiling" => 0i8,
                    "has_raids" => 1i8,
                    "has_skylight" => 1i8,
                    "infiniburn" => "#minecraft:infiniburn_overworld",
                    "logical_height" => 384i32,
                    "monster_spawn_block_light_limit" => 0i32,
                    "monster_spawn_light_level" => 0i32,
                    "natural" => 1i8,
                    "piglin_safe" => 0i8,
                    "respawn_anchor_works" => 0i8,
                    "ultrawarm" => 0i8,
                    "height" => 384i32,
                    "min_y" => -64i32,
                };
                let mut bytes = Vec::new();
                to_binary(&dimension_properties, &mut bytes, "").unwrap();
                bytes.drain(1..3);

                bytes
            }

            Dimension::Nether => {
                let dimension_properties = compound! {
                    "ambient_light" => 0.1f32,
                    "bed_works" => 0i8,
                    "coordinate_scale" => 8.0f64,
                    "effects" => "minecraft:nether",
                    "has_ceiling" => 1i8,
                    "has_raids" => 1i8,
                    "has_skylight" => 0i8,
                    "infiniburn" => "#minecraft:infiniburn_nether",
                    "logical_height" => 256i32,
                    "monster_spawn_block_light_limit" => 0i32,
                    "monster_spawn_light_level" => 15i32,
                    "natural" => 1i8,
                    "piglin_safe" => 0i8,
                    "respawn_anchor_works" => 1i8,
                    "ultrawarm" => 1i8,
                    "height" => 256i32,
                    "min_y" => 0i32,
                };
                let mut bytes = Vec::new();
                to_binary(&dimension_properties, &mut bytes, "").unwrap();
                bytes.drain(1..3);

                bytes
            }

            Dimension::End => {
                let dimension_properties = compound! {
                    "ambient_light" => 0.25f32,
                    "bed_works" => 0i8,
                    "coordinate_scale" => 1.0f64,
                    "effects" => "minecraft:end",
                    "has_ceiling" => 0i8,
                    "has_raids" => 1i8,
                    "has_skylight" => 0i8,
                    "infiniburn" => "#minecraft:infiniburn_end",
                    "logical_height" => 256i32,
                    "monster_spawn_block_light_limit" => 0i32,
                    "monster_spawn_light_level" => 0i32,
                    "natural" => 1i8,
                    "piglin_safe" => 0i8,
                    "respawn_anchor_works" => 0i8,
                    "ultrawarm" => 0i8,
                    "height" => 256i32,
                    "min_y" => 0i32,
                };
                let mut bytes = Vec::new();
                to_binary(&dimension_properties, &mut bytes, "").unwrap();
                bytes.drain(1..3);

                bytes
            }
        }
    }
}

#[derive(Clone, Copy, EnumCount, EnumIter)]
pub enum Biome {
    Plains
}

impl Biome {
    pub const fn name(self) -> &'static str {
        match self {
            Biome::Plains => "minecraft:plains",
        }
    }

    pub fn get_biome_nbt(self) -> Vec<u8> {
        match self {
            Biome::Plains => {
                let biome_properties = compound! {
                    "has_precipitation" => 1i8,
                    "temperature" => 0.8f32,
                    "downfall" => 0.4f32,
                    "effects" => compound! {
                        "sky_color" => 7907327i32,
                        "fog_color" => 12638463i32,
                        "water_color" => 4159204i32,
                        "water_fog_color" => 329011i32,
                        "grass_color_modifier" => "none"
                    }
                };
                let mut bytes = Vec::new();
                to_binary(&biome_properties, &mut bytes, "").unwrap();
                bytes.drain(1..3);

                bytes
            }
        }
    }
}
