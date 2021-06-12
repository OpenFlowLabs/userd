-- Your SQL goes here

CREATE TABLE permissions (
    tenant_id uuid NOT NULL,
    user_id uuid NOT NULL,
    permission varchar NOT NULL,
    PRIMARY KEY (user_id, tenant_id, permission),
    CONSTRAINT fk_user_tenant_permissions
        FOREIGN KEY(tenant_id)
        REFERENCES tenants(id)
        ON DELETE NO ACTION,
    CONSTRAINT fk_user_tenant
        FOREIGN KEY(user_id)
        REFERENCES users(id)
        ON DELETE NO ACTION
)