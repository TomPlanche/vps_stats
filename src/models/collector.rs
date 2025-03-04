use diesel::prelude::{Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::schema::collector;

#[derive(Queryable, Insertable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = collector)]
#[serde(crate = "rocket::serde")]
pub struct Collector {
    pub id: String,
    pub origin: String,
    pub city_id: i32,
    pub os: Option<String>,
    pub browser: Option<String>,
}
