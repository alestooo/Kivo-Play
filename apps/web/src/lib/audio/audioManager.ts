const cache =
  new Map<string, HTMLAudioElement>();

let enabled = true;
let volume = 0.7;

export function setAudioEnabled(
  value: boolean
): void {
  enabled = value;
}

export function setMasterVolume(
  value: number
): void {
  volume = Math.max(0, Math.min(1, value));
}

export function playSound(
  src: string,
  localVolume = 1
): void {
  if (!enabled || typeof Audio === 'undefined') {
    return;
  }

  let audio = cache.get(src);

  if (!audio) {
    audio = new Audio(src);
    audio.preload = 'auto';
    cache.set(src, audio);
  }

  const instance =
    audio.cloneNode(true) as HTMLAudioElement;

  instance.volume =
    Math.max(
      0,
      Math.min(
        1,
        volume * localVolume
      )
    );

  void instance.play().catch(() => {
    // Browsers can block audio until the first user interaction.
  });
}
