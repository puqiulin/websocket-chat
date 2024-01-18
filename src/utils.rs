use chrono::prelude::*;
use chrono::Local;
use tracing::Subscriber;
use tracing_subscriber::{self, layer::Context, registry::LookupSpan, Layer};

pub struct LocalTimeLayer;

impl<S> Layer<S> for LocalTimeLayer
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    fn on_event(&self, event: &tracing::Event<'_>, ctx: Context<'_, S>) {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        println!("[{}] {:?}", timestamp, event);
    }
}

pub fn get_local_time() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}
