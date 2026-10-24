<script lang="ts">
  export let text = '';
  export let serial = 0;
</script>

{#if text}
  {#key serial}
    <div class="impact">
      <span>
        {text}
      </span>
    </div>
  {/key}
{/if}

<style>
  .impact {
    position: absolute;
    z-index: 40;
    inset: 0;
    display: grid;
    place-items: center;
    pointer-events: none;
  }

  span {
    color: #fff7d6;
    font-size:
      clamp(
        3rem,
        10vw,
        7rem
      );
    font-weight: 1000;
    letter-spacing: -.07em;
    text-shadow:
      0 5px 0 #7c2d12,
      0 10px 35px
      rgba(0,0,0,.46);
    transform:
      rotate(-5deg);
    animation:
      impact
      900ms
      cubic-bezier(.15,.8,.2,1)
      forwards;
  }

  @keyframes impact {
    0% {
      opacity: 0;
      transform:
        scale(.25)
        rotate(-12deg);
    }

    35% {
      opacity: 1;
      transform:
        scale(1.2)
        rotate(4deg);
    }

    65% {
      transform:
        scale(.98)
        rotate(-2deg);
    }

    100% {
      opacity: 0;
      transform:
        scale(1.05)
        translateY(-28px)
        rotate(-2deg);
    }
  }
</style>
