<script lang="ts">
  import type {
    TurnOrderPlayer
  } from '@/lib/networking/messages';

  export let playerId = '';

  export let players:
    TurnOrderPlayer[] = [];

  export let rollingDice:
    Set<number> =
      new Set<number>();

  export let onRollDie:
    (index: number) => void;

  export let onLeave:
    () => void;

  function currentPlayer():
    TurnOrderPlayer | undefined {
    return players.find(
      (player) =>
        player.id === playerId
    );
  }

  function dieFace(
    value: number | null
  ): string {
    switch (value) {
      case 1:
        return '⚀';

      case 2:
        return '⚁';

      case 3:
        return '⚂';

      case 4:
        return '⚃';

      case 5:
        return '⚄';

      case 6:
        return '⚅';

      default:
        return '?';
    }
  }

  function completedDice(
    player: TurnOrderPlayer
  ): number {
    return player.rolls.filter(
      (roll: number | null) =>
        roll !== null
    ).length;
  }
</script>

<section class="game-panel">
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
    Roll your dice
  </h1>

  <p class="description">
    Roll all three dice. The highest total starts the match.
  </p>

  {#if currentPlayer()}
    <div class="dice-grid">
      {#each currentPlayer()!.rolls as value, index}
        <button
          class="die"
          class:complete={value !== null}
          class:rolling={rollingDice.has(index)}
          disabled={
            value !== null ||
            rollingDice.has(index)
          }
          onclick={() =>
            onRollDie(index)}
        >
          <span class="die-face">
            {dieFace(value)}
          </span>

          <span class="die-status">
            {#if value === null}
              ROLL
            {:else}
              ✓ {value}
            {/if}
          </span>
        </button>
      {/each}
    </div>

    <div class="total-box">
      <span>
        Your total
      </span>

      <strong>
        {currentPlayer()!.total}
      </strong>
    </div>

    {#if currentPlayer()!.complete}
      <div class="ready-banner">
        ✓ READY
      </div>
    {/if}
  {/if}

  <div class="section-title">
    Players
  </div>

  <div class="player-list">
    {#each players as player}
      <article
        class="player-row"
        class:me={
          player.id === playerId
        }
        class:complete={
          player.complete
        }
      >
        <div class="avatar">
          {player.name
            .charAt(0)
            .toUpperCase()}
        </div>

        <div class="player-info">
          <strong>
            {player.name}
          </strong>

          <div class="mini-dice">
            {#each player.rolls as value}
              <span
                class:complete={
                  value !== null
                }
              >
                {dieFace(value)}
              </span>
            {/each}
          </div>
        </div>

        <div class="player-progress">
          {#if player.complete}
            <strong>
              {player.total}
            </strong>

            <span>
              READY
            </span>
          {:else}
            <span>
              {completedDice(player)}
              / 3
            </span>
          {/if}
        </div>
      </article>
    {/each}
  </div>

  <button
    class="leave"
    onclick={onLeave}
  >
    Leave Match
  </button>
</section>

<style>
  .game-panel {
    width: min(100%, 560px);

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

    backdrop-filter: blur(20px);
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

    color:
      rgba(255, 255, 255, 0.4);

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

    color:
      rgba(255, 255, 255, 0.55);
  }

  .dice-grid {
    display: grid;

    grid-template-columns:
      repeat(3, 1fr);

    gap: 0.8rem;

    margin-top: 2rem;
  }

  .die {
    min-height: 140px;

    display: grid;
    place-items: center;

    gap: 0.3rem;

    padding: 1rem;

    border:
      1px solid
      rgba(255, 255, 255, 0.1);

    border-radius: 20px;

    color: white;

    background:
      rgba(255, 255, 255, 0.04);

    cursor: pointer;

    transition:
      transform 160ms ease,
      background 160ms ease,
      border-color 160ms ease;
  }

  .die:hover:not(:disabled) {
    transform:
      translateY(-4px);

    border-color:
      rgba(168, 85, 247, 0.5);

    background:
      rgba(168, 85, 247, 0.08);
  }

  .die.complete {
    border-color:
      rgba(74, 222, 128, 0.4);

    background:
      rgba(34, 197, 94, 0.08);
  }

  .die.rolling {
    animation:
      roll 400ms linear infinite;
  }

  .die-face {
    font-size: 3.5rem;

    line-height: 1;
  }

  .die-status {
    font-size: 0.65rem;
    font-weight: 900;

    letter-spacing: 0.1em;

    opacity: 0.65;
  }

  .total-box {
    display: flex;

    justify-content:
      space-between;

    align-items: center;

    margin-top: 1rem;

    padding: 1rem;

    border-radius: 14px;

    background:
      rgba(255, 255, 255, 0.04);
  }

  .total-box span {
    opacity: 0.55;
  }

  .total-box strong {
    font-size: 1.5rem;
  }

  .ready-banner {
    margin-top: 0.75rem;

    padding: 0.8rem;

    border-radius: 12px;

    color: #86efac;

    background:
      rgba(34, 197, 94, 0.08);

    text-align: center;

    font-size: 0.8rem;
    font-weight: 900;
  }

  .section-title {
    margin-top: 2rem;

    padding-bottom: 0.7rem;

    border-bottom:
      1px solid
      rgba(255, 255, 255, 0.08);

    font-size: 0.8rem;
    font-weight: 800;

    opacity: 0.6;
  }

  .player-list {
    display: grid;

    gap: 0.7rem;

    margin-top: 0.8rem;
  }

  .player-row {
    display: flex;

    align-items: center;

    gap: 0.8rem;

    padding: 0.85rem;

    border:
      1px solid
      rgba(255, 255, 255, 0.07);

    border-radius: 14px;

    background:
      rgba(255, 255, 255, 0.03);
  }

  .player-row.me {
    border-color:
      rgba(168, 85, 247, 0.35);
  }

  .player-row.complete {
    background:
      rgba(34, 197, 94, 0.04);
  }

  .avatar {
    width: 42px;
    height: 42px;

    display: grid;
    place-items: center;

    flex: 0 0 auto;

    border-radius: 13px;

    background:
      linear-gradient(
        135deg,
        #7c3aed,
        #4f46e5
      );

    font-weight: 900;
  }

  .player-info {
    display: grid;

    flex: 1;
  }

  .mini-dice {
    display: flex;

    gap: 0.3rem;

    margin-top: 0.35rem;
  }

  .mini-dice span {
    width: 27px;
    height: 27px;

    display: grid;
    place-items: center;

    border-radius: 7px;

    background:
      rgba(255, 255, 255, 0.05);

    opacity: 0.4;
  }

  .mini-dice span.complete {
    color: #86efac;

    background:
      rgba(34, 197, 94, 0.1);

    opacity: 1;
  }

  .player-progress {
    display: grid;

    justify-items: end;
  }

  .player-progress span {
    color: #86efac;

    font-size: 0.65rem;
    font-weight: 900;
  }

  .leave {
    width: 100%;

    margin-top: 1.2rem;
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

  @keyframes roll {
    from {
      transform:
        rotate(0deg)
        scale(1);
    }

    50% {
      transform:
        rotate(180deg)
        scale(1.08);
    }

    to {
      transform:
        rotate(360deg)
        scale(1);
    }
  }

  @media (max-width: 520px) {
    .game-panel {
      padding: 1.3rem;
    }

    .dice-grid {
      gap: 0.45rem;
    }

    .die {
      min-height: 110px;

      padding: 0.5rem;
    }

    .die-face {
      font-size: 2.7rem;
    }
  }
</style>