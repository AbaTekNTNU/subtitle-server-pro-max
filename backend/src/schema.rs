// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "animation"))]
    pub struct Animation;
}

diesel::table! {
    use diesel::sql_types::*;
    use pgvector::sql_types::*;

    _sqlx_migrations (version) {
        version -> Int8,
        description -> Text,
        installed_on -> Timestamptz,
        success -> Bool,
        checksum -> Bytea,
        execution_time -> Int8,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use pgvector::sql_types::*;
    use super::sql_types::Animation;

    lines (id) {
        id -> Int4,
        line -> Text,
        song_id -> Int4,
        position -> Vector,
        cam_position -> Vector,
        cam_position_duration -> Nullable<Int4>,
        cam_look_at -> Vector,
        keep_n_last -> Int4,
        rotation -> Nullable<Vector>,
        text_animation -> Nullable<Animation>,
        text_position_duration -> Nullable<Int4>,
        cam_rotation -> Nullable<Vector>,
        end_position -> Nullable<Vector>,
        cam_end_position -> Nullable<Vector>,
        cam_end_look_at -> Nullable<Vector>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use pgvector::sql_types::*;

    song (id) {
        id -> Int4,
        name -> Text,
    }
}

diesel::joinable!(lines -> song (song_id));

diesel::allow_tables_to_appear_in_same_query!(
    _sqlx_migrations,
    lines,
    song,
);
