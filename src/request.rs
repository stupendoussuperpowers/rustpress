use std;
use std::collections::HashMap;

use crate::constants::Method;

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub route: String,
    pub protocol: String,
    pub body: String,
    pub headers: HashMap<String, String>,
}

#[derive(Debug)]
pub struct RequestParserErr;

impl std::str::FromStr for Request {
    type Err = RequestParserErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut body: String = "".to_string();
        let mut headers: HashMap<String, String> = std::collections::HashMap::new();
        let method: Method;
        let route: String;
        let protocol: String;

        let request_vector: Vec<String> = s.split("\r\n").map(|s| s.to_string()).collect();

        let headers_vector: Vec<String> = request_vector[0]
            .split(" ")
            .map(|s| s.to_string())
            .collect();

        (method, route, protocol) = (
            headers_vector[0].to_string().parse().unwrap(),
            headers_vector[1].to_string(),
            headers_vector[2].to_string(),
        );

        let mut body_start = false;

        for line in request_vector.iter().skip(1) {
            if line.is_empty() {
                body_start = true;
            }

            if body_start {
                body.push_str(line);
            } else {
                let keyval: Vec<String> = line.split(":").map(|s| s.to_string()).collect();

                headers.insert(keyval[0].to_string(), keyval[1].to_string());
            }
        }

        Ok(Self {
            method,
            route,
            protocol,
            body,
            headers,
        })
    }
}

impl Request {
    pub fn get_header(self, property: &String) -> String {
        self.headers.get(property).unwrap().to_string()
    }
}
