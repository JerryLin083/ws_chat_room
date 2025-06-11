use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant},
};

use sqlx::{Error, Pool, Postgres};
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
        db_message_sender: mpsc::Sender<RoomCommand>,
    ) -> Result<
        (
            mpsc::Sender<RoomCommand>,
            broadcast::Receiver<RoomCommand>,
            String,
        ),
        Error,
    > {
        let (channel_sender, channel_receiver) = mpsc::channel(128);
        let (subscriber_sender, subscriber_receiver) = broadcast::channel(128);

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

        //insert room to DB
        let query_str = r#"
            insert into rooms(id, room_name)
            values($1, $2);
          "#;

        let result = sqlx::query(query_str)
            .bind(room_id)
            .bind(room_name)
            .execute(&pool)
            .await;

        match result {
            Ok(_query_result) => {
                //spawn room handler
                self.create_room(
                    channel_receiver,
                    subscriber_sender,
                    room_id,
                    db_message_sender,
                );

                let sender = channel_sender.clone();
                let receiver = subscriber_receiver;

                return Ok((sender, receiver, room_id.to_string()));
            }
            Err(err) => {
                return Err(err);
            }
        }
    }

    fn create_room(
        self: Arc<Self>,
        mut channel_receiver: mpsc::Receiver<RoomCommand>,
        subscriber_sender: broadcast::Sender<RoomCommand>,
        room_id: Uuid,
        db_message_sender: mpsc::Sender<RoomCommand>,
    ) {
        tokio::spawn(async move {
            let idle = self.idle;
            let close_time = Arc::new(Mutex::new(Instant::now() + idle));
            let close_time_for_timer = close_time.clone();
            let close_time_for_room = close_time.clone();

            tokio::select! {
                _ = async {
                  let time = close_time_for_timer.lock().await.clone();

                  while Instant::now() < time {
                      sleep(idle).await;
                  };
                } => {}
                _ = async {
                  while let Some(command) = channel_receiver.recv().await {
                    let mut time = close_time_for_room.lock().await;
                    *time = Instant::now() + idle;

                    match command.method {
                        Method::Close => {
                            break;
                        }
                        Method::Send => {
                            let _ = subscriber_sender.send(command.clone());

                            //TODO: insert message to db
                            if let Err(err) = db_message_sender.send(command).await{
                                eprintln!("Failed to insert message: {:?}", err);
                            }
                        }
                        _ => {
                            let _ = subscriber_sender.send(command);
                        }
                    }
                }
              } => {}
            };

            //remove room from hashmap
            self.delete_room(&room_id.to_string()).await;
        });
    }

    pub async fn join(
        self: Arc<Self>,
        room_id: &str,
    ) -> Option<(mpsc::Sender<RoomCommand>, broadcast::Receiver<RoomCommand>)> {
        let room_manager = self.clone();
        let rooms = room_manager.rooms.lock().await;

        if let Some(room_state) = rooms.get(room_id) {
            let sender = room_state.channel_sender.clone();
            let receiver = room_state.subscriber_sender.subscribe();

            return Some((sender, receiver));
        }

        None
    }

    pub async fn delete_room(self: Arc<Self>, room_id: &str) {
        let room_manager = self.clone();
        let mut rooms = room_manager.rooms.lock().await;

        //send close message and remove room from rooms;
        match rooms.get(room_id) {
            Some(room) => {
                let broadcast_sender = room.subscriber_sender.clone();

                let _ = broadcast_sender.send(RoomCommand::close());

                rooms.remove(room_id);
            }

            None => {}
        }
    }
}

pub struct RoomState {
    pub channel_sender: mpsc::Sender<RoomCommand>,
    pub subscriber_sender: broadcast::Sender<RoomCommand>,
}

#[derive(Debug, Clone)]
pub struct RoomCommand {
    pub method: Method,
    pub room_id: Option<String>,
    pub user_id: Option<i32>,
    pub user: Option<String>,
    pub message: Option<String>,
}

impl RoomCommand {
    pub fn join(user: String) -> Self {
        RoomCommand {
            method: Method::Join,
            room_id: None,
            user_id: None,
            user: Some(user),
            message: None,
        }
    }

    pub fn send(user_id: i32, user: String, room_id: String, message: String) -> Self {
        RoomCommand {
            method: Method::Send,
            room_id: Some(room_id),
            user_id: Some(user_id),
            user: Some(user),
            message: Some(message),
        }
    }

    pub fn leave(user: String) -> Self {
        RoomCommand {
            method: Method::Leave,
            room_id: None,
            user_id: None,
            user: Some(user),
            message: None,
        }
    }

    pub fn close() -> Self {
        RoomCommand {
            method: Method::Close,
            room_id: None,
            user_id: None,
            user: None,
            message: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Method {
    Send,
    Leave,
    Join,
    Close,
}
