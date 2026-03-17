use sea_orm::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "todos")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub title: String,
    pub completed: bool,
}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn to_domain(&self) -> crate::domain::models::Todo {
        crate::domain::models::Todo {
            id: self.id,
            title: self.title.clone(),
            completed: self.completed,
        }
    }
}
