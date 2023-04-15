//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "message_role_mentions")]
pub struct Model {
    #[sea_orm(column_name = "messagesId", primary_key, auto_increment = false)]
    pub messages_id: String,
    #[sea_orm(column_name = "rolesId", primary_key, auto_increment = false)]
    pub roles_id: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::messages::Entity",
        from = "Column::MessagesId",
        to = "super::messages::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Messages,
    #[sea_orm(
        belongs_to = "super::roles::Entity",
        from = "Column::RolesId",
        to = "super::roles::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Roles,
}

impl Related<super::messages::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Messages.def()
    }
}

impl Related<super::roles::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Roles.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
