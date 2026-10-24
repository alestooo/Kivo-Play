<script lang="ts">
  export let visible = false;
  export let enabled = false;
  export let targetName = '';
  export let isSelfTarget = false;
  export let onClash:
    (() => void) | undefined;
</script>

{#if visible}
  <div class="clash-wrap">
    <button
      type="button"
      class:self={isSelfTarget}
      disabled={!enabled}
      onclick={() =>
        onClash?.()}
    >
      <span>
        CLASH!
      </span>

      <small>
        {#if isSelfTarget}
          CALL IT NOW
        {:else if enabled}
          CATCH {targetName.toUpperCase()}
        {:else}
          GET READY...
        {/if}
      </small>
    </button>
  </div>
{/if}

<style>
  .clash-wrap {
    position: absolute;
    z-index: 25;
    right: 18px;
    bottom: 18px;
  }

  button {
    min-width: 116px;
    display: grid;
    gap: .05rem;
    padding:
      .72rem .85rem;
    border:
      2px solid
      rgba(255,255,255,.88);
    border-radius: 16px;
    color: #fff;
    background:
      linear-gradient(
        145deg,
        #ef4444,
        #be123c
      );
    box-shadow:
      0 9px 0 #701a2a,
      0 15px 24px
      rgba(70,10,20,.3);
    cursor: pointer;
    transform:
      translateY(-4px);
    animation:
      clash-pulse
      650ms ease-in-out
      infinite alternate;
  }

  button.self {
    background:
      linear-gradient(
        145deg,
        #facc15,
        #f97316
      );
    box-shadow:
      0 9px 0 #92400e,
      0 15px 24px
      rgba(80,38,5,.3);
  }

  button:disabled {
    cursor: not-allowed;
    filter: grayscale(.5);
    opacity: .46;
    animation: none;
    transform: none;
    box-shadow:
      0 5px 0 #4b2c1b;
  }

  span {
    font-size: 1.15rem;
    font-weight: 1000;
    letter-spacing: -.03em;
  }

  small {
    font-size: .43rem;
    font-weight: 950;
    letter-spacing: .08em;
  }

  @keyframes clash-pulse {
    from {
      transform:
        translateY(-4px)
        scale(1);
    }

    to {
      transform:
        translateY(-7px)
        scale(1.04);
    }
  }
</style>
