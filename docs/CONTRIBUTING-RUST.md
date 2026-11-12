# Contributing to FireRed Native (rust-prototype)

Rules for humans and AI agents working on the Rust port. Keep this short.
Ben is casual and terse; PRs should be the same.

Also read: `AGENTS.md`, `rust-prototype/README.md`, `docs/RUST-COLLABORATION.md`,
`docs/RUST-PORT-HANDOFF.md`, `docs/STATE.md`.

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

## Hard rules (from Ben)

- This repo is **PUBLIC**. Never commit ROMs, BIOS, saves, generated game JSON,
  extracted FireRed art/music, or any Pokémon / Let's Go assets (models, textures,
  audio, Switch/NCA files, keys). Commit importers and provenance only.
- Keep the installed C++ remaster (`Pokemon Fire Red.exe`) untouched. Do not
  deploy the Rust exe over it.
- Keep game audio **muted** in QA. Add **no zoom** settings.
- New player-facing features must be **switchable** in Settings and saved in this
  app's `config.ini` (same idea as the remaster extras).
- Unsupported behaviour must **fail explicitly** (`Err`, logged adapter message).
  Never pretend a quest, battle, or script succeeded.
- New core / battle / VM behaviour is **UNVERIFIED** until compared to original C.
  Say so in the PR. Prefer compile + basic checks over giant parity passes every time.
- Prefer small PRs. One concern each.

## Architecture (where to put work)

```
rust-prototype/
  src/lib.rs          World, collision, grid Game tick
  src/core.rs         Pokémon, party, PC, bag, Pokédex, RNG, flags
  src/field.rs        Field script VM (scrcmd)
  src/encounter.rs    Wild land/cave encounters
  src/battle.rs       Bounded battle + AI
  src/actors.rs       Scripted NPC movement / scenes
  src/free.rs         Continuous PC movement (redesign, not GBA parity)
  src/main.rs         Bevy shell, input, Preferences, HUD
  src/field_screen.rs Field VM adapter + encounters + trainers
  src/battle_screen.rs / battle_stage.rs   Battle UI / 3D stage
  src/presentation.rs / models.rs / letsgo_material.rs
  tools/import_*.py   FireRed data importers (need local ROM + reference)
  tools/letsgo/       Let's Go → glTF importers (local RomFS only)
  data/*-provenance.json   Checked-in hashes; not full game tables
```

Gameplay rules belong in the library. Presentation belongs in Bevy modules.
Do not teach the renderer to decide collision, catch odds, or script outcomes.

## File ownership (read before you edit)

Ben authorized Codex and Antigravity/Gemini to split work. Check the latest
entries in `docs/RUST-COLLABORATION.md`, `docs/RUST-CODEX-STATUS.md`, and
`docs/RUST-GEMINI-STATUS.md` before touching contested files.

Typical split (confirm current batch — it moves):

| Area | Usual owner |
|---|---|
| `src/battle.rs`, battle mechanics docs, often `field_screen.rs` | Codex |
| `tools/import_*.py`, generated world/art/actors data, map expansion | Gemini |
| Presentation (`main.rs` HUD/music prefs, `presentation.rs`, models) | Often free if no overlap |
| Docs under `docs/` | Usually fine |

If you need a reserved file: say so in the PR / issue and wait for an ownership
note. Do not package a release while another agent holds the build lock.

## What makes a good first PR

High chance of acceptance:

1. Docs that map ported vs unported ScrCmd / specials (no runtime change).
2. Tiny presentation polish: HUD help text from map connections, music theme
   routing without new WAV commits, `font_safe_text` glyph coverage.
3. One source-backed ScrCmd or one presentation-safe special (with names from
   pret), plus a focused check — after checking ownership.
4. Importer / coverage tooling that does not commit copyrighted output.

Low chance / ask first:

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
- Presentation toggles: load/save through `Preferences` in `src/main.rs` and the
  Settings HUD strings.
- UTF-8 in UI: the bundled font lacks many accents; extend `font_safe_text` rather
  than shipping boxes.

## Local setup (Windows)

1. Own FireRed USA rev0 ROM and, if needed, Let's Go RomFS — never commit them.
2. `powershell -ExecutionPolicy Bypass -File rust-prototype/tools/setup.ps1`
3. Fetch pinned pret checkout: `tools/fetch-reference.ps1` (lands in ignored
   `reference/`).
4. Run the `tools/import_*.py` importers you need so `data/*.json` exist locally
   (`include_str!` expects them). Runtime builds use those files; the public clone
   ships provenance only.
5. `rust-prototype\build.bat` or `tools/build.ps1` — headless library tests, package
   exe + assets + MinGW DLLs.
6. Checks: `tools/check.ps1`, `--check`, `--field-smoke`, `--scene-smoke` as relevant.
   Keep music off.

Linux/cloud agents: you can read and document without a ROM; do not invent data
files to make `cargo test` green for a public PR.

## PR checklist

Before opening:

- [ ] Scope is one concern; diff stays reviewable
- [ ] No copyrighted binaries, JSON game tables, or Let's Go assets in the commit
- [ ] Ownership checked; no silent edit of Codex/Gemini reserved files
- [ ] Source references for any ported behaviour (file + symbol / special name)
- [ ] Unsupported paths still fail loud — no fake success
- [ ] Settings + `config.ini` if it is a player-facing toggle
- [ ] Tests or smoke path named in the PR body; mark **UNVERIFIED** when needed
- [ ] C++ remaster tree left alone unless the PR is explicitly about it
- [ ] Audio muted in your QA notes; no zoom controls added

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

## Maintainer workflow (what actually happens)

1. Read `STATE.md` / collaboration status; claim or respect file ownership.
2. Implement a small batch against pinned pret source.
3. Run library tests + packaged `--check` / relevant smoke (Windows).
4. Deploy only the Rust prototype exe Ben uses — never overwrite the C++ game.
5. Record outcomes in the matching status doc (`RUST-CODEX-STATUS.md` or
   `RUST-GEMINI-STATUS.md`); update `STATE.md` when behaviour ships.
6. Release the build lock so the other lane can continue.

External contributors: open a small PR that fits the checklist. Prefer starting
with docs or isolated presentation files. Ben reviews tersely; if something needs
a larger mechanics batch, expect to coordinate ownership first.

## Out of scope (for now)

- GBA `.sav` compatibility / encrypted BoxPokemon parity
- Quiet “make the story work” stubs
- Committing playable asset packs
- Redesigning Gen III rules or adding other regions before the baseline is solid
- Drive-by reformat of `battle.rs` / `field_screen.rs`

## Licence / legal

No Nintendo / Pokémon Company affiliation. No copyrighted game content in the
repo. Upstream recomp trees keep PolyForm Noncommercial where applicable. See
the root `README.md` Legal section.
