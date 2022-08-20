use super::permission::Permission;

pub struct User{
    pub name: String,
    pub permissions: Vec<Permission>
}