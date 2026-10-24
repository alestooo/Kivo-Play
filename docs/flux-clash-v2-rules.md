# Flux Clash v2 — Rules

## Match
- 2–4 players.
- First player to reach 100 points wins the match.
- A match normally takes about 3–5 rounds because the server clamps each round award to 20–50 points.

## Normal cards
Colors:
- Red
- Blue
- Green
- Yellow

Numbers:
- 0–9

A card may be played if:
- its color matches the current color, OR
- its number matches the top number.

Example:
Top card = Red 8
- Red 2 is valid.
- Green 8 is valid.

## Reverse
Changes direction.

## +2
- Colored.
- Only another +2 can answer.
- Stacks.
- If the victim draws, they draw the full stack and lose the turn.
- Shows `BOOM!`.

## +4
- Wild.
- Choose the next color.
- Only another +4 can answer.
- Stacks.
- Shows `KABOOM!`.

## +6
- Wild attack.
- Choose any rival.
- Rival draws 6.
- Attacker draws 2 as recoil.
- Cannot be stacked.
- Shows `CATAPLUM!`.

## STEAL
- Choose any rival.
- Server randomly selects up to 2 cards from that rival.
- Only the attacker sees those choices.
- Attacker chooses 1 card.
- Attacker then draws +1 card from the normal deck.
- Rest of rival's hand is never revealed.
- Shows `SNATCH!`.

## CLASH
When a player reaches exactly 1 card:
- that player gets the CLASH button immediately.
- rivals get the CLASH button 700 ms later.

If the one-card player presses first:
- they safely call `CLASH!!`
- no penalty.

If a rival presses first after the 700 ms delay:
- the one-card player draws +3.
- their 1 card becomes 4 cards.
- window closes.

## Scoring
At the end of a round, the winner receives points based on cards left in opponents' hands.

Base values:
- Number = face value.
- Reverse = 20.
- STEAL = 20.
- +2 = 20.
- Wild = 40.
- +4 = 40.
- +6 = 50.

To keep matches short, final round award is clamped between 20 and 50 points.
Scores are capped at 100.
