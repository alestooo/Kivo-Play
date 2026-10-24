<script lang="ts">
  import FluxCard
    from './FluxCard.svelte';

  import type {
    FluxCard as FluxCardType
  } from './types';

  export let cards:
    FluxCardType[] = [];

  export let playableCardIds:
    string[] = [];

  export let onPlay:
    ((card: FluxCardType) => void) | undefined;

  /*
   * The cards themselves are 104 px wide on desktop.
   * Instead of allowing a large hand to grow outside the
   * game area, we dynamically reduce the visible step
   * between cards.
   *
   * Small hand  -> cards are mostly open.
   * Large hand  -> cards overlap strongly.
   *
   * Every card remains in the DOM and can still be
   * hovered/clicked.
   */
  const CARD_WIDTH =
    104;

  const MAX_HAND_WIDTH =
    820;

  const MAX_VISIBLE_STEP =
    80;

  const MIN_VISIBLE_STEP =
    19;

  $: cardStep =
    calculateCardStep(
      cards.length
    );

  $: overlap =
    cardStep -
    CARD_WIDTH;

  $: dense =
    cards.length >= 14;

  $: veryDense =
    cards.length >= 22;

  function calculateCardStep(
    count:
      number
  ): number {
    if (count <= 1) {
      return 0;
    }

    const availableStep =
      (
        MAX_HAND_WIDTH -
        CARD_WIDTH
      ) /
      (
        count - 1
      );

    return Math.max(
      MIN_VISIBLE_STEP,
      Math.min(
        MAX_VISIBLE_STEP,
        availableStep
      )
    );
  }
</script>

<section
  class="hand-shell"
  class:dense
  class:very-dense={veryDense}
>
  <div
    class="hand"
    style={`
      --card-step:${cardStep}px;
      --overlap:${overlap}px;
    `}
  >
    {#each cards as card, index (card.id)}
      <div
        class="card-slot"
        style={`
          --index:${index};
          z-index:${index + 1};
        `}
      >
        <FluxCard
          {card}
          playable={
            playableCardIds
              .includes(
                card.id
              )
          }
          {onPlay}
        />
      </div>
    {/each}
  </div>

  {#if cards.length >= 14}
    <div class="hand-count">
      <span>
        YOUR HAND
      </span>

      <strong>
        {cards.length}
      </strong>
    </div>
  {/if}
</section>

<style>
  .hand-shell {
    position: relative;
    z-index: 20;
    width: min(100%, 920px);
    margin: -0.15rem auto 0;
    padding: 0 1rem;
    box-sizing: border-box;
  }

  .hand {
    position: relative;
    min-height: 168px;
    display: flex;
    align-items: flex-end;
    justify-content: center;
    padding:
      1rem 0
      0.25rem;
    overflow: visible;
  }

  .card-slot {
    position: relative;
    flex: 0 0 auto;
    margin-left:
      var(--overlap);
    transform-origin:
      bottom center;
    transition:
      transform 150ms ease,
      margin 150ms ease;
  }

  .card-slot:first-child {
    margin-left: 0;
  }

  /*
   * Raise the complete slot as well as the card's own
   * hover animation. This keeps a heavily overlapped card
   * easy to inspect and click.
   */
  .card-slot:hover {
    z-index: 200 !important;
    transform:
      translateY(-8px);
  }

  /*
   * With many cards the fan is slightly curved instead
   * of becoming one completely flat horizontal strip.
   */
  .dense .card-slot:nth-child(4n + 1) {
    transform:
      rotate(-1deg);
  }

  .dense .card-slot:nth-child(4n + 3) {
    transform:
      rotate(1deg);
  }

  .dense .card-slot:hover {
    transform:
      translateY(-10px)
      rotate(0);
  }

  .very-dense .card-slot:nth-child(3n + 1) {
    transform:
      translateY(3px)
      rotate(-1.2deg);
  }

  .very-dense .card-slot:nth-child(3n + 2) {
    transform:
      translateY(0)
      rotate(0);
  }

  .very-dense .card-slot:nth-child(3n) {
    transform:
      translateY(3px)
      rotate(1.2deg);
  }

  .very-dense .card-slot:hover {
    transform:
      translateY(-12px)
      rotate(0)
      scale(1.02);
  }

  .hand-count {
    position: absolute;
    left: 50%;
    bottom: -0.25rem;
    z-index: 300;
    display: flex;
    align-items: center;
    gap: 0.35rem;
    transform:
      translateX(-50%);
    padding:
      0.22rem
      0.5rem;
    border:
      1px solid
      rgba(
        250,
        204,
        21,
        0.2
      );
    border-radius: 999px;
    color:
      rgba(
        255,
        255,
        255,
        0.48
      );
    background:
      rgba(
        32,
        20,
        7,
        0.88
      );
    backdrop-filter:
      blur(8px);
    pointer-events: none;
  }

  .hand-count span {
    font-size: 0.45rem;
    font-weight: 900;
    letter-spacing:
      0.1em;
  }

  .hand-count strong {
    color: #fde68a;
    font-size: 0.58rem;
  }

  @media (
    max-width: 760px
  ) {
    .hand-shell {
      width: 100%;
      padding:
        0 0.35rem;
    }

    .hand {
      min-height: 145px;
      justify-content:
        center;
      transform:
        scale(0.86);
      transform-origin:
        center bottom;
    }
  }

  @media (
    max-width: 520px
  ) {
    .hand {
      min-height: 132px;
      transform:
        scale(0.76);
    }
  }
</style>
