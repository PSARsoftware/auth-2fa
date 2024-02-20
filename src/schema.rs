// @generated automatically by Diesel CLI.

diesel::table! {
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
    auth_users (id) {
        #[max_length = 50]
        id -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 100]
        password -> Nullable<Varchar>,
        otp_enabled -> Nullable<Bool>,
        otp_verified -> Nullable<Bool>,
        #[max_length = 100]
        otp_base32 -> Nullable<Varchar>,
        #[max_length = 100]
        otp_auth_url -> Nullable<Varchar>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    _sqlx_migrations,
    auth_users,
);
