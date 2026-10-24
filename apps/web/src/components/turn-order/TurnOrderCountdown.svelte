<script lang="ts">
  export let text = '';
</script>

<section class="countdown-screen">
  <div class="countdown-backdrop"></div>

  {#key text}
    <div class="countdown-content">
      <span class="countdown-badge">
        KIVO PLAY · ALPHA
      </span>

      <p class="countdown-label">
        TURN ORDER
      </p>

      <strong class="countdown-value">
        {text}
      </strong>

      <span class="helper">
        Get ready to roll.
      </span>
    </div>
  {/key}
</section>

<style>
  .countdown-screen {
    position: fixed;
    inset: 0;

    z-index: 100;

    display: grid;
    place-items: center;
  }

  .countdown-backdrop {
    position: absolute;
    inset: 0;

    background:
      radial-gradient(
        circle at center,
        rgba(124, 58, 237, 0.3),
        rgba(3, 7, 18, 0.96)
      );

    backdrop-filter: blur(16px);
  }

  .countdown-content {
    position: relative;
    z-index: 1;

    display: grid;
    place-items: center;

    text-align: center;

    animation: appear 400ms ease;
  }

  .countdown-badge {
    padding: 0.4rem 0.75rem;

    border-radius: 999px;

    color: #d8b4fe;

    background:
      rgba(168, 85, 247, 0.15);

    font-size: 0.7rem;
    font-weight: 900;

    letter-spacing: 0.12em;
  }

  .countdown-label {
    margin: 2rem 0 0;

    color:
      rgba(255, 255, 255, 0.45);

    font-size: 0.78rem;
    font-weight: 900;

    letter-spacing: 0.22em;
  }

  .countdown-value {
    min-height: 150px;

    display: grid;
    place-items: center;

    font-size:
      clamp(
        5rem,
        18vw,
        9rem
      );

    line-height: 1;

    text-shadow:
      0 0 50px
      rgba(168, 85, 247, 0.4);
  }

  .helper {
    color:
      rgba(255, 255, 255, 0.5);
  }

  @keyframes appear {
    from {
      opacity: 0;

      transform:
        scale(0.75);
    }

    to {
      opacity: 1;

      transform:
        scale(1);
    }
  }

  @media (
    prefers-reduced-motion:
      reduce
  ) {
    .countdown-content {
      animation: none;
    }
  }
</style>