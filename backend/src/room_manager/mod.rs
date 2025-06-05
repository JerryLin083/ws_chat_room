use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant},
};

use sqlx::{Pool, Postgres};
use tokio::{
    sync::{Mutex, broadcast, mpsc},
    time::sleep,
};
use uuid::Uuid;

pub struct RoomManager {
    pub rooms: Arc<Mutex<HashMap<String, RoomState>>>,
    pub idle: Duration,
}

impl RoomManager {
    pub fn build(idle: Duration) -> Arc<RoomManager> {
        Arc::new(RoomManager {
            rooms: Arc::new(Mutex::new(HashMap::new())),
            idle,
        })
    }

    pub async fn create(
        self: Arc<Self>,
        pool: Pool<Postgres>,
        room_name: &str,
    ) -> Result<u64, String> {
        let (channel_sender, channel_receiver) = mpsc::channel(128);
        let (subscriber_sender, _subscriber_receiver) = broadcast::channel(128);

        //create room_id
        let room_id = Uuid::new_v4();
        let room_manager = self.clone();
        let mut rooms = room_manager.rooms.lock().await;

        rooms.insert(
            room_id.to_string(),
            RoomState {
                channel_sender: channel_sender.clone(),
                subscriber_sender: subscriber_sender.clone(),
            },
        );

        //release mutex lock
        drop(rooms);

        //spawn room handler
        self.create_room(channel_receiver, subscriber_sender, room_id);

        //TODO:insert room to DB
        let query_str = r#"
            insert into rooms(id, room_name)
            values($1, $2);
          "#;

        let result = sqlx::query(query_str)
            .bind(&room_id.to_string())
            .bind(room_name)
            .execute(&pool)
            .await;

        match result {
            Ok(query_result) => Ok(query_result.rows_affected()),
            Err(err) => Err(err.to_string()),
        }
    }

    fn create_room(
        self: Arc<Self>,
        mut channel_receiver: mpsc::Receiver<RoomCommand>,
        subscriber_sender: broadcast::Sender<RoomCommand>,
        room_id: Uuid,
    ) {
        tokio::spawn(async move {
            let idle = self.idle;
            let close_time = Arc::new(Mutex::new(Instant::now() + idle));
            let close_time_for_timer = close_time.clone();
            let close_time_for_room = close_time.clone();

            tokio::select! {
                _ = async {
                  let time = close_time_for_timer.lock().await;

                  while Instant::now() < *time {
                      sleep(idle).await;
                  };
                } => {}
                _ = async  {
                  while let Some(command) = channel_receiver.recv().await {
                    let mut time = close_time_for_room.lock().await;
                    *time = Instant::now() + idle;

                    match command.method {
                        Method::KickOut => { /* TODO */ }
                        Method::Join => { /* TODO */ }
                        Method::Leave => { /* TODO */ }
                        Method::Send => {
                            let _ = subscriber_sender.send(command);
                        }
                        Method::Close => { break; }
                    }
                }
              } => {}
            };

            //remove room from hashmap
            self.delete_room(&room_id.to_string()).await;
        });
    }

    pub async fn delete_room(self: Arc<Self>, room_id: &str) {
        let room_manager = self.clone();
        let mut rooms = room_manager.rooms.lock().await;

        //TODO: send close message;

        rooms.remove(room_id);
    }
}

pub struct RoomState {
    pub channel_sender: mpsc::Sender<RoomCommand>,
    pub subscriber_sender: broadcast::Sender<RoomCommand>,
}

#[derive(Debug, Clone)]
pub struct RoomCommand {
    pub method: Method,
    pub client_id: String,
    pub message: String,
}

#[derive(Debug, Clone)]
pub enum Method {
    KickOut,
    Send,
    Leave,
    Join,
    Close,
}
