//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user_settings")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub index: i32,
    pub afk_timeout: Option<i32>,
    pub allow_accessibility_detection: Option<bool>,
    pub animate_emoji: Option<bool>,
    pub animate_stickers: Option<i32>,
    pub contact_sync_enabled: Option<bool>,
    pub convert_emoticons: Option<bool>,
    pub custom_status: Option<String>,
    pub default_guilds_restricted: Option<bool>,
    pub detect_platform_accounts: Option<bool>,
    pub developer_mode: Option<bool>,
    pub disable_games_tab: Option<bool>,
    pub enable_tts_command: Option<bool>,
    pub explicit_content_filter: Option<i32>,
    pub friend_source_flags: Option<String>,
    pub gateway_connected: Option<bool>,
    pub gif_auto_play: Option<bool>,
    pub guild_folders: Option<String>,
    pub guild_positions: Option<String>,
    pub inline_attachment_media: Option<bool>,
    pub inline_embed_media: Option<bool>,
    pub locale: Option<String>,
    pub message_display_compact: Option<bool>,
    pub native_phone_integration_enabled: Option<bool>,
    pub render_embeds: Option<bool>,
    pub render_reactions: Option<bool>,
    pub restricted_guilds: Option<String>,
    pub show_current_game: Option<bool>,
    pub status: Option<String>,
    pub stream_notifications_enabled: Option<bool>,
    pub theme: Option<String>,
    pub timezone_offset: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::users::Entity")]
    Users,
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}