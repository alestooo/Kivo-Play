<script lang="ts">
  import type { ClashColor } from './types';
  export let color: ClashColor = 'yellow';
  const labels = { red:'RED', blue:'BLUE', green:'GREEN', yellow:'YELLOW' } as const;
</script>

<div class={`current-color color-${color}`}>
  <small>CURRENT COLOR</small>
  <span class="orb" aria-hidden="true"></span>
  <strong>{labels[color]}</strong>
</div>

<style>
  .current-color { display:grid; grid-template-columns:auto auto; align-items:center; gap:.14rem .6rem; min-width:140px; padding:.56rem .7rem; border:1px solid rgba(255,255,255,.18); border-radius:16px; color:#fff; background:rgba(37,24,7,.8); box-shadow:0 10px 28px rgba(0,0,0,.22); backdrop-filter:blur(8px); }
  small { grid-column:1/-1; color:rgba(255,255,255,.48); font-size:.43rem; font-weight:950; letter-spacing:.12em; }
  .orb { width:38px; height:38px; border:4px solid rgba(255,255,255,.92); border-radius:50%; box-shadow:0 0 18px currentColor; }
  strong { font-size:.8rem; font-weight:950; }
  .color-red { color:#fb7185; } .color-red .orb { background:#ef4444; }
  .color-blue { color:#7dd3fc; } .color-blue .orb { background:#0ea5e9; }
  .color-green { color:#86efac; } .color-green .orb { background:#22c55e; }
  .color-yellow { color:#fde68a; } .color-yellow .orb { background:#facc15; }
</style>
