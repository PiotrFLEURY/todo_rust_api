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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_to_domain() {
        let model = Model {
            id: 1,
            title: "Test Todo".to_string(),
            completed: false,
        };

        let domain = model.to_domain();

        assert_eq!(domain.id, 1);
        assert_eq!(domain.title, "Test Todo");
        assert_eq!(domain.completed, false);
    }
}
