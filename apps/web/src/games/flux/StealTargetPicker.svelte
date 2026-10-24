<script lang="ts">
  import type { ClashPublicPlayer } from './types';

  export let open = false;
  export let playerId = '';
  export let players: ClashPublicPlayer[] = [];
  export let onChoose: ((playerId: string) => void) | undefined;

  $: targets = players.filter((player) => player.id !== playerId && player.cardCount > 1);
</script>

{#if open}
  <div class="layer">
    <section class="panel">
      <span>STEAL</span>
      <h2>Choose a rival</h2>
      <p>Flux shows only 2 random cards from the chosen rival. You may steal 1. Players with only 1 card are protected.</p>

      <div class="targets">
        {#if targets.length > 0}
          {#each targets as player}
            <button type="button" onclick={() => onChoose?.(player.id)}>
              <i>{player.name.charAt(0).toUpperCase()}</i>
              <div>
                <strong>{player.name}</strong>
                <small>{player.cardCount} cards</small>
              </div>
              <b>PICK</b>
            </button>
          {/each}
        {:else}
          <div class="empty">No valid steal targets right now.</div>
        {/if}
      </div>
    </section>
  </div>
{/if}

<style>
  .layer { position:absolute; inset:0; z-index:32; display:grid; place-items:center; border-radius:28px; background:rgba(35,22,7,.52); backdrop-filter:blur(8px); }
  .panel { width:min(420px, calc(100% - 2rem)); display:grid; gap:.75rem; padding:1rem; border:1px solid rgba(255,238,182,.25); border-radius:18px; background:#2b1b08; box-shadow:0 24px 60px rgba(0,0,0,.35); text-align:center; }
  .panel > span { color:#38bdf8; font-size:.56rem; font-weight:950; letter-spacing:.14em; }
  h2 { margin:0; color:#fff7d6; }
  p { margin:0; color:rgba(255,255,255,.48); font-size:.66rem; }
  .targets { display:grid; gap:.45rem; }
  button { display:grid; grid-template-columns:auto 1fr auto; align-items:center; gap:.6rem; padding:.55rem; border:1px solid rgba(255,255,255,.09); border-radius:12px; color:#fff; background:rgba(255,255,255,.035); cursor:pointer; text-align:left; }
  button:hover { border-color:rgba(56,189,248,.48); background:rgba(56,189,248,.07); }
  i { width:38px; height:38px; display:grid; place-items:center; border-radius:50%; color:#082f49; background:#7dd3fc; font-style:normal; font-weight:950; }
  button div { display:grid; }
  small { color:rgba(255,255,255,.42); }
  button > b { color:#bae6fd; font-size:.52rem; }
  .empty { padding:.75rem; border-radius:12px; color:#fde68a; background:rgba(255,255,255,.04); font-size:.66rem; }
</style>
