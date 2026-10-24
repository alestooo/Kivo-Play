<script lang="ts">
  import ReadyControls
    from './ReadyControls.svelte';

  import TurnOrderCountdown
    from '../turn-order/TurnOrderCountdown.svelte';

  import TurnOrderGame
    from '../turn-order/TurnOrderGame.svelte';

  import TurnOrderResult
    from '../turn-order/TurnOrderResult.svelte';

  import GameSelector
    from './GameSelector.svelte';

  import SelectedGameCard
    from './SelectedGameCard.svelte';

  import MatchShell
    from '../match/MatchShell.svelte';

  import {
    KivoWebSocket
  } from '@/lib/networking/websocket';

  import {
    mapFluxRoundResult,
    mapFluxState,
    mapFluxStealOffer
  } from '@/lib/networking/messages';

  import type {
    ConnectionStatus,
    PhysicsFrame,
    Player,
    RankingEntry,
    RoomStatePayload,
    RoomSummary,
    ServerMessage,
    TurnOrderPlayer
  } from '@/lib/networking/messages';

  import type {
    GameId
  } from '@/games/types';

  import type {
    ClashColor,
    FluxCard,
    FluxMatchState,
    FluxRoundResult,
    FluxStealOffer
  } from '@/games/flux/types';

  type Screen =
    | 'entry'
    | 'lobby'
    | 'countdown'
    | 'dice'
    | 'ranking'
    | 'match';

  const connection =
    new KivoWebSocket();

  let screen: Screen =
    'entry';

  let connectionStatus:
    ConnectionStatus =
      'disconnected';

  let entryMode:
    'create' | 'join' =
      'create';

  let playerName = '';
  let nicknameError = '';

  let roomName = '';

  let directCode = '';

  let roomSearch = '';

  let privateCode = '';

  let selectedMaxPlayers = 4;

  let selectedVisibility =
    'private';

  let playerId = '';
  let hostId = '';

  let roomCode = '';

  let currentRoomName = '';

  let roomVisibility = '';

  let roomMaxPlayers = 4;

  let selectedGame:
    GameId | null =
      null;

  let showGameSelector =
    false;

  let fluxState:
    FluxMatchState | null =
      null;

  let fluxPlayableCardIds:
    string[] = [];

  let fluxWinnerName = '';

  let fluxRoundResult:
    FluxRoundResult | null =
      null;

  let fluxRoundReadyPlayerIds:
    string[] = [];

  let fluxRoundAllReady =
    false;

  let latestFluxRevision =
    -1;

  let fluxImpactText = '';

  let fluxImpactSerial = 0;

  let fluxStealOffer:
    FluxStealOffer | null =
      null;

  let pendingStealCardId:
    string | null =
      null;

  let resultTransitionDone =
    false;

  let players: Player[] = [];

  let rooms:
    RoomSummary[] = [];

  let currentPage = 1;
  let totalPages = 1;
  let totalRooms = 0;

  let selectedRoom:
    RoomSummary | null =
      null;

  let showPublicConfirm =
    false;

  let showPrivateConfirm =
    false;

  let turnPlayers:
    TurnOrderPlayer[] = [];

  let ranking:
    RankingEntry[] = [];

  let physicsFrames:
    PhysicsFrame[] = [];

  let countdownText = '';

  let rollingDice:
    Set<number> =
      new Set<number>();

  let errorMessage = '';

  let copyMessage = '';

  function handleMessage(
    message: ServerMessage
  ): void {
    switch (message.type) {
      case 'pong':
        return;

      case 'room_directory':
        rooms =
          message.rooms;

        currentPage =
          message.page;

        totalPages =
          message.total_pages;

        totalRooms =
          message.total_rooms;

        return;

      case 'room_created':
      case 'room_joined':
        playerId =
          message.player_id;

        applyRoom(
          message
        );

        selectedRoom =
          null;

        showPublicConfirm =
          false;

        showPrivateConfirm =
          false;

        privateCode = '';

        screen =
          'lobby';

        errorMessage = '';

        return;

      case 'room_state':
        applyRoom(
          message
        );

        return;

      case 'turn_order_started':
        roomCode =
          message.room_code;

        turnPlayers =
          message.players;

        ranking = [];

        physicsFrames = [];

        fluxState =
          null;

        fluxPlayableCardIds = [];

        fluxWinnerName = '';

        resultTransitionDone =
          false;

        rollingDice =
          new Set<number>();

        screen =
          'countdown';

        void runCountdown();

        return;

      case 'turn_order_physics':
        physicsFrames =
          message.frames;

        return;

      case 'turn_order_state':
        turnPlayers =
          message.players;

        clearFinishedRolls();

        return;

      case 'turn_order_finished':
        ranking =
          message.ranking;

        rollingDice =
          new Set<number>();

        screen =
          'ranking';

        void enterSelectedGameAfterResults();

        return;

      case 'flux_state': {
        const nextState =
          mapFluxState(
            message.state
          );

        const currentRound =
          fluxState?.round ??
          0;

        if (
          nextState.round <
          currentRound
        ) {
          return;
        }

        if (
          nextState.round ===
            currentRound &&
          nextState.revision <
            latestFluxRevision
        ) {
          return;
        }

        if (
          nextState.round >
          currentRound
        ) {
          latestFluxRevision =
            -1;
        }

        latestFluxRevision =
          nextState.revision;

        /*
         * A new round state is authoritative.
         * Clear the previous round overlay on every client,
         * not only on the host who clicked Next Round.
         */
        if (
          fluxRoundResult &&
          (
            nextState.round !==
              fluxRoundResult.round ||
            (
              nextState.round === 1 &&
              nextState.scores.every(
                (score) =>
                  score.score === 0
              )
            )
          )
        ) {
          /*
           * Covers both:
           * - normal Next Round (round increases)
           * - Again (new match resets to round 1 / score 0)
           *
           * Every client clears the old result overlay.
           */
          fluxRoundResult =
            null;

          fluxRoundReadyPlayerIds =
            [];

          fluxRoundAllReady =
            false;
        }

        fluxState =
          nextState;

        fluxPlayableCardIds =
          message.playable_card_ids;

        tryEnterSelectedGame();

        return;
      }

      case 'flux_card_played':
        /*
         * This event is the exact card accepted by the server.
         * It updates the center immediately and ignores stale
         * played-card events.
         */
        if (
          fluxState &&
          message.revision >=
            latestFluxRevision
        ) {
          latestFluxRevision =
            message.revision;

          fluxState = {
            ...fluxState,

            revision:
              message.revision,

            discardTop:
              message.card
          };
        }

        return;

      case 'flux_steal_offer':
        fluxStealOffer =
          mapFluxStealOffer(
            message
          );

        return;

      case 'flux_steal_cancelled':
        fluxStealOffer =
          null;

        pendingStealCardId =
          null;

        return;

      case 'flux_round_ready_state':
        fluxRoundReadyPlayerIds =
          message.ready_player_ids;

        fluxRoundAllReady =
          message.all_ready;

        return;

      case 'flux_impact':
        fluxImpactText =
          message.text;

        fluxImpactSerial += 1;

        return;

      case 'flux_round_finished':
        fluxRoundResult =
          mapFluxRoundResult(
            message
          );

        fluxRoundReadyPlayerIds =
          [];

        fluxRoundAllReady =
          false;

        return;

      case 'error':
        errorMessage =
          message.message;

        return;
    }
  }

  function applyRoom(
    room: RoomStatePayload
  ): void {
    roomCode =
      room.room_code;

    currentRoomName =
      room.room_name;

    hostId =
      room.host_id;

    roomVisibility =
      room.visibility;

    roomMaxPlayers =
      room.max_players;

    selectedGame =
      room.selected_game;

    players =
      room.players;
  }

  async function connect():
    Promise<boolean> {
    try {
      await connection.connect(
        handleMessage,
        (
          status:
            ConnectionStatus
        ) => {
          connectionStatus =
            status;
        }
      );

      return true;
    } catch {
      errorMessage =
        'Could not connect to the Kivo Play server.';

      return false;
    }
  }

  function validateNickname():
    boolean {
    if (
      !playerName.trim()
    ) {
      nicknameError =
        'Nickname is required.';

      return false;
    }

    nicknameError = '';

    return true;
  }

  function clearNicknameError():
    void {
    if (
      playerName.trim()
    ) {
      nicknameError = '';
    }
  }

  async function createRoom():
    Promise<void> {
    clearMessages();

    if (
      !validateNickname()
    ) {
      return;
    }

    const name =
      playerName.trim();

    const newRoomName =
      roomName.trim();

    if (
      newRoomName.length < 3
    ) {
      errorMessage =
        'Room name must contain at least 3 characters.';

      return;
    }

    const connected =
      await connect();

    if (!connected) {
      return;
    }

    connection.send({
      type:
        'create_room',

      player_name:
        name,

      room_name:
        newRoomName,

      max_players:
        selectedMaxPlayers,

      visibility:
        selectedVisibility
    });
  }

  async function openJoinBrowser():
    Promise<void> {
    entryMode =
      'join';

    clearMessages();

    const connected =
      await connect();

    if (!connected) {
      return;
    }

    requestRooms(1);
  }

  function requestRooms(
    page: number
  ): void {
    if (
      !connection.isConnected()
    ) {
      return;
    }

    connection.send({
      type:
        'list_rooms',

      page,

      search:
        roomSearch.trim()
    });
  }

  function searchRooms():
    void {
    requestRooms(1);
  }

  function previousPage():
    void {
    if (
      currentPage <= 1
    ) {
      return;
    }

    requestRooms(
      currentPage - 1
    );
  }

  function nextPage():
    void {
    if (
      currentPage >=
      totalPages
    ) {
      return;
    }

    requestRooms(
      currentPage + 1
    );
  }

  async function joinByCode():
    Promise<void> {
    clearMessages();

    if (
      !validateNickname()
    ) {
      return;
    }

    const code =
      normalizeRoomCode(
        directCode
      );

    if (
      code.length !== 6
    ) {
      errorMessage =
        'Enter a valid 6-character room code.';

      return;
    }

    const connected =
      await connect();

    if (!connected) {
      return;
    }

    connection.send({
      type:
        'join_room',

      player_name:
        playerName.trim(),

      room_code:
        code
    });
  }

  function selectRoom(
    room: RoomSummary
  ): void {
    errorMessage = '';

    if (
      !room.joinable
    ) {
      errorMessage =
        'This room is full.';

      return;
    }

    selectedRoom =
      room;

    privateCode = '';

    if (
      room.visibility ===
      'public'
    ) {
      showPublicConfirm =
        true;

      showPrivateConfirm =
        false;
    } else {
      showPrivateConfirm =
        true;

      showPublicConfirm =
        false;
    }
  }

  function cancelRoomSelection():
    void {
    selectedRoom =
      null;

    showPublicConfirm =
      false;

    showPrivateConfirm =
      false;

    privateCode = '';

    errorMessage = '';
  }

  function joinSelectedPublicRoom():
    void {
    if (
      !selectedRoom
    ) {
      return;
    }

    if (
      !validateNickname()
    ) {
      cancelRoomSelection();

      return;
    }

    connection.send({
      type:
        'join_public_room',

      player_name:
        playerName.trim(),

      room_id:
        selectedRoom.room_id
    });
  }

  function joinSelectedPrivateRoom():
    void {
    if (
      !selectedRoom
    ) {
      return;
    }

    if (
      !validateNickname()
    ) {
      cancelRoomSelection();

      return;
    }

    const code =
      normalizeRoomCode(
        privateCode
      );

    if (
      code.length !== 6
    ) {
      errorMessage =
        'Enter the 6-character room code.';

      return;
    }

    connection.send({
      type:
        'join_room',

      player_name:
        playerName.trim(),

      room_code:
        code
    });
  }

  function openGameSelector():
    void {
    if (
      playerId !==
      hostId
    ) {
      return;
    }

    showGameSelector =
      true;
  }

  function closeGameSelector():
    void {
    showGameSelector =
      false;
  }

  function selectGame(
    gameId:
      GameId
  ): void {
    if (
      playerId !==
      hostId
    ) {
      return;
    }

    errorMessage = '';

    const sent =
      connection.send({
        type:
          'select_game',

        game_id:
          gameId
      });

    if (!sent) {
      errorMessage =
        'Connection lost.';

      return;
    }

    showGameSelector =
      false;
  }

  async function enterSelectedGameAfterResults():
    Promise<void> {
    /*
     * TurnOrderResult uses:
     * 3.5 s calculating
     * + 4 s winner/loser
     * = 7.5 s before the ranking.
     *
     * We wait 10.5 s total so the ranking
     * remains visible for roughly 3 seconds.
     */
    await sleep(
      10500
    );

    resultTransitionDone =
      true;

    tryEnterSelectedGame();
  }

  function tryEnterSelectedGame():
    void {
    if (
      !resultTransitionDone ||
      screen !== 'ranking'
    ) {
      return;
    }

    if (
      selectedGame ===
        'flux' &&
      fluxState
    ) {
      screen =
        'match';
    }
  }

  function playFluxCard(
    card:
      FluxCard,

    color?:
      ClashColor,

    targetPlayerId?:
      string
  ): void {
    errorMessage = '';

    const sent =
      connection.send({
        type:
          'flux_play_card',

        card_id:
          card.id,

        chosen_color:
          color ?? null,

        target_player_id:
          targetPlayerId ?? null
      });

    if (!sent) {
      errorMessage =
        'Connection lost.';
    }
  }

  function beginFluxSteal(
    card:
      FluxCard,

    targetPlayerId:
      string
  ): void {
    errorMessage = '';

    pendingStealCardId =
      card.id;

    const sent =
      connection.send({
        type:
          'flux_begin_steal',

        card_id:
          card.id,

        target_player_id:
          targetPlayerId
      });

    if (!sent) {
      pendingStealCardId =
        null;

      errorMessage =
        'Connection lost.';
    }
  }

  function chooseFluxStealCard(
    cardId:
      string
  ): void {
    if (
      !fluxStealOffer
    ) {
      return;
    }

    errorMessage = '';

    const sent =
      connection.send({
        type:
          'flux_choose_steal',

        offer_id:
          fluxStealOffer
            .offerId,

        card_id:
          cardId
      });

    if (!sent) {
      errorMessage =
        'Connection lost.';

      return;
    }

    fluxStealOffer =
      null;

    pendingStealCardId =
      null;
  }

  function callFluxClash():
    void {
    connection.send({
      type:
        'flux_clash'
    });
  }

  function toggleFluxRoundReady():
    void {
    const ready =
      !fluxRoundReadyPlayerIds
        .includes(
          playerId
        );

    connection.send({
      type:
        'flux_round_ready',
      ready
    });
  }

  function nextFluxRound():
    void {
    connection.send({
      type:
        'flux_next_round'
    });
  }

  function playFluxAgain():
    void {
    fluxRoundResult =
      null;

    connection.send({
      type:
        'flux_play_again'
    });
  }

  function drawFluxCard():
    void {
    errorMessage = '';

    const sent =
      connection.send({
        type:
          'flux_draw'
      });

    if (!sent) {
      errorMessage =
        'Connection lost.';
    }
  }

  function toggleReady():
    void {
    const me =
      players.find(
        (
          player:
            Player
        ) =>
          player.id ===
          playerId
      );

    if (!me) {
      return;
    }

    connection.send({
      type:
        'set_ready',

      ready:
        !me.ready
    });
  }

  function startGame():
    void {
    errorMessage = '';

    if (!selectedGame) {
      errorMessage =
        'Select a game before starting.';

      return;
    }

    connection.send({
      type:
        'start_game'
    });
  }

  function rollDie(
    index: number
  ): void {
    const me =
      turnPlayers.find(
        (
          player:
            TurnOrderPlayer
        ) =>
          player.id ===
          playerId
      );

    if (!me) {
      return;
    }

    if (
      me.rolls[index] !==
        null ||
      me.rolled[index] ||
      rollingDice.size > 0
    ) {
      return;
    }

    const next =
      new Set<number>();

    next.add(index);

    rollingDice =
      next;

    const sent =
      connection.send({
        type:
          'roll_turn_die',

        die_index:
          index
      });

    if (!sent) {
      rollingDice =
        new Set<number>();

      errorMessage =
        'Connection lost.';
    }
  }

  function clearFinishedRolls():
    void {
    const me =
      turnPlayers.find(
        (
          player:
            TurnOrderPlayer
        ) =>
          player.id ===
          playerId
      );

    if (!me) {
      return;
    }

    const activeIndex =
      Array.from(
        rollingDice
      )[0];

    if (
      activeIndex !==
        undefined &&
      me.rolls[
        activeIndex
      ] !== null
    ) {
      rollingDice =
        new Set<number>();
    }
  }

  async function runCountdown():
    Promise<void> {
    const steps = [
      {
        text: '3',
        time: 800
      },

      {
        text: '2',
        time: 800
      },

      {
        text: '1',
        time: 800
      },

      {
        text: '¡GIRA!',
        time: 1000
      }
    ];

    for (
      const step of steps
    ) {
      countdownText =
        step.text;

      await sleep(
        step.time
      );
    }

    if (
      screen ===
      'countdown'
    ) {
      screen =
        'dice';
    }
  }

  function leaveRoom():
    void {
    connection.close();

    screen =
      'entry';

    entryMode =
      'create';

    connectionStatus =
      'disconnected';

    playerId = '';
    hostId = '';

    roomCode = '';

    currentRoomName = '';

    roomVisibility = '';

    roomMaxPlayers = 4;

    players = [];

    turnPlayers = [];

    ranking = [];

    physicsFrames = [];

    selectedGame =
      null;

    showGameSelector =
      false;

    fluxState =
      null;

    fluxPlayableCardIds = [];

    fluxWinnerName = '';

    fluxRoundResult =
      null;

    fluxRoundReadyPlayerIds =
      [];

    fluxRoundAllReady =
      false;

    latestFluxRevision =
      -1;

    fluxImpactText = '';

    fluxImpactSerial = 0;

    fluxStealOffer =
      null;

    pendingStealCardId =
      null;

    resultTransitionDone =
      false;

    rooms = [];

    currentPage = 1;

    totalPages = 1;

    totalRooms = 0;

    selectedRoom =
      null;

    showPublicConfirm =
      false;

    showPrivateConfirm =
      false;

    rollingDice =
      new Set<number>();

    countdownText = '';

    errorMessage = '';

    copyMessage = '';
  }

  async function copyRoomCode():
    Promise<void> {
    try {
      await navigator
        .clipboard
        .writeText(
          roomCode
        );

      copyMessage =
        'Copied!';

      setTimeout(
        () => {
          copyMessage = '';
        },

        1500
      );
    } catch {
      copyMessage =
        'Could not copy code.';
    }
  }

  function updateDirectCode():
    void {
    directCode =
      normalizeRoomCode(
        directCode
      );
  }

  function updatePrivateCode():
    void {
    privateCode =
      normalizeRoomCode(
        privateCode
      );
  }

  function normalizeRoomCode(
    value: string
  ): string {
    return value
      .toUpperCase()
      .replace(
        /[^A-Z0-9]/g,
        ''
      )
      .slice(
        0,
        6
      );
  }

  function clearMessages():
    void {
    errorMessage = '';

    copyMessage = '';
  }

  function sleep(
    milliseconds: number
  ): Promise<void> {
    return new Promise<void>(
      (resolve) => {
        setTimeout(
          resolve,
          milliseconds
        );
      }
    );
  }
</script>

{#if screen === 'entry'}
  <section
    class="entry-shell"
    class:browser-mode={
      entryMode === 'join'
    }
  >
    <div class="entry-header">
      <div>
        <span class="alpha-badge">
          KIVO PLAY · ALPHA
        </span>

        <h1>
          Multiplayer
        </h1>

        <p>
          Create a room or find a game.
        </p>
      </div>

      <span
        class="connection-status"
        class:connected={
          connectionStatus ===
          'connected'
        }
      >
        ● {connectionStatus}
      </span>
    </div>

    <div class="mode-tabs">
      <button
        class:active={
          entryMode ===
          'create'
        }
        onclick={() => {
          entryMode =
            'create';

          clearMessages();
        }}
      >
        Create Room
      </button>

      <button
        class:active={
          entryMode ===
          'join'
        }
        onclick={
          openJoinBrowser
        }
      >
        Join Room
      </button>
    </div>

    {#if entryMode === 'create'}
      <label
        class="field"
        class:field-error={
          nicknameError !== ''
        }
      >
        <span>
          Nickname
        </span>

        <input
          class:error-input={
            nicknameError !== ''
          }
          bind:value={
            playerName
          }
          oninput={
            clearNicknameError
          }
          maxlength="24"
          placeholder="Nickname"
          autocomplete="nickname"
        />

        {#if nicknameError}
          <small class="field-error-message">
            {nicknameError}
          </small>
        {/if}
      </label>

      <label class="field">
        <span>
          Room name
        </span>

        <input
          bind:value={
            roomName
          }
          maxlength="32"
          placeholder="Friday Night Games"
        />
      </label>

      <div class="two-fields">
        <label class="field">
          <span>
            Players
          </span>

          <select
            bind:value={
              selectedMaxPlayers
            }
          >
            <option value={1}>
              1 player
            </option>

            <option value={2}>
              2 players
            </option>

            <option value={3}>
              3 players
            </option>

            <option value={4}>
              4 players
            </option>
          </select>
        </label>

        <label class="field">
          <span>
            Visibility
          </span>

          <select
            bind:value={
              selectedVisibility
            }
          >
            <option value="public">
              🌐 Public
            </option>

            <option value="private">
              🔒 Private
            </option>
          </select>
        </label>
      </div>

      <div class="visibility-help">
        <span>
          {selectedVisibility ===
          'public'
            ? '🌐'
            : '🔒'}
        </span>

        <div>
          <strong>
            {selectedVisibility ===
            'public'
              ? 'Public room'
              : 'Private room'}
          </strong>

          <p>
            {selectedVisibility ===
            'public'
              ? 'Players can join directly from the room browser.'
              : 'Players can see the room but need its code to enter.'}
          </p>
        </div>
      </div>

      <button
        class="primary-button"
        onclick={createRoom}
      >
        Create Room
      </button>

    {:else}
      <label
        class="field nickname-join"
        class:field-error={
          nicknameError !== ''
        }
      >
        <span>
          Nickname
        </span>

        <input
          class:error-input={
            nicknameError !== ''
          }
          bind:value={
            playerName
          }
          oninput={
            clearNicknameError
          }
          maxlength="24"
          placeholder="Nickname"
          autocomplete="nickname"
        />

        {#if nicknameError}
          <small class="field-error-message">
            {nicknameError}
          </small>
        {/if}
      </label>

      <div class="quick-join">
        <div>
          <span>
            QUICK JOIN
          </span>

          <strong>
            Have a room code?
          </strong>
        </div>

        <div class="quick-code">
          <input
            bind:value={
              directCode
            }
            oninput={
              updateDirectCode
            }
            maxlength="6"
            placeholder="A2B3C4"
          />

          <button
            onclick={
              joinByCode
            }
          >
            Join
          </button>
        </div>
      </div>

      <div class="browser-toolbar">
        <div class="search-box">
          <span>
            🔎
          </span>

          <input
            bind:value={
              roomSearch
            }
            placeholder="Search room name..."
            onkeydown={(event) => {
              if (
                event.key ===
                'Enter'
              ) {
                searchRooms();
              }
            }}
          />

          <button
            onclick={
              searchRooms
            }
          >
            Search
          </button>
        </div>

        <button
          class="refresh-button"
          onclick={() =>
            requestRooms(
              currentPage
            )}
          title="Refresh rooms"
        >
          ↻
        </button>
      </div>

      <div class="directory-header">
        <div>
          <span>
            ROOM BROWSER
          </span>

          <strong>
            Available rooms
          </strong>
        </div>

        <small>
          {totalRooms}
          {totalRooms === 1
            ? ' room'
            : ' rooms'}
        </small>
      </div>

      {#if rooms.length === 0}
        <div class="empty-state">
          <span>
            🎮
          </span>

          <strong>
            No rooms found
          </strong>

          <small>
            Create one or try another search.
          </small>
        </div>
      {:else}
        <div class="room-list">
          {#each rooms as room}
            <button
              class="room-card"
              class:private={
                room.visibility ===
                'private'
              }
              class:full={
                !room.joinable
              }
              onclick={() =>
                selectRoom(room)}
            >
              <div
                class="room-icon"
                class:private={
                  room.visibility ===
                  'private'
                }
              >
                {room.visibility ===
                'public'
                  ? '🌐'
                  : '🔒'}
              </div>

              <div class="room-data">
                <div class="room-title">
                  <strong>
                    {room.room_name}
                  </strong>

                  <span
                    class:private={
                      room.visibility ===
                      'private'
                    }
                  >
                    {room.visibility}
                  </span>
                </div>

                <small>
                  Hosted by
                  {room.host_name}
                </small>
              </div>

              <div class="population">
                <strong>
                  {room.player_count}
                  /
                  {room.max_players}
                </strong>

                <small>
                  Players
                </small>
              </div>

              <div class="room-arrow">
                {room.joinable
                  ? '›'
                  : 'FULL'}
              </div>
            </button>
          {/each}
        </div>
      {/if}

      <div class="pagination">
        <button
          disabled={
            currentPage <= 1
          }
          onclick={
            previousPage
          }
        >
          ← Previous
        </button>

        <span>
          Page
          <strong>
            {currentPage}
          </strong>
          of
          <strong>
            {totalPages}
          </strong>
        </span>

        <button
          disabled={
            currentPage >=
            totalPages
          }
          onclick={
            nextPage
          }
        >
          Next →
        </button>
      </div>
    {/if}

    {#if errorMessage}
      <p class="error-message">
        {errorMessage}
      </p>
    {/if}
  </section>

{:else if screen === 'lobby'}
  <section class="lobby-panel">
    <div class="lobby-header">
      <div>
        <span class="alpha-badge">
          KIVO PLAY · ALPHA
        </span>

        <p class="visibility-label">
          {roomVisibility ===
          'public'
            ? '🌐 PUBLIC ROOM'
            : '🔒 PRIVATE ROOM'}
        </p>

        <h1>
          {currentRoomName}
        </h1>

        <small>
          Lobby
        </small>
      </div>

      <span class="online-label">
        ● ONLINE
      </span>
    </div>

    <div class="code-panel">
      <span>
        ROOM CODE
      </span>

      <strong>
        {roomCode}
      </strong>

      <button
        onclick={
          copyRoomCode
        }
      >
        Copy
      </button>
    </div>

    {#if copyMessage}
      <p class="copy-message">
        {copyMessage}
      </p>
    {/if}

    <div class="players-heading">
      <strong>
        Players
      </strong>

      <span>
        {players.length}
        /
        {roomMaxPlayers}
      </span>
    </div>

    <div class="lobby-players">
      {#each players as player}
        <article
          class="player-card"
          class:ready={
            player.ready
          }
          class:me={
            player.id ===
            playerId
          }
        >
          <div class="avatar">
            {player.name
              .charAt(0)
              .toUpperCase()}
          </div>

          <div class="player-data">
            <strong>
              {player.name}
            </strong>

            <span>
              {#if player.id === playerId}
                You
              {/if}

              {#if player.is_host}
                · Host
              {/if}
            </span>
          </div>

          <span
            class="ready-label"
            class:ready={
              player.ready
            }
          >
            {player.ready
              ? 'READY'
              : 'NOT READY'}
          </span>
        </article>
      {/each}
    </div>

    <div class="waiting">
      {#if players.length < roomMaxPlayers}
        ● Waiting for players...
      {:else}
        Room full
      {/if}
    </div>

    <SelectedGameCard
      gameId={selectedGame}
      canChange={
        playerId === hostId
      }
      onChange={
        openGameSelector
      }
    />

    {#if !selectedGame && playerId === hostId}
      <button
        class="choose-game-button"
        onclick={
          openGameSelector
        }
      >
        Choose Game
      </button>
    {/if}

    <ReadyControls
      {playerId}
      {hostId}
      {players}
      onToggleReady={
        toggleReady
      }
      onStartGame={
        startGame
      }
    />

    <button
      class="leave-button"
      onclick={
        leaveRoom
      }
    >
      Leave Room
    </button>

    {#if errorMessage}
      <p class="error-message">
        {errorMessage}
      </p>
    {/if}
  </section>

{:else if screen === 'countdown'}
  <TurnOrderCountdown
    text={countdownText}
  />

{:else if screen === 'dice'}
  <TurnOrderGame
    {playerId}
    players={turnPlayers}
    frames={physicsFrames}
    onRoll={rollDie}
    onLeave={leaveRoom}
  />

{:else if screen === 'ranking'}
  <TurnOrderResult
    {playerId}
    {ranking}
    onLeave={leaveRoom}
  />

{:else if screen === 'match' && selectedGame === 'flux' && fluxState}
  <MatchShell
    gameId="flux"
    {playerId}
    {fluxState}
    fluxPlayableCardIds={
      fluxPlayableCardIds
    }
    fluxStealOffer={
      fluxStealOffer
    }
    fluxRoundResult={
      fluxRoundResult
    }
    fluxRoundReadyPlayerIds={
      fluxRoundReadyPlayerIds
    }
    fluxRoundAllReady={
      fluxRoundAllReady
    }
    fluxImpactText={
      fluxImpactText
    }
    fluxImpactSerial={
      fluxImpactSerial
    }
    {hostId}
    onFluxPlay={
      playFluxCard
    }
    onFluxDraw={
      drawFluxCard
    }
    onFluxBeginSteal={
      beginFluxSteal
    }
    onFluxChooseSteal={
      chooseFluxStealCard
    }
    onFluxClash={
      callFluxClash
    }
    onFluxToggleRoundReady={
      toggleFluxRoundReady
    }
    onFluxNextRound={
      nextFluxRound
    }
    onFluxPlayAgain={
      playFluxAgain
    }
    onLeave={
      leaveRoom
    }
  />
{/if}

{#if showGameSelector}
  <div class="modal-layer">
    <button
      class="modal-backdrop"
      aria-label="Close game selector"
      onclick={
        closeGameSelector
      }
    ></button>

    <section class="game-selector-modal">
      <GameSelector
        selectedGame={selectedGame}
        disabled={
          playerId !== hostId
        }
        onSelect={
          selectGame
        }
      />

      <button
        class="cancel-button"
        onclick={
          closeGameSelector
        }
      >
        Cancel
      </button>
    </section>
  </div>
{/if}

{#if showPublicConfirm && selectedRoom}
  <div class="modal-layer">
    <button
      class="modal-backdrop"
      aria-label="Close"
      onclick={
        cancelRoomSelection
      }
    ></button>

    <section class="room-modal">
      <div class="modal-icon">
        🌐
      </div>

      <span class="modal-type">
        PUBLIC ROOM
      </span>

      <h2>
        {selectedRoom.room_name}
      </h2>

      <p>
        Hosted by
        <strong>
          {selectedRoom.host_name}
        </strong>
      </p>

      <div class="modal-info">
        <span>
          👥
          {selectedRoom.player_count}
          /
          {selectedRoom.max_players}
        </span>

        <span>
          No code required
        </span>
      </div>

      <div class="modal-buttons">
        <button
          class="cancel"
          onclick={
            cancelRoomSelection
          }
        >
          Cancel
        </button>

        <button
          class="confirm"
          onclick={
            joinSelectedPublicRoom
          }
        >
          Join Room
        </button>
      </div>
    </section>
  </div>
{/if}

{#if showPrivateConfirm && selectedRoom}
  <div class="modal-layer">
    <button
      class="modal-backdrop"
      aria-label="Close"
      onclick={
        cancelRoomSelection
      }
    ></button>

    <section class="room-modal">
      <div class="modal-icon private">
        🔒
      </div>

      <span class="modal-type private">
        PRIVATE ROOM
      </span>

      <h2>
        {selectedRoom.room_name}
      </h2>

      <p>
        Hosted by
        <strong>
          {selectedRoom.host_name}
        </strong>
      </p>

      <div class="modal-info">
        <span>
          👥
          {selectedRoom.player_count}
          /
          {selectedRoom.max_players}
        </span>

        <span>
          Code required
        </span>
      </div>

      <label class="private-code">
        <span>
          Room code
        </span>

        <input
          bind:value={
            privateCode
          }
          oninput={
            updatePrivateCode
          }
          maxlength="6"
          placeholder="A2B3C4"
        />
      </label>

      {#if errorMessage}
        <p class="modal-error">
          {errorMessage}
        </p>
      {/if}

      <div class="modal-buttons">
        <button
          class="cancel"
          onclick={
            cancelRoomSelection
          }
        >
          Cancel
        </button>

        <button
          class="confirm private"
          onclick={
            joinSelectedPrivateRoom
          }
        >
          Unlock & Join
        </button>
      </div>
    </section>
  </div>
{/if}

<style>
  .entry-shell,
  .lobby-panel {
    width: min(100%, 620px);

    margin: 0 auto;
    padding: 2rem;

    border:
      1px solid
      rgba(255, 255, 255, 0.09);

    border-radius: 28px;

    background:
      linear-gradient(
        145deg,
        rgba(255, 247, 237, 0.065),
        rgba(15, 23, 42, 0.75)
      );

    box-shadow:
      0 40px 120px
      rgba(0, 0, 0, 0.35);

    backdrop-filter:
      blur(22px);
  }

  .entry-shell.browser-mode {
    width: min(100%, 900px);
  }

  .entry-header,
  .lobby-header {
    display: flex;

    justify-content:
      space-between;

    align-items:
      flex-start;

    gap: 1rem;
  }

  .alpha-badge {
    display: inline-flex;

    padding:
      0.35rem 0.65rem;

    border:
      1px solid
      rgba(251, 191, 36, 0.18);

    border-radius: 999px;

    color: #fcd34d;

    background:
      rgba(245, 158, 11, 0.09);

    font-size: 0.65rem;
    font-weight: 900;

    letter-spacing: 0.12em;
  }

  h1 {
    margin:
      0.9rem 0 0.25rem;

    font-size:
      clamp(
        2.1rem,
        6vw,
        3.1rem
      );

    letter-spacing:
      -0.045em;
  }

  .entry-header p {
    margin: 0;

    color:
      rgba(255, 255, 255, 0.5);
  }

  .connection-status {
    color:
      rgba(255, 255, 255, 0.35);

    font-size: 0.65rem;
    font-weight: 850;

    text-transform:
      uppercase;
  }

  .connection-status.connected,
  .online-label {
    color: #6ee7b7;
  }

  .mode-tabs {
    display: grid;

    grid-template-columns:
      1fr 1fr;

    gap: 0.45rem;

    margin:
      2rem 0;

    padding: 0.35rem;

    border-radius: 15px;

    background:
      rgba(0, 0, 0, 0.18);
  }

  .mode-tabs button {
    padding: 0.8rem;

    border: 0;
    border-radius: 11px;

    color:
      rgba(255, 255, 255, 0.45);

    background:
      transparent;

    cursor: pointer;

    font: inherit;
    font-weight: 850;
  }

  .mode-tabs button.active {
    color: #fff7ed;

    background:
      linear-gradient(
        135deg,
        rgba(249, 115, 22, 0.2),
        rgba(245, 158, 11, 0.1)
      );
  }

  .field {
    display: grid;

    gap: 0.5rem;

    margin-bottom: 1rem;
  }

  .field > span,
  .private-code > span {
    color:
      rgba(255, 255, 255, 0.72);

    font-size: 0.76rem;
    font-weight: 800;
  }

  input,
  select {
    width: 100%;

    box-sizing:
      border-box;

    padding:
      0.9rem 1rem;

    border:
      1px solid
      rgba(255, 255, 255, 0.1);

    border-radius: 13px;

    outline: none;

    color: #fff7ed;

    background:
      rgba(7, 10, 18, 0.55);

    font: inherit;
  }

  input:focus,
  select:focus {
    border-color:
      rgba(251, 146, 60, 0.55);

    box-shadow:
      0 0 0 3px
      rgba(249, 115, 22, 0.08);
  }

  .field-error > span {
    color: #fb7185;
  }

  .error-input {
    border-color:
      rgba(251, 113, 133, 0.8)
      !important;

    background:
      rgba(244, 63, 94, 0.055)
      !important;

    box-shadow:
      0 0 0 3px
      rgba(244, 63, 94, 0.08)
      !important;
  }

  .field-error-message {
    color: #fb7185;

    font-size: 0.68rem;
    font-weight: 800;
  }

  .two-fields {
    display: grid;

    grid-template-columns:
      1fr 1fr;

    gap: 0.8rem;
  }

  .visibility-help {
    display: flex;

    align-items: center;

    gap: 0.8rem;

    margin-bottom: 1rem;

    padding: 0.9rem;

    border:
      1px solid
      rgba(255, 255, 255, 0.07);

    border-radius: 14px;

    background:
      rgba(255, 255, 255, 0.025);
  }

  .visibility-help > span {
    font-size: 1.5rem;
  }

  .visibility-help p {
    margin:
      0.15rem 0 0;

    color:
      rgba(255, 255, 255, 0.45);

    font-size: 0.72rem;
  }

  .primary-button {
    width: 100%;

    padding: 1rem;

    border: 0;
    border-radius: 14px;

    color: #251407;

    background:
      linear-gradient(
        135deg,
        #fbbf24,
        #fb923c
      );

    cursor: pointer;

    font: inherit;
    font-weight: 950;
  }

  .nickname-join {
    max-width: 440px;
  }

  .quick-join {
    display: flex;

    align-items: center;
    justify-content:
      space-between;

    gap: 1rem;

    margin-bottom: 1.4rem;

    padding: 1rem;

    border:
      1px solid
      rgba(125, 211, 252, 0.14);

    border-radius: 16px;

    background:
      linear-gradient(
        135deg,
        rgba(59, 130, 246, 0.07),
        rgba(45, 212, 191, 0.035)
      );
  }

  .quick-join > div:first-child {
    display: grid;

    gap: 0.15rem;
  }

  .quick-join > div:first-child span {
    color: #93c5fd;

    font-size: 0.6rem;
    font-weight: 900;
  }

  .quick-code {
    display: flex;

    gap: 0.5rem;
  }

  .quick-code input {
    width: 145px;

    text-align: center;

    font-weight: 950;

    letter-spacing: 0.12em;

    text-transform:
      uppercase;
  }

  .quick-code button,
  .search-box button {
    border: 0;

    border-radius: 11px;

    color: #082f49;

    background: #7dd3fc;

    cursor: pointer;

    font: inherit;
    font-weight: 900;
  }

  .quick-code button {
    padding:
      0 1rem;
  }

  .browser-toolbar {
    display: grid;

    grid-template-columns:
      1fr auto;

    gap: 0.6rem;

    margin-bottom: 1.3rem;
  }

  .search-box {
    display: grid;

    grid-template-columns:
      auto 1fr auto;

    align-items: center;

    gap: 0.5rem;

    padding-left: 0.8rem;

    border:
      1px solid
      rgba(255, 255, 255, 0.09);

    border-radius: 13px;

    background:
      rgba(0, 0, 0, 0.16);
  }

  .search-box input {
    border: 0;

    background:
      transparent;

    box-shadow: none;
  }

  .search-box button {
    height: 100%;

    padding:
      0 1rem;

    border-radius:
      0 12px 12px 0;
  }

  .refresh-button {
    width: 48px;

    border:
      1px solid
      rgba(255, 255, 255, 0.09);

    border-radius: 13px;

    color: #fde68a;

    background:
      rgba(245, 158, 11, 0.06);

    cursor: pointer;

    font-size: 1.2rem;
  }

  .directory-header {
    display: flex;

    align-items:
      flex-end;

    justify-content:
      space-between;

    margin-bottom: 0.8rem;
  }

  .directory-header > div {
    display: grid;

    gap: 0.1rem;
  }

  .directory-header > div span {
    color: #fb923c;

    font-size: 0.58rem;
    font-weight: 950;

    letter-spacing: 0.14em;
  }

  .directory-header small {
    color:
      rgba(255, 255, 255, 0.4);
  }

  .room-list {
    display: grid;

    gap: 0.6rem;
  }

  .room-card {
    width: 100%;

    display: grid;

    grid-template-columns:
      auto 1fr auto auto;

    align-items: center;

    gap: 0.85rem;

    padding: 0.9rem;

    border:
      1px solid
      rgba(255, 255, 255, 0.07);

    border-radius: 15px;

    color: inherit;

    background:
      linear-gradient(
        135deg,
        rgba(45, 212, 191, 0.035),
        rgba(255, 255, 255, 0.02)
      );

    cursor: pointer;

    text-align: left;

    transition:
      transform 160ms ease,
      border-color 160ms ease;
  }

  .room-card:hover:not(.full) {
    transform:
      translateY(-2px);

    border-color:
      rgba(45, 212, 191, 0.3);
  }

  .room-card.private {
    background:
      linear-gradient(
        135deg,
        rgba(251, 146, 60, 0.045),
        rgba(255, 255, 255, 0.02)
      );
  }

  .room-card.private:hover:not(.full) {
    border-color:
      rgba(251, 146, 60, 0.3);
  }

  .room-card.full {
    cursor: not-allowed;

    opacity: 0.45;
  }

  .room-icon {
    width: 44px;
    height: 44px;

    display: grid;
    place-items: center;

    border-radius: 13px;

    background:
      rgba(45, 212, 191, 0.08);

    font-size: 1.2rem;
  }

  .room-icon.private {
    background:
      rgba(251, 146, 60, 0.08);
  }

  .room-data {
    min-width: 0;

    display: grid;

    gap: 0.2rem;
  }

  .room-title {
    display: flex;

    align-items: center;

    gap: 0.5rem;
  }

  .room-title strong {
    overflow: hidden;

    text-overflow:
      ellipsis;

    white-space: nowrap;
  }

  .room-title span {
    padding:
      0.2rem 0.4rem;

    border-radius: 999px;

    color: #6ee7b7;

    background:
      rgba(45, 212, 191, 0.08);

    font-size: 0.52rem;
    font-weight: 900;

    text-transform:
      uppercase;
  }

  .room-title span.private {
    color: #fdba74;

    background:
      rgba(249, 115, 22, 0.08);
  }

  .room-data small {
    color:
      rgba(255, 255, 255, 0.4);
  }

  .population {
    display: grid;

    justify-items: end;
  }

  .population strong {
    color: #fde68a;
  }

  .population small {
    color:
      rgba(255, 255, 255, 0.35);
  }

  .room-arrow {
    min-width: 40px;

    color: #fbbf24;

    text-align: center;

    font-size: 1.5rem;
    font-weight: 900;
  }

  .full .room-arrow {
    font-size: 0.55rem;
  }

  .empty-state {
    display: grid;

    place-items: center;

    gap: 0.35rem;

    min-height: 190px;

    border:
      1px dashed
      rgba(255, 255, 255, 0.08);

    border-radius: 16px;

    color:
      rgba(255, 255, 255, 0.4);
  }

  .empty-state > span {
    font-size: 2rem;
  }

  .empty-state strong {
    color:
      rgba(255, 255, 255, 0.75);
  }

  .pagination {
    display: grid;

    grid-template-columns:
      auto 1fr auto;

    align-items: center;

    gap: 1rem;

    margin-top: 1rem;
  }

  .pagination button {
    padding:
      0.65rem 0.9rem;

    border:
      1px solid
      rgba(255, 255, 255, 0.08);

    border-radius: 10px;

    color: #fef3c7;

    background:
      rgba(255, 255, 255, 0.035);

    cursor: pointer;
  }

  .pagination button:disabled {
    cursor: not-allowed;

    opacity: 0.3;
  }

  .pagination span {
    text-align: center;

    color:
      rgba(255, 255, 255, 0.4);

    font-size: 0.7rem;
  }

  .pagination strong {
    color: #fbbf24;
  }

  .visibility-label {
    margin:
      1rem 0 0;

    color: #fbbf24;

    font-size: 0.65rem;
    font-weight: 900;
  }

  .lobby-header small {
    color:
      rgba(255, 255, 255, 0.4);
  }

  .online-label {
    font-size: 0.65rem;
    font-weight: 900;
  }

  .code-panel {
    display: grid;

    grid-template-columns:
      1fr auto auto;

    align-items: center;

    gap: 1rem;

    margin-top: 1.6rem;

    padding: 1rem;

    border:
      1px solid
      rgba(251, 191, 36, 0.12);

    border-radius: 15px;

    background:
      rgba(245, 158, 11, 0.045);
  }

  .code-panel > span {
    color:
      rgba(255, 255, 255, 0.35);

    font-size: 0.6rem;
    font-weight: 900;
  }

  .code-panel strong {
    color: #fef3c7;

    font-size: 1.5rem;

    letter-spacing: 0.13em;
  }

  .code-panel button {
    padding:
      0.55rem 0.8rem;

    border:
      1px solid
      rgba(251, 191, 36, 0.16);

    border-radius: 9px;

    color: #fde68a;

    background:
      rgba(245, 158, 11, 0.06);

    cursor: pointer;
  }

  .copy-message {
    color: #6ee7b7;

    text-align: center;

    font-size: 0.7rem;
  }

  .players-heading {
    display: flex;

    justify-content:
      space-between;

    margin-top: 1.6rem;

    padding-bottom: 0.7rem;

    border-bottom:
      1px solid
      rgba(255, 255, 255, 0.07);
  }

  .players-heading span {
    color:
      rgba(255, 255, 255, 0.4);
  }

  .lobby-players {
    display: grid;

    gap: 0.65rem;

    margin-top: 0.75rem;
  }

  .player-card {
    display: flex;

    align-items: center;

    gap: 0.8rem;

    padding: 0.85rem;

    border:
      1px solid
      rgba(255, 255, 255, 0.07);

    border-radius: 14px;

    background:
      rgba(255, 255, 255, 0.025);
  }

  .player-card.ready {
    border-color:
      rgba(52, 211, 153, 0.2);

    background:
      rgba(16, 185, 129, 0.045);
  }

  .player-card.me {
    box-shadow:
      inset 3px 0 0
      rgba(251, 191, 36, 0.8);
  }

  .avatar {
    width: 42px;
    height: 42px;

    display: grid;
    place-items: center;

    flex: 0 0 auto;

    border-radius: 12px;

    color: #1c1917;

    background:
      linear-gradient(
        135deg,
        #fde68a,
        #fb923c
      );

    font-weight: 950;
  }

  .player-data {
    display: grid;

    flex: 1;
  }

  .player-data span {
    color:
      rgba(255, 255, 255, 0.4);

    font-size: 0.67rem;
  }

  .ready-label {
    color: #fbbf24;

    font-size: 0.58rem;
    font-weight: 950;
  }

  .ready-label.ready {
    color: #6ee7b7;
  }

  .waiting {
    margin-top: 0.9rem;

    padding: 0.75rem;

    border-radius: 12px;

    color:
      rgba(255, 255, 255, 0.4);

    background:
      rgba(255, 255, 255, 0.025);

    text-align: center;

    font-size: 0.72rem;
  }

  .leave-button {
    width: 100%;

    margin-top: 0.75rem;
    padding: 0.9rem;

    border:
      1px solid
      rgba(251, 113, 133, 0.22);

    border-radius: 14px;

    color: #fda4af;

    background:
      rgba(244, 63, 94, 0.055);

    cursor: pointer;

    font: inherit;
    font-weight: 850;
  }

  .error-message,
  .modal-error {
    color: #fb7185;

    font-size: 0.75rem;
    font-weight: 800;
  }

  .modal-layer {
    position: fixed;
    inset: 0;

    z-index: 300;

    display: grid;
    place-items: center;

    padding: 1rem;
  }

  .modal-backdrop {
    position: absolute;
    inset: 0;

    border: 0;

    background:
      rgba(3, 7, 18, 0.78);

    backdrop-filter:
      blur(12px);
  }

  .room-modal {
    position: relative;

    z-index: 1;

    width: min(100%, 430px);

    padding: 2rem;

    border:
      1px solid
      rgba(255, 255, 255, 0.1);

    border-radius: 24px;

    background:
      linear-gradient(
        155deg,
        #182126,
        #121317
      );

    box-shadow:
      0 45px 130px
      rgba(0, 0, 0, 0.55);

    text-align: center;
  }

  .modal-icon {
    width: 58px;
    height: 58px;

    display: grid;
    place-items: center;

    margin:
      0 auto 1rem;

    border-radius: 18px;

    background:
      rgba(45, 212, 191, 0.1);

    font-size: 1.8rem;
  }

  .modal-icon.private {
    background:
      rgba(249, 115, 22, 0.1);
  }

  .modal-type {
    color: #5eead4;

    font-size: 0.6rem;
    font-weight: 950;

    letter-spacing: 0.13em;
  }

  .modal-type.private {
    color: #fb923c;
  }

  .room-modal h2 {
    margin:
      0.5rem 0;
  }

  .room-modal > p {
    color:
      rgba(255, 255, 255, 0.5);
  }

  .modal-info {
    display: flex;

    justify-content: center;

    gap: 0.5rem;

    margin: 1rem 0;
  }

  .modal-info span {
    padding:
      0.45rem 0.65rem;

    border-radius: 999px;

    color:
      rgba(255, 255, 255, 0.55);

    background:
      rgba(255, 255, 255, 0.045);

    font-size: 0.65rem;
  }

  .private-code {
    display: grid;

    gap: 0.5rem;

    margin:
      1.3rem 0;

    text-align: left;
  }

  .private-code input {
    text-align: center;

    font-size: 1.4rem;
    font-weight: 950;

    letter-spacing: 0.15em;
  }

  .modal-buttons {
    display: grid;

    grid-template-columns:
      1fr 1fr;

    gap: 0.7rem;

    margin-top: 1.3rem;
  }

  .modal-buttons button {
    padding: 0.85rem;

    border-radius: 12px;

    cursor: pointer;

    font: inherit;
    font-weight: 850;
  }

  .cancel {
    border:
      1px solid
      rgba(255, 255, 255, 0.09);

    color:
      rgba(255, 255, 255, 0.6);

    background:
      rgba(255, 255, 255, 0.035);
  }

  .confirm {
    border: 0;

    color: #052e2b;

    background:
      linear-gradient(
        135deg,
        #5eead4,
        #7dd3fc
      );
  }

  .confirm.private {
    color: #271006;

    background:
      linear-gradient(
        135deg,
        #fbbf24,
        #fb923c
      );
  }

  @media (
    max-width: 650px
  ) {
    .entry-shell,
    .lobby-panel {
      padding: 1.25rem;
    }

    .two-fields {
      grid-template-columns:
        1fr;
    }

    .quick-join {
      align-items: stretch;

      flex-direction: column;
    }

    .quick-code {
      display: grid;

      grid-template-columns:
        1fr auto;
    }

    .quick-code input {
      width: 100%;
    }

    .room-card {
      grid-template-columns:
        auto 1fr auto;
    }

    .population {
      display: none;
    }

    .pagination {
      grid-template-columns:
        1fr 1fr;
    }

    .pagination span {
      grid-column:
        1 / -1;

      grid-row: 1;
    }
  }

  .choose-game-button {
    width: 100%;
    padding: 0.78rem;
    border: 1px solid rgba(251, 191, 36, 0.28);
    border-radius: 13px;
    color: #fde68a;
    background: rgba(245, 158, 11, 0.06);
    cursor: pointer;
    font: inherit;
    font-weight: 900;
  }

  .game-selector-modal {
    position: relative;
    z-index: 2;
    width: min(680px, calc(100% - 2rem));
    max-height: calc(100dvh - 3rem);
    overflow-y: auto;
    padding: 1.1rem;
    border: 1px solid rgba(255, 255, 255, 0.09);
    border-radius: 20px;
    background: rgba(17, 19, 26, 0.98);
    box-shadow: 0 30px 90px rgba(0, 0, 0, 0.45);
  }

  .game-selector-modal .cancel-button {
    width: 100%;
    margin-top: 0.8rem;
    padding: 0.7rem;
    border: 1px solid rgba(255, 255, 255, 0.09);
    border-radius: 11px;
    color: rgba(255, 255, 255, 0.65);
    background: rgba(255, 255, 255, 0.03);
    cursor: pointer;
    font: inherit;
    font-weight: 850;
  }
</style>