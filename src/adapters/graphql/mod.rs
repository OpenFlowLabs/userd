use crate::services::tenant::{Tenant, TenantService};
use juniper::{
    EmptySubscription, RootNode, Context,
    graphql_object, FieldResult
};
use crate::services::tenant::TenantInput;
use crate::adapters::database::tenant::DieselTenantRepository;

pub struct QueryRoot;
pub struct MutationRoot;

#[graphql_object(Context = RootContext)]
impl QueryRoot {
    #[graphql(arguments(name(description = "name of the tenant")))]
    fn tenant(ctx: &RootContext, name: String) -> FieldResult<Option<Tenant>> {
        ctx.tenant_service.get_tenant(&name)
    }

    fn tenants(ctx: &RootContext, limit: Option<i32>, offset: Option<i32>) -> FieldResult<Option<Vec<Tenant>>> {
        ctx.tenant_service.tenants(limit.unwrap_or(0) as usize, offset.unwrap_or(0) as usize)
    }
}

#[graphql_object(Context = RootContext)]
impl MutationRoot {
    fn add_tenant(ctx: &RootContext, input: TenantInput) -> FieldResult<Tenant> {
        ctx.tenant_service.add_tenant(input)
    }
}

pub struct RootContext {
    tenant_service: TenantService<DieselTenantRepository>
}

impl Context for RootContext {}

impl RootContext {
    pub fn new(database_url: &str) -> RootContext {
        RootContext {tenant_service: TenantService::<DieselTenantRepository>::new(database_url) }
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<RootContext>>;
