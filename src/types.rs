use std::collections::HashSet;

// Create a new struct to parse the HTTP request
pub struct HttpRequestStatus {
    pub method: String,
    pub http_version: f32,
    pub path: String
}

// Insert all HTTP methods into a set
pub fn generate_http_method_set() -> HashSet<String> {
    let mut http_methods = HashSet::new();

    http_methods.insert("GET".to_string());
    http_methods.insert("HEAD".to_string());
    http_methods.insert("POST".to_string());
    http_methods.insert("PUT".to_string());
    http_methods.insert("DELETE".to_string());
    http_methods.insert("CONNECT".to_string());
    http_methods.insert("OPTIONS".to_string());
    http_methods.insert("TRACE".to_string());
    http_methods.insert("PATCH".to_string());

    return http_methods;
}