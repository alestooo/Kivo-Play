<script lang="ts">
  import type {
    FluxCard as FluxCardType
  } from './types';

  export let card:
    FluxCardType;

  export let playable =
    false;

  export let selected =
    false;

  export let compact =
    false;

  export let onPlay:
    ((card: FluxCardType) => void) | undefined;

  function play():
    void {
    if (
      !playable ||
      !onPlay
    ) {
      return;
    }

    onPlay(
      card
    );
  }

  function mainLabel():
    string {
    if (
      card.kind ===
        'number'
    ) {
      return String(
        card.value ?? ''
      );
    }

    switch (
      card.special
    ) {
      case 'draw_two':
        return '+2';

      case 'reverse':
        return '↻';

      case 'draw_four':
        return '+4';

      case 'draw_six':
        return '+6';

      case 'wild':
        return '✦';

      case 'steal':
        return '⇄';

      default:
        return '?';
    }
  }

  function subLabel():
    string {
    switch (
      card.special
    ) {
      case 'draw_two':
        return 'DRAW';

      case 'reverse':
        return 'REVERSE';

      case 'draw_four':
        return 'COLOR';

      case 'draw_six':
        return 'ATTACK';

      case 'wild':
        return 'COLOR';

      case 'steal':
        return 'STEAL';

      default:
        return '';
    }
  }
</script>

<button
  type="button"
  class={`clash-card color-${card.color}`}
  class:playable
  class:selected
  class:compact
  disabled={!playable}
  onclick={play}
>
  <span class="corner">
    {mainLabel()}
  </span>

  <div class="oval">
    <strong>
      {mainLabel()}
    </strong>

    {#if card.kind === 'special'}
      <small>
        {subLabel()}
      </small>
    {/if}
  </div>

  <span class="corner bottom">
    {mainLabel()}
  </span>

  {#if card.color === 'wild'}
    <div
      class="wild-mark"
      aria-hidden="true"
    >
      <i></i>
      <i></i>
      <i></i>
      <i></i>
    </div>
  {/if}
</button>

<style>
  .clash-card {
    position: relative;
    width: 104px;
    aspect-ratio: 0.68;
    flex: 0 0 auto;
    overflow: hidden;
    padding: 0;
    border: 4px solid #f7f4ed;
    border-radius: 17px;
    color: white;
    background: #202126;
    box-shadow:
      0 10px 18px rgba(44, 25, 4, 0.28),
      inset 0 -18px 26px rgba(0, 0, 0, 0.12);
    transform-origin: bottom center;
    transition:
      transform 150ms ease,
      box-shadow 150ms ease,
      filter 150ms ease;
  }

  .color-red {
    background:
      linear-gradient(
        145deg,
        #f43f5e,
        #dc2626
      );
  }

  .color-blue {
    background:
      linear-gradient(
        145deg,
        #38bdf8,
        #0284c7
      );
  }

  .color-green {
    background:
      linear-gradient(
        145deg,
        #4ade80,
        #16a34a
      );
  }

  .color-yellow {
    color: #362108;
    background:
      linear-gradient(
        145deg,
        #fde047,
        #eab308
      );
  }

  .color-wild {
    background:
      linear-gradient(
        145deg,
        #292a31,
        #111217
      );
  }

  .clash-card.playable {
    cursor: pointer;
    box-shadow:
      0 10px 18px rgba(44, 25, 4, 0.28),
      0 0 0 2px rgba(255, 249, 205, 0.4),
      0 0 24px rgba(250, 204, 21, 0.28);
  }

  .clash-card.playable:hover {
    transform:
      translateY(-14px)
      rotate(-1.5deg)
      scale(1.035);
  }

  .clash-card.selected {
    transform:
      translateY(-18px)
      scale(1.05);
  }

  .corner {
    position: absolute;
    z-index: 3;
    top: 7px;
    left: 8px;
    font-size: 0.82rem;
    font-weight: 1000;
    text-shadow:
      0 2px 0
      rgba(0, 0, 0, 0.25);
  }

  .bottom {
    top: auto;
    left: auto;
    right: 8px;
    bottom: 7px;
    transform:
      rotate(180deg);
  }

  .oval {
    position: absolute;
    inset: 19% 12%;
    display: grid;
    place-items: center;
    align-content: center;
    gap: 0.16rem;
    border-radius: 50%;
    color: inherit;
    background:
      rgba(
        255,
        255,
        255,
        0.88
      );
    transform:
      rotate(-12deg);
  }

  .color-wild .oval {
    color: #fff;
    background:
      rgba(
        255,
        255,
        255,
        0.1
      );
  }

  .oval strong,
  .oval small {
    transform:
      rotate(12deg);
  }

  .oval strong {
    font-size: 2rem;
    line-height: 1;
    text-shadow:
      0 2px 0
      rgba(0, 0, 0, 0.16);
  }

  .oval small {
    font-size: 0.42rem;
    font-weight: 950;
    letter-spacing: 0.08em;
  }

  .wild-mark {
    position: absolute;
    z-index: 1;
    left: 50%;
    top: 50%;
    width: 49px;
    height: 49px;
    transform:
      translate(-50%, -50%)
      rotate(45deg);
  }

  .wild-mark i {
    position: absolute;
    width: 24px;
    height: 24px;
  }

  .wild-mark i:nth-child(1) {
    top: 0;
    left: 0;
    background: #ef4444;
  }

  .wild-mark i:nth-child(2) {
    top: 0;
    right: 0;
    background: #38bdf8;
  }

  .wild-mark i:nth-child(3) {
    left: 0;
    bottom: 0;
    background: #facc15;
  }

  .wild-mark i:nth-child(4) {
    right: 0;
    bottom: 0;
    background: #22c55e;
  }

  .compact {
    width: 88px;
  }

  .compact .oval strong {
    font-size: 1.55rem;
  }

  @media (max-width: 700px) {
    .clash-card {
      width: 84px;
    }
  }
</style>
