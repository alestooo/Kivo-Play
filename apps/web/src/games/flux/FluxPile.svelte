<script lang="ts">
  import CardBack from './CardBack.svelte';
  import FluxCard from './FluxCard.svelte';
  import type { ClashColor, FluxCard as FluxCardType } from './types';

  export let topCard: FluxCardType | null = null;
  export let activeColor: ClashColor = 'yellow';
  export let drawPileCount = 0;
  export let pendingDraw = 0;
  export let canDraw = false;
  export let onDraw: (() => void) | undefined;

  const colorLabel = { red: 'RED', blue: 'BLUE', green: 'GREEN', yellow: 'YELLOW', wild: 'WILD' } as const;

  function topCardSummary(): string {
    if (!topCard) return 'WAITING';

    if (topCard.kind === 'number') {
      return `${colorLabel[topCard.color]} ${topCard.value ?? ''}`;
    }

    switch (topCard.special) {
      case 'draw_two':
        return `${colorLabel[topCard.color]} +2`;
      case 'draw_four':
        return `WILD +4`;
      case 'draw_six':
        return `${colorLabel[topCard.color]} +6`;
      case 'reverse':
        return `${colorLabel[topCard.color]} REVERSE`;
      case 'wild':
        return `WILD COLOR`;
      case 'steal':
        return `${colorLabel[topCard.color]} STEAL`;
      default:
        return 'CARD';
    }
  }
</script>

<div class="piles">
  <button class="draw-pile" class:enabled={canDraw} type="button" disabled={!canDraw} onclick={() => onDraw?.()}>
    <div class="stack">
      <CardBack />
      <CardBack />
      <CardBack />
    </div>
    <strong>DRAW</strong>
    <span>{drawPileCount}</span>
  </button>

  <div class="discard">
    {#if topCard}
      <FluxCard card={topCard} compact />
    {:else}
      <div class="empty">EMPTY</div>
    {/if}

    <span class={`active-color color-${activeColor}`}>{activeColor}</span>
    <small class="top-note">LAST CARD</small>
    <strong class="top-summary">{topCardSummary()}</strong>

    {#if pendingDraw > 0}
      <b class="pending">+{pendingDraw}</b>
    {/if}
  </div>
</div>

<style>
  .piles { display:flex; align-items:center; justify-content:center; gap:1.4rem; }
  .draw-pile { display:grid; justify-items:center; gap:.25rem; border:0; color:#4c2c08; background:transparent; cursor:default; }
  .draw-pile.enabled { cursor:pointer; }
  .stack { position:relative; width:72px; height:102px; }
  .stack :global(.card-back) { position:absolute; left:7px; top:8px; width:62px; }
  .stack :global(.card-back:nth-child(1)) { transform:translate(-5px,4px) rotate(-2deg); }
  .stack :global(.card-back:nth-child(2)) { transform:translate(0,2px) rotate(1deg); }
  .stack :global(.card-back:nth-child(3)) { transform:translate(4px,0) rotate(0deg); }
  .draw-pile strong { padding:.25rem .65rem; border-radius:999px; color:#fff1b2; background:rgba(70,41,6,.7); font-size:.54rem; letter-spacing:.1em; }
  .draw-pile span { color:rgba(69,42,8,.7); font-size:.52rem; font-weight:900; }
  .discard { position:relative; display:grid; justify-items:center; gap:.18rem; }
  .empty { width:88px; aspect-ratio:.68; display:grid; place-items:center; border:2px dashed rgba(96,59,8,.32); border-radius:14px; color:rgba(91,55,7,.5); font-size:.55rem; font-weight:950; }
  .active-color { min-width:58px; padding:.22rem .5rem; border-radius:999px; color:white; font-size:.48rem; font-weight:950; text-align:center; text-transform:uppercase; }
  .top-note { color: rgba(73, 45, 8, .7); font-size: .42rem; font-weight: 900; letter-spacing: .12em; }
  .top-summary { color:#4b2f09; font-size:.68rem; font-weight:950; text-align:center; }
  .color-red { background:#dc2626; }
  .color-blue { background:#0284c7; }
  .color-green { background:#16a34a; }
  .color-yellow { color:#3f2905; background:#facc15; }
  .pending { position:absolute; right:-25px; top:-15px; min-width:42px; height:42px; display:grid; place-items:center; border:3px solid #fff4cc; border-radius:50%; color:#fff; background:#dc2626; box-shadow:0 0 18px rgba(220,38,38,.45); font-size:.92rem; }
</style>
