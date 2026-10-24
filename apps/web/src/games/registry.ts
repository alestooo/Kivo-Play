import type {
  GameDefinition,
  GameId
} from './types';

export const games: GameDefinition[] = [
  {
    id: 'flux',

    name: 'Flux Clash',

    tagline:
      'Match colors. Stack attacks. Break the table.',

    description:
      'A fast 2–4 player Flux card battle with stacking +2/+4 attacks, targeted +8 cards and explosive table effects.',

    minPlayers: 2,

    maxPlayers: 4,

    supportsBots: true,

    status: 'available',

    accent: '#facc15',

    engine: 'dom'
  },

  {
    id: 'sudorush',

    name: 'SudoRush',

    tagline:
      'Solve fast. Pressure everyone.',

    description:
      'Competitive Sudoku where speed, accuracy and pressure matter.',

    minPlayers: 1,

    maxPlayers: 4,

    supportsBots: true,

    status: 'coming_soon',

    accent: '#38bdf8',

    engine: 'dom'
  },

  {
    id: 'stackfall',

    name: 'Stackfall',

    tagline:
      'Build. Chain. Destroy.',

    description:
      'A high-speed falling-block battle focused on chains and attacks.',

    minPlayers: 1,

    maxPlayers: 4,

    supportsBots: true,

    status: 'coming_soon',

    accent: '#a78bfa',

    engine: 'phaser'
  },

  {
    id: 'diceforge',

    name: 'Diceforge',

    tagline:
      'Roll. Forge. Break probability.',

    description:
      'Build powerful dice combinations, modifiers and relic-driven strategies.',

    minPlayers: 1,

    maxPlayers: 4,

    supportsBots: true,

    status: 'coming_soon',

    accent: '#34d399',

    engine: 'pixi'
  }
];

export function getGame(
  id: GameId
): GameDefinition | undefined {
  return games.find(
    (game) =>
      game.id === id
  );
}
