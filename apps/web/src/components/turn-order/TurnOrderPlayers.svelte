<script lang="ts">
  import type { TurnOrderPlayer } from '@/lib/networking/messages';

  export let playerId = '';
  export let players: TurnOrderPlayer[] = [];

  const COLORS = ['#fb923c', '#38bdf8', '#34d399', '#fb7185'];

  function playerColor(index: number): string {
    return COLORS[index % COLORS.length];
  }

  function ownValue(
    player: TurnOrderPlayer,
    index: number
  ): number | null {
    if (player.id !== playerId) {
      return null;
    }

    return player.rolls[index] ?? null;
  }
</script>

<div class="players-list">
  {#each players as player, playerIndex}
    <article class:self={player.id === playerId}>
      <div
        class="avatar"
        style={`--player-color:${playerColor(playerIndex)}`}
      >
        {player.name.charAt(0).toUpperCase()}
      </div>

      <div class="identity">
        <strong>
          {player.name}
          {#if player.id === playerId}
            <span>(You)</span>
          {/if}
        </strong>

        <small>
          {player.id === playerId
            ? 'Your results'
            : 'Opponent results hidden'}
        </small>
      </div>

      <div class="dice-status">
        {#each [0, 1, 2] as index}
          {@const value = ownValue(player, index)}

          <div
            class:rolled={player.rolled[index]}
            class:own={player.id === playerId && value !== null}
          >
            {#if player.id === playerId}
              {#if value !== null}
                {value}
              {/if}
            {:else if player.rolled[index]}
              ?
            {/if}
          </div>
        {/each}
      </div>

      <b>{player.roll_count}/3</b>
    </article>
  {/each}
</div>

<style>
  .players-list {
    display: grid;
    gap: 0.38rem;
  }

  article {
    min-height: 54px;
    display: grid;
    grid-template-columns: auto minmax(0, 1fr) auto 34px;
    align-items: center;
    gap: 0.6rem;
    padding: 0.55rem 0.65rem;
    border: 1px solid rgba(255, 255, 255, 0.07);
    border-radius: 12px;
    background: rgba(255, 255, 255, 0.024);
  }

  article.self {
    border-color: rgba(251, 191, 36, 0.34);
    box-shadow: inset 3px 0 0 #fbbf24;
  }

  .avatar {
    width: 35px;
    height: 35px;
    display: grid;
    place-items: center;
    border-radius: 10px;
    color: white;
    background: var(--player-color);
    font-size: 0.76rem;
    font-weight: 950;
  }

  .identity {
    min-width: 0;
    display: grid;
    gap: 0.02rem;
  }

  .identity strong {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    font-size: 0.82rem;
  }

  .identity strong span {
    color: rgba(255, 255, 255, 0.48);
  }

  .identity small {
    color: rgba(255, 255, 255, 0.34);
    font-size: 0.58rem;
  }

  .dice-status {
    display: flex;
    gap: 0.28rem;
  }

  .dice-status div {
    width: 31px;
    height: 31px;
    display: grid;
    place-items: center;
    border: 1px solid rgba(255, 255, 255, 0.09);
    border-radius: 8px;
    color: rgba(255, 255, 255, 0.5);
    background: rgba(255, 255, 255, 0.035);
    font-size: 0.7rem;
    font-weight: 950;
  }

  .dice-status div.rolled {
    color: #bbf7d0;
    border-color: rgba(52, 211, 153, 0.48);
    background: rgba(16, 185, 129, 0.13);
  }

  .dice-status div.own {
    color: #271503;
    border-color: #fbbf24;
    background: linear-gradient(135deg, #fde68a, #fbbf24);
  }

  article > b {
    color: #86efac;
    font-size: 0.62rem;
    text-align: right;
  }

  @media (max-width: 620px) {
    article {
      grid-template-columns: auto 1fr auto;
    }

    .dice-status {
      grid-column: 1 / -1;
      padding-left: 2.8rem;
    }
  }
</style>
