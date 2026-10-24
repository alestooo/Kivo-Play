<script lang="ts">
  export let text = 'COMBO';
  export let value = 2;
  export let active = false;
</script>

{#if active}
  {#key `${text}-${value}`}
    <div class="combo-popup">
      <small>{text}</small>
      <strong>x{value}</strong>
    </div>
  {/key}
{/if}

<style>
  .combo-popup {
    display: grid;
    justify-items: center;
    pointer-events: none;
    animation: combo-in 480ms cubic-bezier(.2,.9,.2,1);
  }

  small {
    color: #fda4af;
    font-size: .6rem;
    font-weight: 950;
    letter-spacing: .14em;
  }

  strong {
    color: #fff7ed;
    font-size: 2.8rem;
    line-height: 1;
  }

  @keyframes combo-in {
    0% {
      opacity: 0;
      transform: translateY(18px) scale(.55);
    }
    65% {
      opacity: 1;
      transform: translateY(-4px) scale(1.08);
    }
    100% {
      transform: translateY(0) scale(1);
    }
  }
</style>
