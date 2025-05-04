use anyhow::{anyhow, Error};
use reqwest::blocking::Response;
use serde::de::DeserializeOwned;
use std::{
    io::{BufRead, BufReader},
    marker::PhantomData,
};

pub struct JsonStream<T> {
    _ph: PhantomData<T>,
    lines: std::io::Lines<BufReader<Response>>,
}

impl<T: DeserializeOwned> JsonStream<T> {
    pub fn new(response: Response) -> Self {
        let lines = BufReader::new(response).lines();
        JsonStream {
            _ph: PhantomData,
            lines,
        }
    }
}

impl<T: DeserializeOwned> Iterator for JsonStream<T> {
    type Item = Result<T, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        for line_result in &mut self.lines {
            match line_result {
                Ok(line) => {
                    let line = line.trim();
                    if line == "data: [DONE]" {
                        return None;
                    }
                    if line.is_empty() || line == ": keep-alive" {
                        continue;
                    }
                    if let Some(json_str) = line.strip_prefix("data: ") {
                        match serde_json::from_str::<T>(json_str) {
                            Ok(value) => return Some(Ok(value)),
                            Err(err) => {
                                return Some(Err(anyhow!("jsonstr: {} reason {}", json_str, err)))
                            }
                        }
                    } else {
                        return Some(Err(anyhow!("{} Missing 'data: ' prefix", line)));
                    }
                }
                Err(e) => return Some(Err(Error::new(e))),
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use http::Response;
    use reqwest::blocking::Response as ReqwestResponse;
    use serde::Deserialize;

    #[derive(Debug, Deserialize, PartialEq)]
    struct TestData {
        id: u32,
        value: String,
    }

    fn mock_response(body: &str) -> ReqwestResponse {
        let http_response = Response::builder()
            .body(body.to_owned().into_bytes())
            .unwrap();
        ReqwestResponse::from(http_response)
    }

    #[test]
    fn test_normal_stream() {
        let response = mock_response(
            r#"data: {"id":1,"value":"test1"}
            data: {"id":2,"value":"test2"}
            data: [DONE]"#,
        );

        let mut stream = JsonStream::<TestData>::new(response);
        let first = stream.next().unwrap().unwrap();
        assert_eq!(
            first,
            TestData {
                id: 1,
                value: "test1".into()
            }
        );

        let second = stream.next().unwrap().unwrap();
        assert_eq!(
            second,
            TestData {
                id: 2,
                value: "test2".into()
            }
        );

        assert!(stream.next().is_none());
    }

    #[test]
    fn test_invalid_json() {
        let response = mock_response(r#"data: {invalid_json}"#);
        let mut stream = JsonStream::<TestData>::new(response);

        let err = stream.next().unwrap().unwrap_err();
        assert!(err.to_string().contains("reason"));
        assert!(err.to_string().contains("invalid_json"));
    }

    #[test]
    fn test_missing_prefix() {
        let response = mock_response(r#"{"id":3,"value":"error"}"#);
        let mut stream = JsonStream::<TestData>::new(response);

        let err = stream.next().unwrap().unwrap_err();
        assert!(err.to_string().contains("Missing 'data: ' prefix"));
    }

    #[test]
    fn test_skip_empty_lines() {
        let response = mock_response(
            r#"
            : keep-alive
            
            data: {"id":4,"value":"empty"}
            data: [DONE]"#,
        );

        let mut stream = JsonStream::<TestData>::new(response);
        let item = stream.next().unwrap().unwrap();
        assert_eq!(item.id, 4);
        assert!(stream.next().is_none());
    }
}
