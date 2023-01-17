use crate::routes::product::product::ProductRow;
use actix_web::{get, post, web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct FormData {
    name: String,
    description: String,
    attributes: String,
    images_urls: String,
}

#[derive(serde::Serialize)]
pub struct Product {
    name: String,
    description: String,
    attributes: String,
    images_urls: String,
}

impl Product {
    pub fn new(
        name: String,
        description: String,
        attributes: String,
        images_urls: String,
    ) -> Product {
        Product {
            name,
            description,
            attributes,
            images_urls,
        }
    }
}

#[get("/products")]
pub async fn get_products(pool: web::Data<PgPool>) -> HttpResponse {
    match retrieve_products(&pool).await {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn retrieve_products(pool: &PgPool) -> Result<Vec<Product>, sqlx::Error> {
    let rows = sqlx::query_as!(ProductRow, "SELECT * FROM products")
        .fetch_all(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            e
        })?;

    let results: Vec<Product> = rows
        .iter()
        .map(|item| {
            Product::new(
                item.name.to_string(),
                item.description.to_string(),
                item.attributes.to_string(),
                item.images_urls.to_string(),
            )
        })
        .collect();

    Ok(results)
}

#[tracing::instrument(
    name = "Adding a new product", 
    skip(form, pool),
    fields(name = %form.name, description = %form.description
    ) )]
#[post("/products")]
pub async fn post_products(form: web::Json<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    match insert_product(&pool, &form).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(name = "Saving new product in the database", skip(form, pool))]
async fn insert_product(pool: &PgPool, form: &FormData) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
    INSERT INTO products (id, name, description, attributes, images_urls, created_at, updated_at)
    VALUES ($1, $2, $3, $4, $5, $6, $7)
"#,
        Uuid::new_v4(),
        form.name,
        form.description,
        form.attributes,
        form.images_urls,
        Utc::now(),
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;

    Ok(())
}
