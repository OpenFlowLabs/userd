use crate::services::tenant::{Tenant, TenantService};
use juniper::{
    EmptySubscription, RootNode, Context,
    graphql_object, FieldResult
};
use crate::services::tenant::TenantInput;
use crate::services::user::{UserService, User, UserInput};
use uuid::Uuid;

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

    fn user(ctx: &RootContext, tenant: Uuid, email: Option<String>, username: Option<String>) -> FieldResult<Option<User>> {
        ctx.user_service.get_user(tenant, email, username)
    }

    fn users(ctx: &RootContext, tenant: Uuid, limit: Option<i32>, offset: Option<i32>) -> FieldResult<Option<Vec<User>>> {
        ctx.user_service.users(tenant, limit.unwrap_or(0) as usize, offset.unwrap_or(0) as usize)
    }
}

#[graphql_object(Context = RootContext)]
impl MutationRoot {
    fn add_tenant(ctx: &RootContext, input: TenantInput) -> FieldResult<Tenant> {
        ctx.tenant_service.add_tenant(input)
    }

    fn add_user(ctx: &RootContext, user: UserInput) -> FieldResult<User> {
        ctx.user_service.add_user(user)
    }
}

pub struct RootContext {
    tenant_service: TenantService,
    user_service: UserService
}

impl Context for RootContext {}

impl RootContext {
    pub fn new(database_url: &str) -> RootContext {
        RootContext {
            tenant_service: TenantService::new(database_url),
            user_service: UserService::new(database_url)
        }
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<RootContext>>;
