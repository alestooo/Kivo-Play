<script lang="ts">
  import CurrentColorBadge from './CurrentColorBadge.svelte';
  import FluxPile from './FluxPile.svelte';
  import FluxTurnIndicator from './FluxTurnIndicator.svelte';
  import PlayerSeat from './PlayerSeat.svelte';
  import TableBurst from './TableBurst.svelte';
  import type { FluxMatchState } from './types';

  export let state: FluxMatchState;
  export let localPlayerId = '';
  export let burst = false;
  export let onDraw: (() => void) | undefined;

  $: currentPlayer = state.players.find((player) => player.id === state.currentPlayerId);
  $: opponents = state.players.filter((player) => player.id !== localPlayerId);
  $: currentPlayerName = currentPlayer
    ? currentPlayer.id === localPlayerId
      ? `${currentPlayer.name} (YOU)`
      : currentPlayer.name
    : 'Waiting...';

  function seatFor(index: number, count: number): 'left' | 'top' | 'right' {
    if (count === 1) return 'top';
    if (count === 2) return index === 0 ? 'left' : 'right';
    return (['left', 'top', 'right'] as const)[index] ?? 'top';
  }
</script>

<section class="table-wrap">
  <div class="wood-frame">
    <div class="felt-table">
      <div class="felt-pattern"></div>

      <FluxTurnIndicator
        playerName={currentPlayerName}
        isYourTurn={state.currentPlayerId === localPlayerId}
        direction={state.direction}
        pendingDraw={state.pendingDraw}
      />

      {#each opponents as player, index}
        <PlayerSeat
          name={player.name}
          cardCount={player.cardCount}
          active={player.id === state.currentPlayerId}
          position={seatFor(index, opponents.length)}
          variant={index}
        />
      {/each}

      <div class="center-play">
        <FluxPile
          topCard={state.discardTop}
          activeColor={state.activeColor}
          drawPileCount={state.drawPileCount}
          pendingDraw={state.pendingDraw}
          canDraw={state.currentPlayerId === localPlayerId}
          {onDraw}
        />

        <CurrentColorBadge color={state.activeColor} />
      </div>

      <div class="self-marker">
        <div class="self-face">
          <i class="hair"></i>
          <i class="eye one"></i>
          <i class="eye two"></i>
          <i class="mouth"></i>
        </div>

        <span>YOU · {state.privateState.hand.length} cards</span>
      </div>

      <TableBurst active={burst} tone="gold" />
    </div>
  </div>
</section>

<style>
  .table-wrap { width: 100%; display: grid; place-items: center; }
  .wood-frame { width: min(100%, 980px); padding: 17px; border-radius: 38px; background: linear-gradient(145deg,#6d3f13,#a7661d 48%,#5b3210); box-shadow: 0 24px 60px rgba(0,0,0,.34), inset 0 2px 0 rgba(255,238,180,.3); }
  .felt-table { position: relative; min-height: 500px; overflow: hidden; border-radius: 29px; background: radial-gradient(circle at center, rgba(255,234,127,.38), transparent 43%), linear-gradient(145deg,#c88b12,#ad7309 58%,#8f5905); box-shadow: inset 0 0 50px rgba(79,45,2,.35), inset 0 0 0 2px rgba(255,239,173,.15); }
  .felt-pattern { position:absolute; inset:0; opacity:.16; background-image: radial-gradient(rgba(255,247,194,.28) 1px, transparent 1px); background-size: 5px 5px; pointer-events:none; }
  .felt-table > :global(.turn-indicator) { position:absolute; z-index:12; left:50%; top:12px; width:min(300px, 50%); transform:translateX(-50%); }
  .center-play { position:absolute; z-index:6; left:50%; top:54%; display:flex; align-items:center; gap:1.1rem; transform:translate(-50%, -50%); }
  .self-marker { position:absolute; z-index:10; left:50%; bottom:10px; display:flex; align-items:center; gap:.45rem; transform:translateX(-50%); padding:.34rem .55rem; border:1px solid rgba(255,246,201,.3); border-radius:999px; color:#fff9dc; background:rgba(66,37,5,.72); backdrop-filter:blur(8px); }
  .self-face { position:relative; width:36px; height:36px; overflow:hidden; border:2px solid #fde68a; border-radius:50%; background:#efbd8a; }
  .self-face .hair { position:absolute; left:-2px; top:-3px; width:40px; height:16px; border-radius:50%; background:#24150d; }
  .self-face .eye { position:absolute; top:17px; width:4px; height:5px; border-radius:50%; background:#2a170d; }
  .self-face .eye.one { left:9px; }
  .self-face .eye.two { right:9px; }
  .self-face .mouth { position:absolute; left:50%; bottom:6px; width:10px; height:5px; border-bottom:2px solid #7f3f2b; border-radius:50%; transform:translateX(-50%); }
  .self-marker span { font-size:.56rem; font-weight:900; }
</style>
