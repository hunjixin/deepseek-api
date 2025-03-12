use anyhow::{anyhow, Error};
use reqwest::blocking::Response;
use serde::de::DeserializeOwned;
use std::{
    io::{BufRead, BufReader},
    marker::PhantomData,
};

pub struct JsonStream<T> {
    ph: PhantomData<T>,
    lines: std::io::Lines<BufReader<Response>>,
}

impl<T: DeserializeOwned> JsonStream<T> {
    pub fn new(response: Response) -> Self {
        let reader = BufReader::new(response);
        let lines = reader.lines();
        JsonStream {
            ph: PhantomData,
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
                Err(e) => return Some(Err(anyhow::Error::new(e))),
            }
        }
        None
    }
}
