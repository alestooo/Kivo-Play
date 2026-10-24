<script lang="ts">
  import FluxCard
    from './FluxCard.svelte';

  import type {
    FluxCard as FluxCardType,
    FluxStealOffer
  } from './types';

  export let offer:
    FluxStealOffer | null =
      null;

  export let onChoose:
    ((cardId: string) => void) | undefined;

  function choose(
    card:
      FluxCardType
  ): void {
    onChoose?.(
      card.id
    );
  }
</script>

{#if offer}
  <div class="layer">
    <section class="panel">
      <span>
        STEAL FROM
        {offer.targetPlayerName.toUpperCase()}
      </span>

      <h2>
        Pick one of these 2
      </h2>

      <p>
        These were selected
        randomly from the rival's
        hand. You cannot inspect
        any other cards.
      </p>

      <div class="options">
        {#each offer.options as card}
          <button
            type="button"
            onclick={() =>
              choose(card)}
          >
            <FluxCard
              {card}
              playable
              compact
            />
          </button>
        {/each}
      </div>

      <small>
        After stealing, you draw
        +1 card from the deck as
        recoil.
      </small>
    </section>
  </div>
{/if}

<style>
  .layer {
    position: absolute;
    inset: 0;
    z-index: 33;
    display: grid;
    place-items: center;
    border-radius: 28px;
    background: rgba(35,22,7,.58);
    backdrop-filter: blur(9px);
  }

  .panel {
    width: min(440px, calc(100% - 2rem));
    display: grid;
    justify-items: center;
    gap: .7rem;
    padding: 1rem;
    border: 1px solid rgba(186,230,253,.25);
    border-radius: 18px;
    background: #22170a;
    box-shadow: 0 26px 65px rgba(0,0,0,.4);
    text-align: center;
  }

  .panel > span {
    color: #7dd3fc;
    font-size: .56rem;
    font-weight: 950;
    letter-spacing: .12em;
  }

  h2 {
    margin: 0;
    color: #fff7d6;
  }

  p {
    margin: 0;
    max-width: 330px;
    color: rgba(255,255,255,.48);
    font-size: .65rem;
  }

  .options {
    display: flex;
    gap: 1rem;
    padding: .4rem 0;
  }

  .options > button {
    padding: 0;
    border: 0;
    background: transparent;
    cursor: pointer;
  }

  .options > button:hover {
    transform: translateY(-5px);
  }

  .panel > small {
    color: #fde68a;
    font-size: .57rem;
  }
</style>
