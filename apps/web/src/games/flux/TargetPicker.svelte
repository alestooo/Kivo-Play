<script lang="ts">
  import type {
    ClashPublicPlayer
  } from './types';

  export let open = false;
  export let playerId = '';
  export let players:
    ClashPublicPlayer[] = [];
  export let onChoose:
    ((playerId: string) => void) | undefined;

  $: targets =
    players.filter(
      (player) =>
        player.id !==
        playerId
    );
</script>

{#if open}
  <div class="layer">
    <div class="panel">
      <span>+8 TARGET</span>

      <strong>
        Choose who draws 8 cards
      </strong>

      <p>
        You will draw 3 cards as recoil.
      </p>

      <div class="targets">
        {#each targets as player}
          <button
            type="button"
            onclick={() =>
              onChoose?.(
                player.id
              )}
          >
            <i>
              {player.name
                .charAt(0)
                .toUpperCase()}
            </i>

            <b>{player.name}</b>

            <small>
              {player.cardCount}
              cards
            </small>
          </button>
        {/each}
      </div>
    </div>
  </div>
{/if}

<style>
  .layer {
    position: absolute;
    inset: 0;
    z-index: 31;
    display: grid;
    place-items: center;
    border-radius: 28px;
    background: rgba(35, 22, 7, 0.48);
    backdrop-filter: blur(7px);
  }

  .panel {
    width: min(390px, calc(100% - 2rem));
    display: grid;
    gap: 0.7rem;
    padding: 1rem;
    border: 1px solid rgba(255, 238, 182, 0.25);
    border-radius: 18px;
    background: #2b1b08;
    text-align: center;
  }

  span {
    color: #fb7185;
    font-size: 0.55rem;
    font-weight: 950;
    letter-spacing: 0.14em;
  }

  strong {
    color: #fff7d6;
  }

  p {
    margin: 0;
    color: rgba(255, 255, 255, 0.48);
    font-size: 0.65rem;
  }

  .targets {
    display: grid;
    gap: 0.45rem;
  }

  button {
    display: grid;
    grid-template-columns: auto 1fr auto;
    align-items: center;
    gap: 0.6rem;
    padding: 0.55rem;
    border: 1px solid rgba(255, 255, 255, 0.09);
    border-radius: 12px;
    color: white;
    background: rgba(255, 255, 255, 0.035);
    cursor: pointer;
    text-align: left;
  }

  button:hover {
    border-color: rgba(251, 191, 36, 0.4);
    background: rgba(251, 191, 36, 0.07);
  }

  i {
    width: 36px;
    height: 36px;
    display: grid;
    place-items: center;
    border-radius: 50%;
    color: #3c2507;
    background: #facc15;
    font-style: normal;
    font-weight: 950;
  }

  small {
    color: rgba(255, 255, 255, 0.4);
  }
</style>
