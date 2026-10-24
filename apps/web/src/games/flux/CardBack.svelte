<div class="card-back" aria-hidden="true">
  <div class="mark">
    <i></i>
    <i></i>
    <i></i>
    <i></i>
  </div>
</div>

<style>
  .card-back {
    width: 56px;
    aspect-ratio: 0.69;
    display: grid;
    place-items: center;
    flex: 0 0 auto;
    border: 2px solid rgba(255, 255, 255, 0.82);
    border-radius: 10px;
    background:
      repeating-linear-gradient(
        135deg,
        #111318 0 5px,
        #181b22 5px 10px
      );
    box-shadow:
      0 7px 16px rgba(0, 0, 0, 0.25);
  }

  .mark {
    position: relative;
    width: 27px;
    height: 27px;
    transform: rotate(45deg);
  }

  .mark i {
    position: absolute;
    width: 13px;
    height: 13px;
    border-radius: 2px;
  }

  .mark i:nth-child(1) {
    top: 0;
    left: 0;
    background: #ef4444;
  }

  .mark i:nth-child(2) {
    top: 0;
    right: 0;
    background: #38bdf8;
  }

  .mark i:nth-child(3) {
    bottom: 0;
    left: 0;
    background: #facc15;
  }

  .mark i:nth-child(4) {
    right: 0;
    bottom: 0;
    background: #22c55e;
  }
</style>
