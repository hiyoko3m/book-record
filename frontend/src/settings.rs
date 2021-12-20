use url::Url;

#[derive(Clone, Debug, PartialEq)]
pub struct Settings {
    pub base_url: Url,
}
