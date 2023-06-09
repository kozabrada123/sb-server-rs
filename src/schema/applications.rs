//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "applications")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
    pub description: Option<String>,
    pub summary: Option<String>,
    pub r#type: Option<String>,
    pub hook: bool,
    pub bot_public: bool,
    pub bot_require_code_grant: bool,
    pub verify_key: String,
    pub flags: i32,
    pub redirect_uris: Option<String>,
    pub rpc_application_state: Option<i32>,
    pub store_application_state: Option<i32>,
    pub verification_state: Option<i32>,
    pub interactions_endpoint_url: Option<String>,
    pub integration_public: Option<bool>,
    pub integration_require_code_grant: Option<bool>,
    pub discoverability_state: Option<i32>,
    pub discovery_eligibility_flags: Option<i32>,
    pub tags: Option<String>,
    pub cover_image: Option<String>,
    pub install_params: Option<String>,
    pub terms_of_service_url: Option<String>,
    pub privacy_policy_url: Option<String>,
    pub owner_id: Option<String>,
    pub bot_user_id: Option<String>,
    pub team_id: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::messages::Entity")]
    Messages,
    #[sea_orm(
        belongs_to = "super::teams::Entity",
        from = "Column::TeamId",
        to = "super::teams::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Teams,
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::BotUserId",
        to = "super::users::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Users2,
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::OwnerId",
        to = "super::users::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Users1,
    #[sea_orm(has_many = "super::webhooks::Entity")]
    Webhooks,
}

impl Related<super::messages::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Messages.def()
    }
}

impl Related<super::teams::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Teams.def()
    }
}

impl Related<super::webhooks::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Webhooks.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
