use axum::async_trait;

use super::super::entity::{
    user::{
        LoginError, LoginSession, RefreshToken, RefreshTokenError, RefreshTokenExtract, SignUpCode,
        SignUpError, UserEntity, UserEntityForCreation, UserError,
    },
    Pid,
};

#[async_trait]
pub trait UserRepository {
    async fn get_user(&self, id: Pid) -> Result<UserEntity, UserError>;

    async fn get_user_from_subject(&self, subject: &str) -> Result<UserEntity, UserError>;

    async fn get_user_id_from_subject(&self, subject: &str) -> Result<Pid, UserError>;

    async fn create_user(
        &self,
        subject: String,
        user: UserEntityForCreation,
    ) -> Result<Pid, UserError>;

    async fn does_exist_user_id(&self, user_id: Pid) -> Result<bool, UserError>;

    async fn make_login_session(&self) -> Result<LoginSession, LoginError>;

    /// ログインセッションに紐づくログイン要求か検証し、
    /// その場合にIdPの提供するユーザ識別子を返す。
    /// 二度目以降の呼び出しではErrになる。
    async fn fetch_user_subject(
        &self,
        session_id: String,
        code: String,
    ) -> Result<String, LoginError>;

    /// ユーザ作成用のone-time codeを発行する。
    async fn issue_sign_up_code(&self, subject: String) -> Result<SignUpCode, SignUpError>;

    /// ユーザ作成用のcodeを検証する。
    /// IdP提供のsubjectを返却する。
    async fn verify_sign_up_code(&self, code: SignUpCode) -> Result<String, SignUpError>;

    /// 新しいrefresh tokenを発行する。
    /// 古いrefresh tokenがある場合は無効になる。
    async fn issue_refresh_token(&self, userid: Pid) -> Result<RefreshToken, RefreshTokenError>;

    /// Refresh tokenを検証する。
    /// 紐づけられたuser idを返す。
    async fn verify_refresh_token(
        &self,
        token: RefreshTokenExtract,
    ) -> Result<Pid, RefreshTokenError>;
}
