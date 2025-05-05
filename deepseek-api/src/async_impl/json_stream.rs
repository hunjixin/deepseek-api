use anyhow::{Context, Error, Result};
use futures_util::future;
use futures_util::io::{AsyncBufReadExt, BufReader};
use futures_util::stream::{Stream, StreamExt, TryStreamExt};
use reqwest::Response;
use serde::de::DeserializeOwned;
use std::{
    pin::Pin,
    task::{Context as TaskContext, Poll},
};

/// A stream that processes Server-Sent Events (SSE) and deserializes JSON data.
///
/// The `JsonStream` struct wraps an asynchronous stream of lines from an HTTP response,
/// where each line is expected to be a JSON object prefixed with "data: ". The stream
/// terminates when it encounters a line with "data: [DONE]".
///
/// # Type Parameters
///
/// * `T`: The type of the deserialized JSON objects. It must implement `DeserializeOwned` and `Send`.
///
/// # Examples
///
/// ```rust
/// use reqwest::Response;
/// use serde::Deserialize;
/// use futures_util::stream::StreamExt;
/// use deepseek_api::json_stream::JsonStream;
///
/// #[derive(Debug, Deserialize)]
/// struct MyData {
///     id: String,
///     value: u32,
/// }
///
/// async fn process_response(response: Response) {
///     let mut stream = JsonStream::<MyData>::new(response);
///
///     while let Some(item) = stream.next().await {
///         match item {
///             Ok(data) => println!("{:?}", data),
///             Err(e) => eprintln!("Error: {:?}", e),
///         }
///     }
/// }
/// ```
///
/// # Errors
///
/// The stream yields `anyhow::Error` if:
/// - The line does not start with "data: "
/// - The JSON deserialization fails
///
/// # Methods
///
/// * `new(response: Response) -> Self`: Creates a new `JsonStream` from an HTTP response.
///
/// # Trait Implementations
///
/// * `Stream` for `JsonStream<T>`: Allows the `JsonStream` to be used as a stream of `Result<T, anyhow::Error>`.
pub struct JsonStream<T> {
    inner: Pin<Box<dyn Stream<Item = Result<T, Error>> + Send>>,
}

impl<T: DeserializeOwned + Send + 'static> JsonStream<T> {
    pub fn new(response: Response) -> Self {
        let byte_stream = response
            .bytes_stream()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e));

        let async_read = byte_stream.into_async_read();
        let processed = BufReader::new(async_read)
            .lines()
            .map_err(Error::from)
            .take_while(|res| {
                future::ready(match res {
                    Ok(ref line) => line != "data: [DONE]",
                    Err(_) => true,
                })
            })
            .try_filter_map(|line| async move {
                let line = line.trim();
                if line.is_empty() || line == ": keep-alive" {
                    return Ok(None);
                }
                let json = line
                    .strip_prefix("data: ")
                    .context("Missing 'data: ' prefix")?;
                let obj = serde_json::from_str(json)?;
                Ok(Some(obj))
            });

        JsonStream {
            inner: Box::pin(processed),
        }
    }
}

impl<T: Unpin> Stream for JsonStream<T> {
    type Item = Result<T, Error>;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut TaskContext<'_>) -> Poll<Option<Self::Item>> {
        self.inner.as_mut().poll_next(cx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;
    use futures_util::stream::StreamExt;
    use http::StatusCode;
    use reqwest::Response;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct TestData {
        id: String,
        value: u32,
    }

    fn mock_response(data: Vec<Result<Bytes, reqwest::Error>>) -> Response {
        let body = reqwest::Body::wrap_stream(futures_util::stream::iter(data));
        let http_response = http::response::Response::builder()
            .status(StatusCode::OK)
            .body(body)
            .unwrap();
        Response::from(http_response)
    }

    #[tokio::test]
    async fn test_normal_sse_stream() {
        let data = vec![
            Ok(Bytes::from("data: {\"id\":\"1\",\"value\":100}\n")),
            Ok(Bytes::from("data: {\"id\":\"2\",\"value\":200}\n")),
        ];
        let response = mock_response(data);
        let mut stream = JsonStream::<TestData>::new(response);

        let mut results = vec![];
        while let Some(item) = stream.next().await {
            results.push(item.unwrap());
        }

        assert_eq!(
            results,
            vec![
                TestData {
                    id: "1".into(),
                    value: 100
                },
                TestData {
                    id: "2".into(),
                    value: 200
                }
            ]
        );
    }

    #[tokio::test]
    async fn test_chunked_data() {
        let data = vec![
            Ok(Bytes::from("data: {\"id\":\"3\",\"")),
            Ok(Bytes::from("value\":300}\n")),
        ];
        let response = mock_response(data);
        let mut stream = JsonStream::<TestData>::new(response);

        assert_eq!(
            stream.next().await.unwrap().unwrap(),
            TestData {
                id: "3".into(),
                value: 300
            }
        );
        assert!(stream.next().await.is_none());
    }

    #[tokio::test]
    async fn test_empty_lines_and_done() {
        let data = vec![
            Ok(Bytes::from("\n")),
            Ok(Bytes::from("data: {\"id\":\"4\",\"value\":400}\n")),
            Ok(Bytes::from("data: [DONE]\n")),
            Ok(Bytes::from("data: {\"id\":\"5\",\"value\":500}\n")),
        ];
        let response = mock_response(data);
        let mut stream = JsonStream::<TestData>::new(response);

        let result = stream.next().await.unwrap().unwrap();
        assert_eq!(
            result,
            TestData {
                id: "4".into(),
                value: 400
            }
        );
        assert!(stream.next().await.is_none());
    }

    #[tokio::test]
    async fn test_invalid_prefix() {
        let data = vec![Ok(Bytes::from("invalid data\n"))];
        let response = mock_response(data);
        let mut stream = JsonStream::<TestData>::new(response);

        let err = stream.next().await.unwrap().unwrap_err();
        assert!(err.to_string().contains("Missing 'data: ' prefix"));
    }

    #[tokio::test]
    async fn test_malformed_json() {
        let data = vec![Ok(Bytes::from("data: {invalid}\n"))];
        let response = mock_response(data);
        let mut stream = JsonStream::<TestData>::new(response);

        let err = stream.next().await.unwrap().unwrap_err();
        assert!(err.is::<serde_json::Error>());
    }
}
