use slog::error;
use slog::info;
use slog::o;

use std::convert::Infallible;
use std::time::Instant;

use warp::filters::log::Info;
use warp::Filter;

#[derive(Clone)]
pub struct RequestLogger {
    logger: slog::Logger,
}

impl RequestLogger {
    pub fn new(logger: slog::Logger) -> Self {
        Self {
            logger: logger.new(o!("mod" => "rocket")),
        }
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
            self.logger,
            "{}", info.method();
            "route" => info.path().to_string(),
            "status" => info.status().to_string(),
            "ip" => info.remote_addr().map(|x| x.to_string()).unwrap_or("???.???.???.???".into()),
            "duration" => duration,
            "duration_tag" => tag,
        );
    }
}
