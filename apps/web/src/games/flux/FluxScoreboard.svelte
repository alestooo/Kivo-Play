<script lang="ts">
  import type {
    FluxScoreEntry
  } from './types';

  export let scores:
    FluxScoreEntry[] = [];

  export let playerId = '';
</script>

<div class="scores">
  {#each scores as score}
    <article
      class:me={
        score.playerId ===
        playerId
      }
    >
      <span>
        #{score.position}
      </span>

      <strong>
        {score.name}
      </strong>

      <div>
        <b>
          {score.score}
        </b>

        <small>/100</small>
      </div>
    </article>
  {/each}
</div>

<style>
  .scores {
    display: grid;
    gap: .38rem;
  }

  article {
    display: grid;
    grid-template-columns:
      30px 1fr auto;
    align-items: center;
    gap: .5rem;
    padding:
      .48rem .58rem;
    border:
      1px solid
      rgba(255,255,255,.06);
    border-radius: 10px;
    background:
      rgba(255,255,255,.025);
  }

  article.me {
    border-color:
      rgba(250,204,21,.25);
    background:
      rgba(250,204,21,.05);
  }

  span {
    color: #fde68a;
    font-size: .62rem;
    font-weight: 950;
  }

  strong {
    font-size: .68rem;
  }

  div {
    display: flex;
    align-items: baseline;
    gap: .12rem;
  }

  b {
    color: #fff3bd;
  }

  small {
    color:
      rgba(255,255,255,.34);
    font-size: .48rem;
  }
</style>
