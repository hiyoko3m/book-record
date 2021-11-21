use axum::{
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
};
use serde::Deserialize;

use crate::domain::entity::{
    book::{BookEntity, BookEntityForCreation},
    user::{AccessToken, RefreshToken},
};

#[derive(Debug, Deserialize)]
pub struct BookExtract {
    title: String,
}

impl From<BookExtract> for BookEntityForCreation {
    fn from(book_extract: BookExtract) -> BookEntityForCreation {
        Self {
            title: book_extract.title,
        }
    }
}

impl From<(u32, BookExtract)> for BookEntity {
    fn from((id, book_extract): (u32, BookExtract)) -> BookEntity {
        Self {
            id: id,
            title: book_extract.title,
        }
    }
}

#[derive(Debug)]
pub struct CredTokens {
    refresh_token: RefreshToken,
    access_token: AccessToken,
}

impl From<(RefreshToken, AccessToken)> for CredTokens {
    fn from((refresh_token, access_token): (RefreshToken, AccessToken)) -> CredTokens {
        Self {
            refresh_token,
            access_token,
        }
    }
}

impl IntoResponse for CredTokens {
    type Body = Body;
    type BodyError = <Self::Body as axum::body::HttpBody>::Error;

    fn into_response(self) -> Response<Self::Body> {
        unimplemented!();
        /*
        let cookie_value = format!(
            "refresh_token={}; Expires={}; Path=/; HttpOnly",
            refresh_token.token,
            refresh_token.expires_at.to_rfc2822()
        );
        (Headers(vec![("Set-Cookie", cookie_value)]), access_token.0)*/
    }
}
