use serde::Serialize;

#[derive(Serialize)]
pub struct ApiVersion {
    pub version: &'static str,
}
