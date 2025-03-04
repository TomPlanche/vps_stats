use chrono::NaiveDateTime;
use diesel::prelude::{Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::schema::city;

#[derive(Queryable, Insertable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = city)]
#[serde(crate = "rocket::serde")]
pub struct City {
    pub id: i32,
    pub name: String,
    pub country: String,
    pub latitude: Option<f32>,
    pub longitude: Option<f32>,
    pub created_at: Option<NaiveDateTime>,
}
