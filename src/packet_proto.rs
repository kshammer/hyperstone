use bytes::Bytes;
use hyperstone_proto::dota_proto::*;
use std::{alloc::Global, io::Cursor};

use bitstream_io::{BitRead, BitReader, LittleEndian};
use tracing::{debug, info};

use crate::byte_utils::Peek;

pub fn parse_packet(reader: &mut BitReader<Cursor<&Bytes>, LittleEndian>, size: u32) {
    let mut messages: Vec<Peek> = vec![];
    while size - reader.position_in_bits().unwrap() as u32 > 8 {
        messages.push(read_packet_segment(reader));
    }
    for message in messages {
        decode_packet(message);
    }
}

pub fn read_bits(reader: &mut BitReader<Cursor<&Bytes>, LittleEndian>) -> u32 {
    // each bit chunk is taken from a byte
    let ret = reader.read::<u32>(6).unwrap();

    debug!("ret {}", ret);

    let cool_ret = ret & 0x30;

    debug!("Cool ret {}", cool_ret);

    match cool_ret {
        16 => {
            return (ret & 15) | (reader.read::<u32>(4).unwrap() << 4);
        }
        32 => {
            return (ret & 15) | (reader.read::<u32>(8).unwrap() << 4);
        }
        48 => {
            return (ret & 15) | (reader.read::<u32>(28).unwrap() << 4);
        }
        u32::MIN..u32::MAX => return ret,
        u32::MAX => todo!()
    }
}

pub fn read_packet_segment(reader: &mut BitReader<Cursor<&Bytes>, LittleEndian>) -> Peek {
    let kind = read_bits(reader);
    debug!("kind {}", kind);

    let size = read_varint_bit(reader);
    debug!("size {}", size);

    let message = get_message_bit(reader, size);
    debug!("message {:?}", message);

    Peek {
        _tick: 0,
        message_type: kind,
        _tell: 0,
        _size: size,
        message: message,
    }
}

pub fn read_varint_bit(reader: &mut BitReader<Cursor<&Bytes>, LittleEndian>) -> u32 {
    let mut count = 0;
    let mut result = 0 as u32;
    loop {
        let byte = reader.read::<u8>(8).unwrap();
        result |= (byte as u32 & 0x7f) << (7 * count);
        count += 1;
        if byte & 0x80 == 0 || 7 * count == 35 {
            return result;
        }
    }
}

pub fn get_message_bit(reader: &mut BitReader<Cursor<&Bytes>, LittleEndian>, size: u32) -> Bytes {
    let mut message: Vec<u8, Global> = vec![0; size.try_into().unwrap()];
    reader.read_bytes(&mut message).unwrap();
    let bytes = Bytes::from(message);
    bytes
}

pub fn decode_packet(message: Peek) {
    let message_type = message.message_type as i32;

    match message_type {
        0..14 => {
            let net_message = NetMessages::from_i32(message_type).unwrap();
            match net_message {
                NetMessages::NetNop => {
                    info!("Net Nop");
                }
                NetMessages::NetDisconnect => {
                    info!("Net Disconnect");
                }
                NetMessages::NetSplitScreenUser => {
                    info!("Net Split Screen User");
                }
                NetMessages::NetTick => {
                    info!("Net Tick");
                }
                NetMessages::NetStringCmd => {
                    info!("Net String Ccmd");
                }
                NetMessages::NetSetConVar => {
                    info!("Net Set Con Var");
                }
                NetMessages::NetSignonState => {
                    info!("Net Signon State");
                }
                NetMessages::NetSpawnGroupLoad => {
                    info!("Net Spawn Group Load");
                }
                NetMessages::NetSpawnGroupManifestUpdate => {
                    info!("Net Spawn Group Manifest Update");
                }
                NetMessages::NetSpawnGroupSetCreationTick => {
                    info!("Net Spawn Group Set Creation Tick");
                }
                NetMessages::NetSpawnGroupUnload => {
                    info!("Net Spawn Group Unload");
                }
                NetMessages::NetSpawnGroupLoadCompleted => {
                    info!("Net Spawn Group Load Completed");
                }
            }
        }
        40..73 => {
            let dota_message = SvcMessages::from_i32(message_type).unwrap();
            match dota_message {
                SvcMessages::SvcServerInfo => {
                    info!("Svc Server info");
                }
                SvcMessages::SvcFlattenedSerializer => {
                    info!("Svc Flattened Serializer");
                }
                SvcMessages::SvcClassInfo => {
                    info!("Svc Class info");
                }
                SvcMessages::SvcSetPause => {
                    info!("Svc Set pause");
                }
                SvcMessages::SvcCreateStringTable => {
                    info!("Svc Create String table");
                }
                SvcMessages::SvcUpdateStringTable => {
                    info!("Svc update string table");
                }
                SvcMessages::SvcVoiceInit => {
                    info!("Svc voice init");
                }
                SvcMessages::SvcVoiceData => {
                    info!("Svc voice data");
                }
                SvcMessages::SvcPrint => {
                    info!("svc print");
                }
                SvcMessages::SvcSounds => {
                    info!("Svc sounds");
                }
                SvcMessages::SvcSetView => {
                    info!("Svc set view");
                }
                SvcMessages::SvcClearAllStringTables => {
                    info!("Svc clear all string tables");
                }
                SvcMessages::SvcCmdKeyValues => {
                    info!("svc cmd key values");
                }
                SvcMessages::SvcBspDecal => {
                    info!("svc bsp decal");
                }
                SvcMessages::SvcSplitScreen => {
                    info!("svc split screen");
                }
                SvcMessages::SvcPacketEntities => {
                    info!("svc packet entities");
                }
                SvcMessages::SvcPrefetch => {
                    info!("svc refresh");
                }
                SvcMessages::SvcMenu => {
                    info!("svc menu");
                }
                SvcMessages::SvcGetCvarValue => {
                    info!("svc get cvar value");
                }
                SvcMessages::SvcStopSound => {
                    info!("svc stop sound");
                }
                SvcMessages::SvcPeerList => {
                    info!("svc peer list");
                }
                SvcMessages::SvcPacketReliable => {
                    info!("svc packet reliable");
                }
                SvcMessages::SvcHltvStatus => {
                    info!("svc hltv status");
                }
                SvcMessages::SvcServerSteamId => {
                    info!("svc server steam id");
                }
                SvcMessages::SvcFullFrameSplit => {
                    info!("svc full frame split");
                }
                SvcMessages::SvcRconServerDetails => {
                    info!("svc recon server details");
                }
                SvcMessages::SvcUserMessage => {
                    info!("svc user message");
                }
            }
        }
        101..153 => {
            let base_user_message = EBaseUserMessages::from_i32(message_type).unwrap();
            match base_user_message {
                EBaseUserMessages::UmAchievementEvent => {
                    info!("um achievemnet event");
                }
                EBaseUserMessages::UmCloseCaption => {
                    info!("close caption");
                }
                EBaseUserMessages::UmCloseCaptionDirect => {
                    info!("close caption direct");
                }
                EBaseUserMessages::UmCurrentTimescale => {
                    info!("current timescael");
                }
                EBaseUserMessages::UmDesiredTimescale => {
                    info!("desired timescael");
                }
                EBaseUserMessages::UmFade => {
                    info!("fade");
                }
                EBaseUserMessages::UmGameTitle => {
                    info!("game tile ");
                }
                EBaseUserMessages::UmHudMsg => {
                    info!("hud msg");
                }
                EBaseUserMessages::UmHudText => {
                    info!("hud text");
                }
                EBaseUserMessages::UmColoredText => {
                    info!("colored text");
                }
                EBaseUserMessages::UmRequestState => {
                    info!("request state");
                }
                EBaseUserMessages::UmResetHud => {
                    info!("resest hud");
                }
                EBaseUserMessages::UmRumble => {
                    info!("rumble");
                }
                EBaseUserMessages::UmSayText => {
                    info!("say text");
                }
                EBaseUserMessages::UmSayText2 => {
                    info!("say text 2");
                }
                EBaseUserMessages::UmSayTextChannel => {
                    info!("say text channel");
                }
                EBaseUserMessages::UmShake => {
                    info!("shake");
                }
                EBaseUserMessages::UmShakeDir => {
                    info!("shake dir");
                }
                EBaseUserMessages::UmTextMsg => {
                    info!("text mmsg");
                }
                EBaseUserMessages::UmScreenTilt => {
                    info!("screen tilt");
                }
                EBaseUserMessages::UmVoiceMask => {
                    info!("voice mask");
                }
                EBaseUserMessages::UmVoiceSubtitle => {
                    info!("voice subtitle");
                }
                EBaseUserMessages::UmSendAudio => {
                    info!("send audio");
                }
                EBaseUserMessages::UmItemPickup => {
                    info!("item pickup");
                }
                EBaseUserMessages::UmAmmoDenied => {
                    info!("ammo denied");
                }
                EBaseUserMessages::UmShowMenu => {
                    info!("show menu");
                }
                EBaseUserMessages::UmCreditsMsg => {
                    info!("credits msg");
                }
                EBaseUserMessages::UmCloseCaptionPlaceholder => {
                    info!("close capiton placeholder");
                }
                EBaseUserMessages::UmCameraTransition => {
                    info!("camera transition");
                }
                EBaseUserMessages::UmAudioParameter => {
                    info!("audio parameter");
                }
                EBaseUserMessages::UmParticleManager => {
                    info!("particle manager");
                }
                EBaseUserMessages::UmHudError => {
                    info!("hud errors");
                }
                EBaseUserMessages::UmCustomGameEvent => {
                    info!("custom game event");
                }
                EBaseUserMessages::UmAnimGraphUpdate => {
                    info!("anim graph update");
                }
                EBaseUserMessages::UmHapticsManagerPulse => {
                    info!("haptics manager pulse");
                }
                EBaseUserMessages::UmHapticsManagerEffect => {
                    info!("haptics manager effect");
                }
                EBaseUserMessages::UmCommandQueueState => {
                    info!("Command queue state");
                }
                EBaseUserMessages::UmMaxBase => {
                    info!("Max abse");
                }
            }
        }
        200..213 => {
            let game_event = EBaseGameEvents::from_i32(message_type).unwrap();
            match game_event {
                EBaseGameEvents::GeVDebugGameSessionIdEvent => {
                    info!("debug game session id event");
                }
                EBaseGameEvents::GePlaceDecalEvent => {
                    info!("Place decal");
                }
                EBaseGameEvents::GeClearWorldDecalsEvent => {
                    info!("clear world decals");
                }
                EBaseGameEvents::GeClearEntityDecalsEvent => {
                    info!("entity decals");
                }
                EBaseGameEvents::GeClearDecalsForSkeletonInstanceEvent => {
                    info!("clean decals for skeleton instance");
                }
                EBaseGameEvents::GeSource1LegacyGameEventList => {
                    info!("souce 1 legacy game events list");
                }
                EBaseGameEvents::GeSource1LegacyListenEvents => {
                    info!("Souce 1 listen events");
                }
                EBaseGameEvents::GeSource1LegacyGameEvent => {
                    info!("legacy game event");
                }
                EBaseGameEvents::GeSosStartSoundEvent => {
                    info!("sos start sound event");
                }
                EBaseGameEvents::GeSosStopSoundEvent => {
                    info!("sos stop sound event");
                }
                EBaseGameEvents::GeSosSetSoundEventParams => {
                    info!("sos set sound event params");
                }
                EBaseGameEvents::GeSosSetLibraryStackFields => {
                    info!("sos set library stack fields ");
                }
                EBaseGameEvents::GeSosStopSoundEventHash => {
                    info!("sos stop sound event hash");
                }
            }
        }
        464..613 => {
            let dota_user_message = EDotaUserMessages::from_i32(message_type).unwrap();
            match dota_user_message {
                EDotaUserMessages::DotaUmAddUnitToSelection => {
                    info!("um add unit to selection");
                }
                EDotaUserMessages::DotaUmAiDebugLine => {
                    info!("ai debug line");
                }
                EDotaUserMessages::DotaUmChatEvent => {
                    info!("chat event");
                }
                EDotaUserMessages::DotaUmCombatHeroPositions => {
                    info!("combat hero positions");
                }
                EDotaUserMessages::DotaUmCombatLogData => {
                    info!("combat log data");
                }
                EDotaUserMessages::DotaUmCombatLogBulkData => {
                    info!("combat log bulk data");
                }
                EDotaUserMessages::DotaUmCreateLinearProjectile => {
                    info!("create linear projectile");
                }
                EDotaUserMessages::DotaUmDestroyLinearProjectile => {
                    info!("destory linear projectile");
                }
                EDotaUserMessages::DotaUmDodgeTrackingProjectiles => {
                    info!("dodge tracking projectiles");
                }
                EDotaUserMessages::DotaUmGlobalLightColor => {
                    info!("globla light color");
                }
                EDotaUserMessages::DotaUmGlobalLightDirection => {
                    info!("global light direction");
                }
                EDotaUserMessages::DotaUmInvalidCommand => {
                    info!("invalid command");
                }
                EDotaUserMessages::DotaUmLocationPing => {
                    info!("location ping");
                }
                EDotaUserMessages::DotaUmMapLine => {
                    info!("map line");
                }
                EDotaUserMessages::DotaUmMiniKillCamInfo => {
                    info!("mini kill cam info");
                }
                EDotaUserMessages::DotaUmMinimapDebugPoint => {
                    info!("minimap debug point");
                }
                EDotaUserMessages::DotaUmMinimapEvent => {
                    info!("minimap event");
                }
                EDotaUserMessages::DotaUmNevermoreRequiem => {
                    info!("nevermore requiem");
                }
                EDotaUserMessages::DotaUmOverheadEvent => {
                    info!("overhead event");
                }
                EDotaUserMessages::DotaUmSetNextAutobuyItem => {
                    info!("set next autobuy item");
                }
                EDotaUserMessages::DotaUmSharedCooldown => {
                    info!("shared cooldown");
                }
                EDotaUserMessages::DotaUmSpectatorPlayerClick => {
                    info!("spectator player click");
                }
                EDotaUserMessages::DotaUmTutorialTipInfo => {
                    info!("tutorial tip info");
                }
                EDotaUserMessages::DotaUmUnitEvent => {
                    info!("unit event");
                }
                EDotaUserMessages::DotaUmParticleManager => {
                    info!("particle manager");
                }
                EDotaUserMessages::DotaUmBotChat => {
                    info!("bot chat");
                }
                EDotaUserMessages::DotaUmHudError => {
                    info!("hud error");
                }
                EDotaUserMessages::DotaUmItemPurchased => {
                    info!("itme purchased");
                }
                EDotaUserMessages::DotaUmPing => {
                    info!("ping");
                }
                EDotaUserMessages::DotaUmItemFound => {
                    info!("itme found");
                }
                EDotaUserMessages::DotaUmCharacterSpeakConcept => {
                    info!("character speak concept ");
                }
                EDotaUserMessages::DotaUmSwapVerify => {
                    info!("swap verify");
                }
                EDotaUserMessages::DotaUmWorldLine => {
                    info!("world line");
                }
                EDotaUserMessages::DotaUmTournamentDrop => {
                    info!("tournament drop");
                }
                EDotaUserMessages::DotaUmItemAlert => {
                    info!("item alert");
                }
                EDotaUserMessages::DotaUmHalloweenDrops => {
                    info!("halloween drops");
                }
                EDotaUserMessages::DotaUmChatWheel => {
                    info!("caht wheel");
                }
                EDotaUserMessages::DotaUmReceivedXmasGift => {
                    info!("received xmas gift");
                }
                EDotaUserMessages::DotaUmUpdateSharedContent => {
                    info!("update shared content");
                }
                EDotaUserMessages::DotaUmTutorialRequestExp => {
                    info!("turotiral request exp");
                }
                EDotaUserMessages::DotaUmTutorialPingMinimap => {
                    info!("tutorial ping map");
                }
                EDotaUserMessages::DotaUmGamerulesStateChanged => {
                    info!("Game rules state change");
                }
                EDotaUserMessages::DotaUmShowSurvey => {
                    info!("show survey");
                }
                EDotaUserMessages::DotaUmTutorialFade => {
                    info!("turutal fade");
                }
                EDotaUserMessages::DotaUmAddQuestLogEntry => {
                    info!("add quest log entry");
                }
                EDotaUserMessages::DotaUmSendStatPopup => {
                    info!("send stat popup");
                }
                EDotaUserMessages::DotaUmTutorialFinish => {
                    info!("tutorial finish");
                }
                EDotaUserMessages::DotaUmSendRoshanPopup => {
                    info!("send roshan popup");
                }
                EDotaUserMessages::DotaUmSendGenericToolTip => {
                    info!("send generic tool tip");
                }
                EDotaUserMessages::DotaUmSendFinalGold => {
                    info!("send final gold");
                }
                EDotaUserMessages::DotaUmCustomMsg => {
                    info!("custom msg");
                }
                EDotaUserMessages::DotaUmCoachHudPing => {
                    info!("coach hud ping");
                }
                EDotaUserMessages::DotaUmClientLoadGridNav => {
                    info!("client load grid nav");
                }
                EDotaUserMessages::DotaUmTeProjectile => {
                    info!("te projectile");
                }
                EDotaUserMessages::DotaUmTeProjectileLoc => {
                    info!("te projectile loc");
                }
                EDotaUserMessages::DotaUmTeDotaBloodImpact => {
                    info!("te dota blood impcat");
                }
                EDotaUserMessages::DotaUmTeUnitAnimation => {
                    info!("te unit animation");
                }
                EDotaUserMessages::DotaUmTeUnitAnimationEnd => {
                    info!("unit animation end");
                }
                EDotaUserMessages::DotaUmAbilityPing => {
                    info!("ability ping");
                }
                EDotaUserMessages::DotaUmShowGenericPopup => {
                    info!("show generic popup");
                }
                EDotaUserMessages::DotaUmVoteStart => {
                    info!("vote start");
                }
                EDotaUserMessages::DotaUmVoteUpdate => {
                    info!("vote update");
                }
                EDotaUserMessages::DotaUmVoteEnd => {
                    info!("vote ended");
                }
                EDotaUserMessages::DotaUmBoosterState => {
                    info!("booster state");
                }
                EDotaUserMessages::DotaUmWillPurchaseAlert => {
                    info!("will purchase alert");
                }
                EDotaUserMessages::DotaUmTutorialMinimapPosition => {
                    info!("minimap position tutorial ");
                }
                EDotaUserMessages::DotaUmPlayerMmr => {
                    info!("player mmr");
                }
                EDotaUserMessages::DotaUmAbilitySteal => {
                    info!("ability steal ");
                }
                EDotaUserMessages::DotaUmCourierKilledAlert => {
                    info!("courier killed alert");
                }
                EDotaUserMessages::DotaUmEnemyItemAlert => {
                    info!("enemy item alert");
                }
                EDotaUserMessages::DotaUmStatsMatchDetails => {
                    info!("stat match detials");
                }
                EDotaUserMessages::DotaUmMiniTaunt => {
                    info!("mini taunt");
                }
                EDotaUserMessages::DotaUmBuyBackStateAlert => {
                    info!("buy back state alert");
                }
                EDotaUserMessages::DotaUmSpeechBubble => {
                    info!("speech buble");
                }
                EDotaUserMessages::DotaUmCustomHeaderMessage => {
                    info!("custom header message");
                }
                EDotaUserMessages::DotaUmQuickBuyAlert => {
                    info!("quick buy alert");
                }
                EDotaUserMessages::DotaUmStatsHeroDetails => {
                    info!("stats hero details ");
                }
                EDotaUserMessages::DotaUmPredictionResult => {
                    info!("prediciton result");
                }
                EDotaUserMessages::DotaUmModifierAlert => {
                    info!("modifier alert");
                }
                EDotaUserMessages::DotaUmHpManaAlert => {
                    info!("hp mana alert");
                }
                EDotaUserMessages::DotaUmGlyphAlert => {
                    info!("glyph alert");
                }
                EDotaUserMessages::DotaUmBeastChat => {
                    info!("beast chat");
                }
                EDotaUserMessages::DotaUmSpectatorPlayerUnitOrders => {
                    info!("spec player unit orders");
                }
                EDotaUserMessages::DotaUmCustomHudElementCreate => {
                    info!("custom hud element creaete");
                }
                EDotaUserMessages::DotaUmCustomHudElementModify => {
                    info!("custom hud element modify");
                }
                EDotaUserMessages::DotaUmCustomHudElementDestroy => {
                    info!("Custom hud element destroy ");
                }
                EDotaUserMessages::DotaUmCompendiumState => {
                    info!("Compendium states");
                }
                EDotaUserMessages::DotaUmProjectionAbility => {
                    info!("projection ability");
                }
                EDotaUserMessages::DotaUmProjectionEvent => {
                    info!("projection event");
                }
                EDotaUserMessages::DotaUmCombatLogDataHltv => {
                    info!("Combat Log")
                }
                EDotaUserMessages::DotaUmXpAlert => {
                    info!("xp alert");
                }
                EDotaUserMessages::DotaUmUpdateQuestProgress => {
                    info!("update quest progress");
                }
                EDotaUserMessages::DotaUmMatchMetadata => {
                    info!("match metadata");
                }
                EDotaUserMessages::DotaUmMatchDetails => {
                    info!("match details ");
                }
                EDotaUserMessages::DotaUmQuestStatus => {
                    info!("quest status");
                }
                EDotaUserMessages::DotaUmSuggestHeroPick => {
                    info!("suggest hero pick");
                }
                EDotaUserMessages::DotaUmSuggestHeroRole => {
                    info!("suggest hero role");
                }
                EDotaUserMessages::DotaUmKillcamDamageTaken => {
                    info!("kill cam damage taken");
                }
                EDotaUserMessages::DotaUmSelectPenaltyGold => {
                    info!("select penatly gold");
                }
                EDotaUserMessages::DotaUmRollDiceResult => {
                    info!("roll dice result");
                }
                EDotaUserMessages::DotaUmFlipCoinResult => {
                    info!("flip coin result");
                }
                EDotaUserMessages::DotaUmRequestItemSuggestions => {
                    info!("request item suggestions");
                }
                EDotaUserMessages::DotaUmTeamCaptainChanged => {
                    info!("team captian changed");
                }
                EDotaUserMessages::DotaUmSendRoshanSpectatorPhase => {
                    info!("send roshan spectator phase");
                }
                EDotaUserMessages::DotaUmChatWheelCooldown => {
                    info!("chatwheel cooldown");
                }
                EDotaUserMessages::DotaUmDismissAllStatPopups => {
                    info!("dismiss all stat popups");
                }
                EDotaUserMessages::DotaUmTeDestroyProjectile => {
                    info!("destory projectile ");
                }
                EDotaUserMessages::DotaUmHeroRelicProgress => {
                    info!("hero relic progress");
                }
                EDotaUserMessages::DotaUmAbilityDraftRequestAbility => {
                    info!("draft request ability");
                }
                EDotaUserMessages::DotaUmItemSold => {
                    info!("item sold");
                }
                EDotaUserMessages::DotaUmDamageReport => {
                    info!("damage report");
                }
                EDotaUserMessages::DotaUmSalutePlayer => {
                    info!("salute player");
                }
                EDotaUserMessages::DotaUmTipAlert => {
                    info!("tip alert");
                }
                EDotaUserMessages::DotaUmReplaceQueryUnit => {
                    info!("replace query unit");
                }
                EDotaUserMessages::DotaUmEmptyTeleportAlert => {
                    info!("empty teleport alert");
                }
                EDotaUserMessages::DotaUmMarsArenaOfBloodAttack => {
                    info!("arena of blood attack ");
                }
                EDotaUserMessages::DotaUmEsArcanaCombo => {
                    info!("arcanan combo");
                }
                EDotaUserMessages::DotaUmEsArcanaComboSummary => {
                    info!("es arcana combo summ");
                }
                EDotaUserMessages::DotaUmHighFiveLeftHanging => {
                    info!("high five left hanging");
                }
                EDotaUserMessages::DotaUmHighFiveCompleted => {
                    info!("high five completed");
                }
                EDotaUserMessages::DotaUmShovelUnearth => {
                    info!("shovel unearth");
                }
                EDotaUserMessages::DotaEmInvokerSpellCast => {
                    info!("invoker spell cast");
                }
                EDotaUserMessages::DotaUmRadarAlert => {
                    info!("radar alert");
                }
                EDotaUserMessages::DotaUmAllStarEvent => {
                    info!("all star event");
                }
                EDotaUserMessages::DotaUmTalentTreeAlert => {
                    info!("talent tree alert");
                }
                EDotaUserMessages::DotaUmQueuedOrderRemoved => {
                    info!("queu order ermoved");
                }
                EDotaUserMessages::DotaUmDebugChallenge => {
                    info!("debug challenge");
                }
                EDotaUserMessages::DotaUmOmArcanaCombo => {
                    info!("arcana combo");
                }
                EDotaUserMessages::DotaUmFoundNeutralItem => {
                    info!("found neutral item ");
                }
                EDotaUserMessages::DotaUmOutpostCaptured => {
                    info!("outpost captured");
                }
                EDotaUserMessages::DotaUmOutpostGrantedXp => {
                    info!("outpost grant xp");
                }
                EDotaUserMessages::DotaUmMoveCameraToUnit => {
                    info!("move camera to unit");
                }
                EDotaUserMessages::DotaUmPauseMinigameData => {
                    info!("pause minigame data");
                }
                EDotaUserMessages::DotaUmVersusScenePlayerBehavior => {
                    info!("versus scene player behavoir");
                }
                EDotaUserMessages::DotaUmQoPArcanaSummary => {
                    info!("qop arcana summary");
                }
                EDotaUserMessages::DotaUmHotPotatoCreated => {
                    info!("hot potato created");
                }
                EDotaUserMessages::DotaUmHotPotatoExploded => {
                    info!("hot potato exploded");
                }
                EDotaUserMessages::DotaUmWkArcanaProgress => {
                    info!("wk arcana progress");
                }
                EDotaUserMessages::DotaUmGuildChallengeProgress => {
                    info!("guild challenge progress");
                }
                EDotaUserMessages::DotaUmWrArcanaProgress => {
                    info!("wr arcana progress");
                }
                EDotaUserMessages::DotaUmWrArcanaSummary => {
                    info!("wr arcana suammary ");
                }
                EDotaUserMessages::DotaUmEmptyItemSlotAlert => {
                    info!("empty item slot alert ");
                }
                EDotaUserMessages::DotaUmAghsStatusAlert => {
                    info!("aghs status alert");
                }
                EDotaUserMessages::DotaUmPingConfirmation => {
                    info!("ping ocnfirmatin");
                }
                EDotaUserMessages::DotaUmMutedPlayers => {
                    info!("muted palyer");
                }
                EDotaUserMessages::DotaUmContextualTip => {
                    info!("contextual tip");
                }
                EDotaUserMessages::DotaUmChatMessage => {
                    info!("chat message");
                }
            }
        }
        i32::MIN..=-1_i32 | 13_i32..=i32::MAX => {
            info!("");
        }
    }
}
