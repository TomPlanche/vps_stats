// @generated automatically by Diesel CLI.

diesel::table! {
    city (id) {
        id -> Nullable<Integer>,
        name -> Text,
        country -> Text,
        latitude -> Nullable<Float>,
        longitude -> Nullable<Float>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    collector (id) {
        id -> Text,
        origin -> Text,
        city_id -> Integer,
        os -> Nullable<Text>,
        browser -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    event (id) {
        id -> Text,
        url -> Text,
        referrer -> Nullable<Text>,
        name -> Text,
        collector_id -> Text,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(collector -> city (city_id));
diesel::joinable!(event -> collector (collector_id));

diesel::allow_tables_to_appear_in_same_query!(
    city,
    collector,
    event,
);
