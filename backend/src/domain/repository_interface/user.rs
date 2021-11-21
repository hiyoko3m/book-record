use axum::async_trait;

use super::super::entity::{
    user::{AccessToken, RefreshToken, UserEntity, UserEntityForCreation},
    PID,
};

#[async_trait]
pub trait UserRepositoryInterface {
    async fn get_user(&self, id: PID) -> Option<UserEntity>;

    async fn get_user_from_sub(&self, sub: String) -> Option<UserEntity>;

    async fn create_user(&self, user: UserEntityForCreation) -> Result<u32, ()>;

    async fn issue_nonce(&self) -> String;

    /// 発行したnonceかどうかの検証を行う。
    /// 二度目以降の呼び出しではfalseとなる。
    async fn verity_nonce(&self, nonce: String) -> bool;

    /// 新しいrefresh tokenを発行する。
    /// 古いrefresh tokenがある場合は無効になる。
    async fn issue_refresh_token(&self, userid: PID) -> RefreshToken;

    /// Refresh tokenを検証する。
    /// 紐づけられたuser idを返す。
    async fn verify_refresh_token(&self, token: RefreshToken) -> Option<PID>;

    async fn issue_access_token(&self, userid: PID) -> AccessToken;

    /// Access tokenを検証する。
    /// Tokenが正しければ、token内にあるuser id情報を抽出して返す。
    async fn verify_access_token(&self, token: AccessToken) -> Option<PID>;
}
