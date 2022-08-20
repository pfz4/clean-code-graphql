use application::authentication_provider::AuthenticationProvider;
use domain::authentication::{user::User, permission::Permission};

#[derive(Debug)]
pub struct DemoAuthenticationProvider;

impl Default for DemoAuthenticationProvider{
    fn default() -> Self {
        Self {  }
    }
}

impl AuthenticationProvider for DemoAuthenticationProvider {
    fn from_token(&self, token: &str) -> Option<User> {
        if token == "123456" {
            Some(User{
                name: "Max Mustermann".to_string(),
                permissions: vec![
                    Permission::CreateArtist,
                    Permission::DeleteArtist,
                    Permission::CreateSong
                ]
            })
        }else{
            None
        }
    }
}