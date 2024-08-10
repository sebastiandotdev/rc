use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub enum Methods {
  GET,
  POST,
  PUT,
  DELETE,
  PATCH,
}

#[derive(Debug, Clone, Deserialize)]
enum Env {
  Local,
  Global,
}

impl Methods {
  pub fn from_str(method: &str) -> Option<Self> {
    match method {
      "GET" => Some(Methods::GET),
      "POST" => Some(Methods::POST),
      "PUT" => Some(Methods::PUT),
      "DELETE" => Some(Methods::DELETE),
      "PATCH" => Some(Methods::PATCH),
      _ => None,
    }
  }
}

impl Env {
  pub fn from_str(string: &str) -> Option<Self> {
    match string {
      "Local" => Some(Env::Local),
      "Global" => Some(Env::Global),
      _ => None,
    }
  }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RCConfig {
  pub url: String,
  pub methods: Vec<Methods>,
  pub env: String,
}
