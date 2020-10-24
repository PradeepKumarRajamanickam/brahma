# brahma
*Visual code + fsm editor for rust/bevy*

![Ludiq Bolt](https://i.imgur.com/axrdxaY.jpg)
**Ludiq Bolt**

Inspired by Ludiq Bolt. Brahma aims to provide a simple yet robust editor capable of authoring logic graph (function call graph) and state graph (state machine graph), which gets compiled into consumable rust code.

It aims,

- to reduce the barrier of entry for new comers, i.e. artists, designers and hobby programmers
- to make higher level flow related content authoring easier, i.e. gamelogic, ux flow, ai fsm, animation fsm. General idea is, it is much easier to follow control flows visually than in code.

## Feature Set
### Logic Graph
Functions represented visually
- Author custom functions visually
- Interop with Bevy Resources
- Call existing rust methods

### State Graph
Event driven States and Transistions. Container for logic nodes.
- Utilises Bevy Events
- Support Multiple Instances
- Hierarchical State Machine [TBD]
- Local Events [TBD]

### Editor Features
- Search Palette: fuzzy search types like Events, Resources and Rust methods

## Architecture Overview
![](https://i.imgur.com/psDM6uV.png)

**Editor Architecture Overview**
