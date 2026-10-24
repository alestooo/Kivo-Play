<script lang="ts">
  export let playerName = '';
  export let isYourTurn = false;
  export let direction: 1 | -1 = 1;
  export let pendingDraw = 0;
</script>

<div class:mine={isYourTurn} class="turn-indicator">
  <div>
    <small>{isYourTurn ? 'YOUR TURN' : 'CURRENT TURN'}</small>
    <strong>{playerName || 'Waiting...'}</strong>
  </div>

  <div class="direction">
    <span>{direction === 1 ? '↻' : '↺'}</span>
    {#if pendingDraw > 0}<b>+{pendingDraw}</b>{/if}
  </div>
</div>

<style>
  .turn-indicator { display:flex; align-items:center; justify-content:space-between; gap:1rem; padding:.62rem .82rem; border:1px solid rgba(105,65,7,.24); border-radius:14px; background:rgba(105,65,7,.14); box-shadow: 0 10px 24px rgba(0,0,0,.12); }
  .turn-indicator.mine { border-color: rgba(110,66,3,.48); background: rgba(255,245,184,.28); box-shadow: 0 0 0 2px rgba(250,204,21,.2), 0 10px 24px rgba(0,0,0,.12); }
  .turn-indicator > div:first-child { display:grid; }
  small { color: rgba(64,39,6,.58); font-size: .46rem; font-weight: 950; letter-spacing: .12em; }
  strong { color:#392307; font-size:.94rem; font-weight: 1000; }
  .direction { display:flex; align-items:center; gap:.4rem; }
  .direction span { color:#56330a; font-size:1.25rem; font-weight:950; }
  .direction b { padding:.2rem .45rem; border-radius:999px; color:#fff; background:#dc2626; font-size:.58rem; }
</style>
