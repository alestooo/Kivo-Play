<script lang="ts">
  import type {
    Player
  } from '@/lib/networking/messages';

  export let playerId = '';
  export let hostId = '';

  export let players: Player[] = [];

  export let onToggleReady: () => void;
  export let onStartGame: () => void;

  let currentPlayer:
    Player | undefined;

  let isReady = false;
  let canStart = false;

  $: currentPlayer =
    players.find(
      (player) =>
        player.id === playerId
    );

  $: isReady =
    currentPlayer?.ready ??
    false;

  $: canStart =
    players.length >= 2 &&
    players.every(
      (player) =>
        player.ready
    );
</script>

<div class="ready-controls">
  <button
    class="ready-control"
    class:active={isReady}
    onclick={onToggleReady}
  >
    {isReady
      ? 'Not Ready'
      : 'Ready'}
  </button>

  {#if playerId === hostId}
    <button
      class="start-control"
      disabled={!canStart}
      onclick={onStartGame}
    >
      Start Game
    </button>

    {#if !canStart}
      <p class="start-help">
        All players must be ready.
      </p>
    {/if}
  {/if}
</div>

<style>
  .ready-controls {
    display: grid;
    gap: 0.75rem;

    margin-top: 1rem;
  }

  button {
    width: 100%;

    padding: 0.95rem 1rem;

    border-radius: 14px;

    cursor: pointer;

    font: inherit;
    font-weight: 850;

    transition:
      transform 160ms ease,
      background 160ms ease,
      border-color 160ms ease,
      opacity 160ms ease,
      box-shadow 160ms ease;
  }

  button:hover:not(:disabled) {
    transform:
      translateY(-2px);
  }

  .ready-control {
    border:
      1px solid
      rgba(52, 211, 153, 0.3);

    color: #6ee7b7;

    background:
      rgba(16, 185, 129, 0.08);
  }

  .ready-control.active {
    border-color:
      rgba(251, 191, 36, 0.35);

    color: #fde68a;

    background:
      rgba(245, 158, 11, 0.09);
  }

  .start-control {
    border: 0;

    color: #251407;

    background:
      linear-gradient(
        135deg,
        #fbbf24,
        #fb923c
      );

    box-shadow:
      0 12px 35px
      rgba(249, 115, 22, 0.17);
  }

  .start-control:disabled {
    cursor: not-allowed;

    opacity: 0.28;

    box-shadow: none;
  }

  .start-help {
    margin:
      -0.2rem 0 0;

    color:
      rgba(255, 255, 255, 0.35);

    text-align: center;

    font-size: 0.7rem;
  }
</style>