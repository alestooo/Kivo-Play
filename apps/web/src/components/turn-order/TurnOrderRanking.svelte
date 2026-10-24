<script lang="ts">
  import type {
    RankingEntry
  } from '@/lib/networking/messages';

  export let playerId = '';

  export let ranking:
    RankingEntry[] = [];

  export let onLeave:
    () => void;

  function hasTie(): boolean {
    return ranking.some(
      (entry: RankingEntry) =>
        entry.tied
    );
  }
</script>

<section class="ranking-panel">
  <div class="panel-top">
    <span class="alpha">
      ALPHA
    </span>

    <span class="online">
      ● ONLINE
    </span>
  </div>

  <p class="eyebrow">
    TURN ORDER
  </p>

  <h1>
    Starting Order
  </h1>

  <p class="description">
    Highest total goes first.
  </p>

  <div class="ranking-list">
    {#each ranking as entry}
      <article
        class="ranking-row"
        class:me={
          entry.player_id ===
          playerId
        }
      >
        <div class="position">
          {entry.position}
        </div>

        <div class="player">
          <strong>
            {entry.name}
          </strong>

          {#if entry.player_id === playerId}
            <span>
              You
            </span>
          {/if}
        </div>

        <div class="score">
          <strong>
            {entry.total}
          </strong>

          {#if entry.tied}
            <span>
              TIE
            </span>
          {/if}
        </div>
      </article>
    {/each}
  </div>

  {#if hasTie()}
    <div class="tie">
      Tie detected — tie-break roll comes next.
    </div>
  {:else}
    <div class="success">
      ✓ Turn order ready
    </div>
  {/if}

  <button
    class="leave"
    onclick={onLeave}
  >
    Leave Match
  </button>
</section>

<style>
  .ranking-panel {
    width:
      min(100%, 540px);

    margin: 0 auto;

    padding: 2rem;

    border:
      1px solid
      rgba(255, 255, 255, 0.1);

    border-radius: 26px;

    background:
      linear-gradient(
        145deg,
        rgba(255, 255, 255, 0.075),
        rgba(255, 255, 255, 0.025)
      );

    box-shadow:
      0 35px 90px
      rgba(0, 0, 0, 0.3);
  }

  .panel-top {
    display: flex;

    justify-content:
      space-between;

    align-items: center;
  }

  .alpha {
    padding:
      0.35rem 0.65rem;

    border-radius: 999px;

    color: #d8b4fe;

    background:
      rgba(168, 85, 247, 0.15);

    font-size: 0.68rem;
    font-weight: 900;
  }

  .online {
    color: #86efac;

    font-size: 0.68rem;
    font-weight: 900;
  }

  .eyebrow {
    margin: 1.5rem 0 0;

    opacity: 0.4;

    font-size: 0.7rem;
    font-weight: 900;

    letter-spacing: 0.16em;
  }

  h1 {
    margin: 0.35rem 0;

    font-size: 2rem;
  }

  .description {
    margin-top: 0;

    opacity: 0.55;
  }

  .ranking-list {
    display: grid;

    gap: 0.7rem;

    margin-top: 1.5rem;
  }

  .ranking-row {
    display: flex;

    align-items: center;

    gap: 0.85rem;

    padding: 0.9rem;

    border:
      1px solid
      rgba(255, 255, 255, 0.07);

    border-radius: 15px;

    background:
      rgba(255, 255, 255, 0.035);
  }

  .ranking-row.me {
    border-color:
      rgba(168, 85, 247, 0.4);
  }

  .position {
    width: 42px;
    height: 42px;

    display: grid;
    place-items: center;

    flex: 0 0 auto;

    border-radius: 13px;

    color: #d8b4fe;

    background:
      rgba(168, 85, 247, 0.12);

    font-size: 1.15rem;
    font-weight: 900;
  }

  .player {
    display: grid;

    flex: 1;
  }

  .player span {
    margin-top: 0.15rem;

    font-size: 0.68rem;

    opacity: 0.45;
  }

  .score {
    display: grid;

    justify-items: end;
  }

  .score strong {
    font-size: 1.4rem;
  }

  .score span {
    color: #fbbf24;

    font-size: 0.55rem;
    font-weight: 900;

    letter-spacing: 0.08em;
  }

  .success,
  .tie {
    margin-top: 1rem;

    padding: 0.85rem;

    border-radius: 12px;

    text-align: center;

    font-size: 0.8rem;
    font-weight: 800;
  }

  .success {
    color: #86efac;

    background:
      rgba(34, 197, 94, 0.08);
  }

  .tie {
    color: #fbbf24;

    background:
      rgba(245, 158, 11, 0.08);
  }

  .leave {
    width: 100%;

    margin-top: 1rem;
    padding: 0.9rem;

    border:
      1px solid
      rgba(248, 113, 113, 0.3);

    border-radius: 14px;

    color: #fca5a5;

    background:
      rgba(239, 68, 68, 0.08);

    cursor: pointer;

    font: inherit;
    font-weight: 800;
  }
</style>