<script lang="ts">
  export let text = '';
  export let tone:
    | 'good'
    | 'bad'
    | 'neutral' = 'neutral';
</script>

{#if text}
  {#key text}
    <span class={`floating ${tone}`}>
      {text}
    </span>
  {/key}
{/if}

<style>
  .floating {
    display: inline-block;
    color: #fff;
    font-weight: 950;
    pointer-events: none;
    animation: float 800ms ease-out forwards;
  }

  .good { color: #86efac; }
  .bad { color: #fca5a5; }
  .neutral { color: #fde68a; }

  @keyframes float {
    from {
      opacity: 0;
      transform: translateY(10px) scale(.8);
    }
    25% { opacity: 1; }
    to {
      opacity: 0;
      transform: translateY(-30px) scale(1.08);
    }
  }
</style>
