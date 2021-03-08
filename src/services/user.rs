use uuid::Uuid;
use juniper::{GraphQLObject, GraphQLInputObject, FieldResult, FieldError, graphql_value};
use crate::adapters::database::user::{UserRepository, NewUser, DBUser};
use pwhash::bcrypt;

pub struct UserService {
    user_repository: UserRepository
}

impl UserService {
    pub fn new(database_url: &str) -> UserService {
        UserService {
            user_repository: UserRepository::new(database_url),
        }
    }

    pub fn get_user(&self, tenant: Uuid, email: Option<String>, name: Option<String>) -> FieldResult<Option<User>> {
        match email {
            Some(e) => {
                let result = self.user_repository.get_user_by_email(tenant, e)?;
                return Ok(Some(User::from(result)));
            }
            _ => {}
        }
        match name {
            Some(n) => {
                let result = self.user_repository.get_user_by_name(tenant, n)?;
                return Ok(Some(User::from(result)));
            }
            _ => {}
        }
        Err(FieldError::new(
            "either username or email must be provided",
            graphql_value!({ "bad_request": "not enough arguments" })
        ))
    }

    pub fn users(&self, tenant: Uuid, limit: usize, offset: usize) -> FieldResult<Option<Vec<User>>> {
        let result = self.user_repository.users(tenant, limit, offset)?;
        let ret_vec = result.into_iter().map(|db_user| User::from(db_user)).collect();
        Ok(Some(ret_vec))
    }

    pub fn add_user(&self, input: UserInput) -> FieldResult<User> {
        let result = self.user_repository.add_user(&NewUser{
            username: &input.username,
            tenant_id: &input.tenant,
            email: &input.email,
            pwhash: &bcrypt::hash(input.password)?
        })?;
        //TODO send mail to user
        Ok(User::from(result.0))
    }
}

#[derive(GraphQLInputObject)]
pub struct UserInput {
    username: String,
    password: String,
    tenant: Uuid,
    email: String,
}

#[derive(GraphQLObject, Default, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub tenant_id: Uuid,
    pub email: String,
    pub email_confirmed: bool
}

impl From<DBUser> for User {
    fn from(db_user: DBUser) -> Self {
        User{
            id: db_user.id,
            username: db_user.username,
            tenant_id: db_user.tenant_id,
            email: db_user.email,
            email_confirmed: db_user.email_confirmed
        }
    }
}