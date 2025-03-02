use anyhow::Result;
use reqwest::Response;
use serde::de::DeserializeOwned;
use serde_json::Deserializer;
use futures_util::{Stream, StreamExt,TryStreamExt};
use tokio_util::io::{StreamReader,SyncIoBridge};
use std::{pin::Pin, task::{Context, Poll}};

/// `JSONStream<T>`：流式解析 JSON 数组中的对象
pub struct JSONStream<T> {
    inner: Pin<Box<dyn Stream<Item = Result<T, serde_json::Error>> + Send>>,
}

impl<T: DeserializeOwned + Send + 'static> JSONStream<T> {

    pub  fn from_response(response: Response) -> Self {
        let stream = response.bytes_stream().map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err));
        let async_reader = StreamReader::new(stream);
        let sync_reader = SyncIoBridge::new(async_reader);
        let reader = Deserializer::from_reader(sync_reader);
        let deser = reader.into_iter::<T>();

        let stream = async_stream::stream! {
            for item in deser {
                yield item;
            }
        };

        JSONStream {
            inner: Box::pin(stream),
        }
    }
}

impl<T: Send + 'static> Stream for JSONStream<T> {
    type Item = Result<T, serde_json::Error>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.get_mut().inner.as_mut().poll_next(cx)
    }
}
