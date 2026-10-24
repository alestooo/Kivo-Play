mod games;
mod match_state;
mod turn_order;

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

use std::{
    collections::{HashMap, HashSet},
    net::SocketAddr,
    sync::Arc,
    time::{Duration, Instant},
};

use tokio::sync::{RwLock, mpsc};

use tracing::{info, warn};

use uuid::Uuid;

use games::{
    flux::{
        actions::play_card as flux_play_card_action,
        cards::{ClashCardColor, ClashColor, FluxCard, FluxCardKind, FluxSpecial},
        messages::{FluxPrivateState, FluxPublicPlayer, FluxScoreResponse, FluxStateResponse},
        rules::can_play_card,
        state::{
            FluxPlayerState, FluxState, PendingClash, PendingDrawKind, PendingStealOffer,
            STARTING_HAND_SIZE,
        },
    },
    registry::GameId,
};

use turn_order::{
    DICE_PER_PLAYER, DieState, PhysicsDieInput, PhysicsFrameResponse, physics::simulate_roll,
};

const ROOM_CODE_LETTERS: &[u8] = b"ABCDEFGHJKMNPQRSTUVWXYZ";

const ROOM_CODE_DIGITS: &[u8] = b"23456789";

const ROOM_CODE_LENGTH: usize = 6;

const ROOMS_PER_PAGE: usize = 25;

const PHYSICS_REVEAL_DELAY_MS: u64 = 1050;

type SharedRooms = Arc<RwLock<HashMap<String, Room>>>;

type ClientSender = mpsc::UnboundedSender<ServerMessage>;

#[derive(Clone)]
struct AppState {
    rooms: SharedRooms,
}

struct Room {
    id: Uuid,

    name: String,

    host_id: Uuid,

    max_players: u8,

    visibility: RoomVisibility,

    players: Vec<Player>,

    state: RoomState,

    selected_game: Option<GameId>,

    flux_state: Option<FluxState>,

    flux_scores: HashMap<Uuid, u16>,

    flux_order: Vec<Uuid>,

    flux_round_ready: HashSet<Uuid>,
}

struct Player {
    id: Uuid,

    name: String,

    is_host: bool,

    ready: bool,

    turn_dice: [DieState; DICE_PER_PLAYER],

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

    Playing,

    MatchResult,
}

impl RoomState {
    fn as_str(self) -> &'static str {
        match self {
            Self::Lobby => "lobby",

            Self::TurnOrder => "turn_order",

            Self::TurnOrderResult => "turn_order_result",

            Self::Playing => "playing",

            Self::MatchResult => "match_result",
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

        room_name: String,

        max_players: u8,

        visibility: String,
    },

    JoinRoom {
        player_name: String,

        room_code: String,
    },

    JoinPublicRoom {
        player_name: String,

        room_id: Uuid,
    },

    ListRooms {
        page: usize,

        search: String,
    },

    SelectGame {
        game_id: GameId,
    },

    SetReady {
        ready: bool,
    },

    StartGame,

    RollTurnDie {
        die_index: usize,
    },

    FluxPlayCard {
        card_id: Uuid,

        chosen_color: Option<ClashColor>,

        target_player_id: Option<Uuid>,
    },

    FluxBeginSteal {
        card_id: Uuid,

        target_player_id: Uuid,
    },

    FluxChooseSteal {
        offer_id: Uuid,

        card_id: Uuid,
    },

    FluxClash,

    FluxRoundReady {
        ready: bool,
    },

    FluxNextRound,

    FluxPlayAgain,

    FluxDraw,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum ServerMessage {
    Pong,

    RoomCreated {
        room_code: String,

        room_name: String,

        player_id: Uuid,

        host_id: Uuid,

        max_players: u8,

        visibility: String,

        state: String,

        selected_game: Option<GameId>,

        players: Vec<PlayerResponse>,
    },

    RoomJoined {
        room_code: String,

        room_name: String,

        player_id: Uuid,

        host_id: Uuid,

        max_players: u8,

        visibility: String,

        state: String,

        selected_game: Option<GameId>,

        players: Vec<PlayerResponse>,
    },

    RoomState {
        room_code: String,

        room_name: String,

        host_id: Uuid,

        max_players: u8,

        visibility: String,

        state: String,

        selected_game: Option<GameId>,

        players: Vec<PlayerResponse>,
    },

    RoomDirectory {
        rooms: Vec<RoomSummary>,

        page: usize,

        total_pages: usize,

        total_rooms: usize,
    },

    TurnOrderStarted {
        room_code: String,

        players: Vec<TurnOrderPlayerResponse>,
    },

    TurnOrderPhysics {
        frames: Vec<PhysicsFrameResponse>,
    },

    TurnOrderState {
        players: Vec<TurnOrderPlayerResponse>,

        all_complete: bool,
    },

    TurnOrderFinished {
        ranking: Vec<TurnOrderRankingEntry>,
    },

    FluxState {
        state: FluxStateResponse,

        playable_card_ids: Vec<Uuid>,
    },

    FluxCardPlayed {
        card: FluxCard,

        revision: u64,
    },

    FluxStealOffer {
        offer_id: Uuid,

        target_player_id: Uuid,

        target_player_name: String,

        options: Vec<FluxCard>,
    },

    FluxRoundReadyState {
        ready_player_ids: Vec<Uuid>,

        all_ready: bool,
    },

    FluxImpact {
        text: String,
    },

    FluxRoundFinished {
        winner_id: Uuid,

        winner_name: String,

        points_awarded: u16,

        round: u16,

        scores: Vec<FluxScoreResponse>,

        match_over: bool,
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
struct RoomSummary {
    room_id: Uuid,

    room_name: String,

    host_name: String,

    visibility: String,

    player_count: usize,

    max_players: u8,

    joinable: bool,
}

#[derive(Debug, Clone, Serialize)]
struct TurnOrderPlayerResponse {
    id: Uuid,

    name: String,

    rolls: Vec<Option<u8>>,

    rolled: Vec<bool>,

    total: u16,

    complete: bool,

    roll_count: usize,
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
            incoming =
                socket.recv()
            => {
                match incoming {
                    Some(
                        Ok(
                            Message::Text(
                                text
                            )
                        )
                    ) => {
                        let parsed =
                            serde_json::from_str::<ClientMessage>(
                                &text
                            );

                        match parsed {
                            Ok(
                                ClientMessage::Ping
                            ) => {
                                let _ =
                                    outgoing_tx.send(
                                        ServerMessage::Pong
                                    );
                            }

                            Ok(
                                ClientMessage::ListRooms {
                                    page,
                                    search,
                                }
                            ) => {
                                let message =
                                    list_rooms(
                                        &state,
                                        page,
                                        search,
                                    )
                                    .await;

                                let _ =
                                    outgoing_tx.send(
                                        message
                                    );
                            }

                            Ok(
                                ClientMessage::CreateRoom {
                                    player_name,
                                    room_name,
                                    max_players,
                                    visibility,
                                }
                            ) => {
                                if current_room
                                    .is_some()
                                {
                                    let _ =
                                        outgoing_tx.send(
                                            server_error(
                                                "You are already inside a room"
                                            )
                                        );

                                    continue;
                                }

                                match create_room(
                                    &state,
                                    outgoing_tx.clone(),
                                    player_name,
                                    room_name,
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
                                            Some(
                                                room_code.clone()
                                            );

                                        current_player =
                                            Some(
                                                player_id
                                            );

                                        let _ =
                                            outgoing_tx.send(
                                                response
                                            );

                                        broadcast_room_state(
                                            &state,
                                            &room_code,
                                        )
                                        .await;
                                    }

                                    Err(
                                        error
                                    ) => {
                                        let _ =
                                            outgoing_tx.send(
                                                error
                                            );
                                    }
                                }
                            }

                            Ok(
                                ClientMessage::JoinRoom {
                                    player_name,
                                    room_code,
                                }
                            ) => {
                                if current_room
                                    .is_some()
                                {
                                    let _ =
                                        outgoing_tx.send(
                                            server_error(
                                                "You are already inside a room"
                                            )
                                        );

                                    continue;
                                }

                                match join_room_by_code(
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
                                            Some(
                                                room_code.clone()
                                            );

                                        current_player =
                                            Some(
                                                player_id
                                            );

                                        let _ =
                                            outgoing_tx.send(
                                                response
                                            );

                                        broadcast_room_state(
                                            &state,
                                            &room_code,
                                        )
                                        .await;
                                    }

                                    Err(
                                        error
                                    ) => {
                                        let _ =
                                            outgoing_tx.send(
                                                error
                                            );
                                    }
                                }
                            }

                            Ok(
                                ClientMessage::JoinPublicRoom {
                                    player_name,
                                    room_id,
                                }
                            ) => {
                                if current_room
                                    .is_some()
                                {
                                    let _ =
                                        outgoing_tx.send(
                                            server_error(
                                                "You are already inside a room"
                                            )
                                        );

                                    continue;
                                }

                                match join_public_room(
                                    &state,
                                    outgoing_tx.clone(),
                                    player_name,
                                    room_id,
                                )
                                .await
                                {
                                    Ok((
                                        room_code,
                                        player_id,
                                        response,
                                    )) => {
                                        current_room =
                                            Some(
                                                room_code.clone()
                                            );

                                        current_player =
                                            Some(
                                                player_id
                                            );

                                        let _ =
                                            outgoing_tx.send(
                                                response
                                            );

                                        broadcast_room_state(
                                            &state,
                                            &room_code,
                                        )
                                        .await;
                                    }

                                    Err(
                                        error
                                    ) => {
                                        let _ =
                                            outgoing_tx.send(
                                                error
                                            );
                                    }
                                }
                            }

                            Ok(
                                ClientMessage::SelectGame {
                                    game_id,
                                }
                            ) => {
                                let (
                                    Some(
                                        room_code
                                    ),
                                    Some(
                                        player_id
                                    ),
                                ) = (
                                    &current_room,
                                    current_player,
                                )
                                else {
                                    let _ =
                                        outgoing_tx.send(
                                            server_error(
                                                "You are not inside a room"
                                            )
                                        );

                                    continue;
                                };

                                if let Err(
                                    error
                                ) =
                                    select_game(
                                        &state,
                                        room_code,
                                        player_id,
                                        game_id,
                                    )
                                    .await
                                {
                                    let _ =
                                        outgoing_tx.send(
                                            error
                                        );
                                }
                            }

                            Ok(
                                ClientMessage::SetReady {
                                    ready,
                                }
                            ) => {
                                let (
                                    Some(
                                        room_code
                                    ),
                                    Some(
                                        player_id
                                    ),
                                ) = (
                                    &current_room,
                                    current_player,
                                )
                                else {
                                    let _ =
                                        outgoing_tx.send(
                                            server_error(
                                                "You are not inside a room"
                                            )
                                        );

                                    continue;
                                };

                                if let Err(
                                    error
                                ) =
                                    set_player_ready(
                                        &state,
                                        room_code,
                                        player_id,
                                        ready,
                                    )
                                    .await
                                {
                                    let _ =
                                        outgoing_tx.send(
                                            error
                                        );
                                }
                            }

                            Ok(
                                ClientMessage::StartGame
                            ) => {
                                let (
                                    Some(
                                        room_code
                                    ),
                                    Some(
                                        player_id
                                    ),
                                ) = (
                                    &current_room,
                                    current_player,
                                )
                                else {
                                    let _ =
                                        outgoing_tx.send(
                                            server_error(
                                                "You are not inside a room"
                                            )
                                        );

                                    continue;
                                };

                                if let Err(
                                    error
                                ) =
                                    start_game(
                                        &state,
                                        room_code,
                                        player_id,
                                    )
                                    .await
                                {
                                    let _ =
                                        outgoing_tx.send(
                                            error
                                        );
                                }
                            }

                            Ok(
                                ClientMessage::RollTurnDie {
                                    die_index,
                                }
                            ) => {
                                let (
                                    Some(
                                        room_code
                                    ),
                                    Some(
                                        player_id
                                    ),
                                ) = (
                                    &current_room,
                                    current_player,
                                )
                                else {
                                    let _ =
                                        outgoing_tx.send(
                                            server_error(
                                                "You are not inside a room"
                                            )
                                        );

                                    continue;
                                };

                                if let Err(
                                    error
                                ) =
                                    roll_turn_die(
                                        &state,
                                        room_code,
                                        player_id,
                                        die_index,
                                    )
                                    .await
                                {
                                    let _ =
                                        outgoing_tx.send(
                                            error
                                        );
                                }
                            }

                            Ok(
                                ClientMessage::FluxPlayCard {
                                    card_id,
                                    chosen_color,
                                    target_player_id,
                                }
                            ) => {
                                let (
                                    Some(
                                        room_code
                                    ),
                                    Some(
                                        player_id
                                    ),
                                ) = (
                                    &current_room,
                                    current_player,
                                )
                                else {
                                    let _ =
                                        outgoing_tx.send(
                                            server_error(
                                                "You are not inside a room"
                                            )
                                        );

                                    continue;
                                };

                                if let Err(
                                    error
                                ) =
                                    flux_play_card(
                                        &state,
                                        room_code,
                                        player_id,
                                        card_id,
                                        chosen_color,
                                        target_player_id,
                                    )
                                    .await
                                {
                                    let _ =
                                        outgoing_tx.send(
                                            error
                                        );
                                }
                            }

                            Ok(
                                ClientMessage::FluxBeginSteal {
                                    card_id,
                                    target_player_id,
                                }
                            ) => {
                                let (
                                    Some(
                                        room_code
                                    ),
                                    Some(
                                        player_id
                                    ),
                                ) = (
                                    &current_room,
                                    current_player,
                                )
                                else {
                                    let _ =
                                        outgoing_tx.send(
                                            server_error(
                                                "You are not inside a room"
                                            )
                                        );

                                    continue;
                                };

                                if let Err(
                                    error
                                ) =
                                    flux_begin_steal(
                                        &state,
                                        room_code,
                                        player_id,
                                        card_id,
                                        target_player_id,
                                    )
                                    .await
                                {
                                    let _ =
                                        outgoing_tx.send(
                                            error
                                        );
                                }
                            }

                            Ok(
                                ClientMessage::FluxChooseSteal {
                                    offer_id,
                                    card_id,
                                }
                            ) => {
                                let (
                                    Some(
                                        room_code
                                    ),
                                    Some(
                                        player_id
                                    ),
                                ) = (
                                    &current_room,
                                    current_player,
                                )
                                else {
                                    let _ =
                                        outgoing_tx.send(
                                            server_error(
                                                "You are not inside a room"
                                            )
                                        );

                                    continue;
                                };

                                if let Err(
                                    error
                                ) =
                                    flux_choose_steal(
                                        &state,
                                        room_code,
                                        player_id,
                                        offer_id,
                                        card_id,
                                    )
                                    .await
                                {
                                    let _ =
                                        outgoing_tx.send(
                                            error
                                        );
                                }
                            }

                            Ok(
                                ClientMessage::FluxClash
                            ) => {
                                let (
                                    Some(
                                        room_code
                                    ),
                                    Some(
                                        player_id
                                    ),
                                ) = (
                                    &current_room,
                                    current_player,
                                )
                                else {
                                    continue;
                                };

                                if let Err(
                                    error
                                ) =
                                    flux_clash(
                                        &state,
                                        room_code,
                                        player_id,
                                    )
                                    .await
                                {
                                    let _ =
                                        outgoing_tx.send(
                                            error
                                        );
                                }
                            }

                            Ok(
                                ClientMessage::FluxRoundReady {
                                    ready,
                                }
                            ) => {
                                let (
                                    Some(
                                        room_code
                                    ),
                                    Some(
                                        player_id
                                    ),
                                ) = (
                                    &current_room,
                                    current_player,
                                )
                                else {
                                    continue;
                                };

                                if let Err(
                                    error
                                ) =
                                    flux_set_round_ready(
                                        &state,
                                        room_code,
                                        player_id,
                                        ready,
                                    )
                                    .await
                                {
                                    let _ =
                                        outgoing_tx.send(
                                            error
                                        );
                                }
                            }

                            Ok(
                                ClientMessage::FluxNextRound
                            ) => {
                                let (
                                    Some(
                                        room_code
                                    ),
                                    Some(
                                        player_id
                                    ),
                                ) = (
                                    &current_room,
                                    current_player,
                                )
                                else {
                                    continue;
                                };

                                if let Err(
                                    error
                                ) =
                                    flux_next_round(
                                        &state,
                                        room_code,
                                        player_id,
                                        false,
                                    )
                                    .await
                                {
                                    let _ =
                                        outgoing_tx.send(
                                            error
                                        );
                                }
                            }

                            Ok(
                                ClientMessage::FluxPlayAgain
                            ) => {
                                let (
                                    Some(
                                        room_code
                                    ),
                                    Some(
                                        player_id
                                    ),
                                ) = (
                                    &current_room,
                                    current_player,
                                )
                                else {
                                    continue;
                                };

                                if let Err(
                                    error
                                ) =
                                    flux_next_round(
                                        &state,
                                        room_code,
                                        player_id,
                                        true,
                                    )
                                    .await
                                {
                                    let _ =
                                        outgoing_tx.send(
                                            error
                                        );
                                }
                            }

                            Ok(
                                ClientMessage::FluxDraw
                            ) => {
                                let (
                                    Some(
                                        room_code
                                    ),
                                    Some(
                                        player_id
                                    ),
                                ) = (
                                    &current_room,
                                    current_player,
                                )
                                else {
                                    let _ =
                                        outgoing_tx.send(
                                            server_error(
                                                "You are not inside a room"
                                            )
                                        );

                                    continue;
                                };

                                if let Err(
                                    error
                                ) =
                                    flux_draw(
                                        &state,
                                        room_code,
                                        player_id,
                                    )
                                    .await
                                {
                                    let _ =
                                        outgoing_tx.send(
                                            error
                                        );
                                }
                            }

                            Err(
                                error
                            ) => {
                                warn!(
                                    "Invalid WebSocket message: {}",
                                    error
                                );

                                let _ =
                                    outgoing_tx.send(
                                        server_error(
                                            "Invalid message"
                                        )
                                    );
                            }
                        }
                    }

                    Some(
                        Ok(
                            Message::Close(_)
                        )
                    ) => {
                        break;
                    }

                    Some(
                        Ok(_)
                    ) => {}

                    Some(
                        Err(
                            error
                        )
                    ) => {
                        warn!(
                            "WebSocket error: {}",
                            error
                        );

                        break;
                    }

                    None => {
                        break;
                    }
                }
            }

            outgoing =
                outgoing_rx.recv()
            => {
                let Some(
                    message
                ) = outgoing
                else {
                    break;
                };

                let json =
                    match serde_json::to_string(
                        &message
                    ) {
                        Ok(
                            json
                        ) =>
                            json,

                        Err(
                            error
                        ) => {
                            warn!(
                                "Failed to serialize server message: {}",
                                error
                            );

                            continue;
                        }
                    };

                if socket
                    .send(
                        Message::Text(
                            json.into()
                        )
                    )
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

    info!("WebSocket client disconnected");
}

async fn create_room(
    state: &AppState,

    sender: ClientSender,

    player_name: String,

    room_name: String,

    max_players: u8,

    visibility: String,
) -> Result<(String, Uuid, ServerMessage), ServerMessage> {
    let player_name = validate_player_name(player_name)?;

    let room_name = validate_room_name(room_name)?;

    if !(1..=4).contains(&max_players) {
        return Err(server_error("Max players must be between 1 and 4"));
    }

    let visibility = match visibility.as_str() {
        "public" => RoomVisibility::Public,

        "private" => RoomVisibility::Private,

        _ => {
            return Err(server_error("Invalid room visibility"));
        }
    };

    let player_id = Uuid::new_v4();

    let room_id = Uuid::new_v4();

    let player = Player {
        id: player_id,

        name: player_name,

        is_host: true,

        ready: false,

        turn_dice: [DieState::default(); DICE_PER_PLAYER],

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
        id: room_id,

        name: room_name.clone(),

        host_id: player_id,

        max_players,

        visibility,

        players: vec![player],

        state: RoomState::Lobby,

        selected_game: None,

        flux_state: None,

        flux_scores: HashMap::new(),

        flux_order: Vec::new(),

        flux_round_ready: HashSet::new(),
    };

    let response = ServerMessage::RoomCreated {
        room_code: room_code.clone(),

        room_name,

        player_id,

        host_id: player_id,

        max_players,

        visibility: visibility.as_str().to_string(),

        state: RoomState::Lobby.as_str().to_string(),

        selected_game: room.selected_game,

        players: room_players_response(&room),
    };

    rooms.insert(room_code.clone(), room);

    info!("Room {} created", room_code);

    Ok((room_code, player_id, response))
}

async fn join_room_by_code(
    state: &AppState,

    sender: ClientSender,

    player_name: String,

    room_code: String,
) -> Result<(String, Uuid, ServerMessage), ServerMessage> {
    let room_code = room_code.trim().to_uppercase();

    join_room_internal(state, sender, player_name, room_code).await
}

async fn join_public_room(
    state: &AppState,

    sender: ClientSender,

    player_name: String,

    room_id: Uuid,
) -> Result<(String, Uuid, ServerMessage), ServerMessage> {
    let room_code = {
        let rooms = state.rooms.read().await;

        rooms
            .iter()
            .find_map(|(room_code, room)| {
                if room.id == room_id {
                    Some(room_code.clone())
                } else {
                    None
                }
            })
            .ok_or_else(|| server_error("Room not found"))?
    };

    {
        let rooms = state.rooms.read().await;

        let room = rooms
            .get(&room_code)
            .ok_or_else(|| server_error("Room not found"))?;

        if !matches!(room.visibility, RoomVisibility::Public) {
            return Err(server_error("This room requires a code"));
        }
    }

    join_room_internal(state, sender, player_name, room_code).await
}

async fn join_room_internal(
    state: &AppState,

    sender: ClientSender,

    player_name: String,

    room_code: String,
) -> Result<(String, Uuid, ServerMessage), ServerMessage> {
    let player_name = validate_player_name(player_name)?;

    if room_code.len() != ROOM_CODE_LENGTH {
        return Err(server_error("Invalid room code"));
    }

    let mut rooms = state.rooms.write().await;

    let room = rooms
        .get_mut(&room_code)
        .ok_or_else(|| server_error("Room not found"))?;

    if !matches!(room.state, RoomState::Lobby) {
        return Err(server_error("This game has already started"));
    }

    if room.players.len() >= usize::from(room.max_players) {
        return Err(server_error("Room is full"));
    }

    if room
        .players
        .iter()
        .any(|player| player.name.eq_ignore_ascii_case(&player_name))
    {
        return Err(server_error(
            "That nickname is already being used in this room",
        ));
    }

    let player_id = Uuid::new_v4();

    room.players.push(Player {
        id: player_id,

        name: player_name,

        is_host: false,

        ready: false,

        turn_dice: [DieState::default(); DICE_PER_PLAYER],

        sender,
    });

    let response = ServerMessage::RoomJoined {
        room_code: room_code.clone(),

        room_name: room.name.clone(),

        player_id,

        host_id: room.host_id,

        max_players: room.max_players,

        visibility: room.visibility.as_str().to_string(),

        state: room.state.as_str().to_string(),

        selected_game: room.selected_game,

        players: room_players_response(room),
    };

    Ok((room_code, player_id, response))
}

async fn list_rooms(state: &AppState, requested_page: usize, search: String) -> ServerMessage {
    let rooms = state.rooms.read().await;

    let search = search.trim().to_lowercase();

    let mut summaries = rooms
        .values()
        .filter(|room| matches!(room.state, RoomState::Lobby))
        .filter(|room| search.is_empty() || room.name.to_lowercase().contains(&search))
        .map(|room| {
            let host_name = room
                .players
                .iter()
                .find(|player| player.id == room.host_id)
                .map(|player| player.name.clone())
                .unwrap_or_else(|| "Unknown".to_string());

            RoomSummary {
                room_id: room.id,

                room_name: room.name.clone(),

                host_name,

                visibility: room.visibility.as_str().to_string(),

                player_count: room.players.len(),

                max_players: room.max_players,

                joinable: room.players.len() < usize::from(room.max_players),
            }
        })
        .collect::<Vec<_>>();

    summaries.sort_by(|a, b| a.room_name.to_lowercase().cmp(&b.room_name.to_lowercase()));

    let total_rooms = summaries.len();

    let total_pages = if total_rooms == 0 {
        1
    } else {
        (total_rooms + ROOMS_PER_PAGE - 1) / ROOMS_PER_PAGE
    };

    let page = requested_page.max(1).min(total_pages);

    let start = (page - 1) * ROOMS_PER_PAGE;

    let rooms = summaries
        .into_iter()
        .skip(start)
        .take(ROOMS_PER_PAGE)
        .collect::<Vec<_>>();

    ServerMessage::RoomDirectory {
        rooms,
        page,
        total_pages,
        total_rooms,
    }
}

async fn select_game(
    state: &AppState,

    room_code: &str,

    player_id: Uuid,

    game_id: GameId,
) -> Result<(), ServerMessage> {
    if !game_id.is_available() {
        return Err(server_error(
            "That game is not available in this Alpha build",
        ));
    }

    {
        let mut rooms = state.rooms.write().await;

        let room = rooms
            .get_mut(room_code)
            .ok_or_else(|| server_error("Room not found"))?;

        if !matches!(room.state, RoomState::Lobby) {
            return Err(server_error(
                "Game selection is only available in the lobby",
            ));
        }

        if room.host_id != player_id {
            return Err(server_error("Only the host can select the game"));
        }

        room.selected_game = Some(game_id);

        for player in &mut room.players {
            player.ready = false;
        }
    }

    broadcast_room_state(state, room_code).await;

    Ok(())
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
            .ok_or_else(|| server_error("Room not found"))?;

        if !matches!(room.state, RoomState::Lobby) {
            return Err(server_error(
                "Ready state cannot be changed after the game starts",
            ));
        }

        if room.selected_game.is_none() {
            return Err(server_error("Select a game before getting ready"));
        }

        let player = room
            .players
            .iter_mut()
            .find(|player| player.id == player_id)
            .ok_or_else(|| server_error("Player not found"))?;

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
    let messages = {
        let mut rooms = state.rooms.write().await;

        let room = rooms
            .get_mut(room_code)
            .ok_or_else(|| server_error("Room not found"))?;

        if room.host_id != player_id {
            return Err(server_error("Only the host can start the game"));
        }

        if !matches!(room.state, RoomState::Lobby) {
            return Err(server_error("The game has already started"));
        }

        let selected_game = room
            .selected_game
            .ok_or_else(|| server_error("Select a game before starting"))?;

        if !selected_game.is_available() {
            return Err(server_error("The selected game is not available yet"));
        }

        if room.players.len() < selected_game.min_players() {
            return Err(server_error("Not enough players for the selected game"));
        }

        if !room.players.iter().all(|player| player.ready) {
            return Err(server_error("All players must be ready"));
        }

        room.state = RoomState::TurnOrder;

        room.flux_state = None;

        for player in &mut room.players {
            player.turn_dice = [DieState::default(); DICE_PER_PLAYER];
        }

        room.players
            .iter()
            .map(|viewer| {
                (
                    viewer.sender.clone(),
                    ServerMessage::TurnOrderStarted {
                        room_code: room_code.to_string(),

                        players: turn_order_players_response_for(room, viewer.id),
                    },
                )
            })
            .collect::<Vec<_>>()
    };

    for (sender, message) in messages {
        let _ = sender.send(message);
    }

    info!("Turn-order arena started in room {}", room_code);

    Ok(())
}

async fn roll_turn_die(
    state: &AppState,

    room_code: &str,

    player_id: Uuid,

    die_index: usize,
) -> Result<(), ServerMessage> {
    if die_index >= DICE_PER_PLAYER {
        return Err(server_error("Invalid die index"));
    }

    let (frames, senders) = {
        let mut rooms = state.rooms.write().await;

        let room = rooms
            .get_mut(room_code)
            .ok_or_else(|| server_error("Room not found"))?;

        if !matches!(room.state, RoomState::TurnOrder) {
            return Err(server_error("Turn-order phase is not active"));
        }

        let requesting_player = room
            .players
            .iter()
            .find(|player| player.id == player_id)
            .ok_or_else(|| server_error("Player not found"))?;

        if requesting_player.turn_dice[die_index].value.is_some() {
            return Err(server_error("This die has already been rolled"));
        }

        let existing_dice = room
            .players
            .iter()
            .flat_map(|player| {
                player
                    .turn_dice
                    .iter()
                    .enumerate()
                    .filter_map(move |(index, die)| {
                        if die.value.is_some() {
                            Some(PhysicsDieInput {
                                owner_id: player.id,

                                die_index: index,

                                state: *die,
                            })
                        } else {
                            None
                        }
                    })
            })
            .collect::<Vec<_>>();

        let simulation = simulate_roll(&existing_dice, player_id, die_index);

        for simulated_die in &simulation.dice {
            if let Some(player) = room
                .players
                .iter_mut()
                .find(|player| player.id == simulated_die.owner_id)
            {
                player.turn_dice[simulated_die.die_index] = simulated_die.state;
            }
        }

        let senders = room
            .players
            .iter()
            .map(|player| player.sender.clone())
            .collect::<Vec<_>>();

        (simulation.frames, senders)
    };

    let message = ServerMessage::TurnOrderPhysics { frames };

    for sender in &senders {
        let _ = sender.send(message.clone());
    }

    tokio::time::sleep(Duration::from_millis(PHYSICS_REVEAL_DELAY_MS)).await;

    broadcast_turn_order_state(state, room_code).await;

    finish_turn_order_if_ready(state, room_code).await;

    Ok(())
}

async fn broadcast_turn_order_state(state: &AppState, room_code: &str) {
    let messages = {
        let rooms = state.rooms.read().await;

        let Some(room) = rooms.get(room_code) else {
            return;
        };

        let all_complete = room
            .players
            .iter()
            .all(|player| player.turn_dice.iter().all(|die| die.value.is_some()));

        room.players
            .iter()
            .map(|viewer| {
                (
                    viewer.sender.clone(),
                    ServerMessage::TurnOrderState {
                        players: turn_order_players_response_for(room, viewer.id),

                        all_complete,
                    },
                )
            })
            .collect::<Vec<_>>()
    };

    for (sender, message) in messages {
        let _ = sender.send(message);
    }
}

async fn finish_turn_order_if_ready(state: &AppState, room_code: &str) {
    let result = {
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
            .all(|player| player.turn_dice.iter().all(|die| die.value.is_some()));

        if !all_complete {
            return;
        }

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

        room.state = RoomState::TurnOrderResult;

        let selected_game = room.selected_game;

        if matches!(selected_game, Some(GameId::Flux)) {
            let ordered_ids = totals
                .iter()
                .map(|(player_id, _, _)| *player_id)
                .collect::<Vec<_>>();

            room.flux_order = ordered_ids.clone();

            room.flux_scores = ordered_ids
                .iter()
                .copied()
                .map(|player_id| (player_id, 0u16))
                .collect();

            room.flux_round_ready.clear();

            room.flux_state = Some(create_flux_state(&ordered_ids, 1));

            room.state = RoomState::Playing;
        }

        let turn_order_senders = room
            .players
            .iter()
            .map(|player| player.sender.clone())
            .collect::<Vec<_>>();

        Some((turn_order_senders, ranking, selected_game))
    };

    let Some((senders, ranking, selected_game)) = result else {
        return;
    };

    let ranking_message = ServerMessage::TurnOrderFinished { ranking };

    for sender in senders {
        let _ = sender.send(ranking_message.clone());
    }

    if matches!(selected_game, Some(GameId::Flux)) {
        send_flux_state_to_room(state, room_code).await;
    }
}

fn create_flux_state(ordered_player_ids: &[Uuid], round: u16) -> FluxState {
    let mut deck = build_flux_deck();

    shuffle_flux_deck(&mut deck);

    let mut players = ordered_player_ids
        .iter()
        .copied()
        .map(|player_id| FluxPlayerState {
            player_id,

            hand: Vec::new(),
        })
        .collect::<Vec<_>>();

    for _ in 0..STARTING_HAND_SIZE {
        for player in &mut players {
            if let Some(card) = deck.pop() {
                player.hand.push(card);
            }
        }
    }

    let mut discard_pile = Vec::new();

    let first_discard_index = deck
        .iter()
        .rposition(|card| matches!(card.kind, FluxCardKind::Number { .. }));

    let first_card = if let Some(index) = first_discard_index {
        Some(deck.remove(index))
    } else {
        deck.pop()
    };

    let active_color = first_card
        .as_ref()
        .and_then(|card| card.color.normal_color())
        .unwrap_or(ClashColor::Yellow);

    let last_played_card = first_card.clone();

    if let Some(card) = first_card {
        discard_pile.push(card);
    }

    FluxState {
        players,

        draw_pile: deck,

        discard_pile,

        last_played_card,

        revision: 1,

        current_player_index: 0,

        direction: 1,

        active_color,

        pending_draw: 0,

        pending_draw_kind: None,

        pending_steal: None,

        pending_clash: None,

        round,
    }
}

fn build_flux_deck() -> Vec<FluxCard> {
    let colors = [
        ClashCardColor::Red,
        ClashCardColor::Blue,
        ClashCardColor::Green,
        ClashCardColor::Yellow,
    ];

    let mut deck = Vec::new();

    for color in colors {
        for value in 0..=9 {
            let copies = if value == 0 { 1 } else { 2 };

            for _ in 0..copies {
                deck.push(FluxCard {
                    id: Uuid::new_v4(),

                    color,

                    kind: FluxCardKind::Number { value },
                });
            }
        }

        for _ in 0..2 {
            deck.push(FluxCard {
                id: Uuid::new_v4(),

                color,

                kind: FluxCardKind::Special {
                    special: FluxSpecial::DrawTwo,
                },
            });

            deck.push(FluxCard {
                id: Uuid::new_v4(),

                color,

                kind: FluxCardKind::Special {
                    special: FluxSpecial::Reverse,
                },
            });

            deck.push(FluxCard {
                id: Uuid::new_v4(),

                color,

                kind: FluxCardKind::Special {
                    special: FluxSpecial::Steal,
                },
            });
        }
    }

    for _ in 0..4 {
        deck.push(FluxCard {
            id: Uuid::new_v4(),

            color: ClashCardColor::Wild,

            kind: FluxCardKind::Special {
                special: FluxSpecial::Wild,
            },
        });

        deck.push(FluxCard {
            id: Uuid::new_v4(),

            color: ClashCardColor::Wild,

            kind: FluxCardKind::Special {
                special: FluxSpecial::DrawFour,
            },
        });
    }

    for _ in 0..2 {
        deck.push(FluxCard {
            id: Uuid::new_v4(),

            color: ClashCardColor::Wild,

            kind: FluxCardKind::Special {
                special: FluxSpecial::DrawSix,
            },
        });
    }

    deck
}

fn shuffle_flux_deck(deck: &mut [FluxCard]) {
    if deck.len() < 2 {
        return;
    }

    for index in (1..deck.len()).rev() {
        let swap_index = rand::random_range(0..=index);

        deck.swap(index, swap_index);
    }
}

async fn send_flux_state_to_room(state: &AppState, room_code: &str) {
    let messages = {
        let rooms = state.rooms.read().await;

        let Some(room) = rooms.get(room_code) else {
            return;
        };

        room.players
            .iter()
            .filter_map(|viewer| {
                build_flux_message_for(room, viewer.id)
                    .map(|message| (viewer.sender.clone(), message))
            })
            .collect::<Vec<_>>()
    };

    for (sender, message) in messages {
        let _ = sender.send(message);
    }
}

fn build_flux_message_for(room: &Room, viewer_id: Uuid) -> Option<ServerMessage> {
    let flux = room.flux_state.as_ref()?;

    let current_player_id = flux.current_player_id()?;

    let viewer = flux
        .players
        .iter()
        .find(|player| player.player_id == viewer_id)?;

    let public_players = flux
        .players
        .iter()
        .map(|flux_player| {
            let name = room
                .players
                .iter()
                .find(|room_player| room_player.id == flux_player.player_id)
                .map(|room_player| room_player.name.clone())
                .unwrap_or_else(|| "Player".to_string());

            FluxPublicPlayer {
                id: flux_player.player_id,

                name,

                card_count: flux_player.hand.len(),

                active: flux_player.player_id == current_player_id,
            }
        })
        .collect::<Vec<_>>();

    let playable_card_ids = if current_player_id == viewer_id {
        viewer
            .hand
            .iter()
            .filter(|card| can_play_card(flux, card))
            .map(|card| card.id)
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };

    Some(ServerMessage::FluxState {
        state: FluxStateResponse {
            revision: flux.revision,

            current_player_id,

            direction: flux.direction,

            active_color: flux.active_color,

            discard_top: flux.last_played_card.clone(),

            draw_pile_count: flux.draw_pile.len(),

            pending_draw: flux.pending_draw,

            pending_draw_kind: flux.pending_draw_kind.map(Into::into),

            clash_target_id: flux
                .pending_clash
                .as_ref()
                .map(|clash| clash.target_player_id),

            clash_target_name: flux.pending_clash.as_ref().and_then(|clash| {
                room.players
                    .iter()
                    .find(|player| player.id == clash.target_player_id)
                    .map(|player| player.name.clone())
            }),

            clash_serial: flux.pending_clash.as_ref().map(|clash| clash.serial),

            round: flux.round,

            scores: flux_score_response(room),

            players: public_players,

            private_state: FluxPrivateState {
                hand: viewer.hand.clone(),
            },
        },

        playable_card_ids,
    })
}

async fn flux_play_card(
    state: &AppState,

    room_code: &str,

    player_id: Uuid,

    card_id: Uuid,

    chosen_color: Option<ClashColor>,

    target_player_id: Option<Uuid>,
) -> Result<(), ServerMessage> {
    let winner = {
        let mut rooms = state.rooms.write().await;

        let room = rooms
            .get_mut(room_code)
            .ok_or_else(|| server_error("Room not found"))?;

        if !matches!(room.state, RoomState::Playing) {
            return Err(server_error("The match is not active"));
        }

        if !matches!(room.selected_game, Some(GameId::Flux)) {
            return Err(server_error("Color Clash is not the selected game"));
        }

        let flux = room
            .flux_state
            .as_mut()
            .ok_or_else(|| server_error("Color Clash state is unavailable"))?;

        let pending_card = flux
            .players
            .iter()
            .find(|player| player.player_id == player_id)
            .and_then(|player| player.hand.iter().find(|card| card.id == card_id))
            .cloned()
            .ok_or_else(|| server_error("Card not found"))?;

        if matches!(
            pending_card.kind,
            FluxCardKind::Special {
                special: FluxSpecial::Steal,
            }
        ) {
            return Err(server_error("Use the steal target picker for a STEAL card"));
        }

        let card = flux_play_card_action(flux, player_id, card_id)
            .map_err(|_| server_error("That card cannot be played"))?;

        apply_flux_card_effect(flux, player_id, &card, chosen_color, target_player_id)?;

        flux.last_played_card = Some(card.clone());

        flux.bump_revision();

        let played_revision = flux.revision;

        let impact = flux_impact_for_card(&card);

        let reverse_repeats_turn = matches!(
            card.kind,
            FluxCardKind::Special {
                special: FluxSpecial::Reverse,
            }
        ) && flux.players.len() == 2;

        let winner = flux
            .players
            .iter()
            .find(|player| player.hand.is_empty())
            .map(|player| player.player_id);

        let result = if let Some(winner_id) = winner {
            Some(winner_id)
        } else {
            refresh_clash_window(flux, player_id);

            if !reverse_repeats_turn {
                advance_flux_turn(flux);
            }

            None
        };

        let _ = flux;

        broadcast_flux_card_played(room, &card, played_revision);

        if let Some(text) = impact {
            broadcast_flux_impact(room, text);
        }

        result
    };

    send_flux_state_to_room(state, room_code).await;

    if let Some(winner_id) = winner {
        finish_flux_round(state, room_code, winner_id).await;
    }

    Ok(())
}

async fn flux_begin_steal(
    state: &AppState,

    room_code: &str,

    player_id: Uuid,

    card_id: Uuid,

    target_player_id: Uuid,
) -> Result<(), ServerMessage> {
    let (sender, offer_message) = {
        let mut rooms = state.rooms.write().await;

        let room = rooms
            .get_mut(room_code)
            .ok_or_else(|| server_error("Room not found"))?;

        if !matches!(room.state, RoomState::Playing) {
            return Err(server_error("The match is not active"));
        }

        let flux = room
            .flux_state
            .as_mut()
            .ok_or_else(|| server_error("Flux Clash state is unavailable"))?;

        if flux.pending_steal.is_some() {
            return Err(server_error("Finish the active STEAL selection first"));
        }

        if flux.current_player_id() != Some(player_id) {
            return Err(server_error("It is not your turn"));
        }

        if flux.pending_draw > 0 {
            return Err(server_error(
                "STEAL cannot be used while a +2 or +4 stack is pending",
            ));
        }

        if target_player_id == player_id {
            return Err(server_error("You cannot steal from yourself"));
        }

        let attacker = flux
            .players
            .iter()
            .find(|player| player.player_id == player_id)
            .ok_or_else(|| server_error("Player not found"))?;

        let steal_card = attacker
            .hand
            .iter()
            .find(|card| card.id == card_id)
            .cloned()
            .ok_or_else(|| server_error("STEAL card not found"))?;

        if !matches!(
            steal_card.kind,
            FluxCardKind::Special {
                special: FluxSpecial::Steal,
            }
        ) {
            return Err(server_error("That is not a STEAL card"));
        }

        if !can_play_card(flux, &steal_card) {
            return Err(server_error("STEAL cannot be played now"));
        }

        let target = flux
            .players
            .iter()
            .find(|player| player.player_id == target_player_id)
            .ok_or_else(|| server_error("Target player not found"))?;

        if target.hand.is_empty() {
            return Err(server_error("That player has no cards to steal"));
        }

        if target.hand.len() <= 1 {
            return Err(server_error(
                "You cannot steal from a player who only has 1 card left",
            ));
        }

        let mut indices = (0..target.hand.len()).collect::<Vec<_>>();

        for index in (1..indices.len()).rev() {
            let swap_index = rand::random_range(0..=index);

            indices.swap(index, swap_index);
        }

        let option_indices = indices.into_iter().take(2).collect::<Vec<_>>();

        let options = option_indices
            .iter()
            .map(|index| target.hand[*index].clone())
            .collect::<Vec<_>>();

        let option_card_ids = options.iter().map(|card| card.id).collect::<Vec<_>>();

        let offer_id = Uuid::new_v4();

        let target_name = room
            .players
            .iter()
            .find(|player| player.id == target_player_id)
            .map(|player| player.name.clone())
            .unwrap_or_else(|| "Player".to_string());

        let sender = room
            .players
            .iter()
            .find(|player| player.id == player_id)
            .map(|player| player.sender.clone())
            .ok_or_else(|| server_error("Player connection not found"))?;

        flux.pending_steal = Some(PendingStealOffer {
            offer_id,

            attacker_id: player_id,

            target_player_id,

            steal_card_id: card_id,

            option_card_ids,
        });

        (
            sender,
            ServerMessage::FluxStealOffer {
                offer_id,

                target_player_id,

                target_player_name: target_name,

                options,
            },
        )
    };

    let _ = sender.send(offer_message);

    Ok(())
}

async fn flux_choose_steal(
    state: &AppState,

    room_code: &str,

    player_id: Uuid,

    offer_id: Uuid,

    chosen_card_id: Uuid,
) -> Result<(), ServerMessage> {
    {
        let mut rooms = state.rooms.write().await;

        let room = rooms
            .get_mut(room_code)
            .ok_or_else(|| server_error("Room not found"))?;

        let flux = room
            .flux_state
            .as_mut()
            .ok_or_else(|| server_error("Flux Clash state is unavailable"))?;

        let offer = flux
            .pending_steal
            .clone()
            .ok_or_else(|| server_error("No steal selection is active"))?;

        if offer.offer_id != offer_id || offer.attacker_id != player_id {
            return Err(server_error("Invalid steal offer"));
        }

        if !offer.option_card_ids.contains(&chosen_card_id) {
            return Err(server_error(
                "You can only choose one of the 2 random cards offered",
            ));
        }

        let attacker_index = flux
            .players
            .iter()
            .position(|player| player.player_id == player_id)
            .ok_or_else(|| server_error("Player not found"))?;

        let target_index = flux
            .players
            .iter()
            .position(|player| player.player_id == offer.target_player_id)
            .ok_or_else(|| server_error("Target player not found"))?;

        let steal_card_index = flux.players[attacker_index]
            .hand
            .iter()
            .position(|card| card.id == offer.steal_card_id)
            .ok_or_else(|| server_error("STEAL card is no longer available"))?;

        let steal_card = flux.players[attacker_index].hand.remove(steal_card_index);

        flux.discard_pile.push(steal_card.clone());

        flux.last_played_card = Some(steal_card);

        let stolen_index = flux.players[target_index]
            .hand
            .iter()
            .position(|card| card.id == chosen_card_id)
            .ok_or_else(|| server_error("The selected random card is no longer available"))?;

        let stolen_card = flux.players[target_index].hand.remove(stolen_index);

        flux.players[attacker_index].hand.push(stolen_card);

        flux.pending_steal = None;

        refresh_clash_window(flux, player_id);

        advance_flux_turn(flux);

        flux.bump_revision();

        let _ = flux;

        broadcast_flux_impact(room, "SNATCH!");
    }

    send_flux_state_to_room(state, room_code).await;

    Ok(())
}

async fn flux_draw(
    state: &AppState,

    room_code: &str,

    player_id: Uuid,
) -> Result<(), ServerMessage> {
    {
        let mut rooms = state.rooms.write().await;

        let room = rooms
            .get_mut(room_code)
            .ok_or_else(|| server_error("Room not found"))?;

        if !matches!(room.state, RoomState::Playing) {
            return Err(server_error("The match is not active"));
        }

        let flux = room
            .flux_state
            .as_mut()
            .ok_or_else(|| server_error("Color Clash state is unavailable"))?;

        if flux.current_player_id() != Some(player_id) {
            return Err(server_error("It is not your turn"));
        }

        let draw_amount = if flux.pending_draw > 0 {
            flux.pending_draw as usize
        } else {
            1
        };

        draw_cards_for_player(flux, player_id, draw_amount)?;

        flux.pending_draw = 0;

        flux.pending_draw_kind = None;

        advance_flux_turn(flux);

        flux.bump_revision();
    }

    send_flux_state_to_room(state, room_code).await;

    Ok(())
}

fn apply_flux_card_effect(
    flux: &mut FluxState,

    player_id: Uuid,

    card: &FluxCard,

    chosen_color: Option<ClashColor>,

    target_player_id: Option<Uuid>,
) -> Result<(), ServerMessage> {
    if let Some(normal_color) = card.color.normal_color() {
        flux.active_color = normal_color;
    }

    match card.kind {
        FluxCardKind::Number { .. } => {}

        FluxCardKind::Special { special } => match special {
            FluxSpecial::Reverse => {
                flux.direction *= -1;
            }

            FluxSpecial::DrawTwo => match flux.pending_draw_kind {
                None | Some(PendingDrawKind::DrawTwo) => {
                    flux.pending_draw = flux.pending_draw.saturating_add(2);

                    flux.pending_draw_kind = Some(PendingDrawKind::DrawTwo);
                }

                Some(PendingDrawKind::DrawFour) => {
                    return Err(server_error("Only another +4 can answer a +4 stack"));
                }
            },

            FluxSpecial::Wild => {
                flux.active_color = chosen_color.ok_or_else(|| server_error("Choose a color"))?;
            }

            FluxSpecial::DrawFour => {
                flux.active_color =
                    chosen_color.ok_or_else(|| server_error("Choose a color for +4"))?;

                match flux.pending_draw_kind {
                    None | Some(PendingDrawKind::DrawFour) => {
                        flux.pending_draw = flux.pending_draw.saturating_add(4);

                        flux.pending_draw_kind = Some(PendingDrawKind::DrawFour);
                    }

                    Some(PendingDrawKind::DrawTwo) => {
                        return Err(server_error("A +4 cannot be used to answer a +2 stack"));
                    }
                }
            }

            FluxSpecial::Steal => {
                return Err(server_error(
                    "STEAL must be resolved through its two-card choice",
                ));
            }

            FluxSpecial::DrawSix => {
                if flux.pending_draw > 0 {
                    return Err(server_error("+6 cannot be played during a +2 or +4 stack"));
                }

                let target =
                    target_player_id.ok_or_else(|| server_error("Choose a target for +6"))?;

                if target == player_id {
                    return Err(server_error("You cannot target yourself with +6"));
                }

                if !flux.players.iter().any(|player| player.player_id == target) {
                    return Err(server_error("Invalid +6 target"));
                }

                draw_cards_for_player(flux, target, 6)?;

                draw_cards_for_player(flux, player_id, 2)?;
            }
        },
    }

    Ok(())
}

fn draw_cards_for_player(
    flux: &mut FluxState,

    player_id: Uuid,

    amount: usize,
) -> Result<(), ServerMessage> {
    for _ in 0..amount {
        refill_flux_draw_pile(flux);

        let card = flux
            .draw_pile
            .pop()
            .ok_or_else(|| server_error("No cards are available to draw"))?;

        let player = flux
            .players
            .iter_mut()
            .find(|player| player.player_id == player_id)
            .ok_or_else(|| server_error("Player not found"))?;

        player.hand.push(card);
    }

    Ok(())
}

fn advance_flux_turn(flux: &mut FluxState) {
    let count = flux.players.len();

    if count == 0 {
        return;
    }

    if flux.direction >= 0 {
        flux.current_player_index = (flux.current_player_index + 1) % count;
    } else if flux.current_player_index == 0 {
        flux.current_player_index = count - 1;
    } else {
        flux.current_player_index -= 1;
    }
}

fn refill_flux_draw_pile(flux: &mut FluxState) {
    if !flux.draw_pile.is_empty() {
        return;
    }

    if flux.discard_pile.len() <= 1 {
        return;
    }

    let top = flux.discard_pile.pop();

    let mut recycled = std::mem::take(&mut flux.discard_pile);

    shuffle_flux_deck(&mut recycled);

    flux.draw_pile = recycled;

    if let Some(top_card) = top {
        flux.discard_pile.push(top_card);
    }
}

fn broadcast_flux_card_played(room: &Room, card: &FluxCard, revision: u64) {
    let message = ServerMessage::FluxCardPlayed {
        card: card.clone(),

        revision,
    };

    for player in &room.players {
        let _ = player.sender.send(message.clone());
    }
}

fn flux_impact_for_card(card: &FluxCard) -> Option<&'static str> {
    match card.kind {
        FluxCardKind::Special {
            special: FluxSpecial::DrawTwo,
        } => Some("BOOM!"),

        FluxCardKind::Special {
            special: FluxSpecial::DrawFour,
        } => Some("KABOOM!"),

        FluxCardKind::Special {
            special: FluxSpecial::DrawSix,
        } => Some("CATAPLUM!"),

        FluxCardKind::Special {
            special: FluxSpecial::Reverse,
        } => Some("SWAP!"),

        _ => None,
    }
}

fn broadcast_flux_impact(room: &Room, text: &str) {
    let message = ServerMessage::FluxImpact {
        text: text.to_string(),
    };

    for player in &room.players {
        let _ = player.sender.send(message.clone());
    }
}

fn refresh_clash_window(flux: &mut FluxState, player_id: Uuid) {
    let hand_count = flux
        .players
        .iter()
        .find(|player| player.player_id == player_id)
        .map(|player| player.hand.len())
        .unwrap_or(0);

    if hand_count == 1 {
        flux.pending_clash = Some(PendingClash {
            target_player_id: player_id,

            serial: Uuid::new_v4(),

            opened_at: Instant::now(),
        });
    } else if flux
        .pending_clash
        .as_ref()
        .is_some_and(|clash| clash.target_player_id == player_id)
    {
        flux.pending_clash = None;
    }
}

fn card_points(card: &FluxCard) -> u16 {
    match card.kind {
        FluxCardKind::Number { value } => u16::from(value),

        FluxCardKind::Special {
            special: FluxSpecial::Reverse | FluxSpecial::Steal | FluxSpecial::DrawTwo,
        } => 20,

        FluxCardKind::Special {
            special: FluxSpecial::Wild | FluxSpecial::DrawFour,
        } => 40,

        FluxCardKind::Special {
            special: FluxSpecial::DrawSix,
        } => 50,
    }
}

fn round_points(flux: &FluxState, winner_id: Uuid) -> u16 {
    let raw = flux
        .players
        .iter()
        .filter(|player| player.player_id != winner_id)
        .flat_map(|player| player.hand.iter())
        .map(card_points)
        .sum::<u16>();

    raw.clamp(20, 50)
}

fn flux_score_response(room: &Room) -> Vec<FluxScoreResponse> {
    let mut values = room
        .players
        .iter()
        .map(|player| {
            (
                player.id,
                player.name.clone(),
                *room.flux_scores.get(&player.id).unwrap_or(&0),
            )
        })
        .collect::<Vec<_>>();

    values.sort_by(|left, right| right.2.cmp(&left.2));

    values
        .into_iter()
        .enumerate()
        .map(|(index, (player_id, name, score))| FluxScoreResponse {
            player_id,
            name,
            score,
            position: index + 1,
        })
        .collect()
}

async fn finish_flux_round(state: &AppState, room_code: &str, winner_id: Uuid) {
    let result = {
        let mut rooms = state.rooms.write().await;

        let Some(room) = rooms.get_mut(room_code) else {
            return;
        };

        let Some(flux) = room.flux_state.as_ref() else {
            return;
        };

        let awarded = round_points(flux, winner_id);

        let score = room.flux_scores.entry(winner_id).or_insert(0);

        *score = (*score + awarded).min(100);

        let match_over = *score >= 100;

        room.state = if match_over {
            RoomState::MatchResult
        } else {
            RoomState::Playing
        };

        let winner_name = room
            .players
            .iter()
            .find(|player| player.id == winner_id)
            .map(|player| player.name.clone())
            .unwrap_or_else(|| "Player".to_string());

        let round = flux.round;

        let scores = flux_score_response(room);

        room.flux_round_ready.clear();

        let message = ServerMessage::FluxRoundFinished {
            winner_id,
            winner_name,
            points_awarded: awarded,
            round,
            scores,
            match_over,
        };

        let senders = room
            .players
            .iter()
            .map(|player| player.sender.clone())
            .collect::<Vec<_>>();

        (message, senders)
    };

    let (message, senders) = result;

    for sender in senders {
        let _ = sender.send(message.clone());
    }

    broadcast_flux_round_ready(state, room_code).await;
}

async fn flux_clash(
    state: &AppState,

    room_code: &str,

    caller_id: Uuid,
) -> Result<(), ServerMessage> {
    {
        let mut rooms = state.rooms.write().await;

        let room = rooms
            .get_mut(room_code)
            .ok_or_else(|| server_error("Room not found"))?;

        let flux = room
            .flux_state
            .as_mut()
            .ok_or_else(|| server_error("Flux Clash state unavailable"))?;

        let clash = flux
            .pending_clash
            .clone()
            .ok_or_else(|| server_error("No Clash window is active"))?;

        if caller_id == clash.target_player_id {
            flux.pending_clash = None;

            broadcast_flux_impact(room, "CLASH!!");
        } else {
            if clash.opened_at.elapsed() < Duration::from_millis(700) {
                return Err(server_error("Clash is not ready for rivals yet"));
            }

            draw_cards_for_player(flux, clash.target_player_id, 3)?;

            flux.pending_clash = None;

            broadcast_flux_impact(room, "CLASH!! +3");
        }

        if let Some(flux) = room.flux_state.as_mut() {
            flux.bump_revision();
        }
    }

    send_flux_state_to_room(state, room_code).await;

    Ok(())
}

fn all_flux_players_ready(room: &Room) -> bool {
    !room.players.is_empty()
        && room
            .players
            .iter()
            .all(|player| room.flux_round_ready.contains(&player.id))
}

async fn broadcast_flux_round_ready(state: &AppState, room_code: &str) {
    let (senders, message) = {
        let rooms = state.rooms.read().await;

        let Some(room) = rooms.get(room_code) else {
            return;
        };

        let mut ready_player_ids = room.flux_round_ready.iter().copied().collect::<Vec<_>>();

        ready_player_ids.sort();

        let message = ServerMessage::FluxRoundReadyState {
            ready_player_ids,
            all_ready: all_flux_players_ready(room),
        };

        let senders = room
            .players
            .iter()
            .map(|player| player.sender.clone())
            .collect::<Vec<_>>();

        (senders, message)
    };

    for sender in senders {
        let _ = sender.send(message.clone());
    }
}

async fn flux_set_round_ready(
    state: &AppState,

    room_code: &str,

    player_id: Uuid,

    ready: bool,
) -> Result<(), ServerMessage> {
    {
        let mut rooms = state.rooms.write().await;

        let room = rooms
            .get_mut(room_code)
            .ok_or_else(|| server_error("Room not found"))?;

        if !room.players.iter().any(|player| player.id == player_id) {
            return Err(server_error("Player not found"));
        }

        if ready {
            room.flux_round_ready.insert(player_id);
        } else {
            room.flux_round_ready.remove(&player_id);
        }
    }

    broadcast_flux_round_ready(state, room_code).await;

    Ok(())
}

async fn flux_next_round(
    state: &AppState,

    room_code: &str,

    player_id: Uuid,

    reset_scores: bool,
) -> Result<(), ServerMessage> {
    {
        let mut rooms = state.rooms.write().await;

        let room = rooms
            .get_mut(room_code)
            .ok_or_else(|| server_error("Room not found"))?;

        if room.host_id != player_id {
            return Err(server_error("Only the host can continue"));
        }

        if !reset_scores && !all_flux_players_ready(room) {
            return Err(server_error(
                "All players must be Ready before the next round",
            ));
        }

        let current_round = room.flux_state.as_ref().map(|flux| flux.round).unwrap_or(0);

        if reset_scores {
            for value in room.flux_scores.values_mut() {
                *value = 0;
            }
        }

        let next_round = if reset_scores {
            1
        } else {
            current_round.saturating_add(1)
        };

        let order = if room.flux_order.is_empty() {
            room.players
                .iter()
                .map(|player| player.id)
                .collect::<Vec<_>>()
        } else {
            room.flux_order.clone()
        };

        room.flux_round_ready.clear();

        room.flux_state = Some(create_flux_state(&order, next_round));

        room.state = RoomState::Playing;
    }

    send_flux_state_to_room(state, room_code).await;

    broadcast_flux_round_ready(state, room_code).await;

    Ok(())
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

            room_name: room.name.clone(),

            host_id: room.host_id,

            max_players: room.max_players,

            visibility: room.visibility.as_str().to_string(),

            state: room.state.as_str().to_string(),

            selected_game: room.selected_game,

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

    let mut turn_order = false;

    let mut playing_flux = false;

    {
        let mut rooms = state.rooms.write().await;

        let Some(room) = rooms.get_mut(room_code) else {
            return;
        };

        let was_host = room.host_id == player_id;

        room.players.retain(|player| player.id != player_id);

        if let Some(flux) = room.flux_state.as_mut() {
            flux.players.retain(|player| player.player_id != player_id);

            if !flux.players.is_empty() {
                flux.current_player_index = flux.current_player_index.min(flux.players.len() - 1);
            }
        }

        if room.players.is_empty() {
            delete_room = true;
        } else {
            if was_host {
                let new_host = room.players[0].id;

                room.host_id = new_host;

                for player in &mut room.players {
                    player.is_host = player.id == new_host;
                }
            }

            turn_order = matches!(room.state, RoomState::TurnOrder);

            playing_flux = matches!(room.state, RoomState::Playing)
                && matches!(room.selected_game, Some(GameId::Flux));
        }
    }

    if delete_room {
        let mut rooms = state.rooms.write().await;

        rooms.remove(room_code);

        return;
    }

    if turn_order {
        broadcast_turn_order_state(state, room_code).await;

        finish_turn_order_if_ready(state, room_code).await;
    } else if playing_flux {
        send_flux_state_to_room(state, room_code).await;
    } else {
        broadcast_room_state(state, room_code).await;
    }
}

fn validate_player_name(player_name: String) -> Result<String, ServerMessage> {
    let player_name = player_name.trim().to_string();

    if player_name.is_empty() {
        return Err(server_error("Nickname is required"));
    }

    if player_name.chars().count() > 24 {
        return Err(server_error("Nickname cannot exceed 24 characters"));
    }

    Ok(player_name)
}

fn validate_room_name(room_name: String) -> Result<String, ServerMessage> {
    let room_name = room_name.trim().to_string();

    let length = room_name.chars().count();

    if length < 3 {
        return Err(server_error("Room name must contain at least 3 characters"));
    }

    if length > 32 {
        return Err(server_error("Room name cannot exceed 32 characters"));
    }

    Ok(room_name)
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

fn turn_order_players_response_for(room: &Room, viewer_id: Uuid) -> Vec<TurnOrderPlayerResponse> {
    room.players
        .iter()
        .map(|player| {
            let rolled = player
                .turn_dice
                .iter()
                .map(|die| die.value.is_some())
                .collect::<Vec<_>>();

            let roll_count = rolled.iter().filter(|value| **value).count();

            let is_owner = player.id == viewer_id;

            let rolls = if is_owner {
                player.turn_dice.iter().map(|die| die.value).collect()
            } else {
                vec![None; DICE_PER_PLAYER]
            };

            TurnOrderPlayerResponse {
                id: player.id,

                name: player.name.clone(),

                rolls,

                rolled,

                total: if is_owner { turn_roll_total(player) } else { 0 },

                complete: roll_count == DICE_PER_PLAYER,

                roll_count,
            }
        })
        .collect()
}

fn turn_roll_total(player: &Player) -> u16 {
    player
        .turn_dice
        .iter()
        .filter_map(|die| die.value)
        .map(u16::from)
        .sum()
}

fn generate_room_code() -> String {
    let mut code = String::with_capacity(ROOM_CODE_LENGTH);

    for index in 0..ROOM_CODE_LENGTH {
        if index % 2 == 0 {
            let letter_index = rand::random_range(0..ROOM_CODE_LETTERS.len());

            code.push(ROOM_CODE_LETTERS[letter_index] as char);
        } else {
            let digit_index = rand::random_range(0..ROOM_CODE_DIGITS.len());

            code.push(ROOM_CODE_DIGITS[digit_index] as char);
        }
    }

    code
}

fn server_error(message: &str) -> ServerMessage {
    ServerMessage::Error {
        message: message.to_string(),
    }
}
