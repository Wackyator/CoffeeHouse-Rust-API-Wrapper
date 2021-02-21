use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SuccessResponse {
    pub success: bool,
    pub response_code: u16,
    pub results: SessionPayload,
}

#[derive(Debug, Deserialize)]
pub struct SessionPayload {
    pub session_id: String,
    pub language: String,
    pub available: bool,
    pub expires: i64,
}

#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub response_code: u16,
    pub error: ErrorPayload,
}

#[derive(Debug, Deserialize)]
pub struct ErrorPayload {
    pub error_code: i8,
    #[serde(rename = "type")]
    pub error_type: String,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct ThoughtResponse {
    pub success: bool,
    pub response_code: u16,
    pub results: ThoughtPayload,
}

#[derive(Debug, Deserialize)]
pub struct ThoughtPayload {
    pub output: String,
    pub session: SessionPayload,
    pub attributes: ThoughtAttributes,
}

#[derive(Debug, Deserialize)]
pub struct ThoughtAttributes {
    pub ai_emotion: String,
    pub ai_emotion_probability: f64,
    pub current_language: String,
    pub current_language_probability: f64,
}

