<script lang="ts">
  import { onMount } from 'svelte';
  import TurnOrderConfetti from './TurnOrderConfetti.svelte';
  import type { RankingEntry } from '@/lib/networking/messages';

  export let playerId = '';
  export let ranking: RankingEntry[] = [];
  export let onLeave: () => void;

  type ResultPhase =
    | 'calculating'
    | 'announcement'
    | 'ranking';

  let phase: ResultPhase = 'calculating';
  let me: RankingEntry | undefined;
  let winner: RankingEntry | undefined;
  let won = false;
  let tied = false;

  $: me = ranking.find((entry) => entry.player_id === playerId);
  $: winner = ranking[0];

  $: won = Boolean(
    me &&
    me.position === 1 &&
    !me.tied
  );

  $: tied = Boolean(
    me &&
    me.position === 1 &&
    me.tied
  );

  onMount(() => {
    const announcementTimer = window.setTimeout(() => {
      phase = 'announcement';
    }, 3500);

    const rankingTimer = window.setTimeout(() => {
      phase = 'ranking';
    }, 7500);

    return () => {
      window.clearTimeout(announcementTimer);
      window.clearTimeout(rankingTimer);
    };
  });
</script>

{#if phase === 'calculating'}
  <section class="overlay calculating">
    <div class="overlay-background"></div>

    <div class="calculate-content">
      <span>TURN ORDER</span>

      <div class="loader">
        <i></i>
        <i></i>
        <i></i>
      </div>

      <h1>CALCULATING</h1>
      <p>Checking the final dice...</p>
    </div>
  </section>

{:else if phase === 'announcement'}
  <section
    class="overlay announcement"
    class:winner={won}
    class:loser={!won && !tied}
    class:tie={tied}
  >
    {#if won}
      <TurnOrderConfetti />
    {/if}

    <div class="overlay-background"></div>

    <div class="announcement-content">
      <span>TURN ORDER</span>

      {#if won}
        <div class="winner-burst">★</div>
        <h1 class="winner-title">WINNER!</h1>
        <p>You start the match.</p>
      {:else if tied}
        <div class="tie-icon">⚡</div>
        <h1 class="tie-title">TIE!</h1>
        <p>First place is tied.</p>
      {:else}
        <div class="lose-icon">×</div>
        <h1 class="loser-title">YOU LOSE</h1>
        <p>
          {winner?.name ?? 'Another player'}
          takes first place.
        </p>
      {/if}

      <strong class="announcement-score">
        {me?.total ?? 0}
      </strong>
    </div>
  </section>

{:else}
  <section
    class="ranking-card"
    class:winner={won}
    class:loser={!won && !tied}
  >
    <span class="eyebrow">TURN ORDER RESULT</span>
    <h1>Final ranking</h1>
    <p>Turn order decided.</p>

    <div class="ranking-list">
      {#each ranking as entry}
        <article
          class:self={entry.player_id === playerId}
          class:first={entry.position === 1}
        >
          <span class="position">#{entry.position}</span>

          <div class="identity">
            <strong>
              {entry.name}
              {#if entry.player_id === playerId}
                <span>(You)</span>
              {/if}
            </strong>

            <small>
              {entry.position === 1
                ? 'Starts the match'
                : 'Waiting turn'}
            </small>
          </div>

          <div class="score">
            <b>{entry.total}</b>

            {#if entry.position === 1}
              <span>WINNER</span>
            {/if}
          </div>
        </article>
      {/each}
    </div>

    <button onclick={onLeave}>
      Leave Match
    </button>
  </section>
{/if}

<style>
  .overlay {
    position: fixed;
    inset: 0;
    z-index: 1600;
    display: grid;
    place-items: center;
    overflow: hidden;
  }

  .overlay-background {
    position: absolute;
    inset: 0;
    background: rgba(5, 8, 13, 0.96);
    backdrop-filter: blur(20px);
  }

  .calculating .overlay-background {
    background:
      radial-gradient(circle, rgba(251, 191, 36, 0.08), transparent 38%),
      rgba(5, 8, 13, 0.96);
  }

  .winner .overlay-background {
    background:
      radial-gradient(circle, rgba(34, 197, 94, 0.32), transparent 46%),
      linear-gradient(135deg, #03180d, #07120b);
  }

  .loser .overlay-background {
    background:
      radial-gradient(circle, rgba(239, 68, 68, 0.26), transparent 46%),
      linear-gradient(135deg, #1a0508, #0c0709);
  }

  .tie .overlay-background {
    background:
      radial-gradient(circle, rgba(56, 189, 248, 0.22), transparent 46%),
      rgba(5, 8, 13, 0.96);
  }

  .calculate-content,
  .announcement-content {
    position: relative;
    z-index: 2;
    display: grid;
    justify-items: center;
    text-align: center;
  }

  .calculate-content > span,
  .announcement-content > span {
    color: rgba(255, 255, 255, 0.4);
    font-size: 0.65rem;
    font-weight: 950;
    letter-spacing: 0.18em;
  }

  .calculate-content h1 {
    margin: 1rem 0 0;
    color: #fff7ed;
    font-size: clamp(2.7rem, 8vw, 5.5rem);
    letter-spacing: -0.06em;
  }

  .calculate-content p,
  .announcement-content p {
    color: rgba(255, 255, 255, 0.5);
  }

  .loader {
    height: 60px;
    display: flex;
    align-items: end;
    gap: 0.45rem;
    margin-top: 1.5rem;
  }

  .loader i {
    width: 12px;
    height: 12px;
    border-radius: 50%;
    background: #fbbf24;
    animation: bounce 700ms ease-in-out infinite alternate;
  }

  .loader i:nth-child(2) { animation-delay: 150ms; }
  .loader i:nth-child(3) { animation-delay: 300ms; }

  .announcement-content h1 {
    margin: 0.3rem 0;
    font-size: clamp(4rem, 14vw, 9rem);
    line-height: 0.9;
    letter-spacing: -0.075em;
  }

  .winner-title {
    color: #86efac;
    text-shadow: 0 0 75px rgba(34, 197, 94, 0.65);
    animation: winner-motion 420ms ease-in-out infinite alternate;
  }

  .loser-title {
    color: #f87171;
    text-shadow: 0 0 60px rgba(239, 68, 68, 0.45);
    animation: lose-motion 900ms ease-in-out infinite;
  }

  .tie-title {
    color: #7dd3fc;
  }

  .winner-burst {
    margin-top: 1.2rem;
    color: #fde68a;
    font-size: 5rem;
    animation: burst 650ms ease-in-out infinite alternate;
  }

  .lose-icon,
  .tie-icon {
    margin-top: 1.2rem;
    font-size: 4rem;
  }

  .lose-icon {
    color: #f87171;
    animation: lose-motion 900ms ease-in-out infinite;
  }

  .announcement-score {
    margin-top: 1rem;
    font-size: 2.5rem;
  }

  .ranking-card {
    width: min(calc(100vw - 2rem), 610px);
    margin: 0 auto;
    padding: 1.4rem;
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 20px;
    background: rgba(15, 18, 24, 0.96);
    text-align: center;
  }

  .ranking-card.winner {
    border-color: rgba(34, 197, 94, 0.3);
  }

  .ranking-card.loser {
    border-color: rgba(239, 68, 68, 0.24);
  }

  .eyebrow {
    color: #fde68a;
    font-size: 0.56rem;
    font-weight: 950;
    letter-spacing: 0.12em;
  }

  .ranking-card h1 {
    margin: 0.35rem 0;
    font-size: 2.2rem;
  }

  .ranking-card > p {
    margin: 0 0 1rem;
    color: rgba(255, 255, 255, 0.4);
  }

  .ranking-list {
    display: grid;
    gap: 0.5rem;
  }

  article {
    display: grid;
    grid-template-columns: auto 1fr auto;
    align-items: center;
    gap: 0.7rem;
    padding: 0.7rem;
    border: 1px solid rgba(255, 255, 255, 0.07);
    border-radius: 12px;
    text-align: left;
  }

  article.first {
    border-color: rgba(251, 191, 36, 0.3);
  }

  article.self {
    box-shadow: inset 3px 0 0 #a855f7;
  }

  .position {
    width: 40px;
    height: 40px;
    display: grid;
    place-items: center;
    border-radius: 10px;
    color: #281505;
    background: linear-gradient(135deg, #fde68a, #fb923c);
    font-weight: 950;
  }

  .identity {
    display: grid;
  }

  .identity strong span {
    color: rgba(255, 255, 255, 0.4);
  }

  .identity small {
    color: rgba(255, 255, 255, 0.34);
    font-size: 0.61rem;
  }

  .score {
    display: grid;
    justify-items: end;
  }

  .score b {
    font-size: 1.1rem;
  }

  .score span {
    color: #86efac;
    font-size: 0.48rem;
    font-weight: 950;
  }

  .ranking-card button {
    width: 100%;
    margin-top: 0.8rem;
    padding: 0.7rem;
    border: 1px solid rgba(239, 68, 68, 0.25);
    border-radius: 11px;
    color: #fca5a5;
    background: rgba(127, 29, 29, 0.2);
    cursor: pointer;
    font: inherit;
    font-weight: 900;
  }

  @keyframes bounce {
    from { transform: translateY(0); }
    to { transform: translateY(-20px); }
  }

  @keyframes winner-motion {
    from { transform: scale(0.95) rotate(-1.5deg); }
    to { transform: scale(1.08) rotate(1.5deg); }
  }

  @keyframes burst {
    from { transform: scale(0.85) rotate(-10deg); }
    to { transform: scale(1.12) rotate(10deg); }
  }

  @keyframes lose-motion {
    0% { transform: translateX(0); }
    20% { transform: translateX(-9px); }
    40% { transform: translateX(6px); }
    60% { transform: translateX(-5px); }
    80% { transform: translateX(3px); }
    100% { transform: translateX(0); }
  }
</style>
