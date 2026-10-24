<script lang="ts">
  export let active = false;
  export let amount = 24;
</script>

{#if active}
  <div class="particles" aria-hidden="true">
    {#each Array(amount) as _, index}
      <i
        style={`
          --x:${(index * 37) % 100}%;
          --delay:${(index % 9) * 0.04}s;
          --drift:${((index % 7) - 3) * 18}px;
        `}
      ></i>
    {/each}
  </div>
{/if}

<style>
  .particles {
    position: absolute;
    inset: 0;
    overflow: hidden;
    pointer-events: none;
  }

  i {
    position: absolute;
    left: var(--x);
    bottom: 0;
    width: 5px;
    height: 5px;
    border-radius: 50%;
    background: #fde68a;
    box-shadow: 0 0 12px #f59e0b;
    animation: particle 700ms ease-out var(--delay) forwards;
  }

  @keyframes particle {
    from {
      opacity: 1;
      transform: translate(0, 0) scale(1);
    }
    to {
      opacity: 0;
      transform: translate(var(--drift), -75px) scale(.2);
    }
  }
</style>
