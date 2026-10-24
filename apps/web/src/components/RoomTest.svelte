<script lang="ts">
  type Player = {
    id: string;
    name: string;
    is_host: boolean;
    ready: boolean;
  };

  type RoomMessage = {
    room_code: string;
    host_id: string;
    max_players: number;
    visibility: string;
    state: string;
    players: Player[];
  };

  type TurnOrderPlayer = {
    id: string;
    name: string;
    rolls: Array<number | null>;
    total: number;
    complete: boolean;
  };

  type RankingEntry = {
    player_id: string;
    name: string;
    total: number;
    position: number;
    tied: boolean;
  };

  type ServerMessage =
    | { type: 'pong' }
    | ({ type: 'room_created'; player_id: string } & RoomMessage)
    | ({ type: 'room_joined'; player_id: string } & RoomMessage)
    | ({ type: 'room_state' } & RoomMessage)
    | {
        type: 'turn_order_started';
        room_code: string;
        players: TurnOrderPlayer[];
      }
    | {
        type: 'turn_order_state';
        players: TurnOrderPlayer[];
        all_complete: boolean;
      }
    | {
        type: 'turn_order_finished';
        ranking: RankingEntry[];
      }
    | {
        type: 'error';
        message: string;
      };

  let socket: WebSocket | null = null;

  let connectionStatus = 'disconnected';

  let mode: 'create' | 'join' = 'create';

  let playerName = '';
  let joinCode = '';

  let maxPlayers = 4;
  let visibility = 'private';

  let roomCode = '';
  let playerId = '';
  let hostId = '';

  let roomVisibility = '';
  let roomMaxPlayers = 4;

  let players: Player[] = [];

  let insideRoom = false;

  let countdownActive = false;
  let countdownText = '';

  let turnOrderActive = false;
  let turnOrderPlayers: TurnOrderPlayer[] = [];

  let turnOrderFinished = false;
  let ranking: RankingEntry[] = [];

  let rollingDice = new Set<number>();

  let errorMessage = '';
  let copyMessage = '';

  function connect(): Promise<void> {
    return new Promise((resolve, reject) => {
      if (socket?.readyState === WebSocket.OPEN) {
        resolve();
        return;
      }

      if (socket?.readyState === WebSocket.CONNECTING) {
        const waitForConnection = () => {
          if (socket?.readyState === WebSocket.OPEN) {
            resolve();
            return;
          }

          if (
            socket?.readyState === WebSocket.CLOSED ||
            socket?.readyState === WebSocket.CLOSING
          ) {
            reject(new Error('Connection failed'));
            return;
          }

          setTimeout(waitForConnection, 100);
        };

        waitForConnection();
        return;
      }

      connectionStatus = 'connecting';

      socket = new WebSocket('ws://127.0.0.1:3000/ws');

      socket.addEventListener('open', () => {
        connectionStatus = 'connected';
        resolve();
      });

      socket.addEventListener('message', handleMessage);

      socket.addEventListener('close', () => {
        connectionStatus = 'disconnected';
        socket = null;
      });

      socket.addEventListener('error', () => {
        connectionStatus = 'error';
        reject(new Error('Connection failed'));
      });
    });
  }

  function handleMessage(event: MessageEvent) {
    try {
      const message = JSON.parse(event.data) as ServerMessage;

      if (
        message.type === 'room_created' ||
        message.type === 'room_joined'
      ) {
        playerId = message.player_id;

        applyRoomState(message);

        insideRoom = true;
        errorMessage = '';

        return;
      }

      if (message.type === 'room_state') {
        applyRoomState(message);
        return;
      }

      if (message.type === 'turn_order_started') {
        roomCode = message.room_code;

        turnOrderPlayers = message.players;

        countdownActive = true;
        turnOrderActive = false;
        turnOrderFinished = false;

        ranking = [];

        void runCountdown();

        return;
      }

      if (message.type === 'turn_order_state') {
        turnOrderPlayers = message.players;

        clearCompletedRollingDice();

        return;
      }

      if (message.type === 'turn_order_finished') {
        ranking = message.ranking;

        turnOrderActive = false;
        turnOrderFinished = true;

        rollingDice = new Set();

        return;
      }

      if (message.type === 'error') {
        errorMessage = message.message;
      }
    } catch {
      errorMessage = 'Invalid server response.';
    }
  }

  function applyRoomState(message: RoomMessage) {
    roomCode = message.room_code;
    hostId = message.host_id;

    roomVisibility = message.visibility;
    roomMaxPlayers = message.max_players;

    players = message.players;
  }

  async function createRoom() {
    resetMessages();

    if (!validateName()) {
      return;
    }

    if (!(await ensureConnection())) {
      return;
    }

    socket?.send(
      JSON.stringify({
        type: 'create_room',
        player_name: playerName.trim(),
        max_players: maxPlayers,
        visibility
      })
    );
  }

  async function joinRoom() {
    resetMessages();

    if (!validateName()) {
      return;
    }

    const code = joinCode
      .trim()
      .toUpperCase();

    if (code.length !== 6) {
      errorMessage = 'Enter a valid 6-character room code.';
      return;
    }

    if (!(await ensureConnection())) {
      return;
    }

    socket?.send(
      JSON.stringify({
        type: 'join_room',
        player_name: playerName.trim(),
        room_code: code
      })
    );
  }

  async function ensureConnection() {
    try {
      await connect();

      return true;
    } catch {
      errorMessage = 'Could not connect to Kivo Play server.';

      return false;
    }
  }

  function validateName() {
    if (!playerName.trim()) {
      errorMessage = 'Enter your name first.';

      return false;
    }

    return true;
  }

  async function copyRoomCode() {
    if (!roomCode) {
      return;
    }

    try {
      await navigator.clipboard.writeText(roomCode);

      copyMessage = 'Copied!';
    } catch {
      copyMessage = 'Could not copy the code.';
    }
  }

  function toggleReady() {
    const me = players.find(
      (player) => player.id === playerId
    );

    if (!me) {
      return;
    }

    if (socket?.readyState !== WebSocket.OPEN) {
      return;
    }

    socket.send(
      JSON.stringify({
        type: 'set_ready',
        ready: !me.ready
      })
    );
  }

  function startGame() {
    if (socket?.readyState !== WebSocket.OPEN) {
      return;
    }

    socket.send(
      JSON.stringify({
        type: 'start_game'
      })
    );
  }

  function rollDie(index: number) {
    const me = getMyTurnOrderPlayer();

    if (!me) {
      return;
    }

    if (me.rolls[index] !== null) {
      return;
    }

    if (rollingDice.has(index)) {
      return;
    }

    if (socket?.readyState !== WebSocket.OPEN) {
      return;
    }

    const next = new Set(rollingDice);
    next.add(index);
    rollingDice = next;

    socket.send(
      JSON.stringify({
        type: 'roll_turn_die',
        die_index: index
      })
    );
  }

  function clearCompletedRollingDice() {
    const me = getMyTurnOrderPlayer();

    if (!me) {
      return;
    }

    const next = new Set(rollingDice);

    me.rolls.forEach((value, index) => {
      if (value !== null) {
        next.delete(index);
      }
    });

    rollingDice = next;
  }

  function getMyTurnOrderPlayer() {
    return turnOrderPlayers.find(
      (player) => player.id === playerId
    );
  }

  function allPlayersReady() {
    return (
      players.length >= 2 &&
      players.every((player) => player.ready)
    );
  }

  function isMeReady() {
    return (
      players.find((player) => player.id === playerId)?.ready ??
      false
    );
  }

  function dieFace(value: number | null) {
    switch (value) {
      case 1:
        return '⚀';

      case 2:
        return '⚁';

      case 3:
        return '⚂';

      case 4:
        return '⚃';

      case 5:
        return '⚄';

      case 6:
        return '⚅';

      default:
        return '?';
    }
  }

  function leaveRoom() {
    if (socket) {
      socket.close();
    }

    socket = null;

    insideRoom = false;

    countdownActive = false;
    turnOrderActive = false;
    turnOrderFinished = false;

    roomCode = '';
    playerId = '';
    hostId = '';

    roomVisibility = '';
    roomMaxPlayers = 4;

    players = [];
    turnOrderPlayers = [];
    ranking = [];

    rollingDice = new Set();

    joinCode = '';

    copyMessage = '';
    errorMessage = '';

    countdownText = '';

    connectionStatus = 'disconnected';
  }

  function resetMessages() {
    errorMessage = '';
    copyMessage = '';
  }

  function normalizeJoinCode() {
    joinCode = joinCode
      .toUpperCase()
      .replace(/[^A-Z0-9]/g, '')
      .slice(0, 6);
  }

  async function runCountdown() {
    const steps = ['3', '2', '1', '¡GIRA!'];

    for (const step of steps) {
      countdownText = step;

      await sleep(step === '¡GIRA!' ? 1000 : 800);
    }

    countdownActive = false;
    turnOrderActive = true;
  }

  function sleep(ms: number) {
    return new Promise((resolve) => {
      setTimeout(resolve, ms);
    });
  }
</script>

<section class="room-system">
  {#if countdownActive}
    <div class="start-screen">
      <div class="start-overlay"></div>

      <div class="start-content">
        <span class="alpha">
          KIVO PLAY · ALPHA
        </span>

        <p class="start-label">
          TURN ORDER
        </p>

        <div class="countdown">
          {countdownText}
        </div>

        <p class="start-helper">
          Get ready to roll.
        </p>
      </div>
    </div>

  {:else if turnOrderFinished}
    <div class="card result-card">
      <div class="top">
        <span class="alpha">
          ALPHA
        </span>

        <span class="online">
          ● ONLINE
        </span>
      </div>

      <p class="eyebrow">
        TURN ORDER
      </p>

      <h2>Starting Order</h2>

      <p class="description">
        Highest total goes first.
      </p>

      <div class="ranking">
        {#each ranking as entry}
          <article
            class="ranking-row"
            class:me={entry.player_id === playerId}
          >
            <div class="position">
              {entry.position}
            </div>

            <div class="ranking-info">
              <strong>
                {entry.name}
              </strong>

              {#if entry.player_id === playerId}
                <span>You</span>
              {/if}
            </div>

            <div class="ranking-score">
              {entry.total}

              {#if entry.tied}
                <small>TIE</small>
              {/if}
            </div>
          </article>
        {/each}
      </div>

      {#if ranking.some((entry) => entry.tied)}
        <div class="tie-warning">
          Tie detected — tie-break roll comes next.
        </div>
      {:else}
        <div class="order-ready">
          Turn order ready.
        </div>
      {/if}

      <button
        class="leave"
        onclick={leaveRoom}
      >
        Leave Match
      </button>
    </div>

  {:else if turnOrderActive}
    <div class="card dice-card">
      <div class="top">
        <span class="alpha">
          ALPHA
        </span>

        <span class="online">
          ● ONLINE
        </span>
      </div>

      <p class="eyebrow">
        TURN ORDER
      </p>

      <h2>Roll your dice</h2>

      <p class="description">
        Roll all three dice. Highest total starts.
      </p>

      {#if getMyTurnOrderPlayer()}
        <div class="my-dice">
          {#each getMyTurnOrderPlayer()!.rolls as value, index}
            <button
              class="die"
              class:rolled={value !== null}
              class:rolling={rollingDice.has(index)}
              disabled={value !== null || rollingDice.has(index)}
              onclick={() => rollDie(index)}
            >
              <span class="die-face">
                {dieFace(value)}
              </span>

              <span class="die-label">
                {#if value === null}
                  ROLL
                {:else}
                  ✓ {value}
                {/if}
              </span>
            </button>
          {/each}
        </div>

        <div class="my-total">
          <span>Your total</span>

          <strong>
            {getMyTurnOrderPlayer()!.total}
          </strong>
        </div>

        {#if getMyTurnOrderPlayer()!.complete}
          <div class="dice-ready">
            ✓ READY
          </div>
        {/if}
      {/if}

      <div class="other-players-title">
        Players
      </div>

      <div class="turn-players">
        {#each turnOrderPlayers as player}
          <article
            class="turn-player"
            class:complete={player.complete}
            class:me={player.id === playerId}
          >
            <div class="avatar">
              {player.name.charAt(0).toUpperCase()}
            </div>

            <div class="turn-player-info">
              <strong>
                {player.name}
              </strong>

              <div class="mini-dice">
                {#each player.rolls as roll}
                  <span class:done={roll !== null}>
                    {roll === null ? '?' : dieFace(roll)}
                  </span>
                {/each}
              </div>
            </div>

            <div class="turn-player-result">
              {#if player.complete}
                <strong>{player.total}</strong>
                <span>READY</span>
              {:else}
                <span>
                  {player.rolls.filter((roll) => roll !== null).length}
                  / 3
                </span>
              {/if}
            </div>
          </article>
        {/each}
      </div>

      <button
        class="leave"
        onclick={leaveRoom}
      >
        Leave Match
      </button>
    </div>

  {:else if !insideRoom}
    <div class="card">
      <div class="top">
        <span class="alpha">
          ALPHA
        </span>

        <span class="connection">
          {connectionStatus}
        </span>
      </div>

      <h2>Kivo Play</h2>

      <p class="subtitle">
        Create a room or join your friends.
      </p>

      <div class="tabs">
        <button
          class:active={mode === 'create'}
          onclick={() => {
            mode = 'create';
            resetMessages();
          }}
        >
          Create Room
        </button>

        <button
          class:active={mode === 'join'}
          onclick={() => {
            mode = 'join';
            resetMessages();
          }}
        >
          Join Room
        </button>
      </div>

      <label>
        Your name

        <input
          bind:value={playerName}
          maxlength="24"
          placeholder="Alejandro"
        />
      </label>

      {#if mode === 'create'}
        <label>
          Players

          <select bind:value={maxPlayers}>
            <option value={1}>1 player</option>
            <option value={2}>2 players</option>
            <option value={3}>3 players</option>
            <option value={4}>4 players</option>
          </select>
        </label>

        <label>
          Visibility

          <select bind:value={visibility}>
            <option value="private">
              Private
            </option>

            <option value="public">
              Public
            </option>
          </select>
        </label>

        <button
          class="primary"
          onclick={createRoom}
        >
          Create Room
        </button>

      {:else}
        <label>
          Room code

          <input
            class="code-input"
            bind:value={joinCode}
            oninput={normalizeJoinCode}
            maxlength="6"
            placeholder="K7MF2Q"
            autocomplete="off"
          />
        </label>

        <button
          class="primary"
          onclick={joinRoom}
        >
          Join Room
        </button>
      {/if}

      {#if errorMessage}
        <p class="error">
          {errorMessage}
        </p>
      {/if}
    </div>

  {:else}
    <div class="card lobby">
      <div class="top">
        <span class="alpha">
          ALPHA
        </span>

        <span class="online">
          ● ONLINE
        </span>
      </div>

      <p class="eyebrow">
        {roomVisibility.toUpperCase()} ROOM
      </p>

      <h2>Lobby</h2>

      <p class="description">
        Share this code with your friends.
      </p>

      <div class="room-code">
        {roomCode}
      </div>

      <button
        class="copy"
        onclick={copyRoomCode}
      >
        Copy Code
      </button>

      {#if copyMessage}
        <p class="copy-message">
          {copyMessage}
        </p>
      {/if}

      <div class="lobby-header">
        <span>Players</span>

        <strong>
          {players.length} / {roomMaxPlayers}
        </strong>
      </div>

      <div class="players">
        {#each players as player}
          <article
            class="player"
            class:me={player.id === playerId}
            class:player-ready={player.ready}
          >
            <div class="avatar">
              {player.name.charAt(0).toUpperCase()}
            </div>

            <div class="player-info">
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
              class:ready-status={player.ready}
              class:not-ready-status={!player.ready}
            >
              {player.ready ? 'READY' : 'NOT READY'}
            </span>
          </article>
        {/each}
      </div>

      {#if players.length < roomMaxPlayers}
        <div class="waiting">
          <span class="pulse"></span>

          Waiting for players...
        </div>
      {:else}
        <div class="full">
          Room full
        </div>
      {/if}

      <button
        class="ready-toggle"
        class:ready-button={isMeReady()}
        onclick={toggleReady}
      >
        {#if isMeReady()}
          Not Ready
        {:else}
          Ready
        {/if}
      </button>

      <div class="room-actions">
        <button
          class="leave"
          onclick={leaveRoom}
        >
          Leave Room
        </button>

        {#if playerId === hostId}
          <button
            class="start"
            disabled={!allPlayersReady()}
            onclick={startGame}
          >
            Start Game
          </button>
        {/if}
      </div>

      {#if errorMessage}
        <p class="error">
          {errorMessage}
        </p>
      {/if}
    </div>
  {/if}
</section>

<style>
  .room-system {
    width: min(100%, 520px);
    margin: 3rem auto;

    font-family:
      Inter,
      system-ui,
      sans-serif;
  }

  .card {
    padding: 2rem;

    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 24px;

    background:
      linear-gradient(
        145deg,
        rgba(255, 255, 255, 0.08),
        rgba(255, 255, 255, 0.025)
      );

    box-shadow:
      0 30px 80px rgba(0, 0, 0, 0.28);

    backdrop-filter: blur(20px);
  }

  .top,
  .lobby-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .alpha {
    padding: 0.35rem 0.65rem;

    border-radius: 999px;

    background: rgba(168, 85, 247, 0.16);

    color: #d8b4fe;

    font-size: 0.68rem;
    font-weight: 800;
    letter-spacing: 0.12em;
  }

  .connection {
    font-size: 0.75rem;
    opacity: 0.45;
  }

  .online {
    color: #86efac;

    font-size: 0.7rem;
    font-weight: 800;
  }

  h2 {
    margin: 1.1rem 0 0.4rem;
    font-size: 2rem;
  }

  .subtitle,
  .description {
    margin-top: 0;
    opacity: 0.58;
  }

  .tabs {
    display: grid;
    grid-template-columns: 1fr 1fr;

    gap: 0.4rem;

    margin: 1.6rem 0;
    padding: 0.35rem;

    border-radius: 14px;

    background: rgba(255, 255, 255, 0.04);
  }

  .tabs button {
    padding: 0.75rem;

    border: 0;
    border-radius: 10px;

    color: inherit;
    background: transparent;

    cursor: pointer;

    font-weight: 700;
  }

  .tabs button.active {
    background: rgba(255, 255, 255, 0.1);
  }

  label {
    display: grid;

    gap: 0.5rem;

    margin-bottom: 1rem;

    font-size: 0.85rem;
    font-weight: 650;
  }

  input,
  select {
    width: 100%;

    box-sizing: border-box;

    padding: 0.9rem 1rem;

    border: 1px solid rgba(255, 255, 255, 0.11);
    border-radius: 12px;

    outline: none;

    background: rgba(0, 0, 0, 0.18);

    color: inherit;

    font: inherit;
  }

  input:focus,
  select:focus {
    border-color: rgba(168, 85, 247, 0.75);

    box-shadow:
      0 0 0 3px rgba(168, 85, 247, 0.12);
  }

  .code-input {
    text-align: center;

    font-size: 1.5rem;
    font-weight: 900;

    letter-spacing: 0.18em;

    text-transform: uppercase;
  }

  .primary,
  .start {
    width: 100%;

    padding: 0.95rem;

    border: 0;
    border-radius: 14px;

    cursor: pointer;

    color: white;

    background:
      linear-gradient(
        135deg,
        #7c3aed,
        #a855f7
      );

    font-weight: 800;
  }

  .start:disabled {
    cursor: not-allowed;
    opacity: 0.35;
  }

  .error {
    color: #fda4af;
    font-weight: 600;
  }

  .eyebrow {
    margin: 1.3rem 0 0;

    font-size: 0.72rem;
    font-weight: 800;

    letter-spacing: 0.14em;

    opacity: 0.48;
  }

  .room-code {
    margin: 1.4rem 0 0.65rem;

    padding: 1.2rem;

    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 16px;

    background: rgba(255, 255, 255, 0.045);

    text-align: center;

    font-size: clamp(2rem, 9vw, 3rem);
    font-weight: 900;

    letter-spacing: 0.18em;
  }

  .copy {
    display: block;

    margin: auto;

    padding: 0.55rem 1rem;

    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 10px;

    background: rgba(255, 255, 255, 0.05);

    color: inherit;

    cursor: pointer;

    font-weight: 700;
  }

  .copy-message {
    text-align: center;

    color: #86efac;

    font-size: 0.8rem;
  }

  .lobby-header {
    margin-top: 2rem;

    padding-bottom: 0.75rem;

    border-bottom:
      1px solid rgba(255, 255, 255, 0.08);
  }

  .players,
  .turn-players,
  .ranking {
    display: grid;

    gap: 0.7rem;

    margin-top: 0.8rem;
  }

  .player,
  .turn-player,
  .ranking-row {
    display: flex;

    align-items: center;

    gap: 0.8rem;

    padding: 0.85rem;

    border: 1px solid rgba(255, 255, 255, 0.06);
    border-radius: 14px;

    background: rgba(255, 255, 255, 0.035);
  }

  .player.me,
  .turn-player.me,
  .ranking-row.me {
    border-color: rgba(168, 85, 247, 0.4);
  }

  .player.player-ready,
  .turn-player.complete {
    background: rgba(34, 197, 94, 0.045);

    border-color: rgba(74, 222, 128, 0.2);
  }

  .avatar {
    display: grid;
    place-items: center;

    width: 40px;
    height: 40px;

    flex: 0 0 auto;

    border-radius: 12px;

    background:
      linear-gradient(
        135deg,
        #7c3aed,
        #4f46e5
      );

    font-weight: 900;
  }

  .player-info,
  .turn-player-info,
  .ranking-info {
    display: grid;

    flex: 1;
  }

  .player-info span,
  .ranking-info span {
    margin-top: 0.15rem;

    font-size: 0.72rem;

    opacity: 0.5;
  }

  .ready-status,
  .not-ready-status {
    font-size: 0.62rem;
    font-weight: 900;

    letter-spacing: 0.04em;
  }

  .ready-status {
    color: #86efac;
  }

  .not-ready-status {
    color: #fbbf24;
  }

  .waiting,
  .full {
    display: flex;

    align-items: center;
    justify-content: center;

    gap: 0.6rem;

    margin-top: 1rem;
    padding: 0.8rem;

    border-radius: 12px;

    background: rgba(255, 255, 255, 0.025);

    font-size: 0.8rem;

    opacity: 0.6;
  }

  .pulse {
    width: 8px;
    height: 8px;

    border-radius: 50%;

    background: #a855f7;

    animation: pulse 1.4s infinite;
  }

  .ready-toggle {
    width: 100%;

    margin-top: 1rem;
    padding: 0.9rem;

    border: 1px solid rgba(74, 222, 128, 0.28);
    border-radius: 14px;

    background: rgba(34, 197, 94, 0.08);

    color: #86efac;

    cursor: pointer;

    font-weight: 800;
  }

  .ready-toggle.ready-button {
    background: rgba(168, 85, 247, 0.1);

    border-color: rgba(168, 85, 247, 0.3);

    color: #d8b4fe;
  }

  .room-actions {
    display: grid;

    gap: 0.75rem;

    margin-top: 1.1rem;
  }

  .leave {
    width: 100%;

    margin-top: 1rem;
    padding: 0.9rem;

    border: 1px solid rgba(248, 113, 113, 0.28);
    border-radius: 14px;

    background: rgba(239, 68, 68, 0.08);

    color: #fca5a5;

    cursor: pointer;

    font-weight: 800;
  }

  .start-screen {
    position: fixed;
    inset: 0;

    z-index: 50;

    display: grid;
    place-items: center;
  }

  .start-overlay {
    position: absolute;
    inset: 0;

    background:
      radial-gradient(
        circle at center,
        rgba(124, 58, 237, 0.24),
        rgba(3, 7, 18, 0.93)
      );

    backdrop-filter: blur(16px);
  }

  .start-content {
    position: relative;

    z-index: 1;

    display: grid;
    place-items: center;

    text-align: center;
  }

  .start-label {
    margin-top: 2rem;

    font-size: 0.8rem;
    font-weight: 900;

    letter-spacing: 0.22em;

    opacity: 0.45;
  }

  .countdown {
    min-height: 130px;

    display: grid;
    place-items: center;

    margin-top: 0.5rem;

    font-size: clamp(4rem, 15vw, 8rem);
    font-weight: 950;

    text-shadow:
      0 0 45px rgba(168, 85, 247, 0.35);
  }

  .start-helper {
    margin: 0;
    opacity: 0.5;
  }

  .my-dice {
    display: grid;

    grid-template-columns: repeat(3, 1fr);

    gap: 0.8rem;

    margin-top: 2rem;
  }

  .die {
    min-height: 135px;

    display: grid;
    place-items: center;

    gap: 0.2rem;

    padding: 1rem;

    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 20px;

    background:
      linear-gradient(
        145deg,
        rgba(255, 255, 255, 0.08),
        rgba(255, 255, 255, 0.025)
      );

    color: white;

    cursor: pointer;

    transition:
      transform 160ms ease,
      border-color 160ms ease,
      background 160ms ease;
  }

  .die:not(:disabled):hover {
    transform: translateY(-4px) scale(1.02);

    border-color:
      rgba(168, 85, 247, 0.5);
  }

  .die.rolled {
    cursor: default;

    border-color:
      rgba(74, 222, 128, 0.4);

    background:
      rgba(34, 197, 94, 0.09);
  }

  .die.rolling {
    animation:
      dice-roll 450ms linear infinite;
  }

  .die-face {
    font-size: 3.4rem;
    line-height: 1;
  }

  .die-label {
    font-size: 0.67rem;
    font-weight: 900;

    letter-spacing: 0.1em;

    opacity: 0.6;
  }

  .my-total {
    display: flex;

    justify-content: space-between;
    align-items: center;

    margin-top: 1rem;

    padding: 1rem;

    border-radius: 14px;

    background:
      rgba(255, 255, 255, 0.04);
  }

  .my-total span {
    opacity: 0.6;
  }

  .my-total strong {
    font-size: 1.5rem;
  }

  .dice-ready {
    margin-top: 0.7rem;

    padding: 0.75rem;

    border-radius: 12px;

    background:
      rgba(34, 197, 94, 0.1);

    color: #86efac;

    text-align: center;

    font-size: 0.8rem;
    font-weight: 900;

    letter-spacing: 0.08em;
  }

  .other-players-title {
    margin-top: 2rem;

    padding-bottom: 0.7rem;

    border-bottom:
      1px solid rgba(255, 255, 255, 0.07);

    font-size: 0.8rem;
    font-weight: 800;

    opacity: 0.6;
  }

  .mini-dice {
    display: flex;

    gap: 0.3rem;

    margin-top: 0.35rem;
  }

  .mini-dice span {
    display: grid;
    place-items: center;

    width: 26px;
    height: 26px;

    border-radius: 7px;

    background:
      rgba(255, 255, 255, 0.05);

    font-size: 1rem;

    opacity: 0.45;
  }

  .mini-dice span.done {
    background:
      rgba(34, 197, 94, 0.1);

    color: #86efac;

    opacity: 1;
  }

  .turn-player-result {
    display: grid;

    justify-items: end;
  }

  .turn-player-result strong {
    font-size: 1.25rem;
  }

  .turn-player-result span {
    font-size: 0.63rem;
    font-weight: 900;

    color: #86efac;
  }

  .position {
    display: grid;
    place-items: center;

    width: 42px;
    height: 42px;

    border-radius: 13px;

    background:
      rgba(168, 85, 247, 0.12);

    color: #d8b4fe;

    font-size: 1.1rem;
    font-weight: 900;
  }

  .ranking-score {
    display: grid;

    justify-items: end;

    font-size: 1.3rem;
    font-weight: 900;
  }

  .ranking-score small {
    margin-top: 0.15rem;

    color: #fbbf24;

    font-size: 0.55rem;
    letter-spacing: 0.08em;
  }

  .tie-warning,
  .order-ready {
    margin-top: 1rem;

    padding: 0.85rem;

    border-radius: 12px;

    text-align: center;

    font-size: 0.8rem;
    font-weight: 700;
  }

  .tie-warning {
    background:
      rgba(245, 158, 11, 0.08);

    color: #fbbf24;
  }

  .order-ready {
    background:
      rgba(34, 197, 94, 0.08);

    color: #86efac;
  }

  @keyframes pulse {
    0%,
    100% {
      opacity: 0.35;
      transform: scale(0.8);
    }

    50% {
      opacity: 1;
      transform: scale(1.15);
    }
  }

  @keyframes dice-roll {
    0% {
      transform: rotate(0deg) scale(1);
    }

    25% {
      transform: rotate(90deg) scale(1.08);
    }

    50% {
      transform: rotate(180deg) scale(0.95);
    }

    75% {
      transform: rotate(270deg) scale(1.08);
    }

    100% {
      transform: rotate(360deg) scale(1);
    }
  }

  @media (max-width: 520px) {
    .card {
      padding: 1.3rem;
    }

    .my-dice {
      gap: 0.5rem;
    }

    .die {
      min-height: 110px;

      padding: 0.7rem;
    }

    .die-face {
      font-size: 2.7rem;
    }
  }

  @media (prefers-reduced-motion: reduce) {
    .pulse,
    .die.rolling {
      animation: none;
    }

    .die {
      transition: none;
    }
  }
</style>