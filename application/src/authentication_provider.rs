use domain::authentication::user::User;

pub trait AuthenticationProvider where Self: Send+Sync{
    fn from_token(&self, token: &str) -> Option<User>;
}