use axum::async_trait;

use super::super::entity::{
    user::{
        LoginSession, RefreshToken, RefreshTokenExtract, SignUpCode, UserEntity,
        UserEntityForCreation,
    },
    PID,
};

#[async_trait]
pub trait UserRepository {
    async fn get_user(&self, id: PID) -> Option<UserEntity>;

    async fn get_user_from_sub(&self, sub: &str) -> Option<UserEntity>;

    async fn get_user_id_from_sub(&self, sub: &str) -> Option<PID>;

    async fn create_user(&self, sub: String, user: UserEntityForCreation) -> Result<PID, ()>;

    async fn make_login_session(&self) -> LoginSession;

    /// ログインセッションに紐づくログイン要求か検証し、
    /// その場合にIdPの提供するユーザ識別子を返す。
    /// 二度目以降の呼び出しではNoneになる。
    async fn fetch_authed_user(&self, session_id: String, code: String) -> Option<String>;

    /// ユーザ作成用のone-time codeを発行する。
    async fn issue_sign_up_code(&self, sub: String) -> SignUpCode;

    /// ユーザ作成用のcodeを検証する。
    /// IdP提供のsubを返却する。
    async fn verify_sign_up_code(&self, code: SignUpCode) -> Option<String>;

    /// 新しいrefresh tokenを発行する。
    /// 古いrefresh tokenがある場合は無効になる。
    async fn issue_refresh_token(&self, userid: PID) -> RefreshToken;

    /// Refresh tokenを検証する。
    /// 紐づけられたuser idを返す。
    async fn verify_refresh_token(&self, token: RefreshTokenExtract) -> Option<PID>;
}
