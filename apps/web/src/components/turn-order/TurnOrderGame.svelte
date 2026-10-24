<script lang="ts">
  import TurnOrderArena from './TurnOrderArena.svelte';
  import TurnOrderPlayers from './TurnOrderPlayers.svelte';

  import type {
    PhysicsFrame,
    TurnOrderPlayer
  } from '@/lib/networking/messages';

  export let playerId = '';
  export let players: TurnOrderPlayer[] = [];
  export let frames: PhysicsFrame[] = [];
  export let onRoll: (index: number) => void;
  export let onLeave: () => void;

  let me: TurnOrderPlayer | undefined;
  let total = 0;

  $: me = players.find((player) => player.id === playerId);
  $: total = me?.total ?? 0;
</script>

<section class="turn-game">
  <header>
    <div>
      <span class="alpha">TURN ORDER · DICE ARENA</span>
      <h1>Roll the dice</h1>
      <small>
        Collisions can change everyone's final result.
      </small>
    </div>

    <strong class="online">● ONLINE</strong>
  </header>

  <div class="game-layout">
    <TurnOrderArena
      {playerId}
      {players}
      {frames}
      {onRoll}
    />

    <div class="side-panel">
      <div class="total">
        <div>
          <span>YOUR TOTAL</span>
          <small>Changes with collisions</small>
        </div>

        <strong>{total}</strong>
      </div>

      <div class="players-heading">
        <strong>Players</strong>
        <small>Rival values hidden</small>
      </div>

      <TurnOrderPlayers
        {playerId}
        {players}
      />

      <button
        class="leave"
        onclick={onLeave}
      >
        Leave Match
      </button>
    </div>
  </div>
</section>

<style>
  .turn-game {
    width: min(calc(100vw - 2rem), 900px);
    margin: 0 auto;
    padding: 0.9rem;
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 20px;
    background:
      linear-gradient(
        155deg,
        rgba(255, 247, 237, 0.045),
        rgba(10, 14, 22, 0.95)
      );
    box-shadow: 0 25px 75px rgba(0, 0, 0, 0.28);
  }

  header {
    min-height: 54px;
    display: flex;
    justify-content: space-between;
    gap: 1rem;
    padding: 0.15rem 0.25rem 0.55rem;
  }

  .alpha {
    color: #fde68a;
    font-size: 0.52rem;
    font-weight: 950;
    letter-spacing: 0.12em;
  }

  h1 {
    margin: 0.18rem 0;
    font-size: 1.6rem;
    letter-spacing: -0.04em;
  }

  header small {
    color: rgba(255, 255, 255, 0.4);
    font-size: 0.64rem;
  }

  .online {
    color: #6ee7b7;
    font-size: 0.54rem;
  }

  .game-layout {
    display: grid;
    grid-template-columns: minmax(330px, 410px) minmax(290px, 1fr);
    align-items: center;
    gap: 0.9rem;
  }

  .side-panel {
    display: grid;
    gap: 0.65rem;
  }

  .total {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.7rem 0.85rem;
    border: 1px solid rgba(251, 191, 36, 0.1);
    border-radius: 12px;
    background: rgba(245, 158, 11, 0.035);
  }

  .total > div {
    display: grid;
  }

  .total span {
    color: #fde68a;
    font-size: 0.49rem;
    font-weight: 950;
  }

  .total small {
    color: rgba(255, 255, 255, 0.32);
    font-size: 0.56rem;
  }

  .total > strong {
    font-size: 1.55rem;
  }

  .players-heading {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding-bottom: 0.4rem;
    border-bottom: 1px solid rgba(255, 255, 255, 0.06);
    font-size: 0.74rem;
  }

  .players-heading small {
    color: rgba(255, 255, 255, 0.32);
    font-size: 0.55rem;
  }

  .leave {
    width: 100%;
    padding: 0.65rem;
    border: 1px solid rgba(239, 68, 68, 0.25);
    border-radius: 11px;
    color: #fca5a5;
    background: rgba(127, 29, 29, 0.19);
    cursor: pointer;
    font: inherit;
    font-size: 0.66rem;
    font-weight: 900;
  }

  @media (max-width: 760px) {
    .game-layout {
      grid-template-columns: 1fr;
    }
  }
</style>
