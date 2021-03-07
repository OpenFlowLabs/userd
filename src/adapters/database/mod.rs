pub mod tenant;
pub mod schema;

use juniper::FieldResult;
use crate::services::tenant::Tenant;
use crate::adapters::database::tenant::NewTenant;

pub trait TenantRepository {
    fn get_tenant(&self, name: &str) -> FieldResult<Tenant>;

    fn tenants(&self, limit: usize, offset: usize) -> FieldResult<Vec<Tenant>>;

    fn add_tenant(&self, tenant: &NewTenant) -> FieldResult<Tenant>;
}