//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "channels")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub created_at: DateTime,
    pub name: Option<String>,
    pub icon: Option<String>,
    pub r#type: i32,
    pub last_message_id: Option<String>,
    pub guild_id: Option<String>,
    pub parent_id: Option<String>,
    pub owner_id: Option<String>,
    pub last_pin_timestamp: Option<i32>,
    pub default_auto_archive_duration: Option<i32>,
    pub position: Option<i32>,
    pub permission_overwrites: Option<String>,
    pub video_quality_mode: Option<i32>,
    pub bitrate: Option<i32>,
    pub user_limit: Option<i32>,
    pub nsfw: bool,
    pub rate_limit_per_user: Option<i32>,
    pub topic: Option<String>,
    pub retention_policy_id: Option<String>,
    pub flags: i32,
    pub default_thread_rate_limit_per_user: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "Entity",
        from = "Column::ParentId",
        to = "Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    SelfRef,
    #[sea_orm(
        belongs_to = "super::guilds::Entity",
        from = "Column::GuildId",
        to = "super::guilds::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Guilds,
    #[sea_orm(has_many = "super::invites::Entity")]
    Invites,
    #[sea_orm(has_many = "super::messages::Entity")]
    Messages,
    #[sea_orm(has_many = "super::read_states::Entity")]
    ReadStates,
    #[sea_orm(has_many = "super::recipients::Entity")]
    Recipients,
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::OwnerId",
        to = "super::users::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Users,
    #[sea_orm(has_many = "super::voice_states::Entity")]
    VoiceStates,
    #[sea_orm(has_many = "super::webhooks::Entity")]
    Webhooks,
}

impl Related<super::guilds::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Guilds.def()
    }
}

impl Related<super::invites::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Invites.def()
    }
}

impl Related<super::read_states::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ReadStates.def()
    }
}

impl Related<super::recipients::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Recipients.def()
    }
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

impl Related<super::voice_states::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::VoiceStates.def()
    }
}

impl Related<super::webhooks::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Webhooks.def()
    }
}

impl Related<super::messages::Entity> for Entity {
    fn to() -> RelationDef {
        super::message_channel_mentions::Relation::Messages.def()
    }
    fn via() -> Option<RelationDef> {
        Some(
            super::message_channel_mentions::Relation::Channels
                .def()
                .rev(),
        )
    }
}

impl ActiveModelBehavior for ActiveModel {}
