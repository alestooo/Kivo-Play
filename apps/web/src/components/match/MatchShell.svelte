<script lang="ts">
  import FluxGame
    from '@/games/flux/FluxGame.svelte';

  import type {
    GameId
  } from '@/games/types';

  import type {
    ClashColor,
    FluxCard,
    FluxMatchState,
    FluxRoundResult,
    FluxStealOffer
  } from '@/games/flux/types';

  export let gameId:
    GameId;

  export let playerId = '';
  export let hostId = '';

  export let fluxState:
    FluxMatchState | null =
      null;

  export let fluxPlayableCardIds:
    string[] = [];

  export let fluxStealOffer:
    FluxStealOffer | null =
      null;

  export let fluxRoundResult:
    FluxRoundResult | null =
      null;

  export let fluxRoundReadyPlayerIds:
    string[] = [];

  export let fluxRoundAllReady =
    false;

  export let fluxImpactText = '';
  export let fluxImpactSerial = 0;

  export let onFluxPlay:
    ((
      card:
        FluxCard,

      color?:
        ClashColor,

      targetPlayerId?:
        string
    ) => void) | undefined;

  export let onFluxDraw:
    (() => void) | undefined;

  export let onFluxBeginSteal:
    ((
      card:
        FluxCard,

      targetPlayerId:
        string
    ) => void) | undefined;

  export let onFluxChooseSteal:
    ((cardId: string) => void) | undefined;

  export let onFluxClash:
    (() => void) | undefined;

  export let onFluxToggleRoundReady:
    (() => void) | undefined;

  export let onFluxNextRound:
    (() => void) | undefined;

  export let onFluxPlayAgain:
    (() => void) | undefined;

  export let onLeave:
    (() => void) | undefined;
</script>

{#if gameId === 'flux' && fluxState}
  <FluxGame
    {playerId}
    {hostId}
    state={fluxState}
    playableCardIds={
      fluxPlayableCardIds
    }
    stealOffer={
      fluxStealOffer
    }
    roundResult={
      fluxRoundResult
    }
    roundReadyPlayerIds={
      fluxRoundReadyPlayerIds
    }
    roundAllReady={
      fluxRoundAllReady
    }
    impactText={
      fluxImpactText
    }
    impactSerial={
      fluxImpactSerial
    }
    onPlayCard={
      onFluxPlay
    }
    onDraw={
      onFluxDraw
    }
    onBeginSteal={
      onFluxBeginSteal
    }
    onChooseStealCard={
      onFluxChooseSteal
    }
    onClash={
      onFluxClash
    }
    onToggleRoundReady={
      onFluxToggleRoundReady
    }
    onNextRound={
      onFluxNextRound
    }
    onPlayAgain={
      onFluxPlayAgain
    }
    {onLeave}
  />
{/if}
