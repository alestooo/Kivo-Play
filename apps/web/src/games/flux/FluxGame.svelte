<script lang="ts">
  import {
    onDestroy
  } from 'svelte';

  import {
    CLASH_RIVAL_DELAY_MS
  } from './config';

  import ClashButton
    from './ClashButton.svelte';

  import ColorPicker
    from './ColorPicker.svelte';

  import FluxBoard
    from './FluxBoard.svelte';

  import FluxHand
    from './FluxHand.svelte';

  import FluxResultOverlay
    from './FluxResultOverlay.svelte';

  import ImpactText
    from './ImpactText.svelte';

  import StealChoicePicker
    from './StealChoicePicker.svelte';

  import StealTargetPicker
    from './StealTargetPicker.svelte';

  import TargetPicker
    from './TargetPicker.svelte';

  import type {
    ClashColor,
    FluxCard,
    FluxMatchState,
    FluxRoundResult,
    FluxStealOffer
  } from './types';

  export let playerId = '';
  export let hostId = '';

  export let state: FluxMatchState;
  export let playableCardIds: string[] = [];
  export let stealOffer: FluxStealOffer | null = null;
  export let roundResult: FluxRoundResult | null = null;
  export let roundReadyPlayerIds: string[] = [];
  export let roundAllReady = false;
  export let impactText = '';
  export let impactSerial = 0;

  export let onPlayCard:
    ((card: FluxCard, color?: ClashColor, targetPlayerId?: string) => void)
      | undefined;

  export let onDraw: (() => void) | undefined;
  export let onBeginSteal:
    ((card: FluxCard, targetPlayerId: string) => void)
      | undefined;
  export let onChooseStealCard: ((cardId: string) => void) | undefined;
  export let onClash: (() => void) | undefined;
  export let onToggleRoundReady: (() => void) | undefined;
  export let onNextRound: (() => void) | undefined;
  export let onPlayAgain: (() => void) | undefined;
  export let onLeave: (() => void) | undefined;

  let pendingCard: FluxCard | null = null;
  let chooseColor = false;
  let chooseTarget = false;
  let showStealTargetPicker = false;
  let burst = false;
  let rivalClashReady = false;
  let lastClashSerial = '';

  let shownRoundResult: FluxRoundResult | null = null;
  let roundResultKey = '';
  let roundResultTimer: ReturnType<typeof setTimeout> | null = null;

  function clearRoundTimer(): void {
    if (roundResultTimer) {
      clearTimeout(roundResultTimer);
      roundResultTimer = null;
    }
  }

  onDestroy(() => {
    clearRoundTimer();
  });

  $: if (
    state.clashWindow?.serial !==
    lastClashSerial
  ) {
    lastClashSerial = state.clashWindow?.serial ?? '';
    rivalClashReady = false;

    if (
      state.clashWindow &&
      state.clashWindow.targetPlayerId !== playerId
    ) {
      window.setTimeout(() => {
        rivalClashReady = true;
      }, CLASH_RIVAL_DELAY_MS);
    }
  }

  $: {
    const nextKey = roundResult
      ? `${roundResult.round}-${roundResult.winnerId}-${roundResult.pointsAwarded}-${roundResult.matchOver}`
      : '';

    if (nextKey !== roundResultKey) {
      roundResultKey = nextKey;
      clearRoundTimer();
      shownRoundResult = null;

      if (roundResult) {
        roundResultTimer = setTimeout(() => {
          shownRoundResult = roundResult;
        }, 3000);
      }
    }

    if (!roundResult) {
      shownRoundResult = null;
    }
  }

  $: clashVisible = Boolean(state.clashWindow);
  $: isClashTarget = state.clashWindow?.targetPlayerId === playerId;
  $: clashEnabled = clashVisible && (isClashTarget || rivalClashReady);

  function selectCard(card: FluxCard): void {
    if (card.special === 'wild' || card.special === 'draw_four') {
      pendingCard = card;
      chooseColor = true;
      return;
    }

    if (card.special === 'draw_six') {
      pendingCard = card;
      chooseTarget = true;
      return;
    }

    if (card.special === 'steal') {
      pendingCard = card;
      showStealTargetPicker = true;
      return;
    }

    submitCard(card);
  }

  function chooseCardColor(color: ClashColor): void {
    if (!pendingCard) return;
    const card = pendingCard;
    pendingCard = null;
    chooseColor = false;
    submitCard(card, color);
  }

  function chooseCardTarget(targetPlayerId: string): void {
    if (!pendingCard) return;
    const card = pendingCard;
    pendingCard = null;
    chooseTarget = false;
    submitCard(card, undefined, targetPlayerId);
  }

  function chooseStealTarget(targetPlayerId: string): void {
    if (!pendingCard) return;
    const card = pendingCard;
    pendingCard = null;
    showStealTargetPicker = false;
    onBeginSteal?.(card, targetPlayerId);
  }

  function submitCard(card: FluxCard, color?: ClashColor, targetPlayerId?: string): void {
    burst = false;
    queueMicrotask(() => {
      burst = true;
    });
    onPlayCard?.(card, color, targetPlayerId);
  }
</script>

<section class="clash-game">
  <header>
    <div>
      <span>FLUX CLASH · ALPHA</span>
      <h1>Match. Stack. Attack.</h1>
      <p>First to 100 points wins the match.</p>
    </div>

    <div class="header-score">
      <small>ROUND</small>
      <strong>{state.round}</strong>
    </div>

    <button class="leave" type="button" onclick={() => onLeave?.()}>
      Leave
    </button>
  </header>

  <div class="game-stage">
    <FluxBoard {state} localPlayerId={playerId} {burst} {onDraw} />

    <FluxHand
      cards={state.privateState.hand}
      {playableCardIds}
      onPlay={selectCard}
    />

    <ClashButton
      visible={clashVisible}
      enabled={clashEnabled}
      targetName={state.clashWindow?.targetPlayerName ?? ''}
      isSelfTarget={isClashTarget}
      {onClash}
    />

    <ImpactText text={impactText} serial={impactSerial} />

    <ColorPicker open={chooseColor} onChoose={chooseCardColor} />

    <TargetPicker
      open={chooseTarget}
      {playerId}
      players={state.players}
      onChoose={chooseCardTarget}
    />

    <StealTargetPicker
      open={showStealTargetPicker}
      {playerId}
      players={state.players}
      onChoose={chooseStealTarget}
    />

    <StealChoicePicker offer={stealOffer} onChoose={onChooseStealCard} />

    <FluxResultOverlay
      result={shownRoundResult}
      {playerId}
      isHost={playerId === hostId}
      isReady={roundReadyPlayerIds.includes(playerId)}
      allReady={roundAllReady}
      readyCount={roundReadyPlayerIds.length}
      playerCount={state.players.length}
      onToggleReady={onToggleRoundReady}
      {onNextRound}
      {onPlayAgain}
      {onLeave}
    />
  </div>
</section>

<style>
  .clash-game {
    width: min(calc(100vw - 1.5rem), 1080px);
    margin: 0 auto;
    display: grid;
    gap: .65rem;
  }

  header {
    display: grid;
    grid-template-columns: 1fr auto auto;
    align-items: center;
    gap: .8rem;
    padding: .65rem .8rem;
    border: 1px solid rgba(250,204,21,.1);
    border-radius: 15px;
    background: rgba(32,20,7,.82);
  }

  header span {
    color: #facc15;
    font-size: .54rem;
    font-weight: 950;
    letter-spacing: .14em;
  }

  h1 {
    margin: .12rem 0;
    color: #fff7d6;
    font-size: 1.45rem;
    letter-spacing: -.035em;
  }

  p {
    margin: 0;
    color: rgba(255,255,255,.42);
    font-size: .64rem;
  }

  .header-score {
    display: grid;
    justify-items: center;
    padding: .35rem .55rem;
    border-radius: 10px;
    background: rgba(250,204,21,.08);
  }

  .header-score small {
    color: rgba(255,255,255,.4);
    font-size: .42rem;
    font-weight: 950;
  }

  .header-score strong {
    color: #fde68a;
  }

  .leave {
    padding: .48rem .65rem;
    border: 1px solid rgba(239,68,68,.2);
    border-radius: 9px;
    color: #fecaca;
    background: rgba(127,29,29,.18);
    cursor: pointer;
  }

  .game-stage {
    position: relative;
    padding-bottom: .35rem;
    border-radius: 28px;
  }
</style>
