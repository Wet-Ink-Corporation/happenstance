//! A hand-written slice of HTTP/1.1: enough to make the point, and no more.
//!
//! There is no `axum` here, and the absence is a decision rather than an
//! oversight. A web framework would have brought `hyper`, `tower` and a dozen
//! transitive packages through `cargo deny`'s licence allowlist and its ban on
//! `async-trait`, to buy routing for two endpoints — and every line it added to
//! the example would have been a line about the framework rather than about
//! `happenstance`. What is here is the smallest thing that is genuinely a
//! network server: a socket, a request line, a status code and a header.
//!
//! # What it is not
//!
//! It reads a request line and discards the headers; it has no body, no
//! chunked encoding, no keep-alive and no percent-decoding. Every identifier
//! this application uses is `[A-Za-z0-9-]`, so decoding would be a no-op, and
//! a query string carrying anything else will be read literally. Do not lift
//! this module into anything that faces a network you do not own.

use core::fmt::Write as _;
use std::io;
use std::net::SocketAddr;

use tokio::io::{AsyncReadExt as _, AsyncWriteExt as _};
use tokio::net::TcpStream;

/// The most bytes a request head may occupy before it is refused.
///
/// A server with no limit here is a server that a single client can exhaust by
/// never sending `\r\n\r\n`. Small, because this protocol has no body.
const MAX_REQUEST: usize = 8 * 1024;

/// A parsed request line, with its query string split out.
#[derive(Debug, Clone)]
pub struct Request {
    /// The method, verbatim and uppercase as sent.
    pub method: String,
    /// The path, with the query string removed.
    pub path: String,
    /// The query string's `key=value` pairs, in the order they were sent.
    pub query: Vec<(String, String)>,
}

impl Request {
    /// Reads one request head from `stream`.
    ///
    /// Returns `Ok(None)` for a connection that closed without sending a
    /// request line, or sent one this module will not parse — which a server
    /// should answer by closing rather than by guessing.
    ///
    /// # Errors
    ///
    /// Returns the socket's own error if the read fails.
    pub async fn read(stream: &mut TcpStream) -> io::Result<Option<Self>> {
        let mut buffer = Vec::new();
        let mut chunk = [0_u8; 512];

        loop {
            let read = stream.read(&mut chunk).await?;
            if read == 0 {
                break;
            }
            buffer.extend_from_slice(&chunk[..read]);
            if buffer.windows(4).any(|window| window == b"\r\n\r\n") {
                break;
            }
            if buffer.len() > MAX_REQUEST {
                return Ok(None);
            }
        }

        Ok(Self::parse(&String::from_utf8_lossy(&buffer)))
    }

    /// Parses a request head's first line.
    fn parse(head: &str) -> Option<Self> {
        let mut parts = head.lines().next()?.split_whitespace();
        let method = parts.next()?.to_owned();
        let target = parts.next()?;

        let (path, raw_query) = target.split_once('?').unwrap_or((target, ""));
        let query = raw_query
            .split('&')
            .filter(|pair| !pair.is_empty())
            .map(|pair| {
                let (key, value) = pair.split_once('=').unwrap_or((pair, ""));
                (key.to_owned(), value.to_owned())
            })
            .collect();

        Some(Self {
            method,
            path: path.to_owned(),
            query,
        })
    }

    /// The first value sent for `key`, if any.
    #[must_use]
    pub fn param(&self, key: &str) -> Option<&str> {
        self.query
            .iter()
            .find(|(name, _)| name == key)
            .map(|(_, value)| value.as_str())
    }
}

/// Writes one response and closes the connection.
///
/// `Connection: close` and a `Content-Length` on every reply, which is what
/// lets the client below read to end-of-file and know it has the whole body.
///
/// # Errors
///
/// Returns the socket's own error if the write or the shutdown fails.
pub async fn respond(
    stream: &mut TcpStream,
    status: u16,
    reason: &str,
    headers: &[(&str, String)],
    body: &str,
) -> io::Result<()> {
    let mut head = String::new();
    // Writing into a `String` cannot fail, and the `Result` is discarded
    // rather than `unwrap`ped for that reason: an `unwrap` here would be a
    // panic path that no input can reach, sitting in a file whose subject is
    // what happens under load.
    let _ = write!(head, "HTTP/1.1 {status} {reason}\r\n");
    let _ = write!(head, "Content-Type: text/plain; charset=utf-8\r\n");
    let _ = write!(head, "Content-Length: {}\r\n", body.len());
    for (name, value) in headers {
        let _ = write!(head, "{name}: {value}\r\n");
    }
    head.push_str("Connection: close\r\n\r\n");

    stream.write_all(head.as_bytes()).await?;
    stream.write_all(body.as_bytes()).await?;
    stream.shutdown().await
}

/// A response, as the client below reads it back.
#[derive(Debug, Clone)]
pub struct Response {
    /// The status code from the status line.
    pub status: u16,
    /// Every header, in the order they arrived, with names lowercased.
    pub headers: Vec<(String, String)>,
    /// The body, verbatim.
    pub body: String,
}

impl Response {
    /// The first value sent for `name`, which must be given lowercase.
    #[must_use]
    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(key, _)| key == name)
            .map(|(_, value)| value.as_str())
    }
}

/// Sends one request to `address` and reads the whole reply.
///
/// One connection per request, which is what `Connection: close` above buys:
/// the reply ends at end-of-file, so nothing here has to implement framing.
///
/// # Errors
///
/// Returns the socket's own error if the connection, the write or the read
/// fails, and [`io::ErrorKind::InvalidData`] if the reply has no status line.
pub async fn request(address: SocketAddr, method: &str, target: &str) -> io::Result<Response> {
    let mut stream = TcpStream::connect(address).await?;
    let head =
        format!("{method} {target} HTTP/1.1\r\nHost: {address}\r\nConnection: close\r\n\r\n");
    stream.write_all(head.as_bytes()).await?;

    let mut raw = Vec::new();
    stream.read_to_end(&mut raw).await?;
    let text = String::from_utf8_lossy(&raw).into_owned();

    let (head, body) = text
        .split_once("\r\n\r\n")
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "no header terminator"))?;
    let mut lines = head.lines();
    let status = lines
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .and_then(|code| code.parse().ok())
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "no status code"))?;

    let headers = lines
        .filter_map(|line| line.split_once(':'))
        .map(|(name, value)| (name.trim().to_lowercase(), value.trim().to_owned()))
        .collect();

    Ok(Response {
        status,
        headers,
        body: body.to_owned(),
    })
}
