//
// MODIFIED VERSION OF
// https://github.com/upbasedev/sse-actix-web/blob/89c876580071231cc3fde19918fd055c0ed6c0ac/src/lib.rs
//
// MIT License, upbasedev / sse-actix-web
//
use actix_web::web::{Bytes, Data};
use actix_web::Error;
use futures::Stream;
use std::pin::Pin;
use std::sync::Mutex;
use std::task::{Context, Poll};
use std::time::Duration;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::task;
use tokio::time::{interval_at, Instant};

pub struct Broadcaster {
    clients: Vec<Sender<Bytes>>,
}

impl Broadcaster {
    pub fn create() -> Data<Mutex<Self>> {
        // Data ≃ Arc
        let me = Data::new(Mutex::new(Broadcaster::new()));

        // ping clients every 10 seconds to see if they are alive
        Broadcaster::spawn_ping(me.clone());

        me
    }

    pub fn new() -> Self {
        Broadcaster {
            clients: Vec::new(),
        }
    }

    pub fn spawn_ping(me: Data<Mutex<Self>>) {
        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            let ls = task::LocalSet::new();
            ls.block_on(&rt, async {
                let mut interval = interval_at(Instant::now(), Duration::from_secs(1));
                loop {
                    interval.tick().await;
                    me.lock().unwrap().remove_stale_clients();
                }
            });
        });
    }

    pub fn remove_stale_clients(&mut self) {
        let mut ok_clients = Vec::new();

        let ping = serde_json::json!({
            "time": chrono::Utc::now().naive_utc().format("%Y-%m-%d %H:%M:%S").to_string()
        })
        .to_string();

        // debug!("pinging {} clients", self.clients.len());

        for client in self.clients.iter() {
            let result = client
                .clone()
                .try_send(Bytes::from(["event: ping\ndata: ", &ping, "\n\n"].concat()));

            if let Ok(()) = result {
                ok_clients.push(client.clone());
            }
        }
        self.clients = ok_clients;
    }

    pub fn new_client(&mut self) -> Client {
        let (tx, rx) = channel(100);
        self.clients.push(tx);
        Client(rx)
    }

    pub fn send(&self, msg: &str) {
        let msg = Bytes::from(["event: message\n", "data: ", msg, "\n\n"].concat());

        for client in self.clients.iter() {
            client.clone().try_send(msg.clone()).unwrap_or(());
        }
    }
}

// wrap Receiver in own type, with correct error type
pub struct Client(Receiver<Bytes>);

impl Stream for Client {
    type Item = Result<Bytes, Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match Pin::new(&mut self.0).poll_recv(cx) {
            Poll::Ready(Some(v)) => Poll::Ready(Some(Ok(v))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}
