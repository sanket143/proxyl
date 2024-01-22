use hudsucker::{
    async_trait::async_trait,
    hyper::{
        body,
        http::{HeaderMap, HeaderValue},
        Body, Request,
    },
    HttpContext, HttpHandler, RequestOrResponse,
};
use regex::Regex;
use std::sync::{Arc, Mutex};
use toml::{map::Map, Value};

type Rules = Arc<Mutex<Map<String, Value>>>;

#[derive(Clone)]
pub struct Handler {
    rules: Rules,
}

fn check_headers(headers: &HeaderMap<HeaderValue>, rule: &Value) -> bool {
    for (key, value) in rule
        .get("headers")
        .unwrap_or(&Value::Table(Map::new()))
        .as_table()
        .unwrap()
        .iter()
    {
        if let Some(header_value) = headers.get(key) {
            if header_value != value.as_str().unwrap() {
                return false;
            }
        }
    }

    true
}

impl Handler {
    pub fn new(rules: Rules) -> Self {
        Self { rules }
    }

    async fn get_updated_request(&self, req: Request<Body>) -> Request<Body> {
        let rules = self.rules.clone();
        let (parts, body) = req.into_parts();
        let body_bytes = body::to_bytes(body).await.unwrap();
        let body_string = std::str::from_utf8(&body_bytes).unwrap_or("");
        let mut matched_rule: Option<Value> = None;
        let mut rule_name = String::from("NA");

        for (key, rule) in rules.lock().unwrap().iter() {
            let re = Regex::new(
                rule.get("body_re")
                    .unwrap_or(&Value::String("".to_owned()))
                    .as_str()
                    .unwrap(),
            )
            .unwrap();

            if rule
                .get("enabled")
                .unwrap_or(&Value::Boolean(true))
                .as_bool()
                .unwrap()
                && rule
                    .get("uri")
                    .unwrap_or(&Value::String("".to_owned()))
                    .as_str()
                    .unwrap()
                    == parts.uri
                && rule
                    .get("method")
                    .unwrap_or(&Value::String(parts.method.to_string()))
                    .as_str()
                    .unwrap()
                    .to_lowercase()
                    == parts.method.to_string().to_lowercase()
                && check_headers(&parts.headers, rule)
                && re.is_match(body_string)
            {
                let m_rule = rule.to_owned();
                rule_name = m_rule
                    .get("name")
                    .unwrap_or(&Value::String(key.to_owned()))
                    .as_str()
                    .unwrap()
                    .to_owned();
                matched_rule = Some(m_rule);
                break;
            }
        }

        let mut req = Request::from_parts(parts, Body::from(body_bytes));
        if let Some(rule) = matched_rule {
            *req.uri_mut() = rule["redirect_to"]
                .as_str()
                .unwrap_or(req.uri().to_string().as_str())
                .parse()
                .unwrap();

            println!("Rule applied: {}", rule_name);
        }

        req
    }
}

#[async_trait]
impl HttpHandler for Handler {
    async fn handle_request(
        &mut self,
        _ctx: &HttpContext,
        req: Request<Body>,
    ) -> RequestOrResponse {
        self.get_updated_request(req).await.into()
    }
}
