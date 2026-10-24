<script lang="ts">
  import CardBack
    from './CardBack.svelte';

  export let name = '';
  export let cardCount = 0;
  export let active = false;
  export let position:
    | 'left'
    | 'top'
    | 'right' = 'top';

  export let variant = 0;

  $: visibleCards =
    Math.min(
      cardCount,
      8
    );
</script>

<article
  class={`seat seat-${position} avatar-${variant % 4}`}
  class:active
>
  <div class="portrait">
    <div class="hair">
      <i></i>
      <i></i>
      <i></i>
    </div>

    <div class="face">
      <span class="eye left"></span>
      <span class="eye right"></span>
      <span class="nose"></span>
      <span class="mouth"></span>
    </div>

    <div class="neck"></div>
    <div class="body"></div>
  </div>

  <div class="info">
    <strong>
      {name}
    </strong>

    <span>
      {cardCount}
      {cardCount === 1
        ? ' card'
        : ' cards'}
    </span>
  </div>

  <div class="opponent-hand">
    {#each Array(visibleCards) as _}
      <CardBack />
    {/each}

    {#if cardCount > visibleCards}
      <b>
        +{cardCount - visibleCards}
      </b>
    {/if}
  </div>
</article>

<style>
  .seat {
    position: absolute;
    z-index: 4;
    display: grid;
    align-items: center;
    gap: .42rem;
    pointer-events: none;
  }

  .seat-left {
    left: .35rem;
    top: 50%;
    grid-template-columns:
      auto 122px;
    transform:
      translateY(-50%);
  }

  .seat-top {
    top: .42rem;
    left: 50%;
    justify-items: center;
    transform:
      translateX(-50%);
  }

  .seat-right {
    right: .35rem;
    top: 50%;
    grid-template-columns:
      122px auto;
    transform:
      translateY(-50%);
  }

  .seat-right .portrait,
  .seat-right .info {
    grid-column: 2;
  }

  .seat-right .info {
    text-align: right;
  }

  .seat-right .opponent-hand {
    grid-column: 1;
    grid-row:
      1 / span 2;
    justify-self: end;
  }

  .seat-left .opponent-hand {
    grid-column: 2;
    grid-row:
      1 / span 2;
  }

  .portrait {
    position: relative;
    width: 78px;
    height: 92px;
    filter:
      drop-shadow(
        0 10px 12px
        rgba(0,0,0,.25)
      );
  }

  .face {
    position: absolute;
    z-index: 2;
    left: 17px;
    top: 10px;
    width: 44px;
    height: 49px;
    border:
      2px solid
      rgba(77,43,22,.42);
    border-radius:
      45% 45% 50% 50%;
    background: #efbd8a;
  }

  .eye {
    position: absolute;
    top: 20px;
    width: 5px;
    height: 7px;
    border-radius: 50%;
    background: #26170e;
  }

  .eye.left {
    left: 10px;
  }

  .eye.right {
    right: 10px;
  }

  .nose {
    position: absolute;
    left: 50%;
    top: 27px;
    width: 4px;
    height: 5px;
    border-radius: 50%;
    background:
      rgba(128,69,35,.35);
    transform:
      translateX(-50%);
  }

  .mouth {
    position: absolute;
    left: 50%;
    bottom: 7px;
    width: 13px;
    height: 6px;
    border-bottom:
      2px solid #7f3f2b;
    border-radius: 50%;
    transform:
      translateX(-50%);
  }

  .hair {
    position: absolute;
    z-index: 4;
    left: 11px;
    top: 1px;
    width: 57px;
    height: 31px;
    border-radius:
      55% 55% 32% 32%;
    background: #24150d;
  }

  .hair i {
    position: absolute;
    bottom: -5px;
    width: 18px;
    height: 16px;
    border-radius: 50%;
    background: inherit;
  }

  .hair i:nth-child(1) {
    left: 2px;
  }

  .hair i:nth-child(2) {
    left: 19px;
  }

  .hair i:nth-child(3) {
    right: 2px;
  }

  .neck {
    position: absolute;
    z-index: 1;
    left: 32px;
    top: 53px;
    width: 16px;
    height: 17px;
    background: #d99a67;
  }

  .body {
    position: absolute;
    left: 5px;
    bottom: 0;
    width: 70px;
    height: 42px;
    border-radius:
      50% 50% 14px 14px;
    background:
      linear-gradient(
        145deg,
        #22577a,
        #15384f
      );
  }

  .avatar-1 .hair {
    background: #351c13;
  }

  .avatar-1 .body {
    background:
      linear-gradient(
        145deg,
        #166534,
        #14532d
      );
  }

  .avatar-2 .face {
    background: #b97853;
  }

  .avatar-2 .hair {
    background: #111827;
  }

  .avatar-2 .body {
    background:
      linear-gradient(
        145deg,
        #4338ca,
        #312e81
      );
  }

  .avatar-3 .face {
    background: #f3c6a0;
  }

  .avatar-3 .hair {
    background: #6b3a20;
  }

  .avatar-3 .body {
    background:
      linear-gradient(
        145deg,
        #be123c,
        #881337
      );
  }

  .seat-left .portrait {
    transform:
      rotate(3deg)
      scaleX(-1);
  }

  .seat-right .portrait {
    transform:
      rotate(-3deg);
  }

  .active .portrait {
    filter:
      drop-shadow(
        0 0 12px
        rgba(250,204,21,.75)
      )
      drop-shadow(
        0 10px 12px
        rgba(0,0,0,.25)
      );
  }

  .info {
    min-width: 0;
    display: grid;
    gap: .04rem;
    padding:
      .34rem .5rem;
    border:
      1px solid
      rgba(255,224,133,.2);
    border-radius: 10px;
    background:
      rgba(31,21,9,.78);
    backdrop-filter:
      blur(8px);
  }

  .info strong {
    overflow: hidden;
    text-overflow:
      ellipsis;
    white-space: nowrap;
    color: #fff7dc;
    font-size: .69rem;
  }

  .info span {
    color:
      rgba(255,255,255,.5);
    font-size: .52rem;
  }

  .opponent-hand {
    display: flex;
    align-items: center;
  }

  .opponent-hand :global(.card-back) {
    margin-left: -33px;
  }

  .opponent-hand :global(.card-back:first-child) {
    margin-left: 0;
  }

  .seat-left .opponent-hand,
  .seat-right .opponent-hand {
    flex-direction: column;
  }

  .seat-left .opponent-hand :global(.card-back),
  .seat-right .opponent-hand :global(.card-back) {
    margin-top: -52px;
    margin-left: 0;
  }

  .seat-left .opponent-hand :global(.card-back:first-child),
  .seat-right .opponent-hand :global(.card-back:first-child) {
    margin-top: 0;
  }

  .opponent-hand b {
    margin-left: .35rem;
    color: #fff3bd;
    font-size: .65rem;
  }
</style>
