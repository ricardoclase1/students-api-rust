use std::env;

#[derive(Clone)]
pub struct AppConfig {
    pub api_gateway_url: String,
    pub api_key: String,
    pub sns_topic_arn: String,
    pub aws_region: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            api_gateway_url: env::var("API_GATEWAY_URL").unwrap_or_else(|_| "http://localhost:8080".to_string()),
            api_key: env::var("API_KEY").unwrap_or_else(|_| "local-api-key".to_string()),
            sns_topic_arn: env::var("SNS_TOPIC_ARN").unwrap_or_else(|_| "arn:aws:sns:us-east-1:123456789012:local-topic".to_string()),
            aws_region: env::var("AWS_REGION").unwrap_or_else(|_| "us-east-1".to_string()),
        })
    }
}
