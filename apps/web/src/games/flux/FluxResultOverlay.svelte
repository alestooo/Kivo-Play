<script lang="ts">
  import FluxScoreboard from './FluxScoreboard.svelte';
  import type { FluxRoundResult } from './types';

  export let result: FluxRoundResult | null = null;
  export let playerId = '';
  export let isHost = false;
  export let isReady = false;
  export let allReady = false;
  export let readyCount = 0;
  export let playerCount = 0;

  export let onToggleReady: (() => void) | undefined;
  export let onNextRound: (() => void) | undefined;
  export let onPlayAgain: (() => void) | undefined;
  export let onLeave: (() => void) | undefined;

  $: won = result?.winnerId === playerId;
</script>

{#if result}
  <div class:won class:lost={!won} class="result-layer">
    <section class="result-card">
      <span class="eyebrow">
        {result.matchOver
          ? 'THANKS FOR PLAYING!'
          : `ROUND ${result.round} COMPLETE`}
      </span>

      <h1>
        {#if result.matchOver}
          {won ? 'WINNER!' : 'LOSER!'}
        {:else}
          {won ? 'ROUND WIN!' : `${result.winnerName} WINS`}
        {/if}
      </h1>

      <p>
        {result.winnerName} earned
        <strong>+{result.pointsAwarded}</strong>
        points.
      </p>

      <FluxScoreboard scores={result.scores} {playerId} />

      {#if result.matchOver}
        <div class="actions">
          {#if isHost}
            <button class="primary" onclick={() => onPlayAgain?.()}>
              Again
            </button>
          {/if}

          <button onclick={() => onLeave?.()}>Leave</button>
        </div>
      {:else}
        <div class="ready-area">
          <div class="ready-status">
            <span>{readyCount}/{playerCount} READY</span>
            <div class:complete={allReady} class="ready-bar">
              <i style={`width:${playerCount > 0 ? (readyCount / playerCount) * 100 : 0}%`}></i>
            </div>
          </div>

          <div class="actions">
            <button
              class:ready={isReady}
              class="ready-button"
              onclick={() => onToggleReady?.()}
            >
              {isReady ? 'Ready ✓' : 'Ready'}
            </button>

            {#if isHost}
              <button
                class="primary"
                disabled={!allReady}
                onclick={() => onNextRound?.()}
              >
                Next Round
              </button>
            {:else if allReady}
              <small>Host can start the next round.</small>
            {:else}
              <small>Waiting for everyone to be Ready...</small>
            {/if}
          </div>
        </div>
      {/if}
    </section>
  </div>
{/if}

<style>
  .result-layer { position:absolute; inset:0; z-index:60; display:grid; place-items:center; border-radius:28px; backdrop-filter:blur(10px); }
  .result-layer.won { background:rgba(13,82,52,.76); }
  .result-layer.lost { background:rgba(112,18,31,.76); animation:lose-shake 480ms ease; }
  .result-card { width:min(450px, calc(100% - 2rem)); display:grid; gap:.7rem; padding:1.1rem; border:1px solid rgba(255,255,255,.16); border-radius:20px; background:rgba(14,16,20,.93); box-shadow:0 30px 80px rgba(0,0,0,.4); text-align:center; }
  .eyebrow { color:#fde68a; font-size:.55rem; font-weight:950; letter-spacing:.13em; }
  h1 { margin:0; font-size:clamp(2.5rem,8vw,4.4rem); letter-spacing:-.06em; }
  .won h1 { color:#86efac; } .lost h1 { color:#fca5a5; }
  p { margin:0; color:rgba(255,255,255,.56); font-size:.7rem; } p strong { color:#fde68a; }
  .ready-area { display:grid; gap:.55rem; margin-top:.2rem; }
  .ready-status { display:grid; gap:.3rem; }
  .ready-status > span { color:rgba(255,255,255,.48); font-size:.48rem; font-weight:950; letter-spacing:.1em; }
  .ready-bar { height:6px; overflow:hidden; border-radius:999px; background:rgba(255,255,255,.07); }
  .ready-bar i { display:block; height:100%; border-radius:inherit; background:#facc15; transition:width 180ms ease; }
  .ready-bar.complete i { background:#4ade80; }
  .actions { display:flex; justify-content:center; align-items:center; gap:.55rem; margin-top:.3rem; flex-wrap:wrap; }
  button { padding:.65rem .9rem; border:1px solid rgba(255,255,255,.09); border-radius:10px; color:#fff; background:rgba(255,255,255,.05); cursor:pointer; font-weight:900; }
  button.primary { border:0; color:#2d1b04; background:#facc15; }
  button.primary:disabled { cursor:not-allowed; opacity:.35; filter:grayscale(.45); }
  .ready-button.ready { color:#bbf7d0; border-color:rgba(74,222,128,.32); background:rgba(34,197,94,.12); }
  .actions small { color:rgba(255,255,255,.42); }
  @keyframes lose-shake { 0%,100%{transform:translateX(0)} 20%{transform:translateX(-8px)} 40%{transform:translateX(8px)} 60%{transform:translateX(-5px)} 80%{transform:translateX(5px)} }
</style>
