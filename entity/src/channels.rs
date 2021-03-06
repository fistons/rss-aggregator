//! SeaORM Entity. Generated by sea-orm-codegen 0.8.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "channels"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Serialize, Deserialize)]
pub struct Model {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub registration_timestamp: DateTimeWithTimeZone,
    pub last_update: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    Name,
    Url,
    RegistrationTimestamp,
    LastUpdate,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Id,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = i32;
    fn auto_increment() -> bool {
        true
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    ChannelUsers,
    Items,
    UsersItems,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::Integer.def(),
            Self::Name => ColumnType::String(Some(512u32)).def(),
            Self::Url => ColumnType::String(Some(512u32)).def(),
            Self::RegistrationTimestamp => ColumnType::TimestampWithTimeZone.def(),
            Self::LastUpdate => ColumnType::TimestampWithTimeZone.def().null(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::ChannelUsers => Entity::has_many(super::channel_users::Entity).into(),
            Self::Items => Entity::has_many(super::items::Entity).into(),
            Self::UsersItems => Entity::has_many(super::users_items::Entity).into(),
        }
    }
}

impl Related<super::channel_users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ChannelUsers.def()
    }
}

impl Related<super::items::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Items.def()
    }
}

impl Related<super::users_items::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UsersItems.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
