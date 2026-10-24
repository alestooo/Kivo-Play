<script lang="ts">
  import {
    getGame
  } from '@/games/registry';

  import type {
    GameId
  } from '@/games/types';

  export let gameId: GameId | null = null;
  export let canChange = false;
  export let onChange: (() => void) | undefined;

  $: game = gameId ? getGame(gameId) : undefined;
</script>

<section class="selected-game">
  <div>
    <small>SELECTED GAME</small>

    <strong>
      {game?.name ?? 'No game selected'}
    </strong>

    <p>
      {game?.tagline ?? 'The host needs to choose a game.'}
    </p>
  </div>

  {#if canChange}
    <button
      type="button"
      onclick={() => onChange?.()}
    >
      Change
    </button>
  {/if}
</section>

<style>
  .selected-game {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
    padding: .8rem .9rem;
    border: 1px solid rgba(251,191,36,.18);
    border-radius: 14px;
    background: rgba(245,158,11,.035);
  }

  div {
    min-width: 0;
    display: grid;
  }

  small {
    color: #fde68a;
    font-size: .5rem;
    font-weight: 950;
    letter-spacing: .12em;
  }

  strong {
    margin-top: .1rem;
    font-size: .95rem;
  }

  p {
    margin: .1rem 0 0;
    color: rgba(255,255,255,.4);
    font-size: .62rem;
  }

  button {
    padding: .5rem .7rem;
    border: 1px solid rgba(255,255,255,.1);
    border-radius: 10px;
    color: rgba(255,255,255,.72);
    background: rgba(255,255,255,.03);
    cursor: pointer;
  }
</style>
