<script lang="ts">
  export let text = '';
</script>

<section class="countdown">
  <div class="backdrop"></div>
  <div class="arena-preview">
    <div class="arena-inner"></div>
  </div>

  {#key text}
    <div class="content">
      <span>KIVO PLAY · ALPHA</span>
      <p>DICE ARENA</p>
      <strong>{text}</strong>
      <small>Get ready to roll.</small>
    </div>
  {/key}
</section>

<style>
  .countdown {
    position: fixed;
    inset: 0;
    z-index: 700;
    display: grid;
    place-items: center;
    overflow: hidden;
  }

  .backdrop {
    position: absolute;
    inset: 0;
    background:
      radial-gradient(circle at center, rgba(251, 146, 60, 0.16), transparent 35%),
      rgba(6, 9, 14, 0.95);
    backdrop-filter: blur(18px);
  }

  .arena-preview {
    position: absolute;
    width: min(620px, 80vw);
    aspect-ratio: 1;
    opacity: 0.2;
    clip-path: polygon(
      29% 0, 71% 0, 100% 29%, 100% 71%,
      71% 100%, 29% 100%, 0 71%, 0 29%
    );
    background: linear-gradient(145deg, #d6c3a6, #90785e);
    animation: arena-enter 900ms ease;
  }

  .arena-inner {
    position: absolute;
    inset: 28px;
    clip-path: inherit;
    background: #063a2d;
  }

  .content {
    position: relative;
    z-index: 2;
    display: grid;
    place-items: center;
    text-align: center;
    animation: number-enter 350ms ease;
  }

  .content > span {
    padding: 0.4rem 0.75rem;
    border-radius: 999px;
    color: #fde68a;
    background: rgba(245, 158, 11, 0.1);
    font-size: 0.66rem;
    font-weight: 900;
    letter-spacing: 0.13em;
  }

  p {
    margin: 2rem 0 0;
    color: rgba(255, 255, 255, 0.4);
    font-size: 0.72rem;
    font-weight: 900;
    letter-spacing: 0.2em;
  }

  strong {
    min-height: 145px;
    display: grid;
    place-items: center;
    color: #fff7ed;
    font-size: clamp(5rem, 18vw, 9rem);
    line-height: 1;
    text-shadow: 0 0 60px rgba(251, 146, 60, 0.32);
  }

  small {
    color: rgba(255, 255, 255, 0.48);
  }

  @keyframes number-enter {
    from {
      opacity: 0;
      transform: scale(0.55) rotate(-5deg);
    }
    to {
      opacity: 1;
      transform: scale(1) rotate(0);
    }
  }

  @keyframes arena-enter {
    from {
      opacity: 0;
      transform: scale(0.7) rotate(-8deg);
    }
    to {
      opacity: 0.2;
      transform: scale(1) rotate(0);
    }
  }

  @media (prefers-reduced-motion: reduce) {
    .content,
    .arena-preview {
      animation: none;
    }
  }
</style>
