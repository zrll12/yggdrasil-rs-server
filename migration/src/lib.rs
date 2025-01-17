pub use sea_orm_migration::prelude::*;

mod m20240708_142859_create_user_table;
mod m20240708_155447_create_profile_table;
mod m20240709_091713_create_texture_table;
mod m20240805_064539_add_username;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240708_142859_create_user_table::Migration),
            Box::new(m20240708_155447_create_profile_table::Migration),
            Box::new(m20240709_091713_create_texture_table::Migration),
            Box::new(m20240805_064539_add_username::Migration),
        ]
    }
}