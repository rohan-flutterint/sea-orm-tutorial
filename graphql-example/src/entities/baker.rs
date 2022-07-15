//! SeaORM Entity. Generated by sea-orm-codegen

use async_graphql::SimpleObject;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, SimpleObject)]
#[graphql(complex, name = "Baker")]
#[sea_orm(table_name = "baker")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub contact_details: Option<Json>,
    pub bakery_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::bakery::Entity",
        from = "Column::BakeryId",
        to = "super::bakery::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Bakery,
}

impl Related<super::bakery::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Bakery.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
