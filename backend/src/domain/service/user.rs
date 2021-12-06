use axum::{
    async_trait,
    extract::{Extension, FromRequest, RequestParts, TypedHeader},
};
use chrono::{Duration, Utc};
use headers::{authorization::Bearer, Authorization};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

use crate::domain::entity::{
    user::{
        AccessToken, AccessTokenClaims, LoginError, LoginSession, RefreshToken, RefreshTokenError,
        RefreshTokenExtract, SignUpCode, SignUpError, UserEntityForCreation, UserError,
    },
    AxumError, Pid,
};
use crate::domain::repo_if::user::UserRepository;
use crate::infra::repo::user::UserRepositoryImpl;
use crate::settings::Settings;

pub struct UserService {
    settings: Settings,
    user_repository: UserRepositoryImpl,
}

#[async_trait]
impl<B> FromRequest<B> for UserService
where
    B: Send,
{
    type Rejection = AxumError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Extension(settings) = Extension::<Settings>::from_request(req)
            .await
            .map_err(|_| AxumError::OtherError("Settings extension error".to_string()))?;
        let user_repository = UserRepositoryImpl::from_request(req).await?;

        Ok(Self {
            settings,
            user_repository,
        })
    }
}

impl UserService {
    pub async fn make_login_session(&self) -> Result<LoginSession, LoginError> {
        self.user_repository.make_login_session().await
    }

    async fn issue_tokens(
        &self,
        uid: Pid,
    ) -> Result<(RefreshToken, AccessToken), RefreshTokenError> {
        let refresh_token = self.user_repository.issue_refresh_token(uid).await?;
        let access_token = self.issue_access_token(uid)?;
        Ok((refresh_token, access_token))
    }

    pub async fn login(
        &self,
        session_id: String,
        code: String,
    ) -> Result<(RefreshToken, AccessToken), LoginError> {
        let subject = self
            .user_repository
            .fetch_user_subject(session_id, code)
            .await?;
        tracing::info!("sub: {}", subject);

        match self
            .user_repository
            .get_user_id_from_subject(&subject)
            .await
        {
            Ok(uid) => self.issue_tokens(uid).await.map_err(|_| LoginError::Other),
            Err(UserError::Nonexistent) => {
                let code = self
                    .user_repository
                    .issue_sign_up_code(subject)
                    .await
                    .map_err(|_| LoginError::Other)?;
                Err(LoginError::Nonexistent(code))
            }
            _ => Err(LoginError::Other),
        }
    }

    pub async fn sign_up(
        &self,
        code: SignUpCode,
        user: UserEntityForCreation,
    ) -> Result<(RefreshToken, AccessToken), SignUpError> {
        let subject = self.user_repository.verify_sign_up_code(code).await?;

        let uid = self
            .user_repository
            .create_user(subject, user)
            .await
            .map_err(|_| SignUpError::DuplicatedUser)?;
        self.issue_tokens(uid).await.map_err(|_| SignUpError::Other)
    }

    pub async fn refresh_tokens(
        &self,
        refresh_token: RefreshTokenExtract,
    ) -> Result<(RefreshToken, AccessToken), RefreshTokenError> {
        let uid = self
            .user_repository
            .verify_refresh_token(refresh_token)
            .await?;

        self.issue_tokens(uid).await
    }

    fn issue_access_token(&self, uid: Pid) -> Result<AccessToken, RefreshTokenError> {
        let expires_at = Utc::now() + Duration::seconds(self.settings.access_exp as i64);
        let claims = AccessTokenClaims::new(
            self.settings.access_iss.to_owned(),
            uid,
            expires_at.timestamp() as usize,
        );

        let secret =
            EncodingKey::from_base64_secret(&self.settings.access_secret).map_err(|err| {
                tracing::error!("in issue_access_token: secret key decoding error: {}", err);
                RefreshTokenError::Other
            })?;

        encode(&Header::default(), &claims, &secret)
            .map(AccessToken)
            .map_err(|err| {
                tracing::error!("in issue_access_token: encoding error: {}", err);
                RefreshTokenError::Other
            })
    }
}

pub struct UserId(pub Pid);

#[async_trait]
impl<B> FromRequest<B> for UserId
where
    B: Send,
{
    type Rejection = AxumError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Extension(settings) = Extension::<Settings>::from_request(req)
            .await
            .map_err(|_| AxumError::OtherError("Settings extension error".to_string()))?;
        let secret = DecodingKey::from_base64_secret(&settings.access_secret).map_err(|err| {
            tracing::error!("in UserId: secret key decoding error: {}", err);
            AxumError::OtherError("secret key decoding error".to_string())
        })?;
        let user_repository = UserRepositoryImpl::from_request(req)
            .await
            .map_err(|_| AxumError::OtherError(String::new()))?;

        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request(req)
                .await
                .map_err(|_| AxumError::MissingAccessToken)?;

        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = false;
        validation.iss = Some(settings.access_iss);

        let id = decode::<AccessTokenClaims>(bearer.token(), &secret, &validation)
            .map_err(|err| {
                tracing::info!("in UserId: invalid token: {}", err);
                AxumError::InvalidAccessToken
            })?
            .claims
            .user_id();

        if user_repository
            .does_exist_user_id(id)
            .await
            .map_err(|_| AxumError::OtherError(String::new()))?
        {
            Ok(Self(id))
        } else {
            Err(AxumError::InvalidAccessToken)
        }
    }
}
