<script lang="ts">
  import type {
    ClashColor
  } from './types';

  export let open = false;
  export let onChoose:
    ((color: ClashColor) => void) | undefined;

  const colors:
    Array<{
      id: ClashColor;
      label: string;
    }> = [
      {
        id: 'red',
        label: 'Red'
      },
      {
        id: 'blue',
        label: 'Blue'
      },
      {
        id: 'green',
        label: 'Green'
      },
      {
        id: 'yellow',
        label: 'Yellow'
      }
    ];
</script>

{#if open}
  <div class="layer">
    <div class="panel">
      <span>CHOOSE COLOR</span>

      <strong>
        What color comes next?
      </strong>

      <div class="colors">
        {#each colors as color}
          <button
            type="button"
            class={`color-${color.id}`}
            aria-label={color.label}
            onclick={() =>
              onChoose?.(
                color.id
              )}
          >
            {color.label}
          </button>
        {/each}
      </div>
    </div>
  </div>
{/if}

<style>
  .layer {
    position: absolute;
    inset: 0;
    z-index: 30;
    display: grid;
    place-items: center;
    border-radius: 28px;
    background: rgba(35, 22, 7, 0.45);
    backdrop-filter: blur(7px);
  }

  .panel {
    width: min(360px, calc(100% - 2rem));
    display: grid;
    gap: 0.85rem;
    padding: 1rem;
    border: 1px solid rgba(255, 238, 182, 0.25);
    border-radius: 18px;
    background: #2b1b08;
    box-shadow: 0 24px 60px rgba(0, 0, 0, 0.34);
    text-align: center;
  }

  span {
    color: #facc15;
    font-size: 0.55rem;
    font-weight: 950;
    letter-spacing: 0.14em;
  }

  strong {
    color: #fff7d6;
  }

  .colors {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 0.6rem;
  }

  button {
    min-height: 72px;
    border: 3px solid rgba(255, 255, 255, 0.8);
    border-radius: 14px;
    color: white;
    cursor: pointer;
    font-weight: 950;
    box-shadow: inset 0 -12px 20px rgba(0, 0, 0, 0.16);
  }

  .color-red {
    background: #ef4444;
  }

  .color-blue {
    background: #0ea5e9;
  }

  .color-green {
    background: #22c55e;
  }

  .color-yellow {
    color: #3c2807;
    background: #facc15;
  }
</style>
