use axum::{
    Json, Router,
    extract::{
        State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    response::IntoResponse,
    routing::get,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::sync::{RwLock, mpsc};
use tracing::{info, warn};
use uuid::Uuid;

const ROOM_CODE_CHARS: &[u8] = b"ABCDEFGHJKMNPQRSTUVWXYZ23456789";
const ROOM_CODE_LENGTH: usize = 6;
const TURN_ORDER_DICE: usize = 3;

type SharedRooms = Arc<RwLock<HashMap<String, Room>>>;
type ClientSender = mpsc::UnboundedSender<ServerMessage>;

#[derive(Clone)]
struct AppState {
    rooms: SharedRooms,
}

struct Room {
    host_id: Uuid,
    max_players: u8,
    visibility: RoomVisibility,
    players: Vec<Player>,
    state: RoomState,
}

struct Player {
    id: Uuid,
    name: String,
    is_host: bool,
    ready: bool,
    turn_rolls: [Option<u8>; TURN_ORDER_DICE],
    sender: ClientSender,
}

#[derive(Clone, Copy)]
enum RoomVisibility {
    Public,
    Private,
}

impl RoomVisibility {
    fn as_str(self) -> &'static str {
        match self {
            Self::Public => "public",
            Self::Private => "private",
        }
    }
}

#[derive(Clone, Copy)]
enum RoomState {
    Lobby,
    TurnOrder,
    TurnOrderResult,
}

impl RoomState {
    fn as_str(self) -> &'static str {
        match self {
            Self::Lobby => "lobby",
            Self::TurnOrder => "turn_order",
            Self::TurnOrderResult => "turn_order_result",
        }
    }
}

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum ClientMessage {
    Ping,

    CreateRoom {
        player_name: String,
        max_players: u8,
        visibility: String,
    },

    JoinRoom {
        player_name: String,
        room_code: String,
    },

    SetReady {
        ready: bool,
    },

    StartGame,

    RollTurnDie {
        die_index: usize,
    },
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum ServerMessage {
    Pong,

    RoomCreated {
        room_code: String,
        player_id: Uuid,
        host_id: Uuid,
        max_players: u8,
        visibility: String,
        state: String,
        players: Vec<PlayerResponse>,
    },

    RoomJoined {
        room_code: String,
        player_id: Uuid,
        host_id: Uuid,
        max_players: u8,
        visibility: String,
        state: String,
        players: Vec<PlayerResponse>,
    },

    RoomState {
        room_code: String,
        host_id: Uuid,
        max_players: u8,
        visibility: String,
        state: String,
        players: Vec<PlayerResponse>,
    },

    TurnOrderStarted {
        room_code: String,
        players: Vec<TurnOrderPlayerResponse>,
    },

    TurnOrderState {
        players: Vec<TurnOrderPlayerResponse>,
        all_complete: bool,
    },

    TurnOrderFinished {
        ranking: Vec<TurnOrderRankingEntry>,
    },

    Error {
        message: String,
    },
}

#[derive(Debug, Clone, Serialize)]
struct PlayerResponse {
    id: Uuid,
    name: String,
    is_host: bool,
    ready: bool,
}

#[derive(Debug, Clone, Serialize)]
struct TurnOrderPlayerResponse {
    id: Uuid,
    name: String,
    rolls: Vec<Option<u8>>,
    total: u16,
    complete: bool,
}

#[derive(Debug, Clone, Serialize)]
struct TurnOrderRankingEntry {
    player_id: Uuid,
    name: String,
    total: u16,
    position: usize,
    tied: bool,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let state = AppState {
        rooms: Arc::new(RwLock::new(HashMap::new())),
    };

    let app = Router::new()
        .route("/health", get(health))
        .route("/ws", get(websocket_handler))
        .with_state(state);

    let address = SocketAddr::from(([127, 0, 0, 1], 3000));

    let listener = tokio::net::TcpListener::bind(address)
        .await
        .expect("failed to bind server address");

    info!("Kivo Play server listening on http://{}", address);

    axum::serve(listener, app).await.expect("server failed");
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: AppState) {
    info!("WebSocket client connected");

    let (outgoing_tx, mut outgoing_rx) = mpsc::unbounded_channel::<ServerMessage>();

    let mut current_room: Option<String> = None;
    let mut current_player: Option<Uuid> = None;

    loop {
        tokio::select! {
            incoming = socket.recv() => {
                match incoming {
                    Some(Ok(Message::Text(text))) => {
                        info!("Received message: {}", text);

                        let parsed =
                            serde_json::from_str::<ClientMessage>(&text);

                        match parsed {
                            Ok(ClientMessage::Ping) => {
                                let _ =
                                    outgoing_tx.send(ServerMessage::Pong);
                            }

                            Ok(ClientMessage::CreateRoom {
                                player_name,
                                max_players,
                                visibility,
                            }) => {
                                if current_room.is_some() {
                                    let _ = outgoing_tx.send(
                                        ServerMessage::Error {
                                            message:
                                                "You are already inside a room"
                                                    .to_string(),
                                        },
                                    );

                                    continue;
                                }

                                match create_room(
                                    &state,
                                    outgoing_tx.clone(),
                                    player_name,
                                    max_players,
                                    visibility,
                                )
                                .await
                                {
                                    Ok((
                                        room_code,
                                        player_id,
                                        response,
                                    )) => {
                                        current_room =
                                            Some(room_code.clone());

                                        current_player =
                                            Some(player_id);

                                        let _ =
                                            outgoing_tx.send(response);

                                        broadcast_room_state(
                                            &state,
                                            &room_code,
                                        )
                                        .await;
                                    }

                                    Err(error) => {
                                        let _ =
                                            outgoing_tx.send(error);
                                    }
                                }
                            }

                            Ok(ClientMessage::JoinRoom {
                                player_name,
                                room_code,
                            }) => {
                                if current_room.is_some() {
                                    let _ = outgoing_tx.send(
                                        ServerMessage::Error {
                                            message:
                                                "You are already inside a room"
                                                    .to_string(),
                                        },
                                    );

                                    continue;
                                }

                                match join_room(
                                    &state,
                                    outgoing_tx.clone(),
                                    player_name,
                                    room_code,
                                )
                                .await
                                {
                                    Ok((
                                        room_code,
                                        player_id,
                                        response,
                                    )) => {
                                        current_room =
                                            Some(room_code.clone());

                                        current_player =
                                            Some(player_id);

                                        let _ =
                                            outgoing_tx.send(response);

                                        broadcast_room_state(
                                            &state,
                                            &room_code,
                                        )
                                        .await;
                                    }

                                    Err(error) => {
                                        let _ =
                                            outgoing_tx.send(error);
                                    }
                                }
                            }

                            Ok(ClientMessage::SetReady { ready }) => {
                                let (
                                    Some(room_code),
                                    Some(player_id),
                                ) = (
                                    &current_room,
                                    current_player,
                                )
                                else {
                                    let _ = outgoing_tx.send(
                                        ServerMessage::Error {
                                            message:
                                                "You are not inside a room"
                                                    .to_string(),
                                        },
                                    );

                                    continue;
                                };

                                if let Err(error) =
                                    set_player_ready(
                                        &state,
                                        room_code,
                                        player_id,
                                        ready,
                                    )
                                    .await
                                {
                                    let _ =
                                        outgoing_tx.send(error);
                                }
                            }

                            Ok(ClientMessage::StartGame) => {
                                let (
                                    Some(room_code),
                                    Some(player_id),
                                ) = (
                                    &current_room,
                                    current_player,
                                )
                                else {
                                    let _ = outgoing_tx.send(
                                        ServerMessage::Error {
                                            message:
                                                "You are not inside a room"
                                                    .to_string(),
                                        },
                                    );

                                    continue;
                                };

                                if let Err(error) =
                                    start_game(
                                        &state,
                                        room_code,
                                        player_id,
                                    )
                                    .await
                                {
                                    let _ =
                                        outgoing_tx.send(error);
                                }
                            }

                            Ok(ClientMessage::RollTurnDie {
                                die_index,
                            }) => {
                                let (
                                    Some(room_code),
                                    Some(player_id),
                                ) = (
                                    &current_room,
                                    current_player,
                                )
                                else {
                                    let _ = outgoing_tx.send(
                                        ServerMessage::Error {
                                            message:
                                                "You are not inside a room"
                                                    .to_string(),
                                        },
                                    );

                                    continue;
                                };

                                if let Err(error) =
                                    roll_turn_die(
                                        &state,
                                        room_code,
                                        player_id,
                                        die_index,
                                    )
                                    .await
                                {
                                    let _ =
                                        outgoing_tx.send(error);
                                }
                            }

                            Err(error) => {
                                warn!(
                                    "Invalid WebSocket message: {}",
                                    error
                                );

                                let _ = outgoing_tx.send(
                                    ServerMessage::Error {
                                        message:
                                            "Invalid message".to_string(),
                                    },
                                );
                            }
                        }
                    }

                    Some(Ok(Message::Close(_))) => {
                        info!("WebSocket client disconnected");
                        break;
                    }

                    Some(Ok(_)) => {}

                    Some(Err(error)) => {
                        warn!("WebSocket error: {}", error);
                        break;
                    }

                    None => break,
                }
            }

            outgoing = outgoing_rx.recv() => {
                let Some(message) = outgoing else {
                    break;
                };

                let json =
                    match serde_json::to_string(&message) {
                        Ok(json) => json,

                        Err(error) => {
                            warn!(
                                "Failed to serialize server message: {}",
                                error
                            );

                            continue;
                        }
                    };

                if socket
                    .send(Message::Text(json.into()))
                    .await
                    .is_err()
                {
                    break;
                }
            }
        }
    }

    if let (Some(room_code), Some(player_id)) = (current_room, current_player) {
        remove_player(&state, &room_code, player_id).await;
    }

    info!("WebSocket connection closed");
}

async fn create_room(
    state: &AppState,
    sender: ClientSender,
    player_name: String,
    max_players: u8,
    visibility: String,
) -> Result<(String, Uuid, ServerMessage), ServerMessage> {
    let player_name = validate_player_name(player_name)?;

    if !(1..=4).contains(&max_players) {
        return Err(ServerMessage::Error {
            message: "Max players must be between 1 and 4".to_string(),
        });
    }

    let room_visibility = match visibility.as_str() {
        "private" => RoomVisibility::Private,
        "public" => RoomVisibility::Public,

        _ => {
            return Err(ServerMessage::Error {
                message: "Visibility must be public or private".to_string(),
            });
        }
    };

    let player_id = Uuid::new_v4();

    let player = Player {
        id: player_id,
        name: player_name,
        is_host: true,
        ready: false,
        turn_rolls: [None, None, None],
        sender,
    };

    let mut rooms = state.rooms.write().await;

    let room_code = loop {
        let code = generate_room_code();

        if !rooms.contains_key(&code) {
            break code;
        }
    };

    let room = Room {
        host_id: player_id,
        max_players,
        visibility: room_visibility,
        players: vec![player],
        state: RoomState::Lobby,
    };

    let response = ServerMessage::RoomCreated {
        room_code: room_code.clone(),
        player_id,
        host_id: player_id,
        max_players,
        visibility: room_visibility.as_str().to_string(),
        state: room.state.as_str().to_string(),
        players: room_players_response(&room),
    };

    rooms.insert(room_code.clone(), room);

    info!("Room {} created", room_code);

    Ok((room_code, player_id, response))
}

async fn join_room(
    state: &AppState,
    sender: ClientSender,
    player_name: String,
    room_code: String,
) -> Result<(String, Uuid, ServerMessage), ServerMessage> {
    let player_name = validate_player_name(player_name)?;

    let room_code = room_code.trim().to_uppercase();

    if room_code.len() != ROOM_CODE_LENGTH {
        return Err(ServerMessage::Error {
            message: "Invalid room code".to_string(),
        });
    }

    let mut rooms = state.rooms.write().await;

    let room = rooms
        .get_mut(&room_code)
        .ok_or_else(|| ServerMessage::Error {
            message: "Room not found".to_string(),
        })?;

    if !matches!(room.state, RoomState::Lobby) {
        return Err(ServerMessage::Error {
            message: "This game has already started".to_string(),
        });
    }

    if room.players.len() >= usize::from(room.max_players) {
        return Err(ServerMessage::Error {
            message: "Room is full".to_string(),
        });
    }

    if room
        .players
        .iter()
        .any(|player| player.name.eq_ignore_ascii_case(&player_name))
    {
        return Err(ServerMessage::Error {
            message: "That player name is already being used in this room".to_string(),
        });
    }

    let player_id = Uuid::new_v4();

    room.players.push(Player {
        id: player_id,
        name: player_name,
        is_host: false,
        ready: false,
        turn_rolls: [None, None, None],
        sender,
    });

    let response = ServerMessage::RoomJoined {
        room_code: room_code.clone(),
        player_id,
        host_id: room.host_id,
        max_players: room.max_players,
        visibility: room.visibility.as_str().to_string(),
        state: room.state.as_str().to_string(),
        players: room_players_response(room),
    };

    info!("Player {} joined room {}", player_id, room_code);

    Ok((room_code, player_id, response))
}

async fn set_player_ready(
    state: &AppState,
    room_code: &str,
    player_id: Uuid,
    ready: bool,
) -> Result<(), ServerMessage> {
    {
        let mut rooms = state.rooms.write().await;

        let room = rooms
            .get_mut(room_code)
            .ok_or_else(|| ServerMessage::Error {
                message: "Room not found".to_string(),
            })?;

        if !matches!(room.state, RoomState::Lobby) {
            return Err(ServerMessage::Error {
                message: "Ready state cannot be changed after the game starts".to_string(),
            });
        }

        let player = room
            .players
            .iter_mut()
            .find(|player| player.id == player_id)
            .ok_or_else(|| ServerMessage::Error {
                message: "Player not found".to_string(),
            })?;

        player.ready = ready;
    }

    broadcast_room_state(state, room_code).await;

    Ok(())
}

async fn start_game(
    state: &AppState,
    room_code: &str,
    player_id: Uuid,
) -> Result<(), ServerMessage> {
    let (senders, players) = {
        let mut rooms = state.rooms.write().await;

        let room = rooms
            .get_mut(room_code)
            .ok_or_else(|| ServerMessage::Error {
                message: "Room not found".to_string(),
            })?;

        if room.host_id != player_id {
            return Err(ServerMessage::Error {
                message: "Only the host can start the game".to_string(),
            });
        }

        if !matches!(room.state, RoomState::Lobby) {
            return Err(ServerMessage::Error {
                message: "The game has already started".to_string(),
            });
        }

        if room.players.len() < 2 {
            return Err(ServerMessage::Error {
                message: "At least 2 players are required".to_string(),
            });
        }

        if !room.players.iter().all(|player| player.ready) {
            return Err(ServerMessage::Error {
                message: "All players must be ready".to_string(),
            });
        }

        room.state = RoomState::TurnOrder;

        for player in &mut room.players {
            player.turn_rolls = [None, None, None];
        }

        let senders = room
            .players
            .iter()
            .map(|player| player.sender.clone())
            .collect::<Vec<_>>();

        let players = turn_order_players_response(room);

        (senders, players)
    };

    let message = ServerMessage::TurnOrderStarted {
        room_code: room_code.to_string(),
        players,
    };

    for sender in senders {
        let _ = sender.send(message.clone());
    }

    info!("Turn-order phase started in room {}", room_code);

    Ok(())
}

async fn roll_turn_die(
    state: &AppState,
    room_code: &str,
    player_id: Uuid,
    die_index: usize,
) -> Result<(), ServerMessage> {
    if die_index >= TURN_ORDER_DICE {
        return Err(ServerMessage::Error {
            message: "Invalid die index".to_string(),
        });
    }

    {
        let mut rooms = state.rooms.write().await;

        let room = rooms
            .get_mut(room_code)
            .ok_or_else(|| ServerMessage::Error {
                message: "Room not found".to_string(),
            })?;

        if !matches!(room.state, RoomState::TurnOrder) {
            return Err(ServerMessage::Error {
                message: "Turn-order phase is not active".to_string(),
            });
        }

        let player = room
            .players
            .iter_mut()
            .find(|player| player.id == player_id)
            .ok_or_else(|| ServerMessage::Error {
                message: "Player not found".to_string(),
            })?;

        if player.turn_rolls[die_index].is_some() {
            return Err(ServerMessage::Error {
                message: "This die has already been rolled".to_string(),
            });
        }

        let value: u8 = rand::random_range(1..=6);

        player.turn_rolls[die_index] = Some(value);

        info!("Player {} rolled die {} = {}", player_id, die_index, value);
    }

    broadcast_turn_order_state(state, room_code).await;

    finish_turn_order_if_ready(state, room_code).await;

    Ok(())
}

async fn broadcast_turn_order_state(state: &AppState, room_code: &str) {
    let (senders, players, all_complete) = {
        let rooms = state.rooms.read().await;

        let Some(room) = rooms.get(room_code) else {
            return;
        };

        let senders = room
            .players
            .iter()
            .map(|player| player.sender.clone())
            .collect::<Vec<_>>();

        let players = turn_order_players_response(room);

        let all_complete = room
            .players
            .iter()
            .all(|player| player.turn_rolls.iter().all(Option::is_some));

        (senders, players, all_complete)
    };

    let message = ServerMessage::TurnOrderState {
        players,
        all_complete,
    };

    for sender in senders {
        let _ = sender.send(message.clone());
    }
}

async fn finish_turn_order_if_ready(state: &AppState, room_code: &str) {
    let (senders, ranking) = {
        let mut rooms = state.rooms.write().await;

        let Some(room) = rooms.get_mut(room_code) else {
            return;
        };

        if !matches!(room.state, RoomState::TurnOrder) {
            return;
        }

        let all_complete = room
            .players
            .iter()
            .all(|player| player.turn_rolls.iter().all(Option::is_some));

        if !all_complete {
            return;
        }

        room.state = RoomState::TurnOrderResult;

        let mut totals = room
            .players
            .iter()
            .map(|player| (player.id, player.name.clone(), turn_roll_total(player)))
            .collect::<Vec<_>>();

        totals.sort_by(|a, b| b.2.cmp(&a.2));

        let mut ranking: Vec<TurnOrderRankingEntry> = Vec::with_capacity(totals.len());

        for (index, (player_id, name, total)) in totals.iter().enumerate() {
            let position = if index > 0 && totals[index - 1].2 == *total {
                ranking[index - 1].position
            } else {
                index + 1
            };

            let tied = totals
                .iter()
                .filter(|(_, _, other_total)| other_total == total)
                .count()
                > 1;

            ranking.push(TurnOrderRankingEntry {
                player_id: *player_id,
                name: name.clone(),
                total: *total,
                position,
                tied,
            });
        }

        let senders = room
            .players
            .iter()
            .map(|player| player.sender.clone())
            .collect::<Vec<_>>();

        (senders, ranking)
    };

    let message = ServerMessage::TurnOrderFinished { ranking };

    for sender in senders {
        let _ = sender.send(message.clone());
    }

    info!("Turn-order phase finished in room {}", room_code);
}

async fn broadcast_room_state(state: &AppState, room_code: &str) {
    let (senders, message) = {
        let rooms = state.rooms.read().await;

        let Some(room) = rooms.get(room_code) else {
            return;
        };

        let senders = room
            .players
            .iter()
            .map(|player| player.sender.clone())
            .collect::<Vec<_>>();

        let message = ServerMessage::RoomState {
            room_code: room_code.to_string(),
            host_id: room.host_id,
            max_players: room.max_players,
            visibility: room.visibility.as_str().to_string(),
            state: room.state.as_str().to_string(),
            players: room_players_response(room),
        };

        (senders, message)
    };

    for sender in senders {
        let _ = sender.send(message.clone());
    }
}

async fn remove_player(state: &AppState, room_code: &str, player_id: Uuid) {
    let mut delete_room = false;
    let mut should_check_turn_order = false;

    {
        let mut rooms = state.rooms.write().await;

        let Some(room) = rooms.get_mut(room_code) else {
            return;
        };

        let was_host = room.host_id == player_id;

        room.players.retain(|player| player.id != player_id);

        if room.players.is_empty() {
            delete_room = true;
        } else {
            if was_host {
                let new_host_id = room.players[0].id;

                room.host_id = new_host_id;

                for player in &mut room.players {
                    player.is_host = player.id == new_host_id;
                }

                info!(
                    "Host migrated in room {} to player {}",
                    room_code, new_host_id
                );
            }

            should_check_turn_order = matches!(room.state, RoomState::TurnOrder);
        }
    }

    if delete_room {
        let mut rooms = state.rooms.write().await;

        rooms.remove(room_code);

        info!("Room {} removed because it is empty", room_code);

        return;
    }

    if should_check_turn_order {
        broadcast_turn_order_state(state, room_code).await;

        finish_turn_order_if_ready(state, room_code).await;
    } else {
        broadcast_room_state(state, room_code).await;
    }
}

fn validate_player_name(player_name: String) -> Result<String, ServerMessage> {
    let player_name = player_name.trim().to_string();

    if player_name.is_empty() {
        return Err(ServerMessage::Error {
            message: "Player name cannot be empty".to_string(),
        });
    }

    if player_name.chars().count() > 24 {
        return Err(ServerMessage::Error {
            message: "Player name is too long".to_string(),
        });
    }

    Ok(player_name)
}

fn room_players_response(room: &Room) -> Vec<PlayerResponse> {
    room.players
        .iter()
        .map(|player| PlayerResponse {
            id: player.id,
            name: player.name.clone(),
            is_host: player.is_host,
            ready: player.ready,
        })
        .collect()
}

fn turn_order_players_response(room: &Room) -> Vec<TurnOrderPlayerResponse> {
    room.players
        .iter()
        .map(|player| TurnOrderPlayerResponse {
            id: player.id,
            name: player.name.clone(),
            rolls: player.turn_rolls.to_vec(),
            total: turn_roll_total(player),
            complete: player.turn_rolls.iter().all(Option::is_some),
        })
        .collect()
}

fn turn_roll_total(player: &Player) -> u16 {
    player
        .turn_rolls
        .iter()
        .flatten()
        .map(|value| u16::from(*value))
        .sum()
}

fn generate_room_code() -> String {
    (0..ROOM_CODE_LENGTH)
        .map(|_| {
            let index = rand::random_range(0..ROOM_CODE_CHARS.len());

            ROOM_CODE_CHARS[index] as char
        })
        .collect()
}
