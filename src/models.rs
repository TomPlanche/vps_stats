use crate::schema::{city, collector, event};

use chrono::NaiveDateTime;
use diesel::{Associations, Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = city)]
pub struct City {
    pub id: i32,
    pub name: String,
    pub country: String,
    pub latitude: Option<f32>,
    pub longitude: Option<f32>,
}

#[derive(Queryable, Insertable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = collector)]
pub struct Collector {
    pub id: String,
    pub origin: String,
    pub city_id: i32,
    pub os: Option<String>,
    pub browser: Option<String>,
    pub timestamp: NaiveDateTime,
}

#[derive(Queryable, Associations, Identifiable, Serialize, Deserialize)]
#[diesel(belongs_to(Collector, foreign_key = collector_id))]
#[diesel(table_name = event)]
pub struct Event {
    pub id: String,
    pub url: String,
    pub referrer: Option<String>,
    pub name: String,
    pub timestamp: NaiveDateTime,
    pub collector_id: String,
}

#[derive(Deserialize)]
pub struct EventRequest {
    pub url: String,
    pub name: String,
    pub collector_id: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = event)]
pub struct NewEvent {
    pub id: String,
    pub url: String,
    pub referrer: Option<String>,
    pub name: String,
    pub timestamp: NaiveDateTime,
    pub collector_id: String,
}
