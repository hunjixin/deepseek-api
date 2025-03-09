use anyhow::{Context, Result};
use futures_util::{Stream, TryStreamExt};
use reqwest::Response;
use serde::de::DeserializeOwned;
use std::{
    pin::Pin,
    task::{Context as TaskContext, Poll},
};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_stream::{wrappers::LinesStream, StreamExt};
use tokio_util::io::StreamReader;
pub struct JsonStream<T> {
    inner: Pin<Box<dyn Stream<Item = Result<T, anyhow::Error>> + Send>>,
}

impl<T: DeserializeOwned + Send + 'static> JsonStream<T> {
    pub fn new(response: Response) -> Self {
        let byte_stream = response
            .bytes_stream()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e));
        let async_reader = StreamReader::new(byte_stream);

        let line_stream = LinesStream::new(BufReader::new(async_reader).lines());

        let processed_stream = line_stream
            .map_ok(|line| line.trim().to_string())
            .map_err(anyhow::Error::from) // 将 std::io::Error 转换为 anyhow::Error
            .take_while(|line| line.as_ref().is_ok_and(|data| data != "data: [DONE]")) // 遇到 "data: [DONE]" 终止流
            .try_filter_map(|line| async move {
                if line.is_empty() {
                    return Ok(None);
                }

                let json_str = line
                    .strip_prefix("data: ")
                    .context("Missing 'data: ' prefix")?;

                serde_json::from_str(json_str)
                    .map(Some)
                    .map_err(anyhow::Error::from)
            });

        JsonStream {
            inner: Box::pin(processed_stream),
        }
    }
}

impl<T: Unpin> Stream for JsonStream<T> {
    type Item = Result<T, anyhow::Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut TaskContext<'_>) -> Poll<Option<Self::Item>> {
        self.inner.as_mut().poll_next(cx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;
    use http::StatusCode;
    use reqwest::Response;
    use serde::{Deserialize, Serialize};
    use tokio_stream::{self as stream, StreamExt};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct TestData {
        id: String,
        value: u32,
    }

    fn mock_response(data: Vec<Result<Bytes, reqwest::Error>>) -> Response {
        let body = reqwest::Body::wrap_stream(stream::iter(data));
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
