use uuid::Uuid;
use juniper::{
    GraphQLObject, GraphQLInputObject, FieldResult
};
use crate::adapters::database::TenantRepository;
use crate::adapters::database::tenant::{DieselTenantRepository, NewTenant};

pub struct TenantService<T1: TenantRepository> {
    tenant_repository: T1
}

impl TenantService<DieselTenantRepository> {
    pub fn new(database_url: &str) -> TenantService<DieselTenantRepository> {
        TenantService  {
            tenant_repository: DieselTenantRepository::new(database_url),
        }
    }

    pub fn get_tenant(&self, name: &str) -> FieldResult<Option<Tenant>> {
        let result = self.tenant_repository.get_tenant(name)?;
        Ok(Some(result))
    }

    pub fn tenants(&self, limit: usize, offset: usize) -> FieldResult<Option<Vec<Tenant>>> {
        let tenants = self.tenant_repository.tenants(limit, offset)?;
        Ok(Some(tenants))
    }

    pub fn add_tenant(&self, input: TenantInput) -> FieldResult<Tenant> {
        let result = self.tenant_repository.add_tenant(&NewTenant{
            name: &input.name
        })?;
        Ok(result)
    }
}

#[derive(GraphQLInputObject)]
pub struct TenantInput {
    name: String
}

#[derive(GraphQLObject, Default, Clone, Queryable)]
pub struct Tenant {
    pub id: Uuid,
    pub name: String,
}