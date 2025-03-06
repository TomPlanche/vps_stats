use diesel::{define_sql_function, sql_types::Text};

define_sql_function!(fn lower(x: Text) -> Text);
