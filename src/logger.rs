use rocket::async_trait;
use rocket::config::Config;
use rocket::config::LogLevel;
use rocket::fairing;
use rocket::fairing::Fairing;
use rocket::fairing::Info;
use rocket::fairing::Kind;
use rocket::http::Header;
use rocket::Build;
use rocket::Data;
use rocket::Orbit;
use rocket::Request;
use rocket::Response;
use rocket::Rocket;

use slog::error;
use slog::info;
use slog::o;

use std::time::Instant;

pub struct RequestId;

#[async_trait]
impl Fairing for RequestId {
    fn info(&self) -> Info {
        Info {
            name: "Tracks each request with a request id",
            kind: Kind::Request | Kind::Response,
        }
    }

    async fn on_request(&self, req: &mut Request<'_>, _: &mut Data) {
        req.replace_header(Header::new(
            "x-request-id",
            uuid::Uuid::new_v4().to_hyphenated().to_string(),
        ));
    }

    async fn on_response<'r>(&self, req: &'r Request<'_>, res: &mut Response<'r>) {
        let request_id = req
            .headers()
            .get("x-request-id")
            .next()
            .map(ToString::to_string)
            .unwrap_or(uuid::Uuid::new_v4().to_hyphenated().to_string());

        res.set_header(Header::new("x-request-id", request_id));
    }
}

impl Default for RequestId {
    fn default() -> Self {
        Self
    }
}

pub struct RequestLogger {
    logger: slog::Logger,
    req_id: RequestId,
}

impl RequestLogger {
    pub fn new(logger: slog::Logger) -> Self {
        Self {
            logger: logger.new(o!("mod" => "rocket")),
            req_id: RequestId::default(),
        }
    }
}

#[async_trait]
impl Fairing for RequestLogger {
    fn info(&self) -> Info {
        Info {
            name: "req/resp logger",
            kind: Kind::Ignite | Kind::Liftoff | Kind::Request | Kind::Response,
        }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> fairing::Result {
        // safe to assume that the config is valid.
        let config: Config = rocket.figment().extract().unwrap();

        // we want to turn logging off in rocket
        // as we are gonna log everything ourselves.
        Ok(rocket.configure(Config {
            log_level: LogLevel::Off,
            ..config
        }))
    }

    async fn on_liftoff(&self, rocket: &Rocket<Orbit>) {
        let config = rocket.config();
        info!(
            self.logger,
            "config";
            "profile" => config.profile.to_string(),
            "address" => config.address.to_string(),
            "port" => config.port.to_string(),
        );

        info!(
            self.logger,
            "config";
            "workers" => config.workers.to_string(),
            "keep_alive" => config.keep_alive.to_string(),
            "tls" => config.tls.is_some(),
        );

        for catcher in rocket.catchers() {
            if let Some(ref name) = catcher.name {
                info!(
                    self.logger,
                    "catchers";
                    "catcher" => name.to_string(),
                    "code" => catcher.code.map(|x| x.to_string()).unwrap_or("*".into()),
                    "base" => catcher.base.to_string(),
                );
            } else {
                info!(
                    self.logger,
                    "catchers";
                    "code" => catcher.code.map(|x| x.to_string()).unwrap_or("*".into()),
                    "base" => catcher.base.to_string(),
                );
            }
        }

        for route in rocket.routes() {
            if let Some(ref name) = route.name {
                info!(
                    self.logger,
                    "routes";
                    "name" => name.to_string(),
                    "method" => route.method.to_string(),
                    "base" => route.uri.to_string(),
                    "acccepts" => route.format.as_ref().map(|x| x.to_string()).unwrap_or("*".into()),
                    "rank" => route.rank,
                );
            } else {
                info!(
                    self.logger,
                    "routes";
                    "method" => route.method.to_string(),
                    "base" => route.uri.to_string(),
                    "acccepts" => route.format.as_ref().map(|x| x.to_string()).unwrap_or("*".into()),
                    "rank" => route.rank,
                );
            }
        }
    }

    async fn on_request(&self, req: &mut Request<'_>, data: &mut Data) {
        self.req_id.on_request(req, data).await;

        req.local_cache(|| Instant::now());
    }

    async fn on_response<'r>(&self, req: &'r Request<'_>, res: &mut Response<'r>) {
        let request_speed = req.local_cache(|| Instant::now()).elapsed();
        let (tag, duration) = if request_speed.as_millis() > 0 {
            ("ms", request_speed.as_millis())
        } else {
            ("us", request_speed.as_micros())
        };

        self.req_id.on_response(req, res).await;

        let request_id = res
            .headers()
            .get("x-request-id")
            .next()
            .map(ToString::to_string)
            .unwrap_or_default();

        let handler = req
            .route()
            .and_then(|x| x.name.as_ref().map(|x| x.to_string()));

        info!(
            self.logger,
            "{}", req.method();
            "route" => req.uri().to_string(),
            "status" => res.status().to_string(),
            "handler" => handler,
            "id" => request_id,
            "ip" => req.client_ip().map(|x| x.to_string()).unwrap_or("???.???.???.???".into()),
            "duration" => duration,
            "duration_tag" => tag,
        );
    }
}
