use serde::{Serialize, Deserialize};
use sqlx::{sqlite::{SqlitePool, SqliteRow}, query, Row};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Location {
    pub id: i64,
    pub tracker_id: String,
    pub latitude: f32,
    pub longitude: f32,
    pub timestamp: i64,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} => ({}, {})", self.id, self.latitude, self.longitude)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewLocation {
    pub tid: String,
    pub lat: f32,
    pub lon: f32,
    pub tst: i64,
}


impl Location{
    fn from_row(row: SqliteRow) -> Self{
        Self {
            id: row.get("id"),
            tracker_id: row.get("tracker_id"),
            latitude: row.get("latitude"),
            longitude: row.get("longitude"),
            timestamp: row.get("timestamp"),
        }
    }
    pub async fn create(pool: &SqlitePool, tracker_id: &str, latitude: f32,
            longitude: f32, timestamp: i64) -> Result<Location, sqlx::Error>{
        let sql = "INSERT INTO locations
                  (tracker_id, latitude, longitude, timestamp)
                   VALUES ($1, $2, $3, $4) RETURNING * ;";
        query(sql)
            .bind(tracker_id)
            .bind(latitude)
            .bind(longitude)
            .bind(timestamp)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }

    pub async fn read(pool: &SqlitePool, id: i64) -> Result<Location, sqlx::Error>{
        let sql = "SELECT * FROM locations WHERE id = $1";
        query(sql)
            .bind(id)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }

    pub async fn read_all(pool: &SqlitePool) -> Result<Vec<Location>, sqlx::Error>{
        let sql = "SELECT * FROM locations";
        query(sql)
            .map(Self::from_row)
            .fetch_all(pool)
            .await
    }

    pub async fn update(pool: &SqlitePool, location: Location)
        ->Result<Location, sqlx::Error>{
        let sql = "UPDATE locations
            SET
                tracker_id = COALESCE($2, tracker_id),
                latitude = COALESCE($3, latitude),
                longitude = COALESCE($4, longitude),
                timestamp = COALESCE($5, timestamp)
            WHERE id = $1 RETURNING *;";
        query(sql)
            .bind(location.id)
            .bind(location.tracker_id)
            .bind(location.latitude)
            .bind(location.longitude)
            .bind(location.timestamp)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }
    pub async fn delete(pool: &SqlitePool, id: i64) -> Result<Location, sqlx::Error>{
        let sql = "DELETE FROM locations WHERE id = $1 RETURNING *;";
        query(sql)
            .bind(id)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
    }
}

