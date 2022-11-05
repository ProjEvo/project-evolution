# Project Evolution

Created by Timothy (`tdg3`), Numair (`hajyani2`), Shivjeet (`schana2`), and Marcus (`zhiyu7`).

## Intro

Welcome to Project Evolution! Create your creature and watch it evolve through the generations!

Users will be able to create "creatures" made up of nodes and muscles, where the nodes collide and the muscles can expand and contract to allow movement. The user will be able to simulate evolution, making the creatures perform "better" (farther distance, or possibly other goals if we have time) over time. Evolution will be accomplished using some kind of AI (details below) which will learn and produce better and better results over time.

The goal is to have a fun simulation where you can create a creature and watch it become better and better.

## Technical specification

*Note: Items marked with a (?) are things we might add if we get the time to do so*

Creatures
- Made up of nodes and muscles
- 2D
- Nodes are spherical and do not collide with each other (for simplicity)
- Muscles have a "expansion factor", a value ranging from 0 to 1, where 0 is fully extended and 1 is fully contracted
  - (?) Maybe muscles can have different strengths and lengths
- Goal is to go as far as possible by moving muscles

Creature goal
- Get as far as possible
- (?) Different possible goals selectable by the user

There's three main segments (that should be 3 separate modules) that we need to get working for this project:
- UI
  - Renders the creatures
  - Provides options to run the simulation and evolve the creatures
  - (?) Ability to creature the creature itself using some kind of editor
- Simulator
  - Physics engine that can simulate moving creature
  - Simulates the creatures, given the structure of them and how their muscles extend and contract at different times
  - Should be able to tell how well a given creature did
- AI Evolver
  - Given a set of inputs:
    - Position of each node relative to the center of the creature
    - Velocity of each node
    - (?) Muscle positions
  - Return a set of outputs: 
    - How much to extend each muscle
  - Reward the AI based on how far the creature got
  - *See challenges for an alternative to this that might be simpler if need be*

Notes:
- UI depends on AI evolver and physics
- AI evolver depends on physics

## Possible challenges

- AI Evolver might become just natural selection based off of a bunch of attributes if we can't get the machine learning working. This allows us to have a fallback plan.
  - One solution to this is to have constant creatures, but maybe we have a natural selection vs machine learning mode
  - We also can decide between two different implementations of the AI:
    1. AI runs every frame to determine the extension of each muscle based on a number of factors (hard)
    2. AI runs when a creature is generated to determine certain attributes, such as timing of when to extend and contract muscles (easier)
  - Time will tell what works best, that's why we plan for alternatives in case it doesn't work out
- UI might need to be degraded to ASCII terminal output, due to Rust's very immature UI environment
  - Might be like the days of text-terminal games - could make it look intentional

## Checkpoints
- By checkpoint 1:
  - Determined which method for evolution we will be using
  - Basic working prototype
    - Creature is constant
    - Creature is able to move in some way
    - Able to run multiple simulations at once
    - Creature is able to become better over time
- By checkpoint 2:
  - UI fleshed out
    - Able to choose options for simulation, and step multiple generations at a time
    - See data on how creatures have evolved (max distance over generations)
  - Creatures move consistently
  - Able to edit starting creature
  - (?) Able to edit goal
  - (?) View multiple creatures at once
- By final:
  - If we get time, add a lot of the (?) options
  - More polished and easy to use
  - (?) Music
  
  
## Tooling

We'll use Git and GitHub for VSC.

We'll use cargo to manage formatting, linting, building, and executing.

For the UI, this website seems to have a good list of various uis to use. We'll try them out and see what works.

For the simulator, there's quite a few game engines that seem to support rust. If none of those work, we can always have a custom one since our nodes will just be spheres colliding with a flat ground.

We will probably use tensorflow bindings or something similar for the AI.
