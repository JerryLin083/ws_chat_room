use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant},
};

use tokio::{
    sync::{Mutex, broadcast},
    time::sleep,
};
use uuid::Uuid;

pub struct SessionManager {
    sessions: Arc<Mutex<HashMap<String, Session>>>,
    duration: Duration,
    shutdown: broadcast::Sender<()>,
}

impl SessionManager {
    pub fn build(duration: Duration) -> Arc<SessionManager> {
        let (tx, _rx) = broadcast::channel(2);

        Arc::new(SessionManager {
            sessions: Arc::new(Mutex::new(HashMap::new())),
            duration,
            shutdown: tx,
        })
    }

    pub async fn new_session(self: &Arc<Self>, user_id: i32) -> String {
        let mut sessions = self.sessions.lock().await;
        let session_id = Uuid::new_v4();
        sessions.insert(
            session_id.to_string(),
            Session {
                user_id,
                expiration: Instant::now(),
            },
        );

        session_id.to_string()
    }

    pub async fn update_session(self: &Arc<Self>, session_id: String, user_id: i32) {
        let mut sessions = self.sessions.lock().await;

        sessions.insert(
            session_id,
            Session {
                user_id,
                expiration: Instant::now(),
            },
        );
    }

    pub async fn delete_session(self: &Arc<Self>, session_id: &str) {
        let mut sessions = self.sessions.lock().await;

        sessions.remove(session_id);
    }

    pub async fn run_checker(self: &Arc<Self>) {
        let session_manager = self.clone();
        let mut shutdown_receiver = self.shutdown.subscribe();

        tokio::select! {
          _ = shutdown_receiver.recv() => {
            tracing::info!("Server shutdown...")
          }
          _ = tokio::spawn(async move {
              let check_timing = session_manager.duration;

              loop {
                  sleep(check_timing).await;

                  let mut sessions = session_manager.sessions.lock().await;

                  //session was expired when expiration smaller then now
                  sessions.retain(|_k, session| session.expiration > Instant::now());
              }
          }) => {}
        }
    }
}

pub struct Session {
    user_id: i32,
    expiration: Instant,
}
