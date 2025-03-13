pub mod error;
pub mod request;
pub mod response;

cfg_if::cfg_if! {
    if #[cfg(feature = "is_sync")] {
        mod sync_impl;
        pub use sync_impl::json_stream;
        pub use sync_impl::client::*;
        pub use sync_impl::completions;
    } else {
        mod async_impl;
        pub use async_impl::json_stream;
        pub use async_impl::client::*;
        pub use async_impl::completions;
    }
}
