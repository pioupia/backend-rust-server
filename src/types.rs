use std::collections::HashSet;

// Create a new struct to parse the HTTP request
pub struct HttpRequestStatus {
    pub method: String,
    pub http_version: f32,
    pub path: String
}