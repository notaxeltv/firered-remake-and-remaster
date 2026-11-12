# Contributing to FireRed Native (rust-prototype)

Short rules for PRs against the Rust port. Prefer small, reviewable diffs.

Also useful: `rust-prototype/README.md`, `docs/RUST-COLLABORATION.md`,
`docs/RUST-PORT-HANDOFF.md`.

## What this is

`rust-prototype/` is a **source-backed native Rust port** of Pokémon FireRed
(USA rev0), from pinned pret/pokefirered, plus an optional Bevy 3D / Let's Go
presentation layer. It is **not** the C++ remaster in `FireRedLeafGreenRecomp/`.

- Simulation lives in a renderer-free library (`default-features = false`).
- The Windows exe is Bevy behind the `renderer` feature.
- Goal: a faithful, editable baseline before redesign. Preserve rules and quirks.
- Do **not** invent equivalent-looking mechanics or silent generic fallbacks.

Imported maps and passing smoke checks do **not** mean complete story or battle
behaviour. 180 maps ≠ a finished game.

## Hard rules

- This repo is **PUBLIC**. Never commit ROMs, BIOS, saves, generated game JSON,
  extracted FireRed art/music, or any Pokémon / Let's Go assets (models, textures,
  audio, Switch/NCA files, keys). Commit importers and provenance only.
- Do not overwrite or redeploy over the installed C++ remaster exe.
- Keep game audio **muted** in QA. Add **no zoom** settings.
- New player-facing features must be **switchable** in Settings and saved in this
  app's `config.ini`.
- Unsupported behaviour must **fail explicitly** (`Err`, logged adapter message).
  Never pretend a quest, battle, or script succeeded.
- New core / battle / VM behaviour is **UNVERIFIED** until compared to original C.
  Say so in the PR.
- Prefer small PRs. One concern each.

## Where to put work

```
rust-prototype/
  src/lib.rs           World, collision, grid Game tick
  src/core.rs          Pokémon, party, PC, bag, Pokédex, RNG, flags
  src/field.rs         Field script VM (scrcmd)
  src/encounter.rs     Wild land/cave encounters
  src/battle.rs        Bounded battle + AI
  src/actors.rs        Scripted NPC movement / scenes
  src/free.rs          Continuous PC movement (redesign, not GBA parity)
  src/main.rs          Bevy shell, input, Preferences, HUD
  src/field_screen.rs  Field VM adapter + encounters + trainers
  src/battle_screen.rs / battle_stage.rs   Battle UI / 3D stage
  src/presentation.rs / models.rs / letsgo_material.rs
  tools/import_*.py    FireRed data importers (need local ROM + reference)
  tools/letsgo/        Let's Go → glTF importers (local RomFS only)
  data/*-provenance.json   Checked-in hashes; not full game tables
```

Gameplay rules belong in the library. Presentation belongs in Bevy modules.
Do not teach the renderer to decide collision, catch odds, or script outcomes.

## Contested files

Active agent batches may reserve files. Before editing contested areas, read
`docs/RUST-COLLABORATION.md` (and the Codex / Gemini status notes it points at).
Typical hot spots: `src/battle.rs`, often `src/field_screen.rs`, and importer /
generated-data lanes under `tools/` and `data/`.

If you need a reserved file, say so in the PR and keep the diff elsewhere until
ownership is clear.

Do **not** edit maintainer hand-off / agent instruction files unless the PR is
explicitly about them: `docs/STATE.md`, `AGENTS.md`, `CLAUDE.md`, and the
RUST-*-STATUS coordination notes.

## Good first PRs

High chance of acceptance:

1. Small docs that clarify ported vs unported behaviour (no runtime change).
2. Tiny presentation polish: HUD help text, music theme routing without new WAV
   commits, `font_safe_text` glyph coverage.
3. One source-backed ScrCmd or one presentation-safe special (pret names), plus a
   focused check — after checking ownership.
4. Importer / coverage tooling that does not commit copyrighted output.

Ask first / low chance:

- Large `battle.rs` batches, weather, doubles, party switching, Struggle-wide
  rewrites, “finish the VM”, invented AI, shortened trainer teams, asset dumps.

## Coding conventions (Rust)

- Edition and toolchain: see `rust-prototype/rust-toolchain.toml` and `Cargo.toml`.
- 4-space indent. Comments explain **why** and cite pret symbols when porting
  (`CalculateBaseDamage`, `ScrCmd_*`, `TryStandardWildEncounter`, …).
- Prefer `Result` / explicit `Err("Unported…")` over `unwrap` on gameplay paths.
- Keep renderer-free code free of Bevy types so `default-features = false` works.
- Do not edit generated JSON by hand to change behaviour — change the importer
  and re-import; refresh provenance.
- Presentation toggles: load/save through `Preferences` in `src/main.rs`.
- UTF-8 in UI: extend `font_safe_text` rather than shipping missing glyphs.

## Local setup (Windows)

1. Own FireRed USA rev0 ROM and, if needed, Let's Go RomFS — never commit them.
2. `powershell -ExecutionPolicy Bypass -File rust-prototype/tools/setup.ps1`
3. Fetch pinned pret checkout: `tools/fetch-reference.ps1` (lands in ignored
   `reference/`).
4. Run the `tools/import_*.py` importers you need so `data/*.json` exist locally
   (`include_str!` expects them). The public clone ships provenance only.
5. `rust-prototype\build.bat` or `tools/build.ps1`.
6. Checks: `tools/check.ps1`, `--check`, `--field-smoke`, `--scene-smoke` as
   relevant. Keep music off.

Without a ROM you can still open docs-only PRs. Do not invent data files to make
`cargo test` green for a public PR.

## PR checklist

- [ ] One concern; reviewable diff
- [ ] No copyrighted binaries, JSON game tables, or Let's Go assets
- [ ] Contested-file ownership checked
- [ ] Source references for ported behaviour (file + symbol / special name)
- [ ] Unsupported paths fail loud — no fake success
- [ ] Settings + `config.ini` if player-facing
- [ ] Test / smoke path named in the PR body; mark **UNVERIFIED** when needed
- [ ] C++ remaster tree left alone unless the PR is about it
- [ ] No edits to `docs/STATE.md` / `AGENTS.md` unless explicitly in scope

PR body template:

```text
### Problem
…

### Approach
… (pret symbol / special / ScrCmd if applicable)

### Test
… (cargo test / --check / smoke / manual)

### Notes
UNVERIFIED: yes/no
Ownership: …
```

## Out of scope (for now)

- GBA `.sav` compatibility / encrypted BoxPokemon parity
- Quiet “make the story work” stubs
- Committing playable asset packs
- Redesigning Gen III rules or adding other regions before the baseline is solid
- Drive-by reformat of `battle.rs` / `field_screen.rs`

## Licence / legal

No Nintendo / Pokémon Company affiliation. No copyrighted game content in the
repo. See the root `README.md` Legal section.
