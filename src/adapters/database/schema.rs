table! {
    tenants (id) {
        id -> Uuid,
        name -> Varchar,
    }
}

table! {
    user_confirmations (user_id) {
        user_id -> Uuid,
        tenant_id -> Uuid,
        token -> Varchar,
        email -> Varchar,
    }
}

table! {
    users (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        username -> Varchar,
        pwhash -> Varchar,
        email -> Varchar,
        email_confirmed -> Bool,
    }
}

joinable!(user_confirmations -> tenants (tenant_id));
joinable!(user_confirmations -> users (user_id));
joinable!(users -> tenants (tenant_id));

allow_tables_to_appear_in_same_query!(
    tenants,
    user_confirmations,
    users,
);
