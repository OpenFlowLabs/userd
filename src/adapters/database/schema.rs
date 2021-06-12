table! {
    permissions (user_id, tenant_id, permission) {
        tenant_id -> Uuid,
        user_id -> Uuid,
        permission -> Varchar,
    }
}

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

joinable!(permissions -> tenants (tenant_id));
joinable!(permissions -> users (user_id));
joinable!(user_confirmations -> tenants (tenant_id));
joinable!(user_confirmations -> users (user_id));

allow_tables_to_appear_in_same_query!(
    permissions,
    tenants,
    user_confirmations,
    users,
);
