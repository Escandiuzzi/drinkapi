use chrono::Utc;
use uuid::Uuid;

#[derive(sqlx::FromRow)]
pub struct ProductRow {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub attributes: String,
    pub images_urls: String,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}