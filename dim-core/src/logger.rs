use warp::filters::log::Info;

use tracing::info;

#[derive(Clone)]
pub struct RequestLogger;

impl RequestLogger {
    pub fn new() -> Self {
        Self
    }
}

impl RequestLogger {
    pub fn on_response(&self, info: Info<'_>) {
        let (tag, duration) = if info.elapsed().as_millis() > 0 {
            ("ms", info.elapsed().as_millis())
        } else {
            ("us", info.elapsed().as_micros())
        };

        info!(
            route = ?info.path(),
            status = ?info.status(),
            ip = ?info
                .remote_addr()
                .map(|x| x.to_string())
                .unwrap_or("???.???.???.???".into()),
            duration = ?duration,
            duration_tag = ?tag,
            "{}",
            info.method(),
        );
    }
}
