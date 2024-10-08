use std::collections::HashMap;

use bedrockrs_nbt as nbt;

use bedrockrs_shared::world::permissions_level::PermissionLevel;

use crate::error::WorldError;

/// Represents the `abilities` compound tag in the `level.dat` and `level.dat_old` files,
/// it's for storing the default permissions for players in the world.
#[derive(Debug)]
pub struct LevelDatAbilities {
    /// If the player can attack mobs. (NBT entry: `attackmobs`)
    pub attack_mobs: bool,
    /// If the player can attack other players. (NBT entry: `attackplayers`)
    pub attack_players: bool,
    /// If the player is able to interact with redstone components. (NBT entry: `doorsandswitches`)
    pub redstone_interact: bool,
    /// If the player can place blocks. (NBT entry: `build`)
    pub build: bool,
    /// If the player can destroy blocks. (NBT entry: `mine`)
    pub mine: bool,
    /// If the player can instantly destroy blocks. (NBT entry: `instabuild`)
    pub mine_instantly: bool,
    /// If the player is currently flying. (NBT entry: `flying`)
    pub flying: bool,
    /// The flying speed, always 0.05. (NBT entry: `flySpeed`)
    pub fly_speed: f32,
    /// The walking speed, always 0.1. (NBT entry: `walkSpeed`)
    pub walk_speed: f32,
    /// If the player is immune to all damage and harmful effects. (NBT entry: `invulnerable`)
    pub invulnerable: bool,
    /// If lightning struck the player. (NBT entry: `lightning`)
    pub lightning: bool,
    /// If the player can fly. (NBT entry: `mayfly`)
    pub mayfly: bool,
    /// If the player messages cannot be seen by other players. (NBT entry: `mute`)
    pub mute: Option<bool>,
    /// If the player can phase through blocks. (NBT entry: `noclip`)
    pub no_clip: Option<bool>,
    /// If the player is able to open containers. (NBT entry: `opencontainers`)
    pub open_containers: bool,
    /// If the player is a world builder. (NBT entry: `worldbuilder`)
    pub world_builder: Option<bool>,
    /// If the player is allowed to teleport. (NBT entry: `teleport`)
    pub teleport: bool,
    /// If the player has operator commands. (NBT entry: `op`)
    pub op: bool,

    /// What permissions a player defaults to, when joining a world. (NBT entry: `permissionsLevel`)
    pub permissions_level: Option<PermissionLevel>,
    /// What permissions a player has. (NBT entry: `playerPermissionsLevel`)
    pub permissions_level_default: Option<PermissionLevel>,
}

impl LevelDatAbilities {
    /// Parses a given [`nbt::Value`] into a [`LevelDatAbilities`] object.
    pub fn parse(tag: nbt::Value) -> Result<Self, WorldError> {
        fn get_byte_as_bool(
            map: &mut HashMap<String, nbt::Value>,
            key: &str,
        ) -> Result<bool, WorldError> {
            match map.remove(key) {
                Some(nbt::Value::Byte(v)) => Ok(v != 0),
                Some(other) => Err(WorldError::FormatError(format!(
                    "Expected `{}` in LevelDat abilities to be of type Byte, got {:?}",
                    key, other
                ))),
                None => Err(WorldError::FormatError(format!(
                    "Missing field `{}` in LevelDat abilities",
                    key
                ))),
            }
        }

        fn get_byte_as_bool_option(
            map: &mut HashMap<String, nbt::Value>,
            key: &str,
        ) -> Result<Option<bool>, WorldError> {
            match map.remove(key) {
                Some(nbt::Value::Byte(v)) => Ok(Some(v != 0)),
                Some(other) => Err(WorldError::FormatError(format!(
                    "Expected `{}` in LevelDat abilities to be of type Byte, got {:?}",
                    key, other
                ))),
                None => Ok(None),
            }
        }

        fn get_int32(map: &mut HashMap<String, nbt::Value>, key: &str) -> Result<i32, WorldError> {
            match map.remove(key) {
                Some(nbt::Value::Int(v)) => Ok(v),
                Some(other) => Err(WorldError::FormatError(format!(
                    "Expected `{}` in LevelDat abilities to be of type Int32, got {:?}",
                    key, other
                ))),
                None => Err(WorldError::FormatError(format!(
                    "Missing field `{}` in LevelDat abilities",
                    key
                ))),
            }
        }

        fn get_int32_option(
            map: &mut HashMap<String, nbt::Value>,
            key: &str,
        ) -> Result<Option<i32>, WorldError> {
            match map.remove(key) {
                Some(nbt::Value::Int(v)) => Ok(Some(v)),
                Some(other) => Err(WorldError::FormatError(format!(
                    "Expected `{}` in LevelDat abilities to be of type Int32, got {:?}",
                    key, other
                ))),
                None => Ok(None),
            }
        }

        fn get_int64(map: &mut HashMap<String, nbt::Value>, key: &str) -> Result<i64, WorldError> {
            match map.remove(key) {
                Some(nbt::Value::Long(v)) => Ok(v),
                Some(other) => Err(WorldError::FormatError(format!(
                    "Expected `{}` in LevelDat abilities to be of type Int64, got {:?}",
                    key, other
                ))),
                None => Err(WorldError::FormatError(format!(
                    "Missing field `{}` in LevelDat abilities",
                    key
                ))),
            }
        }

        fn get_f32(map: &mut HashMap<String, nbt::Value>, key: &str) -> Result<f32, WorldError> {
            match map.remove(key) {
                Some(nbt::Value::Float(v)) => Ok(v),
                Some(other) => Err(WorldError::FormatError(format!(
                    "Expected `{}` in LevelDat abilities to be of type Float32, got {:?}",
                    key, other
                ))),
                None => Err(WorldError::FormatError(format!(
                    "Missing field `{}` in LevelDat abilities",
                    key
                ))),
            }
        }

        match tag {
            nbt::Value::Compound(mut map) => Ok(Self {
                attack_mobs: get_byte_as_bool(&mut map, "attackmobs")?,
                attack_players: get_byte_as_bool(&mut map, "attackplayers")?,
                redstone_interact: get_byte_as_bool(&mut map, "doorsandswitches")?,
                build: get_byte_as_bool(&mut map, "build")?,
                mine: get_byte_as_bool(&mut map, "mine")?,
                mine_instantly: get_byte_as_bool(&mut map, "instabuild")?,
                flying: get_byte_as_bool(&mut map, "flying")?,
                fly_speed: get_f32(&mut map, "flySpeed")?,
                walk_speed: get_f32(&mut map, "walkSpeed")?,
                invulnerable: get_byte_as_bool(&mut map, "invulnerable")?,
                lightning: get_byte_as_bool(&mut map, "lightning")?,
                mayfly: get_byte_as_bool(&mut map, "mayfly")?,
                mute: get_byte_as_bool_option(&mut map, "mute")?,
                no_clip: get_byte_as_bool_option(&mut map, "noclip")?,
                open_containers: get_byte_as_bool(&mut map, "opencontainers")?,
                world_builder: get_byte_as_bool_option(&mut map, "worldbuilder")?,
                teleport: get_byte_as_bool(&mut map, "teleport")?,
                op: get_byte_as_bool(&mut map, "op")?,
                permissions_level: match get_int32_option(&mut map, "playerPermissionsLevel")? {
                    Some(0) => Some(PermissionLevel::Default),
                    Some(1) => Some(PermissionLevel::Operator),
                    Some(2) => Some(PermissionLevel::Admin),
                    Some(3) => Some(PermissionLevel::Host),
                    Some(4) => Some(PermissionLevel::Owner),
                    Some(other) => Err(WorldError::FormatError(format!(
                        "Value for `playerPermissionsLevel` is out of bounds, got {:?}",
                        other
                    )))?,
                    None => None,
                },
                permissions_level_default: match get_int32_option(&mut map, "permissionsLevel")? {
                    Some(0) => Some(PermissionLevel::Default),
                    Some(1) => Some(PermissionLevel::Operator),
                    Some(2) => Some(PermissionLevel::Admin),
                    Some(3) => Some(PermissionLevel::Host),
                    Some(4) => Some(PermissionLevel::Owner),
                    Some(other) => Err(WorldError::FormatError(format!(
                        "Value for `playerPermissionsLevel` is out of bounds, got {:?}",
                        other
                    )))?,
                    None => None,
                },
            }),
            other => Err(WorldError::FormatError(format!(
                "Expected root tag in LevelDat abilities to be of type Compound, got {:?}",
                other
            ))),
        }
    }
}
