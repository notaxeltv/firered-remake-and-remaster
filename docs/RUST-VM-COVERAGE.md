# Rust field VM coverage

Inventory of FireRed field-script commands (`ScrCmd_*`) and `gSpecials` against
the native port.

## Method

- **ScrCmd table:** pret/pokefirered `data/script_cmd_table.inc` @
  `037335f4c725d7c9aecdac87066f2002b4bd7e14` (same revision as
  `rust-prototype/data/field-provenance.json`).
- **Specials table:** pret `data/specials.inc` @ the same revision.
- **Native code:** `rust-prototype/src/field.rs` (`Vm::command`) and
  `rust-prototype/src/field_screen.rs` (`Host` adapter + `Host::Special`).
- Rows are classified only from those sources. No behaviour is inferred beyond
  what the match arms and adapter do.

`field-provenance.json` records **213** assembled handlers. The pret `.inc` file
lists 214 `.4byte` lines because `ScrCmd_nop` appears twice in the checked-out
table text; the unique command set is **213**.

### Classes

| Class | Evidence rule used here |
|---|---|
| **implemented** | Name matched in `Vm::command`; performs real VM state updates and/or emits a `Host` value whose adapter arm is non-empty. |
| **partially implemented** | Matched, but code shows a subset of modes, fixture shortcuts, simplified UI, or actor/settings dependency called out in-source. |
| **stubbed** | Matched, but the VM arm only reads arguments and does nothing, and/or the adapter `Host` arm is literally empty `{}`. |
| **missing** | In the pret table; not present in the `match name` arms → `Unported original command`. Specials not listed in the adapter match → `Unported original special`. |

## ScrCmd summary

| Class | Count |
|---|---:|
| implemented | 50 |
| partially implemented | 32 |
| stubbed | 27 |
| missing | 104 |
| **unique pret commands** | **213** |

## ScrCmd — implemented

| Command | Evidence |
|---|---|
| `additem` | Bag add. |
| `addmoney` | If ignore==0. |
| `addvar` | Add immediate to var (source immediate-only quirk retained). |
| `buffernumberstring` | Numeric string buffer. |
| `bufferstring` | Loads text archive string into buffer. |
| `call` | Call with stack push. |
| `call_if` | Conditional call. |
| `callstd` | Standard script call if index in table. |
| `callstd_if` | Conditional std call. |
| `checkflag` | FlagGet → comparison. |
| `checkitem` | Bag has. |
| `checkitemspace` | Bag has_space. |
| `checkitemtype` | Pocket id → VAR_RESULT. |
| `checkmoney` | If ignore==0. |
| `checktrainerflag` | Check trainer flag. |
| `clearflag` | FlagClear. |
| `cleartrainerflag` | Clear trainer flag. |
| `closemessage` | Clears `game.dialogue`. |
| `compare_local_to_local` | Sets comparison. |
| `compare_local_to_value` | Sets comparison. |
| `compare_var_to_value` | Sets comparison. |
| `compare_var_to_var` | Sets comparison. |
| `copylocal` | Copy between locals. |
| `delay` | Sets adapter wait for `frames / TICK_HZ`. |
| `copyvar` | Copy variable. |
| `end` | Clears PC. |
| `getpartysize` | Party len → VAR_RESULT. |
| `givemon` | CreateMon + give_mon + Pokédex caught; result 0/1/2. |
| `goto` | Unconditional jump. |
| `goto_if` | Conditional jump. |
| `gotostd` | Standard script jump if index in table. |
| `gotostd_if` | Conditional std goto. |
| `loadbyte` | Local byte load. |
| `loadword` | Local word load. |
| `nop` | No-op as in source. |
| `nop1` | No-op as in source. |
| `random` | RNG next16 % max → VAR_RESULT. |
| `removeitem` | Bag remove. |
| `removemoney` | If ignore==0. |
| `return` | Pops script stack. |
| `setflag` | FlagSet. |
| `setorcopyvar` | Set-or-copy variable. |
| `setrespawn` | state.respawn = value. |
| `settrainerflag` | Flag at trainer base 0x500. |
| `setvar` | Write variable. |
| `setworldmapflag` | FlagSet(id). |
| `special` | Resolves gSpecials[index] → Host::Special. |
| `specialvar` | Same with destination var. |
| `subvar` | Subtract var from var. |
| `textcolor` | Vars 0x8012/0x8013. |

## ScrCmd — partially implemented

| Command | Evidence |
|---|---|
| `addobject` | Actor present flag; requires actors enabled. |
| `addobjectat` | Same. |
| `applymovement` | Requires actor_scenes; ambient wandering / full movement opcode set deferred (actors.rs). |
| `applymovementat` | Same actor path; map targeting via field.target. |
| `bufferitemname` | Uses core item english field; not full buffer formatting controls. |
| `bufferitemnameplural` | Poké Ball plural heuristic only; not full source plural rules. |
| `bufferleadmonspeciesname` | Same symbol-strip naming as bufferspeciesname. |
| `bufferspeciesname` | Writes SPECIES_* symbol strip, not localized species string tables. |
| `bufferstdstring` | Hardcoded English subset; unknown index Err. |
| `checkplayergender` | Adapter always writes VAR_RESULT=0 (RED fixture). |
| `copyobjectxytoperm` | Same. |
| `dotrainerbattle` | Starts only complete single-mon fixtures; unimported teams fail in start_trainer_battle. |
| `faceplayer` | Turns selected actor; depends on actors selection. |
| `gotopostbattlescript` | Uses post_battle_pc set by supported trainerbattle modes only. |
| `hideobjectat` | Same. |
| `message` | Adapter shows instant pages; print/fanfare timing deferred (field_screen.rs comment). |
| `multichoice` | Adapter presents Yes/No choices, ignores multichoice_id option list. |
| `removeobject` | Same. |
| `removeobjectat` | Same. |
| `setobjectmovementtype` | Same. |
| `setobjectxy` | Same. |
| `setobjectxyperm` | Same. |
| `showobjectat` | Same. |
| `trainerbattle` | Only modes 0,1,2,4,9; other modes Err(Unported trainer battle mode). |
| `turnobject` | Same (player id 0xff supported). |
| `waitbuttonpress` | Uses message_ack / key path; presentation simplified. |
| `waitmessage` | Waits for message_ack only; no typewriter timing. |
| `waitmovement` | Depends on actors subsystem. |
| `waitmovementat` | Depends on actors subsystem. |
| `warp` | Adapter applies map+x+y; warp_id discarded (warp_id: _). |
| `warpsilent` | Same as warp in Host::Warp; warp_id discarded; no silent/transition distinction in adapter. |
| `yesnobox` | Native 1/2 keys; not GBA window coords UI. |

## ScrCmd — stubbed

| Command | Evidence |
|---|---|
| `closedoor` | Emits `Host::Door { open: false, ... }`; adapter no-op. |
| `dofieldeffect` | Reads effect id; ignored. |
| `fadedefaultbgm` | Empty arm. |
| `fadeinbgm` | Reads mode byte; ignored. |
| `fadenewbgm` | Reads song id; ignored. |
| `fadeoutbgm` | Reads mode byte; ignored. |
| `fadescreen` | Reads mode byte; ignored. |
| `fadescreenspeed` | Reads mode and speed; ignored. |
| `hidemonpic` | Emits `Host::HideMonPic`; adapter no-op. |
| `incrementgamestat` | Reads stat byte; ignored. |
| `lock` | Emits `Host::Lock(false)`; adapter no-op. |
| `lockall` | Emits `Host::Lock(true)`; adapter no-op. |
| `opendoor` | Emits `Host::Door { open: true, ... }`; adapter `Host::Door` is `{}`. |
| `playbgm` | Emits `Host::Sound`; adapter no-op (save flag read then ignored in VM). |
| `playfanfare` | Emits `Host::Sound(..., fanfare)`; adapter no-op. |
| `playse` | Emits `Host::Sound`; adapter `Host::Sound` / `WaitSound` are `{}`. |
| `release` | Emits `Host::Release(false)`; adapter no-op. |
| `releaseall` | Emits `Host::Release(true)`; adapter no-op. |
| `savebgm` | Reads song id; ignored. |
| `setdoorclosed` | Reads x,y; ignored. |
| `setdooropen` | Reads x,y; ignored (unlike `opendoor`, does not emit `Host::Door`). |
| `showmonpic` | Emits `Host::ShowMonPic`; adapter no-op. |
| `waitdooranim` | Empty arm. |
| `waitfanfare` | Emits `Host::WaitSound`; adapter no-op. |
| `waitfieldeffect` | Reads effect id; ignored. |
| `waitse` | Emits `Host::WaitSound`; adapter no-op. |
| `waitstate` | Emits `Host::WaitState`; adapter no-op. |

## ScrCmd — missing

Not present in `Vm::command`’s `match name`. Execution hits
`Err(format!("Unported original command {handler}"))`.

| # | Command |
|---:|---|
| 1 | `returnram` |
| 2 | `endram` |
| 3 | `setmysteryeventstatus` |
| 4 | `setptr` |
| 5 | `loadbytefromptr` |
| 6 | `setptrbyte` |
| 7 | `copybyte` |
| 8 | `compare_local_to_ptr` |
| 9 | `compare_ptr_to_local` |
| 10 | `compare_ptr_to_value` |
| 11 | `compare_ptr_to_ptr` |
| 12 | `callnative` |
| 13 | `gotonative` |
| 14 | `initclock` |
| 15 | `dotimebasedevents` |
| 16 | `gettime` |
| 17 | `warpdoor` |
| 18 | `warphole` |
| 19 | `warpteleport` |
| 20 | `setwarp` |
| 21 | `setdynamicwarp` |
| 22 | `setdivewarp` |
| 23 | `setholewarp` |
| 24 | `getplayerxy` |
| 25 | `addpcitem` |
| 26 | `checkpcitem` |
| 27 | `adddecoration` |
| 28 | `removedecoration` |
| 29 | `checkdecor` |
| 30 | `checkdecorspace` |
| 31 | `gotobeatenscript` |
| 32 | `multichoicedefault` |
| 33 | `multichoicegrid` |
| 34 | `drawbox` |
| 35 | `erasebox` |
| 36 | `drawboxtext` |
| 37 | `showcontestpainting` |
| 38 | `braillemessage` |
| 39 | `giveegg` |
| 40 | `setmonmove` |
| 41 | `checkpartymove` |
| 42 | `bufferpartymonnick` |
| 43 | `bufferdecorationname` |
| 44 | `buffermovename` |
| 45 | `pokemart` |
| 46 | `pokemartdecoration` |
| 47 | `pokemartdecoration2` |
| 48 | `playslotmachine` |
| 49 | `setberrytree` |
| 50 | `choosecontestmon` |
| 51 | `startcontest` |
| 52 | `showcontestresults` |
| 53 | `contestlinktransfer` |
| 54 | `showmoneybox` |
| 55 | `hidemoneybox` |
| 56 | `updatemoneybox` |
| 57 | `getpokenewsactive` |
| 58 | `setflashlevel` |
| 59 | `animateflash` |
| 60 | `messageautoscroll` |
| 61 | `setfieldeffectargument` |
| 62 | `playmoncry` |
| 63 | `setmetatile` |
| 64 | `resetweather` |
| 65 | `setweather` |
| 66 | `doweather` |
| 67 | `setstepcallback` |
| 68 | `setmaplayoutindex` |
| 69 | `setobjectsubpriority` |
| 70 | `resetobjectsubpriority` |
| 71 | `createvobject` |
| 72 | `turnvobject` |
| 73 | `addelevmenuitem` |
| 74 | `showelevmenu` |
| 75 | `checkcoins` |
| 76 | `addcoins` |
| 77 | `removecoins` |
| 78 | `setwildbattle` |
| 79 | `dowildbattle` |
| 80 | `setvaddress` |
| 81 | `vgoto` |
| 82 | `vcall` |
| 83 | `vgoto_if` |
| 84 | `vcall_if` |
| 85 | `vmessage` |
| 86 | `vbuffermessage` |
| 87 | `vbufferstring` |
| 88 | `showcoinsbox` |
| 89 | `hidecoinsbox` |
| 90 | `updatecoinsbox` |
| 91 | `setescapewarp` |
| 92 | `waitmoncry` |
| 93 | `bufferboxname` |
| 94 | `loadhelp` |
| 95 | `unloadhelp` |
| 96 | `signmsg` |
| 97 | `normalmsg` |
| 98 | `comparestat` |
| 99 | `setmonmodernfatefulencounter` |
| 100 | `checkmonmodernfatefulencounter` |
| 101 | `trywondercardscript` |
| 102 | `warpspinenter` |
| 103 | `setmonmetlocation` |
| 104 | `getbraillestringwidth` |

## Specials summary

pret `gSpecials` has **445** table slots (**171** are `NullFieldSpecial`).
Unique special symbol names: **273**.

| Class | Count (unique names) |
|---|---:|
| implemented | 6 |
| partially implemented | 4 |
| stubbed | 10 |
| missing | 253 |
| **unique names** | **273** |

`Vm::command` resolves `special` / `specialvar` by reading `gSpecials[index]`
from imported field bytes and mapping the pointer through `data.imports` to a
symbol name, then emits `Host::Special`. Coverage below is that adapter match.

## Specials — implemented

| Special | Evidence |
|---|---|
| `EnableNationalPokedex` | state.enable_national_pokedex |
| `GetPokedexCount` | Writes seen/caught/national to vars from native Pokédex. |
| `HealPlayerParty` | state.heal_party |
| `IsNationalPokedexEnabled` | Reads native national dex flag. |
| `SetSeenMon` | pokedex.set_seen from VAR_0x8004. |
| `SetVermilionTrashCans` | Source-ordered RNG into 0x8004/0x8005. |

## Specials — partially implemented

| Special | Evidence |
|---|---|
| `BufferUnionRoomPlayerName` | Always writes 0; no union-room name buffer. |
| `GetQuestLogState` | Always writes 0; quest-log not ported. |
| `IsPlayerNotInTrainerTowerLobby` | Always writes 1 (not in lobby fixture). |
| `UpdatePickStateFromSpecialVar8005` | No-op in normal play; Err during --scene-smoke (Fame Checker boundary). |

## Specials — stubbed

Adapter arms are empty `{}` (including every pret slot named `NullFieldSpecial`).

| Special | Evidence |
|---|---|
| `ChangePokemonNickname` | Listed in the stub group in `field_screen.rs`; empty arm. |
| `DrawWholeMapView` | Listed in the stub group in `field_screen.rs`; empty arm. |
| `NullFieldSpecial` | Empty arm; pret table contains this symbol **171** times. |
| `PlayTrainerEncounterMusic` | Listed in the stub group in `field_screen.rs`; empty arm. |
| `QuestLog_CutRecording` | Listed in the stub group in `field_screen.rs`; empty arm. |
| `SetFlavorTextFlagFromSpecialVars` | Listed in the stub group in `field_screen.rs`; empty arm. |
| `SetUnlockedPokedexFlags` | Listed in the stub group in `field_screen.rs`; empty arm. |
| `SetUsedPkmnCenterQuestLogEvent` | Listed in the stub group in `field_screen.rs`; empty arm. |
| `ShowTrainerIntroSpeech` | Listed in the stub group in `field_screen.rs`; empty arm. |
| `StartOldManTutorialBattle` | Listed in the stub group in `field_screen.rs`; empty arm. |

## Specials — missing

Unique pret names with no arm in `Host::Special` →
`Err(format!("Unported original special: {name}"))`.

| # | Special |
|---:|---|
| 1 | `ptr` |
| 2 | `SetCableClubWarp` |
| 3 | `DoCableClubWarp` |
| 4 | `ReturnFromLinkRoom` |
| 5 | `CleanupLinkRoomState` |
| 6 | `ExitLinkRoom` |
| 7 | `TryBattleLinkup` |
| 8 | `TryTradeLinkup` |
| 9 | `TryRecordMixLinkup` |
| 10 | `CloseLink` |
| 11 | `EnterColosseumPlayerSpot` |
| 12 | `EnterTradeSeat` |
| 13 | `StartWiredCableClubTrade` |
| 14 | `CableClub_AskSaveTheGame` |
| 15 | `SavePlayerParty` |
| 16 | `LoadPlayerParty` |
| 17 | `ChooseHalfPartyForBattle` |
| 18 | `Script_ShowLinkTrainerCard` |
| 19 | `IsEnigmaBerryValid` |
| 20 | `GetTrainerBattleMode` |
| 21 | `ShowTrainerCantBattleSpeech` |
| 22 | `Script_HasTrainerBeenFought` |
| 23 | `EndTrainerApproach` |
| 24 | `ShouldTryRematchBattle` |
| 25 | `IsTrainerReadyForRematch` |
| 26 | `StartRematchBattle` |
| 27 | `ShowPokemonStorageSystemPC` |
| 28 | `HasEnoughMonsForDoubleBattle` |
| 29 | `HasLeadMonBeenRenamed` |
| 30 | `TryContestLinkup` |
| 31 | `Field_AskSaveTheGame` |
| 32 | `DoWateringBerryTreeAnim` |
| 33 | `ShowEasyChatScreen` |
| 34 | `ShowEasyChatMessage` |
| 35 | `GetHeracrossSizeRecordInfo` |
| 36 | `CompareHeracrossSize` |
| 37 | `GetMagikarpSizeRecordInfo` |
| 38 | `CompareMagikarpSize` |
| 39 | `NameRaterWasNicknameChanged` |
| 40 | `BufferMonNickname` |
| 41 | `IsMonOTIDNotPlayers` |
| 42 | `BufferRandomHobbyOrLifestyleString` |
| 43 | `RegisteredItemHandleBikeSwap` |
| 44 | `CalculatePlayerPartyCount` |
| 45 | `CountPartyNonEggMons` |
| 46 | `CountPartyAliveNonEggMons_IgnoreVar0x8004Slot` |
| 47 | `ShowFieldMessageStringVar4` |
| 48 | `GetPlayerXY` |
| 49 | `GetPlayerTrainerIdOnesDigit` |
| 50 | `BufferBigGuyOrBigGirlString` |
| 51 | `BufferSonOrDaughterString` |
| 52 | `SetHiddenItemFlag` |
| 53 | `Overworld_PlaySpecialMapMusic` |
| 54 | `ChoosePartyMon` |
| 55 | `Script_IsFanClubMemberFanOfPlayer` |
| 56 | `Script_GetNumFansOfPlayerInTrainerFanClub` |
| 57 | `Script_BufferFanClubTrainerName` |
| 58 | `Script_TryLoseFansFromPlayTimeAfterLinkBattle` |
| 59 | `Script_TryLoseFansFromPlayTime` |
| 60 | `Script_SetPlayerGotFirstFans` |
| 61 | `Script_UpdateTrainerFanClubGameClear` |
| 62 | `Script_TryGainNewFanFromCounter` |
| 63 | `RockSmashWildEncounter` |
| 64 | `GetBattleOutcome` |
| 65 | `GetDaycareMonNicknames` |
| 66 | `GetDaycareState` |
| 67 | `RejectEggFromDayCare` |
| 68 | `GiveEggFromDaycare` |
| 69 | `SetDaycareCompatibilityString` |
| 70 | `GetSelectedMonNicknameAndSpecies` |
| 71 | `StoreSelectedPokemonInDaycare` |
| 72 | `ChooseSendDaycareMon` |
| 73 | `ShowDaycareLevelMenu` |
| 74 | `GetNumLevelsGainedFromDaycare` |
| 75 | `GetDaycareCost` |
| 76 | `TakePokemonFromDaycare` |
| 77 | `ScriptHatchMon` |
| 78 | `EggHatch` |
| 79 | `DaycareMonReceivedMail` |
| 80 | `ShowBattleRecords` |
| 81 | `IsEnoughForCostInVar0x8005` |
| 82 | `SubtractMoneyFromVar0x8005` |
| 83 | `TryFieldPoisonWhiteOut` |
| 84 | `SetCB2WhiteOut` |
| 85 | `EnterSafariMode` |
| 86 | `ExitSafariMode` |
| 87 | `GetProfOaksRatingMessage` |
| 88 | `AnimatePcTurnOn` |
| 89 | `AnimatePcTurnOff` |
| 90 | `GetElevatorFloor` |
| 91 | `ChooseMonForMoveRelearner` |
| 92 | `SelectMoveDeleterMove` |
| 93 | `MoveDeleterForgetMove` |
| 94 | `BufferMoveDeleterNicknameAndMove` |
| 95 | `GetNumMovesSelectedMonHas` |
| 96 | `TeachMoveRelearnerMove` |
| 97 | `GetPlayerAvatarBike` |
| 98 | `GetLeadMonFriendship` |
| 99 | `BattleTowerMapScript2` |
| 100 | `ChooseNextBattleTowerTrainer` |
| 101 | `CheckPartyBattleTowerBanlist` |
| 102 | `PrintBattleTowerTrainerGreeting` |
| 103 | `BufferEReaderTrainerGreeting` |
| 104 | `StartSpecialBattle` |
| 105 | `SetBattleTowerProperty` |
| 106 | `BattleTowerUtil` |
| 107 | `SetBattleTowerParty` |
| 108 | `SaveBattleTowerProgress` |
| 109 | `BattleTower_SoftReset` |
| 110 | `DetermineBattleTowerPrize` |
| 111 | `GiveBattleTowerPrize` |
| 112 | `AwardBattleTowerRibbons` |
| 113 | `ChooseBattleTowerPlayerParty` |
| 114 | `ValidateEReaderTrainer` |
| 115 | `ReducePlayerPartyToThree` |
| 116 | `BedroomPC` |
| 117 | `PlayerPC` |
| 118 | `ShowTownMap` |
| 119 | `GetInGameTradeSpeciesInfo` |
| 120 | `CreateInGameTradePokemon` |
| 121 | `DoInGameTradeScene` |
| 122 | `GetTradeSpecies` |
| 123 | `CreatePCMenu` |
| 124 | `HallOfFamePCBeginFade` |
| 125 | `ShowDiploma` |
| 126 | `LookThroughPorthole` |
| 127 | `DoSoftReset` |
| 128 | `EnterHallOfFame` |
| 129 | `AnimateElevator` |
| 130 | `SpawnCameraObject` |
| 131 | `RemoveCameraObject` |
| 132 | `BufferEReaderTrainerName` |
| 133 | `GetRandomSlotMachineId` |
| 134 | `GetPlayerFacingDirection` |
| 135 | `LeadMonHasEffortRibbon` |
| 136 | `GiveLeadMonEffortRibbon` |
| 137 | `AreLeadMonEVsMaxedOut` |
| 138 | `Script_FacePlayer` |
| 139 | `Script_ClearHeldMovement` |
| 140 | `InitRoamer` |
| 141 | `PlayerHasGrassPokemonInParty` |
| 142 | `IsStarterFirstStageInParty` |
| 143 | `IsThereRoomInAnyBoxForMorePokemon` |
| 144 | `DrawElevatorCurrentFloorWindow` |
| 145 | `IsPokerusInParty` |
| 146 | `SetIcefallCaveCrackedIceMetatiles` |
| 147 | `ShakeScreen` |
| 148 | `StartGroudonKyogreBattle` |
| 149 | `StartLegendaryBattle` |
| 150 | `StartRegiBattle` |
| 151 | `SetUpTrainerMovement` |
| 152 | `StartDroughtWeatherBlend` |
| 153 | `DoDiveWarp` |
| 154 | `DoFallWarp` |
| 155 | `SetEReaderTrainerGfxId` |
| 156 | `StartSouthernIslandBattle` |
| 157 | `GetPartyMonSpecies` |
| 158 | `IsSelectedMonEgg` |
| 159 | `LoadPlayerBag` |
| 160 | `Script_FadeOutMapMusic` |
| 161 | `HasAllKantoMons` |
| 162 | `IsMonOTNameNotPlayers` |
| 163 | `Dummy_TryEnableBravoTrainerBattleTower` |
| 164 | `DoesPartyHaveEnigmaBerry` |
| 165 | `SetPostgameFlags` |
| 166 | `StartMarowakBattle` |
| 167 | `ForcePlayerOntoBike` |
| 168 | `ListMenu` |
| 169 | `ReturnToListMenu` |
| 170 | `DoPicboxCancel` |
| 171 | `SeafoamIslandsB4F_CurrentDumpsPlayerOnLand` |
| 172 | `SampleResortGorgeousMonAndReward` |
| 173 | `CheckAddCoins` |
| 174 | `GetDaycarePokemonCount` |
| 175 | `CloseElevatorCurrentFloorWindow` |
| 176 | `ForcePlayerToStartSurfing` |
| 177 | `GetStarterSpecies` |
| 178 | `VsSeekerResetObjectMovementAfterChargeComplete` |
| 179 | `ShouldShowBoxWasFullMessage` |
| 180 | `ChangeBoxPokemonNickname` |
| 181 | `UpdateTrainerCardPhotoIcons` |
| 182 | `StickerManGetBragFlags` |
| 183 | `IsWirelessAdapterConnected` |
| 184 | `TryBecomeLinkLeader` |
| 185 | `TryJoinLinkGroup` |
| 186 | `RunUnionRoom` |
| 187 | `ShowWirelessCommunicationScreen` |
| 188 | `SetWalkingIntoSignVars` |
| 189 | `DisableMsgBoxWalkaway` |
| 190 | `VsSeekerFreezeObjectsAfterChargeComplete` |
| 191 | `OverworldWhiteOutGetMoneyLoss` |
| 192 | `PutMonInRoute5Daycare` |
| 193 | `GetCostToWithdrawRoute5DaycareMon` |
| 194 | `IsThereMonInRoute5Daycare` |
| 195 | `GetNumLevelsGainedForRoute5DaycareMon` |
| 196 | `TakePokemonFromRoute5Daycare` |
| 197 | `DoSeagallopFerryScene` |
| 198 | `DoesPlayerPartyContainSpecies` |
| 199 | `Script_SetHelpContext` |
| 200 | `BackupHelpContext` |
| 201 | `RestoreHelpContext` |
| 202 | `ValidateSavedWonderCard` |
| 203 | `InitUnionRoom` |
| 204 | `QuestLog_StartRecordingInputsAfterDeferredEvent` |
| 205 | `GetMartClerkObjectId` |
| 206 | `GetMysteryGiftCardStat` |
| 207 | `WonderNews_GetRewardInfo` |
| 208 | `GetPCBoxToSendMon` |
| 209 | `OpenMuseumFossilPic` |
| 210 | `CloseMuseumFossilPic` |
| 211 | `ChooseMonForMoveTutor` |
| 212 | `ChooseMonForWirelessMinigame` |
| 213 | `SetBattledTrainerFlag` |
| 214 | `SetHelpContextForMap` |
| 215 | `DoSSAnneDepartureCutscene` |
| 216 | `IsPokemonJumpSpeciesInParty` |
| 217 | `CallTrainerTowerFunc` |
| 218 | `ShowPokemonJumpRecords` |
| 219 | `BufferTMHMMoveName` |
| 220 | `DaisyMassageServices` |
| 221 | `HelpSystem_Disable` |
| 222 | `HelpSystem_Enable` |
| 223 | `HasAtLeastOneBerry` |
| 224 | `DisplayBerryPowderVendorMenu` |
| 225 | `RemoveBerryPowderVendorMenu` |
| 226 | `Script_HasEnoughBerryPowder` |
| 227 | `Script_TakeBerryPowder` |
| 228 | `PrintPlayerBerryPowderAmount` |
| 229 | `DoPokemonLeagueLightingEffect` |
| 230 | `ShowBerryCrushRankings` |
| 231 | `CapeBrinkGetMoveToTeachLeadPokemon` |
| 232 | `HasLearnedAllMovesFromCapeBrinkTutor` |
| 233 | `DoCredits` |
| 234 | `ShowDodrioBerryPickingRecords` |
| 235 | `DrawSeagallopDestinationMenu` |
| 236 | `GetSelectedSeagallopDestination` |
| 237 | `GetSeagallopNumber` |
| 238 | `DoDeoxysTriangleInteraction` |
| 239 | `SetDeoxysTrianglePalette` |
| 240 | `IsPlayerLeftOfVermilionSailor` |
| 241 | `IsBadEggInParty` |
| 242 | `ReadTrainerTowerAndValidate` |
| 243 | `HasAllMons` |
| 244 | `BrailleCursorToggle` |
| 245 | `Script_ResetUnionRoomTrade` |
| 246 | `PlayerPartyContainsSpeciesWithPlayerID` |
| 247 | `AnimateTeleporterHousing` |
| 248 | `IsDodrioInParty` |
| 249 | `AnimateTeleporterCable` |
| 250 | `InitElevatorFloorSelectMenuPos` |
| 251 | `UpdateLoreleiDollCollection` |
| 252 | `LoopWingFlapSound` |
| 253 | `CreateEnemyEventMon` |

## Index of every unique ScrCmd

| Command | Class |
|---|---|
| `nop` | implemented |
| `nop1` | implemented |
| `end` | implemented |
| `return` | implemented |
| `call` | implemented |
| `goto` | implemented |
| `goto_if` | implemented |
| `call_if` | implemented |
| `gotostd` | implemented |
| `callstd` | implemented |
| `gotostd_if` | implemented |
| `callstd_if` | implemented |
| `returnram` | missing |
| `endram` | missing |
| `setmysteryeventstatus` | missing |
| `loadword` | implemented |
| `loadbyte` | implemented |
| `setptr` | missing |
| `loadbytefromptr` | missing |
| `setptrbyte` | missing |
| `copylocal` | implemented |
| `copybyte` | missing |
| `setvar` | implemented |
| `addvar` | implemented |
| `subvar` | implemented |
| `copyvar` | implemented |
| `setorcopyvar` | implemented |
| `compare_local_to_local` | implemented |
| `compare_local_to_value` | implemented |
| `compare_local_to_ptr` | missing |
| `compare_ptr_to_local` | missing |
| `compare_ptr_to_value` | missing |
| `compare_ptr_to_ptr` | missing |
| `compare_var_to_value` | implemented |
| `compare_var_to_var` | implemented |
| `callnative` | missing |
| `gotonative` | missing |
| `special` | implemented |
| `specialvar` | implemented |
| `waitstate` | stubbed |
| `delay` | implemented |
| `setflag` | implemented |
| `clearflag` | implemented |
| `checkflag` | implemented |
| `initclock` | missing |
| `dotimebasedevents` | missing |
| `gettime` | missing |
| `playse` | stubbed |
| `waitse` | stubbed |
| `playfanfare` | stubbed |
| `waitfanfare` | stubbed |
| `playbgm` | stubbed |
| `savebgm` | stubbed |
| `fadedefaultbgm` | stubbed |
| `fadenewbgm` | stubbed |
| `fadeoutbgm` | stubbed |
| `fadeinbgm` | stubbed |
| `warp` | partially implemented |
| `warpsilent` | partially implemented |
| `warpdoor` | missing |
| `warphole` | missing |
| `warpteleport` | missing |
| `setwarp` | missing |
| `setdynamicwarp` | missing |
| `setdivewarp` | missing |
| `setholewarp` | missing |
| `getplayerxy` | missing |
| `getpartysize` | implemented |
| `additem` | implemented |
| `removeitem` | implemented |
| `checkitemspace` | implemented |
| `checkitem` | implemented |
| `checkitemtype` | implemented |
| `addpcitem` | missing |
| `checkpcitem` | missing |
| `adddecoration` | missing |
| `removedecoration` | missing |
| `checkdecor` | missing |
| `checkdecorspace` | missing |
| `applymovement` | partially implemented |
| `applymovementat` | partially implemented |
| `waitmovement` | partially implemented |
| `waitmovementat` | partially implemented |
| `removeobject` | partially implemented |
| `removeobjectat` | partially implemented |
| `addobject` | partially implemented |
| `addobjectat` | partially implemented |
| `setobjectxy` | partially implemented |
| `showobjectat` | partially implemented |
| `hideobjectat` | partially implemented |
| `faceplayer` | partially implemented |
| `turnobject` | partially implemented |
| `trainerbattle` | partially implemented |
| `dotrainerbattle` | partially implemented |
| `gotopostbattlescript` | partially implemented |
| `gotobeatenscript` | missing |
| `checktrainerflag` | implemented |
| `settrainerflag` | implemented |
| `cleartrainerflag` | implemented |
| `setobjectxyperm` | partially implemented |
| `copyobjectxytoperm` | partially implemented |
| `setobjectmovementtype` | partially implemented |
| `waitmessage` | partially implemented |
| `message` | partially implemented |
| `closemessage` | implemented |
| `lockall` | stubbed |
| `lock` | stubbed |
| `releaseall` | stubbed |
| `release` | stubbed |
| `waitbuttonpress` | partially implemented |
| `yesnobox` | partially implemented |
| `multichoice` | partially implemented |
| `multichoicedefault` | missing |
| `multichoicegrid` | missing |
| `drawbox` | missing |
| `erasebox` | missing |
| `drawboxtext` | missing |
| `showmonpic` | stubbed |
| `hidemonpic` | stubbed |
| `showcontestpainting` | missing |
| `braillemessage` | missing |
| `givemon` | implemented |
| `giveegg` | missing |
| `setmonmove` | missing |
| `checkpartymove` | missing |
| `bufferspeciesname` | partially implemented |
| `bufferleadmonspeciesname` | partially implemented |
| `bufferpartymonnick` | missing |
| `bufferitemname` | partially implemented |
| `bufferdecorationname` | missing |
| `buffermovename` | missing |
| `buffernumberstring` | implemented |
| `bufferstdstring` | partially implemented |
| `bufferstring` | implemented |
| `pokemart` | missing |
| `pokemartdecoration` | missing |
| `pokemartdecoration2` | missing |
| `playslotmachine` | missing |
| `setberrytree` | missing |
| `choosecontestmon` | missing |
| `startcontest` | missing |
| `showcontestresults` | missing |
| `contestlinktransfer` | missing |
| `random` | implemented |
| `addmoney` | implemented |
| `removemoney` | implemented |
| `checkmoney` | implemented |
| `showmoneybox` | missing |
| `hidemoneybox` | missing |
| `updatemoneybox` | missing |
| `getpokenewsactive` | missing |
| `fadescreen` | stubbed |
| `fadescreenspeed` | stubbed |
| `setflashlevel` | missing |
| `animateflash` | missing |
| `messageautoscroll` | missing |
| `dofieldeffect` | stubbed |
| `setfieldeffectargument` | missing |
| `waitfieldeffect` | stubbed |
| `setrespawn` | implemented |
| `checkplayergender` | partially implemented |
| `playmoncry` | missing |
| `setmetatile` | missing |
| `resetweather` | missing |
| `setweather` | missing |
| `doweather` | missing |
| `setstepcallback` | missing |
| `setmaplayoutindex` | missing |
| `setobjectsubpriority` | missing |
| `resetobjectsubpriority` | missing |
| `createvobject` | missing |
| `turnvobject` | missing |
| `opendoor` | stubbed |
| `closedoor` | stubbed |
| `waitdooranim` | stubbed |
| `setdooropen` | stubbed |
| `setdoorclosed` | stubbed |
| `addelevmenuitem` | missing |
| `showelevmenu` | missing |
| `checkcoins` | missing |
| `addcoins` | missing |
| `removecoins` | missing |
| `setwildbattle` | missing |
| `dowildbattle` | missing |
| `setvaddress` | missing |
| `vgoto` | missing |
| `vcall` | missing |
| `vgoto_if` | missing |
| `vcall_if` | missing |
| `vmessage` | missing |
| `vbuffermessage` | missing |
| `vbufferstring` | missing |
| `showcoinsbox` | missing |
| `hidecoinsbox` | missing |
| `updatecoinsbox` | missing |
| `incrementgamestat` | stubbed |
| `setescapewarp` | missing |
| `waitmoncry` | missing |
| `bufferboxname` | missing |
| `textcolor` | implemented |
| `loadhelp` | missing |
| `unloadhelp` | missing |
| `signmsg` | missing |
| `normalmsg` | missing |
| `comparestat` | missing |
| `setmonmodernfatefulencounter` | missing |
| `checkmonmodernfatefulencounter` | missing |
| `trywondercardscript` | missing |
| `setworldmapflag` | implemented |
| `warpspinenter` | missing |
| `setmonmetlocation` | missing |
| `getbraillestringwidth` | missing |
| `bufferitemnameplural` | partially implemented |

## Source anchors

| Item | Path |
|---|---|
| VM dispatch | `rust-prototype/src/field.rs` (`fn command`, `match name`) |
| Host adapter | `rust-prototype/src/field_screen.rs` (`Step::Host` / `Host::Special`) |
| Actor movement limits | `rust-prototype/src/actors.rs` module docs (wandering deferred) |
| Provenance | `rust-prototype/data/field-provenance.json` |
| pret commands | `data/script_cmd_table.inc` @ `037335f4…` |
| pret specials | `data/specials.inc` @ `037335f4…` |

