<script lang="ts">
  export let active = false;
  export let tone:
    | 'gold'
    | 'red'
    | 'blue'
    | 'green' = 'gold';
</script>

{#if active}
  {#key `${tone}-${Date.now()}`}
    <div
      class={`burst tone-${tone}`}
      aria-hidden="true"
    >
      {#each Array(28) as _, index}
        <i
          style={`
            --angle:${(360 / 28) * index}deg;
            --distance:${78 + (index % 6) * 12}px;
            --delay:${(index % 5) * 18}ms;
          `}
        ></i>
      {/each}
    </div>
  {/key}
{/if}

<style>
  .burst {
    position: absolute;
    z-index: 15;
    left: 50%;
    top: 50%;
    width: 8px;
    height: 8px;
    pointer-events: none;
    transform: translate(-50%, -50%);
  }

  i {
    position: absolute;
    inset: 0;
    width: 7px;
    height: 7px;
    border-radius: 2px;
    background: #facc15;
    box-shadow: 0 0 10px rgba(250, 204, 21, 0.8);
    animation:
      fly 520ms
      cubic-bezier(.1,.75,.25,1)
      var(--delay)
      forwards;
  }

  .tone-red i {
    background: #fb7185;
  }

  .tone-blue i {
    background: #38bdf8;
  }

  .tone-green i {
    background: #4ade80;
  }

  @keyframes fly {
    from {
      opacity: 1;
      transform:
        rotate(var(--angle))
        translateX(0)
        scale(1.4);
    }

    to {
      opacity: 0;
      transform:
        rotate(var(--angle))
        translateX(var(--distance))
        scale(0.2);
    }
  }
</style>
