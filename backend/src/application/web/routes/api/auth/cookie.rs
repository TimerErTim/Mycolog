use std::time::Duration;

use axum_extra::extract::cookie::{Cookie, SameSite};

use crate::application::database::system::AuthToken;

pub fn build_auth_cookie(token: AuthToken, remember: bool) -> Cookie<'static> {
    let mut builder = Cookie::build(("auth", token.to_insecure()))
        .secure(true)
        .http_only(true)
        .same_site(SameSite::Strict)
        .path("/api")
        .domain("mycolog.net");
    if remember {
        builder = builder.max_age(Duration::from_days(30).try_into().unwrap());
    }
    builder.build()
}
