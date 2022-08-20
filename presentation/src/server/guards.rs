use rocket::{request::{FromRequest, self}, Request, outcome::Outcome, http::Status};

///Request Guard for the Authorization Bearer HTTP Header
pub(crate) struct BearerToken(pub(crate) String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for BearerToken {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let authorization = req.headers().get_one("Authorization");
        match authorization {
            Some(authorization) => match authorization.contains("Bearer "){
                true => {
                    let authorization = authorization.replace("Bearer ", "");
                    Outcome::Success(BearerToken(authorization))
                },
                false => Outcome::Failure((Status::Unauthorized, ()))
            }
            None => Outcome::Forward(())
        }
    }
}
