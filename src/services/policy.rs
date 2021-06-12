use juniper::{GraphQLInputObject};

const RESOURCE_SCOPE_ALL: &str = "*";

#[derive(Debug, Default, GraphQLInputObject)]
pub struct PermissionPolicy {
    pub service: String,
    pub permission: String,
    pub resource_scope: Option<String>,
}

pub fn is_string_a_valid_policy(s: &str) -> bool {
    s.matches(":").count() == 2
}

impl PermissionPolicy {
    pub fn new(service: &str, permission: &str, resource_scope: Option<&str>) -> Self {
        PermissionPolicy{
            service: service.into(),
            permission: permission.into(),
            resource_scope: if let Some(scope) = resource_scope { Some(scope.into()) } else { Some(RESOURCE_SCOPE_ALL.into()) }
        }
    }
}

impl From<String> for PermissionPolicy {
    fn from(s: String) -> Self {
        if !is_string_a_valid_policy(&s) {
            panic!("Invalid policy string passed to from string for PermissionPolicy: got {}", s)
        }

        let mut iter = s.splitn(3, ':');
        let service = iter.next().unwrap();
        let permission = iter.next().unwrap();
        let resource_scope = iter.next().unwrap();
        PermissionPolicy{
            service: service.into(),
            permission: permission.into(),
            resource_scope: Some(resource_scope.into()),
        }
    }
}

impl Into<String> for PermissionPolicy {
    fn into(self) -> String {
        format!("{}:{}:{}", self.service, self.permission, if let Some(scope) = &self.resource_scope {scope} else {RESOURCE_SCOPE_ALL})
    }
}