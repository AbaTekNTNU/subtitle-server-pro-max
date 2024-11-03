// @generated automatically by Diesel CLI.

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

    lines (id) {
        id -> Int4,
        line -> Text,
        song_id -> Int4,
        position -> Vector,
        cam_position -> Vector,
        cam_look_at -> Vector,
        keep_n_last -> Int4,
        rotation -> Nullable<Vector>,
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

diesel::allow_tables_to_appear_in_same_query!(_sqlx_migrations, lines, song,);
