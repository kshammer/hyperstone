use std::{
    io::{BufRead, BufReader, Read, Seek}, alloc::Global,
};
use hyperstone_proto::dota_proto::*;

use bitstream_io::{BitRead, BitReader, LittleEndian, BigEndian};
use tracing::{debug, info};

use crate::byte_utils::{get_message, read_varint, Peek};

pub fn parse_packet<R>(reader: &mut BufReader<R>)
where
    R: Read,
{
    let mut messages: Vec<Peek> = vec![];
    while reader.has_data_left().unwrap() {
        messages.push(read_packet_segment(reader));
        // next sort messages based on demo_packet.go OMEGALUL
    }
    for message in messages {
        decode_packet(message);
    }
}

pub fn read_bits<R>(reader: &mut BufReader<R>) -> u32
where
    R: Read,
{
    // each bit chunk is taken from a byte
    let mut r = BitReader::endian(reader, LittleEndian);

    let ret = r.read::<u32>(6).unwrap();

    info!("ret {}", ret); 

    let cool_ret = ret & 0x30;

    info!("Cool ret {}", cool_ret); 

    match cool_ret {
        16 => {
            return (ret & 15) | (r.read::<u32>(4).unwrap() << 4); 
        }
        32 => {
            let val = r.read::<u32>(8).unwrap();
            info!("val {}", val);
            return (ret & 15) | (val << 4); 
        }
        48 => {
            return (ret & 15) | (r.read::<u32>(28).unwrap() << 4); 

        }
        u32::MIN..u32::MAX => todo!(),
        u32::MAX => todo!(),
    }
}

pub fn read_packet_segment<R>(reader: &mut BufReader<R>) -> Peek
where
    R: Read,
{

    let kind = read_bits(reader); 
    info!("kind {}", kind);
    let mut varintbuf: Vec<u8, Global> = vec![0; 10];
    reader.read_exact(&mut varintbuf).unwrap();

    info!("peeking {:?}", varintbuf);
    
    let size = read_varint(reader).unwrap();
    info!("size {}", size);

    let message = get_message(reader, size, false);
    info!("message {:?}", message);

    Peek {
        _tick: 0,
        message_type: kind,
        _tell: 0,
        _size: size,
        message: message,
    }
}

pub fn decode_packet(message: Peek) {
    println!("Cool ass packet");
    let message_type = message.message_type as i32;

    match message_type {
        0..14 => {
            let net_message = NetMessages::from_i32(message_type).unwrap();
            match net_message {
                NetMessages::NetNop => todo!(),
                NetMessages::NetDisconnect => todo!(),
                NetMessages::NetSplitScreenUser => todo!(),
                NetMessages::NetTick => todo!(),
                NetMessages::NetStringCmd => todo!(),
                NetMessages::NetSetConVar => todo!(),
                NetMessages::NetSignonState => todo!(),
                NetMessages::NetSpawnGroupLoad => todo!(),
                NetMessages::NetSpawnGroupManifestUpdate => todo!(),
                NetMessages::NetSpawnGroupSetCreationTick => todo!(),
                NetMessages::NetSpawnGroupUnload => todo!(),
                NetMessages::NetSpawnGroupLoadCompleted => todo!(),
            }
        }
        40..73 => {
            let dota_message = SvcMessages::from_i32(message_type).unwrap();
            match dota_message {
                SvcMessages::SvcServerInfo => todo!(),
                SvcMessages::SvcFlattenedSerializer => todo!(),
                SvcMessages::SvcClassInfo => todo!(),
                SvcMessages::SvcSetPause => todo!(),
                SvcMessages::SvcCreateStringTable => todo!(),
                SvcMessages::SvcUpdateStringTable => todo!(),
                SvcMessages::SvcVoiceInit => todo!(),
                SvcMessages::SvcVoiceData => todo!(),
                SvcMessages::SvcPrint => todo!(),
                SvcMessages::SvcSounds => todo!(),
                SvcMessages::SvcSetView => todo!(),
                SvcMessages::SvcClearAllStringTables => todo!(),
                SvcMessages::SvcCmdKeyValues => todo!(),
                SvcMessages::SvcBspDecal => todo!(),
                SvcMessages::SvcSplitScreen => todo!(),
                SvcMessages::SvcPacketEntities => todo!(),
                SvcMessages::SvcPrefetch => todo!(),
                SvcMessages::SvcMenu => todo!(),
                SvcMessages::SvcGetCvarValue => todo!(),
                SvcMessages::SvcStopSound => todo!(),
                SvcMessages::SvcPeerList => todo!(),
                SvcMessages::SvcPacketReliable => todo!(),
                SvcMessages::SvcHltvStatus => todo!(),
                SvcMessages::SvcServerSteamId => todo!(),
                SvcMessages::SvcFullFrameSplit => todo!(),
                SvcMessages::SvcRconServerDetails => todo!(),
                SvcMessages::SvcUserMessage => todo!(),
            }
        }
        101..153 => {
            let base_user_message = EBaseUserMessages::from_i32(message_type).unwrap();
            match base_user_message {
                EBaseUserMessages::UmAchievementEvent => todo!(),
                EBaseUserMessages::UmCloseCaption => todo!(),
                EBaseUserMessages::UmCloseCaptionDirect => todo!(),
                EBaseUserMessages::UmCurrentTimescale => todo!(),
                EBaseUserMessages::UmDesiredTimescale => todo!(),
                EBaseUserMessages::UmFade => todo!(),
                EBaseUserMessages::UmGameTitle => todo!(),
                EBaseUserMessages::UmHudMsg => todo!(),
                EBaseUserMessages::UmHudText => todo!(),
                EBaseUserMessages::UmColoredText => todo!(),
                EBaseUserMessages::UmRequestState => todo!(),
                EBaseUserMessages::UmResetHud => todo!(),
                EBaseUserMessages::UmRumble => todo!(),
                EBaseUserMessages::UmSayText => todo!(),
                EBaseUserMessages::UmSayText2 => todo!(),
                EBaseUserMessages::UmSayTextChannel => todo!(),
                EBaseUserMessages::UmShake => todo!(),
                EBaseUserMessages::UmShakeDir => todo!(),
                EBaseUserMessages::UmTextMsg => todo!(),
                EBaseUserMessages::UmScreenTilt => todo!(),
                EBaseUserMessages::UmVoiceMask => todo!(),
                EBaseUserMessages::UmVoiceSubtitle => todo!(),
                EBaseUserMessages::UmSendAudio => todo!(),
                EBaseUserMessages::UmItemPickup => todo!(),
                EBaseUserMessages::UmAmmoDenied => todo!(),
                EBaseUserMessages::UmShowMenu => todo!(),
                EBaseUserMessages::UmCreditsMsg => todo!(),
                EBaseUserMessages::UmCloseCaptionPlaceholder => todo!(),
                EBaseUserMessages::UmCameraTransition => todo!(),
                EBaseUserMessages::UmAudioParameter => todo!(),
                EBaseUserMessages::UmParticleManager => todo!(),
                EBaseUserMessages::UmHudError => todo!(),
                EBaseUserMessages::UmCustomGameEvent => todo!(),
                EBaseUserMessages::UmAnimGraphUpdate => todo!(),
                EBaseUserMessages::UmHapticsManagerPulse => todo!(),
                EBaseUserMessages::UmHapticsManagerEffect => todo!(),
                EBaseUserMessages::UmCommandQueueState => todo!(),
                EBaseUserMessages::UmMaxBase => todo!(),
            }
        }
        200..213 => {
            let game_event = EBaseGameEvents::from_i32(message_type).unwrap();
            match game_event {
                EBaseGameEvents::GeVDebugGameSessionIdEvent => todo!(),
                EBaseGameEvents::GePlaceDecalEvent => todo!(),
                EBaseGameEvents::GeClearWorldDecalsEvent => todo!(),
                EBaseGameEvents::GeClearEntityDecalsEvent => todo!(),
                EBaseGameEvents::GeClearDecalsForSkeletonInstanceEvent => todo!(),
                EBaseGameEvents::GeSource1LegacyGameEventList => todo!(),
                EBaseGameEvents::GeSource1LegacyListenEvents => todo!(),
                EBaseGameEvents::GeSource1LegacyGameEvent => todo!(),
                EBaseGameEvents::GeSosStartSoundEvent => todo!(),
                EBaseGameEvents::GeSosStopSoundEvent => todo!(),
                EBaseGameEvents::GeSosSetSoundEventParams => todo!(),
                EBaseGameEvents::GeSosSetLibraryStackFields => todo!(),
                EBaseGameEvents::GeSosStopSoundEventHash => todo!(),
            }
        }
        464..613 => {
            let dota_user_message = EDotaUserMessages::from_i32(message_type).unwrap();
            match dota_user_message {
                EDotaUserMessages::DotaUmAddUnitToSelection => todo!(),
                EDotaUserMessages::DotaUmAiDebugLine => todo!(),
                EDotaUserMessages::DotaUmChatEvent => todo!(),
                EDotaUserMessages::DotaUmCombatHeroPositions => todo!(),
                EDotaUserMessages::DotaUmCombatLogData => todo!(),
                EDotaUserMessages::DotaUmCombatLogBulkData => todo!(),
                EDotaUserMessages::DotaUmCreateLinearProjectile => todo!(),
                EDotaUserMessages::DotaUmDestroyLinearProjectile => todo!(),
                EDotaUserMessages::DotaUmDodgeTrackingProjectiles => todo!(),
                EDotaUserMessages::DotaUmGlobalLightColor => todo!(),
                EDotaUserMessages::DotaUmGlobalLightDirection => todo!(),
                EDotaUserMessages::DotaUmInvalidCommand => todo!(),
                EDotaUserMessages::DotaUmLocationPing => todo!(),
                EDotaUserMessages::DotaUmMapLine => todo!(),
                EDotaUserMessages::DotaUmMiniKillCamInfo => todo!(),
                EDotaUserMessages::DotaUmMinimapDebugPoint => todo!(),
                EDotaUserMessages::DotaUmMinimapEvent => todo!(),
                EDotaUserMessages::DotaUmNevermoreRequiem => todo!(),
                EDotaUserMessages::DotaUmOverheadEvent => todo!(),
                EDotaUserMessages::DotaUmSetNextAutobuyItem => todo!(),
                EDotaUserMessages::DotaUmSharedCooldown => todo!(),
                EDotaUserMessages::DotaUmSpectatorPlayerClick => todo!(),
                EDotaUserMessages::DotaUmTutorialTipInfo => todo!(),
                EDotaUserMessages::DotaUmUnitEvent => todo!(),
                EDotaUserMessages::DotaUmParticleManager => todo!(),
                EDotaUserMessages::DotaUmBotChat => todo!(),
                EDotaUserMessages::DotaUmHudError => todo!(),
                EDotaUserMessages::DotaUmItemPurchased => todo!(),
                EDotaUserMessages::DotaUmPing => todo!(),
                EDotaUserMessages::DotaUmItemFound => todo!(),
                EDotaUserMessages::DotaUmCharacterSpeakConcept => todo!(),
                EDotaUserMessages::DotaUmSwapVerify => todo!(),
                EDotaUserMessages::DotaUmWorldLine => todo!(),
                EDotaUserMessages::DotaUmTournamentDrop => todo!(),
                EDotaUserMessages::DotaUmItemAlert => todo!(),
                EDotaUserMessages::DotaUmHalloweenDrops => todo!(),
                EDotaUserMessages::DotaUmChatWheel => todo!(),
                EDotaUserMessages::DotaUmReceivedXmasGift => todo!(),
                EDotaUserMessages::DotaUmUpdateSharedContent => todo!(),
                EDotaUserMessages::DotaUmTutorialRequestExp => todo!(),
                EDotaUserMessages::DotaUmTutorialPingMinimap => todo!(),
                EDotaUserMessages::DotaUmGamerulesStateChanged => {
                    info!("Game rules state change");
                },
                EDotaUserMessages::DotaUmShowSurvey => todo!(),
                EDotaUserMessages::DotaUmTutorialFade => todo!(),
                EDotaUserMessages::DotaUmAddQuestLogEntry => todo!(),
                EDotaUserMessages::DotaUmSendStatPopup => todo!(),
                EDotaUserMessages::DotaUmTutorialFinish => todo!(),
                EDotaUserMessages::DotaUmSendRoshanPopup => todo!(),
                EDotaUserMessages::DotaUmSendGenericToolTip => todo!(),
                EDotaUserMessages::DotaUmSendFinalGold => todo!(),
                EDotaUserMessages::DotaUmCustomMsg => todo!(),
                EDotaUserMessages::DotaUmCoachHudPing => todo!(),
                EDotaUserMessages::DotaUmClientLoadGridNav => todo!(),
                EDotaUserMessages::DotaUmTeProjectile => todo!(),
                EDotaUserMessages::DotaUmTeProjectileLoc => todo!(),
                EDotaUserMessages::DotaUmTeDotaBloodImpact => todo!(),
                EDotaUserMessages::DotaUmTeUnitAnimation => todo!(),
                EDotaUserMessages::DotaUmTeUnitAnimationEnd => todo!(),
                EDotaUserMessages::DotaUmAbilityPing => todo!(),
                EDotaUserMessages::DotaUmShowGenericPopup => todo!(),
                EDotaUserMessages::DotaUmVoteStart => todo!(),
                EDotaUserMessages::DotaUmVoteUpdate => todo!(),
                EDotaUserMessages::DotaUmVoteEnd => todo!(),
                EDotaUserMessages::DotaUmBoosterState => todo!(),
                EDotaUserMessages::DotaUmWillPurchaseAlert => todo!(),
                EDotaUserMessages::DotaUmTutorialMinimapPosition => todo!(),
                EDotaUserMessages::DotaUmPlayerMmr => todo!(),
                EDotaUserMessages::DotaUmAbilitySteal => todo!(),
                EDotaUserMessages::DotaUmCourierKilledAlert => todo!(),
                EDotaUserMessages::DotaUmEnemyItemAlert => todo!(),
                EDotaUserMessages::DotaUmStatsMatchDetails => todo!(),
                EDotaUserMessages::DotaUmMiniTaunt => todo!(),
                EDotaUserMessages::DotaUmBuyBackStateAlert => todo!(),
                EDotaUserMessages::DotaUmSpeechBubble => todo!(),
                EDotaUserMessages::DotaUmCustomHeaderMessage => todo!(),
                EDotaUserMessages::DotaUmQuickBuyAlert => todo!(),
                EDotaUserMessages::DotaUmStatsHeroDetails => todo!(),
                EDotaUserMessages::DotaUmPredictionResult => todo!(),
                EDotaUserMessages::DotaUmModifierAlert => todo!(),
                EDotaUserMessages::DotaUmHpManaAlert => todo!(),
                EDotaUserMessages::DotaUmGlyphAlert => todo!(),
                EDotaUserMessages::DotaUmBeastChat => todo!(),
                EDotaUserMessages::DotaUmSpectatorPlayerUnitOrders => todo!(),
                EDotaUserMessages::DotaUmCustomHudElementCreate => todo!(),
                EDotaUserMessages::DotaUmCustomHudElementModify => todo!(),
                EDotaUserMessages::DotaUmCustomHudElementDestroy => todo!(),
                EDotaUserMessages::DotaUmCompendiumState => todo!(),
                EDotaUserMessages::DotaUmProjectionAbility => todo!(),
                EDotaUserMessages::DotaUmProjectionEvent => todo!(),
                EDotaUserMessages::DotaUmCombatLogDataHltv => todo!(),
                EDotaUserMessages::DotaUmXpAlert => todo!(),
                EDotaUserMessages::DotaUmUpdateQuestProgress => todo!(),
                EDotaUserMessages::DotaUmMatchMetadata => todo!(),
                EDotaUserMessages::DotaUmMatchDetails => todo!(),
                EDotaUserMessages::DotaUmQuestStatus => todo!(),
                EDotaUserMessages::DotaUmSuggestHeroPick => todo!(),
                EDotaUserMessages::DotaUmSuggestHeroRole => todo!(),
                EDotaUserMessages::DotaUmKillcamDamageTaken => todo!(),
                EDotaUserMessages::DotaUmSelectPenaltyGold => todo!(),
                EDotaUserMessages::DotaUmRollDiceResult => todo!(),
                EDotaUserMessages::DotaUmFlipCoinResult => todo!(),
                EDotaUserMessages::DotaUmRequestItemSuggestions => todo!(),
                EDotaUserMessages::DotaUmTeamCaptainChanged => todo!(),
                EDotaUserMessages::DotaUmSendRoshanSpectatorPhase => todo!(),
                EDotaUserMessages::DotaUmChatWheelCooldown => todo!(),
                EDotaUserMessages::DotaUmDismissAllStatPopups => todo!(),
                EDotaUserMessages::DotaUmTeDestroyProjectile => todo!(),
                EDotaUserMessages::DotaUmHeroRelicProgress => todo!(),
                EDotaUserMessages::DotaUmAbilityDraftRequestAbility => todo!(),
                EDotaUserMessages::DotaUmItemSold => todo!(),
                EDotaUserMessages::DotaUmDamageReport => todo!(),
                EDotaUserMessages::DotaUmSalutePlayer => todo!(),
                EDotaUserMessages::DotaUmTipAlert => todo!(),
                EDotaUserMessages::DotaUmReplaceQueryUnit => todo!(),
                EDotaUserMessages::DotaUmEmptyTeleportAlert => todo!(),
                EDotaUserMessages::DotaUmMarsArenaOfBloodAttack => todo!(),
                EDotaUserMessages::DotaUmEsArcanaCombo => todo!(),
                EDotaUserMessages::DotaUmEsArcanaComboSummary => todo!(),
                EDotaUserMessages::DotaUmHighFiveLeftHanging => todo!(),
                EDotaUserMessages::DotaUmHighFiveCompleted => todo!(),
                EDotaUserMessages::DotaUmShovelUnearth => todo!(),
                EDotaUserMessages::DotaEmInvokerSpellCast => todo!(),
                EDotaUserMessages::DotaUmRadarAlert => todo!(),
                EDotaUserMessages::DotaUmAllStarEvent => todo!(),
                EDotaUserMessages::DotaUmTalentTreeAlert => todo!(),
                EDotaUserMessages::DotaUmQueuedOrderRemoved => todo!(),
                EDotaUserMessages::DotaUmDebugChallenge => todo!(),
                EDotaUserMessages::DotaUmOmArcanaCombo => todo!(),
                EDotaUserMessages::DotaUmFoundNeutralItem => todo!(),
                EDotaUserMessages::DotaUmOutpostCaptured => todo!(),
                EDotaUserMessages::DotaUmOutpostGrantedXp => todo!(),
                EDotaUserMessages::DotaUmMoveCameraToUnit => todo!(),
                EDotaUserMessages::DotaUmPauseMinigameData => todo!(),
                EDotaUserMessages::DotaUmVersusScenePlayerBehavior => todo!(),
                EDotaUserMessages::DotaUmQoPArcanaSummary => todo!(),
                EDotaUserMessages::DotaUmHotPotatoCreated => todo!(),
                EDotaUserMessages::DotaUmHotPotatoExploded => todo!(),
                EDotaUserMessages::DotaUmWkArcanaProgress => todo!(),
                EDotaUserMessages::DotaUmGuildChallengeProgress => todo!(),
                EDotaUserMessages::DotaUmWrArcanaProgress => todo!(),
                EDotaUserMessages::DotaUmWrArcanaSummary => todo!(),
                EDotaUserMessages::DotaUmEmptyItemSlotAlert => todo!(),
                EDotaUserMessages::DotaUmAghsStatusAlert => todo!(),
                EDotaUserMessages::DotaUmPingConfirmation => todo!(),
                EDotaUserMessages::DotaUmMutedPlayers => todo!(),
                EDotaUserMessages::DotaUmContextualTip => todo!(),
                EDotaUserMessages::DotaUmChatMessage => todo!(),
            }
        }
        i32::MIN..=-1_i32 | 13_i32..=i32::MAX => todo!(),
    }
}
