# brahma
A general purpose data driven hfsm plugin for bevy.

## Goals
- Provide a simple but robust visual editor (similar to bolt)
- Support multiple simultaneous fsm instances

## Usecases
- Gamelogic
- UX Flow
- AI
- Animation State Machine

## Why?
It is much easier to follow control flows visually than in code. Reduces the barrier of entry for new comers. Much easier to maintain large complex flows.

## What it is not?
A performance critical system. It is better to write performance critical aspects as seperate systems and bridging them with the graph for higher level flow control.

Inspired By:
[Ludiq Bolt](https://assetstore.unity.com/packages/tools/visual-scripting/bolt-163802)