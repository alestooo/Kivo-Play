<script lang="ts">
  import {
    games
  } from '@/games/registry';

  import type {
    GameId
  } from '@/games/types';

  export let selectedGame: GameId | null = null;
  export let disabled = false;
  export let onSelect:
    ((gameId: GameId) => void) | undefined;
</script>

<section class="selector">
  <header>
    <span>CHOOSE A GAME</span>
    <small>
      {disabled
        ? 'Only the host can change the game.'
        : 'Select the next match.'}
    </small>
  </header>

  <div class="grid">
    {#each games as game}
      <button
        type="button"
        class:selected={selectedGame === game.id}
        disabled={disabled || game.status !== 'available'}
        onclick={() => onSelect?.(game.id)}
      >
        <div class="top">
          <strong>{game.name}</strong>

          <span>
            {game.status === 'available'
              ? 'PLAYABLE'
              : 'SOON'}
          </span>
        </div>

        <p>{game.tagline}</p>

        <small>
          {game.minPlayers}–{game.maxPlayers} players
          ·
          {game.engine.toUpperCase()}
        </small>
      </button>
    {/each}
  </div>
</section>

<style>
  .selector {
    display: grid;
    gap: .7rem;
  }

  header {
    display: flex;
    justify-content: space-between;
    gap: 1rem;
  }

  header span {
    color: #fde68a;
    font-size: .58rem;
    font-weight: 950;
    letter-spacing: .12em;
  }

  header small {
    color: rgba(255,255,255,.34);
    font-size: .58rem;
  }

  .grid {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: .55rem;
  }

  button {
    min-width: 0;
    padding: .8rem;
    border: 1px solid rgba(255,255,255,.07);
    border-radius: 14px;
    color: #fff;
    background: rgba(255,255,255,.025);
    text-align: left;
    cursor: pointer;
  }

  button.selected {
    border-color: rgba(251,191,36,.42);
    background: rgba(245,158,11,.06);
  }

  button:disabled {
    cursor: not-allowed;
    opacity: .45;
  }

  .top {
    display: flex;
    justify-content: space-between;
    gap: .6rem;
  }

  strong {
    font-size: .9rem;
  }

  .top span {
    color: #86efac;
    font-size: .46rem;
    font-weight: 950;
  }

  p {
    margin: .3rem 0;
    color: rgba(255,255,255,.5);
    font-size: .66rem;
  }

  button > small {
    color: rgba(255,255,255,.3);
    font-size: .53rem;
  }

  @media (max-width: 600px) {
    .grid {
      grid-template-columns: 1fr;
    }
  }
</style>
