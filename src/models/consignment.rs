use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct Lines {
    id: u32,
    name: String,
}

#[derive(Serialize)]
pub struct ConsignmentRequest {
    site_id: u32,
    user_name: String,
    ticket_id: u32,
    lines: Lines,
}

impl ConsignmentRequest {
    pub fn test() -> Self {
        Self {
            site_id: 18,
            user_name: "Admin".to_string(),
            ticket_id: 3132,
            lines: Lines {
                id: 5,
                name: "Monitor".to_string(),
            },
        }
    }
}

#[derive(Deserialize)]
pub struct ConsignmentResponse {
    id: u32,
}

impl ConsignmentResponse {
    pub fn id(&self) -> u32 {
        self.id
    }
}
