//! A PoC implementation of the plugin API
//!
//! # Brief
//!
//! At startup when the server-side settings are loaded we look for a section
//! describing the "plugins" to enable, this would just be an array of strings
//! of commands to execute that Dim will run in parallel.
//!
//! It is assumed that every command will spawn a process that connects to the
//! plugin server, which is a WebSocket server running at some unspecified address,
//! and every plugin process will authenticate themselves with a single-use secret.
//!
//! The address of the plugin server and the plugins single-use secret are passed
//! in as environment variables to the child process Dim launches via the:
//! `$DIM_PLUGIN_HOST` and `$DIM_PLUGIN_SECRET` environment variables.
//!
//! The reasoning for this approach is so we:
//!
//! a) Launch and authenticate only those plugins we trust (i.e. the one's specified in the settings.)
//! b) Don't rely on any hardcoded details about the address the plugins are supposed to connect to.
//!
//! Even if some foreign process gets the value of the plugin address the backend will just reject it
//! because the auth process involves single-use secrets used for authenticating the processes, important
//! since plugins would basically have near-unrestricted access to Dim's capabilities and are a potential
//! attack vector.
//!
//! Note that the plugin host address would probably be an internal address such as
//! localhost, some named-pipe, or a unix domain socket (depending on the platform.)
//!
//! ## Plugin Host API
//!
//! Currently the underlying host API uses websockets + JSON encoded messages
//! and exposes a fixed number of capabilities for the plugin process.
//!
//! ```json
//! {"t": "COMMAND_NAME", "d": {...command_specific_input}, "r": "MESSAGE_REFERENCE"}
//! ```
//!
//! * The `"t"` field is short for "trigger" and specifies what command to invoke.
//! * The `"d"` field is any JSON object, it is unspecified and depends on the command being triggered.
//! * The `"r"` field is a reference field, it is used to differentiate between responses to commands.
//!   the plugin host runs commands in parallel and will reply to a command invokation using this
//!   reference.
//!
//! When the plugin host emits event messages to the plugin process the `"r"` field will be omitted.
//!

pub mod host;
pub mod runner;

use std::collections::HashMap;
use std::collections::HashSet;
use std::io;

use slog::error;
use slog::info;
use tokio::{net::TcpListener, runtime::Handle, sync::mpsc::unbounded_channel};
use xtra::Actor;

use crate::plugin::runner::Envelope;
use crate::{
    plugin::{
        host::PluginHost,
        runner::{PluginServer, RunnerQuery},
    },
    websocket::WebsocketServer,
};

/// Start a plugin host, a plugin server actor, and launch all provided plugin subprocesses.
///
/// `plugins` shall be an iterable of anything that implements `AsRef<str>` and each string
/// is treated as a shell command and is spawned like `/usr/bin/sh -c {command}` for the sake
/// of flexibility.
///
/// Every plugin subprocess has `DIM_PLUGIN_HOST` AND `DIM_PLUGIN_SECRET` in the process'
/// environment variables. The host envvar is the websocket address that the plugin process
/// should connect to and the secret envvar is a single-use UUID token used for authenticating
/// with the server. The subprocess's are started in parallel but the order is left unspecified.
///
pub async fn start_plugin_host<I, S>(logger: slog::Logger, plugins: I) -> io::Result<()>
where
    I: Iterator<Item = S>,
    S: AsRef<str>,
{
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let plugin_host_addr = listener.local_addr()?;

    let plugins: HashSet<_> = plugins.map(|s| s.as_ref().to_string()).collect();

    let plugin_server = PluginServer::default()
        .create(None)
        .spawn(&mut xtra::spawn::Tokio::Global);

    let (tx, rx) = unbounded_channel::<Envelope>();

    let mut plugin_host = PluginHost {
        listener: Some(listener),
        inner: plugin_server.clone(),
        logger: logger.clone(),
        out_tx: tx
    };

    let handle = Handle::current();

    handle.spawn(async move {
        let _ = plugin_host
            .serve("<already bound>", Handle::current(), rx)
            .await
            .unwrap();
    });

    let mut pending = vec![];

    for command in plugins {
        let plugin_secret = format!("{}", uuid::Uuid::new_v4());

        let fut = async move {
            match tokio::process::Command::new("/usr/bin/sh")
                .args(["-c", &command])
                .env_clear()
                .env("DIM_PLUGIN_HOST", format!("{}", plugin_host_addr))
                .env("DIM_PLUGIN_SECRET", plugin_secret.clone())
                .spawn()
            {
                Ok(ch) => Ok((ch, plugin_secret)),
                Err(why) => Err(why),
            }
        };

        pending.push(handle.spawn(fut));
    }

    for task in pending.drain(..) {
        let (child, secret) = match task.await.unwrap() {
            Ok((ch, secret)) => (ch, secret),
            Err(why) => {
                error!(logger, "Failed to spawn plugin subprocess: {:?}", why);
                continue;
            }
        };

        info!(logger, "Spawned Plugin subprocess {:?}", child);

        plugin_server
            .send(RunnerQuery::AddPending {
                token: secret,
                child,
            })
            .await
            .unwrap();
    }

    plugin_server
        .send(RunnerQuery::NotifyStartupComplete)
        .await
        .unwrap();

    Ok(())
}
