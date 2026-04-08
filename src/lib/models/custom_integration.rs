use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct CustomIntegration {
    #[serde(rename = "resourcebaseurl")]
    resource_base_url: String,
    new_client_secret: String,
    id: i32,
}

impl CustomIntegration {
    pub fn new(
        resource_base_url: impl Into<String>,
        new_client_secret: impl Into<String>,
        id: i32,
    ) -> Self {
        Self {
            resource_base_url: resource_base_url.into(),
            new_client_secret: new_client_secret.into(),
            id,
        }
    }
}
