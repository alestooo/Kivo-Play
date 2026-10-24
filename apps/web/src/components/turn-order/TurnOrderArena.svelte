<script lang="ts">
  import { onDestroy, onMount } from 'svelte';

  import type {
    DiePose,
    PhysicsFrame,
    TurnOrderPlayer
  } from '@/lib/networking/messages';

  export let playerId = '';
  export let players: TurnOrderPlayer[] = [];
  export let frames: PhysicsFrame[] = [];
  export let onRoll: (index: number) => void;

  let canvas: HTMLCanvasElement;
  let context: CanvasRenderingContext2D | null = null;
  let cssSize = 410;
  let animationHandle = 0;
  let currentDice: DiePose[] = [];
  let frameQueue: PhysicsFrame[][] = [];
  let activeBatch: PhysicsFrame[] = [];
  let activeFrameIndex = 0;
  let lastFrameTime = 0;
  let lastFramesReference: PhysicsFrame[] | null = null;
  let pendingRoll: number | null = null;

  const FRAME_TIME = 1000 / 30;
  const WORLD_RADIUS = 5.4;
  const COLORS = ['#fb923c', '#38bdf8', '#34d399', '#fb7185'];

  $: if (
    frames !== lastFramesReference &&
    frames.length > 0
  ) {
    lastFramesReference = frames;
    queueFrames(frames);
  }

  $: {
    const me = players.find((player) => player.id === playerId);

    if (
      me &&
      pendingRoll !== null &&
      me.rolls[pendingRoll] !== null
    ) {
      pendingRoll = null;
    }

    draw();
  }

  function queueFrames(batch: PhysicsFrame[]): void {
    frameQueue = [...frameQueue, batch];

    if (activeBatch.length === 0) {
      startNextBatch();
    }
  }

  function startNextBatch(): void {
    const next = frameQueue[0];

    if (!next) {
      activeBatch = [];
      return;
    }

    frameQueue = frameQueue.slice(1);
    activeBatch = next;
    activeFrameIndex = 0;
    lastFrameTime = performance.now();

    cancelAnimationFrame(animationHandle);
    animationHandle = requestAnimationFrame(animate);
  }

  function animate(timestamp: number): void {
    if (timestamp - lastFrameTime >= FRAME_TIME) {
      const frame = activeBatch[activeFrameIndex];

      if (frame) {
        currentDice = frame.dice;
        draw();
        activeFrameIndex += 1;
        lastFrameTime = timestamp;
      }
    }

    if (activeFrameIndex < activeBatch.length) {
      animationHandle = requestAnimationFrame(animate);
      return;
    }

    activeBatch = [];
    startNextBatch();
  }

  function resize(): void {
    if (!canvas) {
      return;
    }

    const bounds = canvas.getBoundingClientRect();
    cssSize = bounds.width;

    const ratio = Math.max(1, window.devicePixelRatio || 1);

    canvas.width = Math.round(cssSize * ratio);
    canvas.height = Math.round(cssSize * ratio);

    context = canvas.getContext('2d');

    if (!context) {
      return;
    }

    context.setTransform(ratio, 0, 0, ratio, 0, 0);
    draw();
  }

  function draw(): void {
    const ctx = context;

    if (!ctx) {
      return;
    }

    ctx.clearRect(0, 0, cssSize, cssSize);
    drawArena(ctx);

    for (const die of currentDice) {
      drawDie(ctx, die);
    }
  }

  function drawArena(ctx: CanvasRenderingContext2D): void {
    const center = cssSize / 2;
    const radius = cssSize * 0.425;

    ctx.save();
    ctx.translate(center, center);

    createOctagonPath(ctx, radius);

    const wood = ctx.createLinearGradient(
      -radius,
      -radius,
      radius,
      radius
    );

    wood.addColorStop(0, '#e6d2b0');
    wood.addColorStop(0.48, '#b79973');
    wood.addColorStop(1, '#80664c');

    ctx.lineWidth = Math.max(16, cssSize * 0.05);
    ctx.strokeStyle = wood;
    ctx.stroke();

    createOctagonPath(ctx, radius * 0.92);

    const felt = ctx.createRadialGradient(
      -radius * 0.2,
      -radius * 0.25,
      5,
      0,
      0,
      radius
    );

    felt.addColorStop(0, '#0d5945');
    felt.addColorStop(0.6, '#073d30');
    felt.addColorStop(1, '#052b22');

    ctx.fillStyle = felt;
    ctx.fill();

    ctx.lineWidth = 3;
    ctx.strokeStyle = 'rgba(255,255,255,.1)';
    ctx.stroke();

    ctx.restore();
  }

  function createOctagonPath(
    ctx: CanvasRenderingContext2D,
    radius: number
  ): void {
    ctx.beginPath();

    for (let index = 0; index < 8; index += 1) {
      const angle =
        Math.PI / 8 +
        (index * Math.PI * 2) / 8;

      const x = Math.cos(angle) * radius;
      const y = Math.sin(angle) * radius;

      if (index === 0) {
        ctx.moveTo(x, y);
      } else {
        ctx.lineTo(x, y);
      }
    }

    ctx.closePath();
  }

  function drawDie(
    ctx: CanvasRenderingContext2D,
    die: DiePose
  ): void {
    const ownerIndex = Math.max(
      0,
      players.findIndex((player) => player.id === die.owner_id)
    );

    const color = COLORS[ownerIndex % COLORS.length];

    const scale = (cssSize * 0.425) / WORLD_RADIUS;
    const x = cssSize / 2 + die.x * scale;
    const y = cssSize / 2 + die.y * scale;
    const size = Math.max(32, scale * 0.78);

    ctx.save();
    ctx.translate(x, y);
    ctx.rotate(die.rotation);

    ctx.shadowBlur = 14;
    ctx.shadowColor = color;

    roundedRect(
      ctx,
      -size / 2,
      -size / 2,
      size,
      size,
      size * 0.19
    );

    ctx.fillStyle = '#fffaf0';
    ctx.fill();

    ctx.lineWidth = 4;
    ctx.strokeStyle = color;
    ctx.stroke();

    ctx.shadowBlur = 0;
    drawDieContent(ctx, die, size);

    ctx.restore();
  }

  function drawDieContent(
    ctx: CanvasRenderingContext2D,
    die: DiePose,
    size: number
  ): void {
    const owner = players.find((player) => player.id === die.owner_id);
    const isMine = die.owner_id === playerId;

    const value =
      isMine && owner?.rolled[die.die_index]
        ? owner.rolls[die.die_index] ?? null
        : null;

    ctx.fillStyle = '#18140f';
    ctx.textAlign = 'center';
    ctx.textBaseline = 'middle';

    if (value === null) {
      ctx.font = `900 ${size * 0.31}px Inter, sans-serif`;
      ctx.fillText('?', 0, 0);
      return;
    }

    drawPips(ctx, value, size);
  }

  function drawPips(
    ctx: CanvasRenderingContext2D,
    value: number,
    size: number
  ): void {
    const offset = size * 0.23;
    const radius = Math.max(2.3, size * 0.055);

    const positions: Record<number, Array<[number, number]>> = {
      1: [[0, 0]],
      2: [[-1, -1], [1, 1]],
      3: [[-1, -1], [0, 0], [1, 1]],
      4: [[-1, -1], [1, -1], [-1, 1], [1, 1]],
      5: [[-1, -1], [1, -1], [0, 0], [-1, 1], [1, 1]],
      6: [[-1, -1], [1, -1], [-1, 0], [1, 0], [-1, 1], [1, 1]]
    };

    for (const [x, y] of positions[value] ?? []) {
      ctx.beginPath();
      ctx.arc(
        x * offset,
        y * offset,
        radius,
        0,
        Math.PI * 2
      );
      ctx.fill();
    }
  }

  function roundedRect(
    ctx: CanvasRenderingContext2D,
    x: number,
    y: number,
    width: number,
    height: number,
    radius: number
  ): void {
    ctx.beginPath();
    ctx.roundRect(x, y, width, height, radius);
    ctx.closePath();
  }

  function isAvailable(index: number): boolean {
    const me = players.find((player) => player.id === playerId);

    if (!me) {
      return false;
    }

    return (
      pendingRoll === null &&
      !me.rolled[index] &&
      me.rolls[index] === null
    );
  }

  function roll(index: number): void {
    if (!isAvailable(index)) {
      return;
    }

    pendingRoll = index;
    onRoll(index);
  }

  onMount(() => {
    context = canvas.getContext('2d');
    resize();

    window.addEventListener('resize', resize);
  });

  onDestroy(() => {
    cancelAnimationFrame(animationHandle);
    window.removeEventListener('resize', resize);
  });
</script>

<div class="arena-shell">
  <canvas bind:this={canvas}></canvas>

  <div class="launch-rack">
    {#each [0, 1, 2] as index}
      <button
        class:used={!isAvailable(index)}
        class:rolling={pendingRoll === index}
        disabled={!isAvailable(index)}
        onclick={() => roll(index)}
      >
        {#if pendingRoll === index}
          <span class="spinner">↻</span>
          <small>ROLLING</small>
        {:else if isAvailable(index)}
          <span>?</span>
          <small>ROLL</small>
        {:else}
          <span>✓</span>
          <small>DONE</small>
        {/if}
      </button>
    {/each}
  </div>
</div>

<style>
  .arena-shell {
    position: relative;
    width: min(100%, 410px);
    margin: -0.2rem auto 0;
  }

  canvas {
    width: 100%;
    aspect-ratio: 1;
    display: block;
  }

  .launch-rack {
    position: absolute;
    left: 50%;
    bottom: 7%;
    display: flex;
    gap: 0.35rem;
    transform: translateX(-50%);
  }

  .launch-rack button {
    width: 52px;
    height: 52px;
    display: grid;
    place-items: center;
    border: 2px solid rgba(251, 146, 60, 0.8);
    border-radius: 12px;
    color: #23160b;
    background: rgba(255, 250, 240, 0.93);
    cursor: pointer;
    box-shadow: 0 10px 22px rgba(0, 0, 0, 0.2);
  }

  .launch-rack button.used:not(.rolling) {
    opacity: 0.2;
  }

  .launch-rack button.rolling {
    opacity: 0.8;
  }

  .launch-rack span {
    font-size: 1.15rem;
    font-weight: 950;
  }

  .launch-rack small {
    font-size: 0.39rem;
    font-weight: 950;
  }

  .spinner {
    animation: spin 400ms linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }
</style>
