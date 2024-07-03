use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlPool;
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct HardwareGoods {
    pub id: i32,
    pub goods_name: String,
    pub stock: i32,
    pub price: f64,
}

impl HardwareGoods {
    pub async fn find_all(pool: &MySqlPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, HardwareGoods>(
            "SELECT id, goods_name, stock, price FROM hardware_goods",
        )
        .fetch_all(pool)
        .await
    }

    // Add more methods as needed
}
