# Nautical Mile

A cozy, accurate sailing sandbox in Rust + Bevy. Sail between tropical islands (Maldives, Pacific, Caribbean), explore them, enjoy sea and island life.
Full answers to the design questionnaire: https://claude.ai/artifact/YMfjJVWrNGGaWr9kxmZT6L

## How we work (most important)

- **The user writes all code.** Claude does not write or edit game code unless explicitly asked.
- For each step, Claude tells the user which files to create or change, each with: file name, one-line purpose, and the types/functions it needs (signatures only, no bodies). More detail only on request.
- **Tiny steps.** One small, testable thing at a time.
- **Short messages.** Never send huge replies or open many new topics at once. One question at most per message. The user has ADHD; keep it streamlined.
- Explain the "why" only for non-obvious decisions. Explain sailing physics only briefly, unless asked.
- Feedback: direct and blunt. Challenge decisions when there is a better approach.
- Code reviews only on request.
- Keep design docs in `docs/` up to date as decisions are made.

## Vision

- Mood: cozy, wondrous, adventurous. Endless sandbox, no progression, no economy, no PvP/pirates.
- Feel: pre-1800s seafaring. No GPS or HUD. Compass, binoculars, paper chart that fills in, celestial navigation (latitude from stars), maybe a depth sounder.
- First-person, walking the deck and going ashore. Keyboard + mouse for now.
- Real maneuvers: tacking, gybing, apparent wind, irons, heeling, capsizing, running aground. Should still feel manageable.
- Boat: ~20–30 ft monohull, possibly bigger so it feels roomy. Full sail controls and rigging eventually.
- Real-world units (1 unit = 1 m), possibly a smaller world. Accelerated time of day, moon phases.
- Mild danger only: storms, reefs, running aground.
- Visuals: semi-realistic like Sea of Thieves, water is the star (GPU FFT ocean + LOD mesh around the camera). Islands modelled in Blender. Minimal UI.
- Audio: dynamic ambient soundscape, sparse music.
- Wildlife: decorative at first (fish schools, jellyfish, crabs), realistic later.
- Targets: Windows, macOS, Linux. Main dev machine: macOS (Apple Silicon). Aim: 60 fps on a high-end laptop.

## Architecture rules

- Feature-first modules (`sailing/`, `wind/`, `ocean/`, `islands/`, …), one Bevy `Plugin` per feature.
- Small files, one responsibility each. Each system does exactly one job.
- As little coupling as possible. Shared things are pulled into their own module; no dependency cycles (A → B → C → A is forbidden).
- Physics uses Avian (`avian3d`) directly: no custom rigid body or separate physics module. Keep it as simple as possible.
- Physics model: the boat is one Avian rigid body with gravity at its center of mass. Each force is one small system applying force at a point: buoyancy probes, sail (on the mast), keel, rudder. Heeling and turning emerge from these forces. Sail, keel and rudder share one lift/drag foil function, driven by angle of attack to the apparent flow. No polar curve.
- Physics build order: rigid body + gravity → buoyancy probes → foil + keel → adjustable sail → rudder → real rigging.
- Fixed timestep for simulation (force systems run in `FixedUpdate`).
- Few, larger components. Newtypes for units (`Knots(f32)`, `Degrees(f32)`).
- Constants in code for now. Data files only when they clearly help (e.g. generic fish tables).
- Error handling: simple and robust. No `unsafe` unless truly needed.
- Comments: English, concise, only where necessary. Tests only for critical things for now.
- Crates: minimal. Suggest one with its purpose and wait for approval. Floating origin only once precision becomes a problem.

## Tooling

- Latest stable Bevy, upgrade when easy.
- Commit straight to `main`, free-form messages. No CI yet. Git LFS when assets need it.
- Wanted: auto-format all files before committing.
- Debug gizmos for wind and force vectors are welcome.

## Current milestone

Milestone 1: a boat sailing on a flat ocean, plus one very small prototype island.
