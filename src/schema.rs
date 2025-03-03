// @generated automatically by Diesel CLI.

diesel::table! {
    city (id) {
        id -> Nullable<Integer>,
        name -> Text,
        country -> Text,
        latitude -> Nullable<Float>,
        longitude -> Nullable<Float>,
    }
}

diesel::table! {
    collector (id) {
        id -> Text,
        origin -> Text,
        city_id -> Integer,
        os -> Nullable<Text>,
        browser -> Nullable<Text>,
        timestamp -> Timestamp,
    }
}

diesel::table! {
    event (id) {
        id -> Text,
        url -> Text,
        referrer -> Nullable<Text>,
        name -> Text,
        timestamp -> Timestamp,
        collector_id -> Text,
    }
}

diesel::joinable!(collector -> city (city_id));

diesel::allow_tables_to_appear_in_same_query!(
    city,
    collector,
    event,
);
