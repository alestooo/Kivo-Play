<div class="celebration" aria-hidden="true">
  {#each Array(100) as _, index}
    <i
      class="confetti"
      style={`
        --x:${(index * 37) % 100}vw;
        --delay:${(index % 16) * 0.055}s;
        --duration:${2 + (index % 8) * 0.16}s;
        --drift:${((index % 9) - 4) * 24}px;
        --rotate:${(index * 61) % 360}deg;
      `}
    ></i>
  {/each}

  {#each Array(42) as _, index}
    <i
      class="spark"
      style={`
        --x:${(index * 53) % 100}vw;
        --y:${15 + ((index * 29) % 70)}vh;
        --delay:${(index % 10) * 0.08}s;
      `}
    ></i>
  {/each}
</div>

<style>
  .celebration {
    position: fixed;
    inset: 0;
    z-index: 2000;
    overflow: hidden;
    pointer-events: none;
  }

  .confetti {
    position: absolute;
    top: -30px;
    left: var(--x);
    width: 10px;
    height: 20px;
    border-radius: 2px;
    animation:
      confetti-fall var(--duration)
      cubic-bezier(0.2, 0.7, 0.2, 1)
      var(--delay) infinite;
  }

  .confetti:nth-child(5n + 1) { background: #fbbf24; }
  .confetti:nth-child(5n + 2) { background: #22c55e; }
  .confetti:nth-child(5n + 3) { background: #38bdf8; }
  .confetti:nth-child(5n + 4) { background: #fb7185; }
  .confetti:nth-child(5n + 5) { background: #c084fc; }

  .spark {
    position: absolute;
    left: var(--x);
    top: var(--y);
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: #fde68a;
    box-shadow: 0 0 16px #fbbf24;
    animation: sparkle 900ms ease-in-out var(--delay) infinite alternate;
  }

  @keyframes confetti-fall {
    0% {
      opacity: 0;
      transform: translate3d(0, -10vh, 0) rotate(var(--rotate));
    }
    8% { opacity: 1; }
    100% {
      opacity: 1;
      transform: translate3d(var(--drift), 115vh, 0) rotate(1080deg);
    }
  }

  @keyframes sparkle {
    from {
      opacity: 0.2;
      transform: scale(0.3);
    }
    to {
      opacity: 1;
      transform: scale(1.8);
    }
  }

  @media (prefers-reduced-motion: reduce) {
    .confetti,
    .spark {
      animation: none;
      display: none;
    }
  }
</style>
