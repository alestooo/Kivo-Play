<script lang="ts">
  export let active = false;
  export let intensity: 1 | 2 | 3 = 1;
</script>

{#if active}
  <div
    class={`flash intensity-${intensity}`}
    aria-hidden="true"
  ></div>
{/if}

<style>
  .flash {
    position: fixed;
    inset: 0;
    z-index: 1400;
    pointer-events: none;
    background: rgba(255,255,255,.18);
    animation: flash 180ms ease-out forwards;
  }

  .intensity-2 {
    background: rgba(251,191,36,.24);
  }

  .intensity-3 {
    background: rgba(255,255,255,.34);
  }

  @keyframes flash {
    from { opacity: 1; }
    to { opacity: 0; }
  }
</style>
