pub(crate) mod addon;
pub use addon::*;
pub(crate) mod addon_type;
pub use addon_type::*;
pub(crate) mod area;
pub use area::*;
pub(crate) mod auction_command_action;
pub use auction_command_action::*;
pub(crate) mod auction_command_result;
pub use auction_command_result::*;
pub(crate) mod auction_list_item;
pub use auction_list_item::*;
pub(crate) mod aura_log;
pub use aura_log::*;
pub(crate) mod aura_type;
pub use aura_type::*;
pub(crate) mod battleground_bracket;
pub use battleground_bracket::*;
pub(crate) mod battleground_end_status;
pub use battleground_end_status::*;
pub(crate) mod battleground_player;
pub use battleground_player::*;
pub(crate) mod battleground_winner;
pub use battleground_winner::*;
pub(crate) mod bg_type_id;
pub use bg_type_id::*;
pub(crate) mod cast_failure_reason;
pub use cast_failure_reason::*;
pub(crate) mod cast_flags;
pub use cast_flags::*;
pub(crate) mod character;
pub use character::*;
pub(crate) mod character_flags;
pub use character_flags::*;
pub(crate) mod character_gear;
pub use character_gear::*;
pub(crate) mod chat_notify;
pub use chat_notify::*;
pub(crate) mod chat_type;
pub use chat_type::*;
pub(crate) mod cinematic_sequence_id;
pub use cinematic_sequence_id::*;
pub(crate) mod cmsg_add_friend;
pub use cmsg_add_friend::*;
pub(crate) mod cmsg_auction_list_items;
pub use cmsg_auction_list_items::*;
pub(crate) mod cmsg_battlefield_join;
pub use cmsg_battlefield_join::*;
pub(crate) mod cmsg_battlefield_list;
pub use cmsg_battlefield_list::*;
pub(crate) mod cmsg_battlefield_port;
pub use cmsg_battlefield_port::*;
pub(crate) mod cmsg_battlemaster_join;
pub use cmsg_battlemaster_join::*;
pub(crate) mod cmsg_cast_spell;
pub use cmsg_cast_spell::*;
pub(crate) mod cmsg_char_create;
pub use cmsg_char_create::*;
pub(crate) mod cmsg_chat_ignored;
pub use cmsg_chat_ignored::*;
pub(crate) mod cmsg_emote;
pub use cmsg_emote::*;
pub(crate) mod cmsg_force_move_root_ack;
pub use cmsg_force_move_root_ack::*;
pub(crate) mod cmsg_force_move_unroot_ack;
pub use cmsg_force_move_unroot_ack::*;
pub(crate) mod cmsg_force_run_back_speed_change_ack;
pub use cmsg_force_run_back_speed_change_ack::*;
pub(crate) mod cmsg_force_run_speed_change_ack;
pub use cmsg_force_run_speed_change_ack::*;
pub(crate) mod cmsg_force_swim_back_speed_change_ack;
pub use cmsg_force_swim_back_speed_change_ack::*;
pub(crate) mod cmsg_force_swim_speed_change_ack;
pub use cmsg_force_swim_speed_change_ack::*;
pub(crate) mod cmsg_force_turn_rate_change_ack;
pub use cmsg_force_turn_rate_change_ack::*;
pub(crate) mod cmsg_force_walk_speed_change_ack;
pub use cmsg_force_walk_speed_change_ack::*;
pub(crate) mod cmsg_friend_list;
pub use cmsg_friend_list::*;
pub(crate) mod cmsg_gmticket_create;
pub use cmsg_gmticket_create::*;
pub(crate) mod cmsg_gmticket_updatetext;
pub use cmsg_gmticket_updatetext::*;
pub(crate) mod cmsg_gossip_select_option;
pub use cmsg_gossip_select_option::*;
pub(crate) mod cmsg_guild_rank;
pub use cmsg_guild_rank::*;
pub(crate) mod cmsg_item_query_single;
pub use cmsg_item_query_single::*;
pub(crate) mod cmsg_join_channel;
pub use cmsg_join_channel::*;
pub(crate) mod cmsg_leave_battlefield;
pub use cmsg_leave_battlefield::*;
pub(crate) mod cmsg_leave_channel;
pub use cmsg_leave_channel::*;
pub(crate) mod cmsg_loot_method;
pub use cmsg_loot_method::*;
pub(crate) mod cmsg_loot_roll;
pub use cmsg_loot_roll::*;
pub(crate) mod cmsg_mail_delete;
pub use cmsg_mail_delete::*;
pub(crate) mod cmsg_mail_take_item;
pub use cmsg_mail_take_item::*;
pub(crate) mod cmsg_meetingstone_join;
pub use cmsg_meetingstone_join::*;
pub(crate) mod cmsg_meetingstone_leave;
pub use cmsg_meetingstone_leave::*;
pub(crate) mod cmsg_messagechat;
pub use cmsg_messagechat::*;
pub(crate) mod cmsg_move_fall_reset;
pub use cmsg_move_fall_reset::*;
pub(crate) mod cmsg_move_feather_fall_ack;
pub use cmsg_move_feather_fall_ack::*;
pub(crate) mod cmsg_move_hover_ack;
pub use cmsg_move_hover_ack::*;
pub(crate) mod cmsg_move_knock_back_ack;
pub use cmsg_move_knock_back_ack::*;
pub(crate) mod cmsg_move_not_active_mover;
pub use cmsg_move_not_active_mover::*;
pub(crate) mod cmsg_move_spline_done;
pub use cmsg_move_spline_done::*;
pub(crate) mod cmsg_move_water_walk_ack;
pub use cmsg_move_water_walk_ack::*;
pub(crate) mod cmsg_offer_petition;
pub use cmsg_offer_petition::*;
pub(crate) mod cmsg_page_text_query;
pub use cmsg_page_text_query::*;
pub(crate) mod cmsg_pet_cast_spell;
pub use cmsg_pet_cast_spell::*;
pub(crate) mod cmsg_pet_rename;
pub use cmsg_pet_rename::*;
pub(crate) mod cmsg_played_time;
pub use cmsg_played_time::*;
pub(crate) mod cmsg_repair_item;
pub use cmsg_repair_item::*;
pub(crate) mod cmsg_send_mail;
pub use cmsg_send_mail::*;
pub(crate) mod cmsg_summon_response;
pub use cmsg_summon_response::*;
pub(crate) mod cmsg_text_emote;
pub use cmsg_text_emote::*;
pub(crate) mod cmsg_unlearn_skill;
pub use cmsg_unlearn_skill::*;
pub(crate) mod cmsg_update_account_data;
pub use cmsg_update_account_data::*;
pub(crate) mod cmsg_use_item;
pub use cmsg_use_item::*;
pub(crate) mod cmsg_warden_data;
pub use cmsg_warden_data::*;
pub(crate) mod cmsg_world_teleport;
pub use cmsg_world_teleport::*;
pub(crate) mod cmsg_zoneupdate;
pub use cmsg_zoneupdate::*;
pub use crate::helper::vanilla::*;
pub(crate) mod damage_info;
pub use damage_info::*;
pub(crate) mod dismount_result;
pub use dismount_result::*;
pub(crate) mod emote;
pub use emote::*;
pub(crate) mod friend;
pub use friend::*;
pub(crate) mod friend_status;
pub use friend_status::*;
pub(crate) mod gm_ticket_type;
pub use gm_ticket_type::*;
pub(crate) mod gossip_item;
pub use gossip_item::*;
pub(crate) mod group_list_member;
pub use group_list_member::*;
pub(crate) mod group_member_online_status;
pub use group_member_online_status::*;
pub(crate) mod group_type;
pub use group_type::*;
pub(crate) mod group_update_flags;
pub use group_update_flags::*;
pub(crate) mod guild_command_result;
pub use guild_command_result::*;
pub(crate) mod guild_emblem_result;
pub use guild_emblem_result::*;
pub(crate) mod guild_event;
pub use guild_event::*;
pub(crate) mod guild_member;
pub use guild_member::*;
pub(crate) mod hit_info;
pub use hit_info::*;
pub(crate) mod info_block;
pub use info_block::*;
pub(crate) mod instance_reset_failed_reason;
pub use instance_reset_failed_reason::*;
pub(crate) mod inventory_result;
pub use inventory_result::*;
pub(crate) mod item_class;
pub use item_class::*;
pub(crate) mod key_version;
pub use key_version::*;
pub(crate) mod list_inventory_item;
pub use list_inventory_item::*;
pub(crate) mod loot_method;
pub use loot_method::*;
pub(crate) mod mail;
pub use mail::*;
pub(crate) mod mail_action;
pub use mail_action::*;
pub(crate) mod mail_result;
pub use mail_result::*;
pub(crate) mod mail_type;
pub use mail_type::*;
pub(crate) mod map;
pub use map::*;
pub(crate) mod meeting_stone_failure;
pub use meeting_stone_failure::*;
pub(crate) mod monster_move_type;
pub use monster_move_type::*;
pub(crate) mod movement_block;
pub use movement_block::*;
pub(crate) mod movement_flags;
pub use movement_flags::*;
pub(crate) mod movement_info;
pub use movement_info::*;
pub(crate) mod msg_corpse_query_server;
pub use msg_corpse_query_server::*;
pub(crate) mod msg_inspect_honor_stats_server;
pub use msg_inspect_honor_stats_server::*;
pub(crate) mod msg_looking_for_group_client;
pub use msg_looking_for_group_client::*;
pub(crate) mod msg_looking_for_group_server;
pub use msg_looking_for_group_server::*;
pub(crate) mod msg_minimap_ping_client;
pub use msg_minimap_ping_client::*;
pub(crate) mod msg_minimap_ping_server;
pub use msg_minimap_ping_server::*;
pub(crate) mod msg_move_fall_land_client;
pub use msg_move_fall_land_client::*;
pub(crate) mod msg_move_fall_land_server;
pub use msg_move_fall_land_server::*;
pub(crate) mod msg_move_heartbeat_client;
pub use msg_move_heartbeat_client::*;
pub(crate) mod msg_move_heartbeat_server;
pub use msg_move_heartbeat_server::*;
pub(crate) mod msg_move_jump_client;
pub use msg_move_jump_client::*;
pub(crate) mod msg_move_jump_server;
pub use msg_move_jump_server::*;
pub(crate) mod msg_move_set_facing_client;
pub use msg_move_set_facing_client::*;
pub(crate) mod msg_move_set_facing_server;
pub use msg_move_set_facing_server::*;
pub(crate) mod msg_move_set_pitch_client;
pub use msg_move_set_pitch_client::*;
pub(crate) mod msg_move_set_pitch_server;
pub use msg_move_set_pitch_server::*;
pub(crate) mod msg_move_set_run_mode_client;
pub use msg_move_set_run_mode_client::*;
pub(crate) mod msg_move_set_run_mode_server;
pub use msg_move_set_run_mode_server::*;
pub(crate) mod msg_move_set_walk_mode_client;
pub use msg_move_set_walk_mode_client::*;
pub(crate) mod msg_move_set_walk_mode_server;
pub use msg_move_set_walk_mode_server::*;
pub(crate) mod msg_move_start_backward_client;
pub use msg_move_start_backward_client::*;
pub(crate) mod msg_move_start_backward_server;
pub use msg_move_start_backward_server::*;
pub(crate) mod msg_move_start_forward_client;
pub use msg_move_start_forward_client::*;
pub(crate) mod msg_move_start_forward_server;
pub use msg_move_start_forward_server::*;
pub(crate) mod msg_move_start_pitch_down_client;
pub use msg_move_start_pitch_down_client::*;
pub(crate) mod msg_move_start_pitch_down_server;
pub use msg_move_start_pitch_down_server::*;
pub(crate) mod msg_move_start_pitch_up_client;
pub use msg_move_start_pitch_up_client::*;
pub(crate) mod msg_move_start_pitch_up_server;
pub use msg_move_start_pitch_up_server::*;
pub(crate) mod msg_move_start_strafe_left_client;
pub use msg_move_start_strafe_left_client::*;
pub(crate) mod msg_move_start_strafe_left_server;
pub use msg_move_start_strafe_left_server::*;
pub(crate) mod msg_move_start_strafe_right_client;
pub use msg_move_start_strafe_right_client::*;
pub(crate) mod msg_move_start_strafe_right_server;
pub use msg_move_start_strafe_right_server::*;
pub(crate) mod msg_move_start_swim_client;
pub use msg_move_start_swim_client::*;
pub(crate) mod msg_move_start_swim_server;
pub use msg_move_start_swim_server::*;
pub(crate) mod msg_move_start_turn_left_client;
pub use msg_move_start_turn_left_client::*;
pub(crate) mod msg_move_start_turn_left_server;
pub use msg_move_start_turn_left_server::*;
pub(crate) mod msg_move_start_turn_right_client;
pub use msg_move_start_turn_right_client::*;
pub(crate) mod msg_move_start_turn_right_server;
pub use msg_move_start_turn_right_server::*;
pub(crate) mod msg_move_stop_client;
pub use msg_move_stop_client::*;
pub(crate) mod msg_move_stop_pitch_client;
pub use msg_move_stop_pitch_client::*;
pub(crate) mod msg_move_stop_pitch_server;
pub use msg_move_stop_pitch_server::*;
pub(crate) mod msg_move_stop_server;
pub use msg_move_stop_server::*;
pub(crate) mod msg_move_stop_strafe_client;
pub use msg_move_stop_strafe_client::*;
pub(crate) mod msg_move_stop_strafe_server;
pub use msg_move_stop_strafe_server::*;
pub(crate) mod msg_move_stop_swim_client;
pub use msg_move_stop_swim_client::*;
pub(crate) mod msg_move_stop_swim_server;
pub use msg_move_stop_swim_server::*;
pub(crate) mod msg_move_stop_turn_client;
pub use msg_move_stop_turn_client::*;
pub(crate) mod msg_move_stop_turn_server;
pub use msg_move_stop_turn_server::*;
pub(crate) mod msg_move_teleport_ack_server;
pub use msg_move_teleport_ack_server::*;
pub(crate) mod msg_pvp_log_data_server;
pub use msg_pvp_log_data_server::*;
pub(crate) mod msg_query_next_mail_time_server;
pub use msg_query_next_mail_time_server::*;
pub(crate) mod msg_raid_target_update_client;
pub use msg_raid_target_update_client::*;
pub(crate) mod msg_raid_target_update_server;
pub use msg_raid_target_update_server::*;
pub(crate) mod msg_save_guild_emblem_server;
pub use msg_save_guild_emblem_server::*;
pub(crate) mod msg_talent_wipe_confirm_client;
pub use msg_talent_wipe_confirm_client::*;
pub(crate) mod msg_talent_wipe_confirm_server;
pub use msg_talent_wipe_confirm_server::*;
pub(crate) mod npc_text_update;
pub use npc_text_update::*;
pub(crate) mod object;
pub use object::*;
pub(crate) mod object_type;
pub use object_type::*;
pub mod opcodes;
pub(crate) mod party_operation;
pub use party_operation::*;
pub(crate) mod party_result;
pub use party_result::*;
pub(crate) mod pet_spell_cooldown;
pub use pet_spell_cooldown::*;
pub(crate) mod petition_result;
pub use petition_result::*;
pub(crate) mod petition_showlist;
pub use petition_showlist::*;
pub(crate) mod power_type;
pub use power_type::*;
pub use crate::shared::activate_taxi_reply_vanilla_tbc_wrath::ActivateTaxiReply;

pub use crate::shared::addon_info_vanilla_tbc_wrath::AddonInfo;

pub use crate::shared::ai_reaction_vanilla_tbc_wrath::AiReaction;

pub use crate::shared::battlefield_port_action_vanilla_tbc_wrath::BattlefieldPortAction;

pub use crate::shared::battleground_player_position_vanilla_tbc_wrath::BattlegroundPlayerPosition;

pub use crate::shared::buy_bank_slot_result_vanilla_tbc_wrath::BuyBankSlotResult;

pub use crate::shared::buy_result_vanilla_tbc_wrath::BuyResult;

pub use crate::shared::buyback_slot_vanilla_tbc_wrath::BuybackSlot;

pub use crate::shared::channel_flags_vanilla_tbc_wrath::ChannelFlags;

pub use crate::shared::channel_member_flags_vanilla_tbc_wrath::ChannelMemberFlags;

pub use crate::shared::channel_member_vanilla_tbc_wrath::ChannelMember;

pub use crate::shared::cmsg_accept_trade_vanilla_tbc_wrath::CMSG_ACCEPT_TRADE;

pub use crate::shared::cmsg_activatetaxi_vanilla_tbc_wrath::CMSG_ACTIVATETAXI;

pub use crate::shared::cmsg_activatetaxiexpress_vanilla_tbc::CMSG_ACTIVATETAXIEXPRESS;

pub use crate::shared::cmsg_add_ignore_vanilla_tbc_wrath::CMSG_ADD_IGNORE;

pub use crate::shared::cmsg_area_spirit_healer_query_vanilla_tbc_wrath::CMSG_AREA_SPIRIT_HEALER_QUERY;

pub use crate::shared::cmsg_area_spirit_healer_queue_vanilla_tbc_wrath::CMSG_AREA_SPIRIT_HEALER_QUEUE;

pub use crate::shared::cmsg_areatrigger_vanilla_tbc_wrath::CMSG_AREATRIGGER;

pub use crate::shared::cmsg_attackstop_vanilla_tbc_wrath::CMSG_ATTACKSTOP;

pub use crate::shared::cmsg_attackswing_vanilla_tbc_wrath::CMSG_ATTACKSWING;

pub use crate::shared::cmsg_auction_list_bidder_items_vanilla_tbc_wrath::CMSG_AUCTION_LIST_BIDDER_ITEMS;

pub use crate::shared::cmsg_auction_list_owner_items_vanilla_tbc_wrath::CMSG_AUCTION_LIST_OWNER_ITEMS;

pub use crate::shared::cmsg_auction_place_bid_vanilla_tbc_wrath::CMSG_AUCTION_PLACE_BID;

pub use crate::shared::cmsg_auction_remove_item_vanilla_tbc_wrath::CMSG_AUCTION_REMOVE_ITEM;

pub use crate::shared::cmsg_auction_sell_item_vanilla_tbc::CMSG_AUCTION_SELL_ITEM;

pub use crate::shared::cmsg_auth_session_vanilla_tbc::CMSG_AUTH_SESSION;

pub use crate::shared::cmsg_autobank_item_vanilla_tbc_wrath::CMSG_AUTOBANK_ITEM;

pub use crate::shared::cmsg_autoequip_item_slot_vanilla_tbc_wrath::CMSG_AUTOEQUIP_ITEM_SLOT;

pub use crate::shared::cmsg_autoequip_item_vanilla_tbc_wrath::CMSG_AUTOEQUIP_ITEM;

pub use crate::shared::cmsg_autostore_bag_item_vanilla_tbc_wrath::CMSG_AUTOSTORE_BAG_ITEM;

pub use crate::shared::cmsg_autostore_bank_item_vanilla_tbc_wrath::CMSG_AUTOSTORE_BANK_ITEM;

pub use crate::shared::cmsg_autostore_loot_item_vanilla_tbc_wrath::CMSG_AUTOSTORE_LOOT_ITEM;

pub use crate::shared::cmsg_banker_activate_vanilla_tbc_wrath::CMSG_BANKER_ACTIVATE;

pub use crate::shared::cmsg_battlefield_status_vanilla_tbc_wrath::CMSG_BATTLEFIELD_STATUS;

pub use crate::shared::cmsg_battlemaster_hello_vanilla_tbc_wrath::CMSG_BATTLEMASTER_HELLO;

pub use crate::shared::cmsg_begin_trade_vanilla_tbc_wrath::CMSG_BEGIN_TRADE;

pub use crate::shared::cmsg_binder_activate_vanilla_tbc_wrath::CMSG_BINDER_ACTIVATE;

pub use crate::shared::cmsg_bootme_vanilla_tbc_wrath::CMSG_BOOTME;

pub use crate::shared::cmsg_bug_vanilla_tbc_wrath::CMSG_BUG;

pub use crate::shared::cmsg_busy_trade_vanilla_tbc_wrath::CMSG_BUSY_TRADE;

pub use crate::shared::cmsg_buy_bank_slot_vanilla_tbc_wrath::CMSG_BUY_BANK_SLOT;

pub use crate::shared::cmsg_buy_item_in_slot_vanilla_tbc::CMSG_BUY_ITEM_IN_SLOT;

pub use crate::shared::cmsg_buy_item_vanilla_tbc::CMSG_BUY_ITEM;

pub use crate::shared::cmsg_buy_stable_slot_vanilla_tbc_wrath::CMSG_BUY_STABLE_SLOT;

pub use crate::shared::cmsg_buyback_item_vanilla_tbc_wrath::CMSG_BUYBACK_ITEM;

pub use crate::shared::cmsg_cancel_aura_vanilla_tbc_wrath::CMSG_CANCEL_AURA;

pub use crate::shared::cmsg_cancel_auto_repeat_spell_vanilla_tbc_wrath::CMSG_CANCEL_AUTO_REPEAT_SPELL;

pub use crate::shared::cmsg_cancel_cast_vanilla_tbc_wrath::CMSG_CANCEL_CAST;

pub use crate::shared::cmsg_cancel_channelling_vanilla_tbc_wrath::CMSG_CANCEL_CHANNELLING;

pub use crate::shared::cmsg_cancel_growth_aura_vanilla_tbc_wrath::CMSG_CANCEL_GROWTH_AURA;

pub use crate::shared::cmsg_cancel_trade_vanilla_tbc_wrath::CMSG_CANCEL_TRADE;

pub use crate::shared::cmsg_channel_announcements_vanilla_tbc_wrath::CMSG_CHANNEL_ANNOUNCEMENTS;

pub use crate::shared::cmsg_channel_ban_vanilla_tbc_wrath::CMSG_CHANNEL_BAN;

pub use crate::shared::cmsg_channel_invite_vanilla_tbc_wrath::CMSG_CHANNEL_INVITE;

pub use crate::shared::cmsg_channel_kick_vanilla_tbc_wrath::CMSG_CHANNEL_KICK;

pub use crate::shared::cmsg_channel_list_vanilla_tbc_wrath::CMSG_CHANNEL_LIST;

pub use crate::shared::cmsg_channel_moderate_vanilla_tbc_wrath::CMSG_CHANNEL_MODERATE;

pub use crate::shared::cmsg_channel_moderator_vanilla_tbc_wrath::CMSG_CHANNEL_MODERATOR;

pub use crate::shared::cmsg_channel_mute_vanilla_tbc_wrath::CMSG_CHANNEL_MUTE;

pub use crate::shared::cmsg_channel_owner_vanilla_tbc_wrath::CMSG_CHANNEL_OWNER;

pub use crate::shared::cmsg_channel_password_vanilla_tbc_wrath::CMSG_CHANNEL_PASSWORD;

pub use crate::shared::cmsg_channel_set_owner_vanilla_tbc_wrath::CMSG_CHANNEL_SET_OWNER;

pub use crate::shared::cmsg_channel_unban_vanilla_tbc_wrath::CMSG_CHANNEL_UNBAN;

pub use crate::shared::cmsg_channel_unmoderator_vanilla_tbc_wrath::CMSG_CHANNEL_UNMODERATOR;

pub use crate::shared::cmsg_channel_unmute_vanilla_tbc_wrath::CMSG_CHANNEL_UNMUTE;

pub use crate::shared::cmsg_char_delete_vanilla_tbc_wrath::CMSG_CHAR_DELETE;

pub use crate::shared::cmsg_char_enum_vanilla_tbc_wrath::CMSG_CHAR_ENUM;

pub use crate::shared::cmsg_char_rename_vanilla_tbc_wrath::CMSG_CHAR_RENAME;

pub use crate::shared::cmsg_clear_trade_item_vanilla_tbc_wrath::CMSG_CLEAR_TRADE_ITEM;

pub use crate::shared::cmsg_complete_cinematic_vanilla_tbc_wrath::CMSG_COMPLETE_CINEMATIC;

pub use crate::shared::cmsg_creature_query_vanilla_tbc_wrath::CMSG_CREATURE_QUERY;

pub use crate::shared::cmsg_del_friend_vanilla_tbc_wrath::CMSG_DEL_FRIEND;

pub use crate::shared::cmsg_del_ignore_vanilla_tbc_wrath::CMSG_DEL_IGNORE;

pub use crate::shared::cmsg_destroyitem_vanilla_tbc_wrath::CMSG_DESTROYITEM;

pub use crate::shared::cmsg_duel_accepted_vanilla_tbc_wrath::CMSG_DUEL_ACCEPTED;

pub use crate::shared::cmsg_duel_cancelled_vanilla_tbc_wrath::CMSG_DUEL_CANCELLED;

pub use crate::shared::cmsg_far_sight_vanilla_tbc_wrath::CMSG_FAR_SIGHT;

pub use crate::shared::cmsg_gameobj_use_vanilla_tbc_wrath::CMSG_GAMEOBJ_USE;

pub use crate::shared::cmsg_gameobject_query_vanilla_tbc_wrath::CMSG_GAMEOBJECT_QUERY;

pub use crate::shared::cmsg_get_mail_list_vanilla_tbc_wrath::CMSG_GET_MAIL_LIST;

pub use crate::shared::cmsg_gmsurvey_submit_vanilla_tbc_wrath::CMSG_GMSURVEY_SUBMIT;

pub use crate::shared::cmsg_gmticket_deleteticket_vanilla_tbc_wrath::CMSG_GMTICKET_DELETETICKET;

pub use crate::shared::cmsg_gmticket_getticket_vanilla_tbc_wrath::CMSG_GMTICKET_GETTICKET;

pub use crate::shared::cmsg_gmticket_systemstatus_vanilla_tbc_wrath::CMSG_GMTICKET_SYSTEMSTATUS;

pub use crate::shared::cmsg_gossip_hello_vanilla_tbc_wrath::CMSG_GOSSIP_HELLO;

pub use crate::shared::cmsg_group_accept_vanilla_tbc::CMSG_GROUP_ACCEPT;

pub use crate::shared::cmsg_group_assistant_leader_vanilla_tbc_wrath::CMSG_GROUP_ASSISTANT_LEADER;

pub use crate::shared::cmsg_group_change_sub_group_vanilla_tbc_wrath::CMSG_GROUP_CHANGE_SUB_GROUP;

pub use crate::shared::cmsg_group_decline_vanilla_tbc_wrath::CMSG_GROUP_DECLINE;

pub use crate::shared::cmsg_group_disband_vanilla_tbc_wrath::CMSG_GROUP_DISBAND;

pub use crate::shared::cmsg_group_invite_vanilla_tbc::CMSG_GROUP_INVITE;

pub use crate::shared::cmsg_group_raid_convert_vanilla_tbc_wrath::CMSG_GROUP_RAID_CONVERT;

pub use crate::shared::cmsg_group_set_leader_vanilla_tbc_wrath::CMSG_GROUP_SET_LEADER;

pub use crate::shared::cmsg_group_swap_sub_group_vanilla_tbc_wrath::CMSG_GROUP_SWAP_SUB_GROUP;

pub use crate::shared::cmsg_group_uninvite_guid_vanilla_tbc::CMSG_GROUP_UNINVITE_GUID;

pub use crate::shared::cmsg_group_uninvite_vanilla_tbc_wrath::CMSG_GROUP_UNINVITE;

pub use crate::shared::cmsg_guild_accept_vanilla_tbc_wrath::CMSG_GUILD_ACCEPT;

pub use crate::shared::cmsg_guild_add_rank_vanilla_tbc_wrath::CMSG_GUILD_ADD_RANK;

pub use crate::shared::cmsg_guild_create_vanilla_tbc_wrath::CMSG_GUILD_CREATE;

pub use crate::shared::cmsg_guild_decline_vanilla_tbc_wrath::CMSG_GUILD_DECLINE;

pub use crate::shared::cmsg_guild_del_rank_vanilla_tbc_wrath::CMSG_GUILD_DEL_RANK;

pub use crate::shared::cmsg_guild_demote_vanilla_tbc_wrath::CMSG_GUILD_DEMOTE;

pub use crate::shared::cmsg_guild_disband_vanilla_tbc_wrath::CMSG_GUILD_DISBAND;

pub use crate::shared::cmsg_guild_info_text_vanilla_tbc_wrath::CMSG_GUILD_INFO_TEXT;

pub use crate::shared::cmsg_guild_info_vanilla_tbc_wrath::CMSG_GUILD_INFO;

pub use crate::shared::cmsg_guild_invite_vanilla_tbc_wrath::CMSG_GUILD_INVITE;

pub use crate::shared::cmsg_guild_leader_vanilla_tbc_wrath::CMSG_GUILD_LEADER;

pub use crate::shared::cmsg_guild_leave_vanilla_tbc_wrath::CMSG_GUILD_LEAVE;

pub use crate::shared::cmsg_guild_motd_vanilla_tbc_wrath::CMSG_GUILD_MOTD;

pub use crate::shared::cmsg_guild_promote_vanilla_tbc_wrath::CMSG_GUILD_PROMOTE;

pub use crate::shared::cmsg_guild_query_vanilla_tbc_wrath::CMSG_GUILD_QUERY;

pub use crate::shared::cmsg_guild_remove_vanilla_tbc_wrath::CMSG_GUILD_REMOVE;

pub use crate::shared::cmsg_guild_roster_vanilla_tbc_wrath::CMSG_GUILD_ROSTER;

pub use crate::shared::cmsg_guild_set_officer_note_vanilla_tbc_wrath::CMSG_GUILD_SET_OFFICER_NOTE;

pub use crate::shared::cmsg_guild_set_public_note_vanilla_tbc_wrath::CMSG_GUILD_SET_PUBLIC_NOTE;

pub use crate::shared::cmsg_ignore_trade_vanilla_tbc_wrath::CMSG_IGNORE_TRADE;

pub use crate::shared::cmsg_initiate_trade_vanilla_tbc_wrath::CMSG_INITIATE_TRADE;

pub use crate::shared::cmsg_inspect_vanilla_tbc_wrath::CMSG_INSPECT;

pub use crate::shared::cmsg_item_name_query_vanilla_tbc_wrath::CMSG_ITEM_NAME_QUERY;

pub use crate::shared::cmsg_item_text_query_vanilla_tbc::CMSG_ITEM_TEXT_QUERY;

pub use crate::shared::cmsg_learn_talent_vanilla_tbc_wrath::CMSG_LEARN_TALENT;

pub use crate::shared::cmsg_list_inventory_vanilla_tbc_wrath::CMSG_LIST_INVENTORY;

pub use crate::shared::cmsg_logout_cancel_vanilla_tbc_wrath::CMSG_LOGOUT_CANCEL;

pub use crate::shared::cmsg_logout_request_vanilla_tbc_wrath::CMSG_LOGOUT_REQUEST;

pub use crate::shared::cmsg_loot_master_give_vanilla_tbc_wrath::CMSG_LOOT_MASTER_GIVE;

pub use crate::shared::cmsg_loot_money_vanilla_tbc_wrath::CMSG_LOOT_MONEY;

pub use crate::shared::cmsg_loot_release_vanilla_tbc_wrath::CMSG_LOOT_RELEASE;

pub use crate::shared::cmsg_loot_vanilla_tbc_wrath::CMSG_LOOT;

pub use crate::shared::cmsg_mail_create_text_item_vanilla_tbc::CMSG_MAIL_CREATE_TEXT_ITEM;

pub use crate::shared::cmsg_mail_mark_as_read_vanilla_tbc_wrath::CMSG_MAIL_MARK_AS_READ;

pub use crate::shared::cmsg_mail_return_to_sender_vanilla_tbc_wrath::CMSG_MAIL_RETURN_TO_SENDER;

pub use crate::shared::cmsg_mail_take_money_vanilla_tbc_wrath::CMSG_MAIL_TAKE_MONEY;

pub use crate::shared::cmsg_meetingstone_info_vanilla_tbc::CMSG_MEETINGSTONE_INFO;

pub use crate::shared::cmsg_mountspecial_anim_vanilla_tbc_wrath::CMSG_MOUNTSPECIAL_ANIM;

pub use crate::shared::cmsg_move_set_raw_position_vanilla_tbc_wrath::CMSG_MOVE_SET_RAW_POSITION;

pub use crate::shared::cmsg_move_time_skipped_vanilla_tbc::CMSG_MOVE_TIME_SKIPPED;

pub use crate::shared::cmsg_name_query_vanilla_tbc_wrath::CMSG_NAME_QUERY;

pub use crate::shared::cmsg_next_cinematic_camera_vanilla_tbc_wrath::CMSG_NEXT_CINEMATIC_CAMERA;

pub use crate::shared::cmsg_npc_text_query_vanilla_tbc_wrath::CMSG_NPC_TEXT_QUERY;

pub use crate::shared::cmsg_open_item_vanilla_tbc_wrath::CMSG_OPEN_ITEM;

pub use crate::shared::cmsg_pet_abandon_vanilla_tbc_wrath::CMSG_PET_ABANDON;

pub use crate::shared::cmsg_pet_action_vanilla_tbc_wrath::CMSG_PET_ACTION;

pub use crate::shared::cmsg_pet_cancel_aura_vanilla_tbc_wrath::CMSG_PET_CANCEL_AURA;

pub use crate::shared::cmsg_pet_name_query_vanilla_tbc_wrath::CMSG_PET_NAME_QUERY;

pub use crate::shared::cmsg_pet_set_action_vanilla_tbc_wrath::CMSG_PET_SET_ACTION;

pub use crate::shared::cmsg_pet_spell_autocast_vanilla_tbc_wrath::CMSG_PET_SPELL_AUTOCAST;

pub use crate::shared::cmsg_pet_stop_attack_vanilla_tbc_wrath::CMSG_PET_STOP_ATTACK;

pub use crate::shared::cmsg_pet_unlearn_vanilla_tbc_wrath::CMSG_PET_UNLEARN;

pub use crate::shared::cmsg_petition_buy_vanilla_tbc::CMSG_PETITION_BUY;

pub use crate::shared::cmsg_petition_query_vanilla_tbc_wrath::CMSG_PETITION_QUERY;

pub use crate::shared::cmsg_petition_show_signatures_vanilla_tbc_wrath::CMSG_PETITION_SHOW_SIGNATURES;

pub use crate::shared::cmsg_petition_showlist_vanilla_tbc_wrath::CMSG_PETITION_SHOWLIST;

pub use crate::shared::cmsg_petition_sign_vanilla_tbc_wrath::CMSG_PETITION_SIGN;

pub use crate::shared::cmsg_ping_vanilla_tbc_wrath::CMSG_PING;

pub use crate::shared::cmsg_player_login_vanilla_tbc_wrath::CMSG_PLAYER_LOGIN;

pub use crate::shared::cmsg_player_logout_vanilla_tbc_wrath::CMSG_PLAYER_LOGOUT;

pub use crate::shared::cmsg_pushquesttoparty_vanilla_tbc_wrath::CMSG_PUSHQUESTTOPARTY;

pub use crate::shared::cmsg_query_time_vanilla_tbc_wrath::CMSG_QUERY_TIME;

pub use crate::shared::cmsg_quest_confirm_accept_vanilla_tbc_wrath::CMSG_QUEST_CONFIRM_ACCEPT;

pub use crate::shared::cmsg_quest_query_vanilla_tbc_wrath::CMSG_QUEST_QUERY;

pub use crate::shared::cmsg_questgiver_accept_quest_vanilla_tbc_wrath::CMSG_QUESTGIVER_ACCEPT_QUEST;

pub use crate::shared::cmsg_questgiver_cancel_vanilla_tbc_wrath::CMSG_QUESTGIVER_CANCEL;

pub use crate::shared::cmsg_questgiver_choose_reward_vanilla_tbc_wrath::CMSG_QUESTGIVER_CHOOSE_REWARD;

pub use crate::shared::cmsg_questgiver_complete_quest_vanilla_tbc_wrath::CMSG_QUESTGIVER_COMPLETE_QUEST;

pub use crate::shared::cmsg_questgiver_hello_vanilla_tbc_wrath::CMSG_QUESTGIVER_HELLO;

pub use crate::shared::cmsg_questgiver_query_quest_vanilla_tbc::CMSG_QUESTGIVER_QUERY_QUEST;

pub use crate::shared::cmsg_questgiver_quest_autolaunch_vanilla_tbc_wrath::CMSG_QUESTGIVER_QUEST_AUTOLAUNCH;

pub use crate::shared::cmsg_questgiver_request_reward_vanilla_tbc_wrath::CMSG_QUESTGIVER_REQUEST_REWARD;

pub use crate::shared::cmsg_questgiver_status_query_vanilla_tbc_wrath::CMSG_QUESTGIVER_STATUS_QUERY;

pub use crate::shared::cmsg_questlog_remove_quest_vanilla_tbc_wrath::CMSG_QUESTLOG_REMOVE_QUEST;

pub use crate::shared::cmsg_questlog_swap_quest_vanilla_tbc_wrath::CMSG_QUESTLOG_SWAP_QUEST;

pub use crate::shared::cmsg_read_item_vanilla_tbc_wrath::CMSG_READ_ITEM;

pub use crate::shared::cmsg_reclaim_corpse_vanilla_tbc_wrath::CMSG_RECLAIM_CORPSE;

pub use crate::shared::cmsg_repop_request_vanilla_tbc_wrath::CMSG_REPOP_REQUEST;

pub use crate::shared::cmsg_request_account_data_vanilla_tbc_wrath::CMSG_REQUEST_ACCOUNT_DATA;

pub use crate::shared::cmsg_request_party_member_stats_vanilla_tbc_wrath::CMSG_REQUEST_PARTY_MEMBER_STATS;

pub use crate::shared::cmsg_request_pet_info_vanilla_tbc_wrath::CMSG_REQUEST_PET_INFO;

pub use crate::shared::cmsg_request_raid_info_vanilla_tbc_wrath::CMSG_REQUEST_RAID_INFO;

pub use crate::shared::cmsg_reset_instances_vanilla_tbc_wrath::CMSG_RESET_INSTANCES;

pub use crate::shared::cmsg_resurrect_response_vanilla_tbc_wrath::CMSG_RESURRECT_RESPONSE;

pub use crate::shared::cmsg_self_res_vanilla_tbc_wrath::CMSG_SELF_RES;

pub use crate::shared::cmsg_sell_item_vanilla_tbc_wrath::CMSG_SELL_ITEM;

pub use crate::shared::cmsg_set_action_button_vanilla_tbc_wrath::CMSG_SET_ACTION_BUTTON;

pub use crate::shared::cmsg_set_actionbar_toggles_vanilla_tbc_wrath::CMSG_SET_ACTIONBAR_TOGGLES;

pub use crate::shared::cmsg_set_active_mover_vanilla_tbc_wrath::CMSG_SET_ACTIVE_MOVER;

pub use crate::shared::cmsg_set_ammo_vanilla_tbc_wrath::CMSG_SET_AMMO;

pub use crate::shared::cmsg_set_faction_atwar_vanilla_tbc::CMSG_SET_FACTION_ATWAR;

pub use crate::shared::cmsg_set_faction_inactive_vanilla_tbc_wrath::CMSG_SET_FACTION_INACTIVE;

pub use crate::shared::cmsg_set_selection_vanilla_tbc_wrath::CMSG_SET_SELECTION;

pub use crate::shared::cmsg_set_target_obsolete_vanilla_tbc::CMSG_SET_TARGET_OBSOLETE;

pub use crate::shared::cmsg_set_trade_gold_vanilla_tbc_wrath::CMSG_SET_TRADE_GOLD;

pub use crate::shared::cmsg_set_trade_item_vanilla_tbc_wrath::CMSG_SET_TRADE_ITEM;

pub use crate::shared::cmsg_set_watched_faction_vanilla_tbc_wrath::CMSG_SET_WATCHED_FACTION;

pub use crate::shared::cmsg_setsheathed_vanilla_tbc_wrath::CMSG_SETSHEATHED;

pub use crate::shared::cmsg_spirit_healer_activate_vanilla_tbc_wrath::CMSG_SPIRIT_HEALER_ACTIVATE;

pub use crate::shared::cmsg_split_item_vanilla_tbc::CMSG_SPLIT_ITEM;

pub use crate::shared::cmsg_stable_pet_vanilla_tbc_wrath::CMSG_STABLE_PET;

pub use crate::shared::cmsg_stable_swap_pet_vanilla_tbc_wrath::CMSG_STABLE_SWAP_PET;

pub use crate::shared::cmsg_standstatechange_vanilla_tbc_wrath::CMSG_STANDSTATECHANGE;

pub use crate::shared::cmsg_swap_inv_item_vanilla_tbc_wrath::CMSG_SWAP_INV_ITEM;

pub use crate::shared::cmsg_swap_item_vanilla_tbc_wrath::CMSG_SWAP_ITEM;

pub use crate::shared::cmsg_taxinode_status_query_vanilla_tbc_wrath::CMSG_TAXINODE_STATUS_QUERY;

pub use crate::shared::cmsg_taxiqueryavailablenodes_vanilla_tbc_wrath::CMSG_TAXIQUERYAVAILABLENODES;

pub use crate::shared::cmsg_teleport_to_unit_vanilla_tbc_wrath::CMSG_TELEPORT_TO_UNIT;

pub use crate::shared::cmsg_toggle_cloak_vanilla_tbc_wrath::CMSG_TOGGLE_CLOAK;

pub use crate::shared::cmsg_toggle_helm_vanilla_tbc_wrath::CMSG_TOGGLE_HELM;

pub use crate::shared::cmsg_toggle_pvp_vanilla_tbc_wrath::CMSG_TOGGLE_PVP;

pub use crate::shared::cmsg_trainer_buy_spell_vanilla_tbc_wrath::CMSG_TRAINER_BUY_SPELL;

pub use crate::shared::cmsg_trainer_list_vanilla_tbc_wrath::CMSG_TRAINER_LIST;

pub use crate::shared::cmsg_turn_in_petition_vanilla_tbc_wrath::CMSG_TURN_IN_PETITION;

pub use crate::shared::cmsg_tutorial_clear_vanilla_tbc_wrath::CMSG_TUTORIAL_CLEAR;

pub use crate::shared::cmsg_tutorial_flag_vanilla_tbc_wrath::CMSG_TUTORIAL_FLAG;

pub use crate::shared::cmsg_tutorial_reset_vanilla_tbc_wrath::CMSG_TUTORIAL_RESET;

pub use crate::shared::cmsg_unaccept_trade_vanilla_tbc_wrath::CMSG_UNACCEPT_TRADE;

pub use crate::shared::cmsg_unstable_pet_vanilla_tbc_wrath::CMSG_UNSTABLE_PET;

pub use crate::shared::cmsg_who_vanilla_tbc_wrath::CMSG_WHO;

pub use crate::shared::cmsg_whois_vanilla_tbc_wrath::CMSG_WHOIS;

pub use crate::shared::cmsg_wrap_item_vanilla_tbc_wrath::CMSG_WRAP_ITEM;

pub use crate::shared::cooldown_spell_vanilla_tbc_wrath::CooldownSpell;

pub use crate::shared::corpse_query_result_vanilla_tbc_wrath::CorpseQueryResult;

pub use crate::shared::duel_winner_reason_vanilla_tbc_wrath::DuelWinnerReason;

pub use crate::shared::environmental_damage_type_vanilla_tbc_wrath::EnvironmentalDamageType;

pub use crate::shared::experience_award_type_vanilla_tbc_wrath::ExperienceAwardType;

pub use crate::shared::faction_flag_vanilla_tbc::FactionFlag;

pub use crate::shared::faction_initializer_vanilla_tbc::FactionInitializer;

pub use crate::shared::faction_vanilla_tbc_wrath::Faction;

pub use crate::shared::far_sight_operation_vanilla_tbc_wrath::FarSightOperation;

pub use crate::shared::forced_reaction_vanilla_tbc_wrath::ForcedReaction;

pub use crate::shared::friend_result_vanilla_tbc::FriendResult;

pub use crate::shared::gm_survey_question_vanilla_tbc_wrath::GmSurveyQuestion;

pub use crate::shared::gm_ticket_escalation_status_vanilla_tbc_wrath::GmTicketEscalationStatus;

pub use crate::shared::gm_ticket_queue_status_vanilla_tbc_wrath::GmTicketQueueStatus;

pub use crate::shared::gm_ticket_response_vanilla_tbc_wrath::GmTicketResponse;

pub use crate::shared::gm_ticket_status_response_vanilla_tbc_wrath::GmTicketStatusResponse;

pub use crate::shared::gm_ticket_status_vanilla_tbc_wrath::GmTicketStatus;

pub use crate::shared::group_loot_setting_vanilla_tbc_wrath::GroupLootSetting;

pub use crate::shared::guild_command_vanilla_tbc::GuildCommand;

pub use crate::shared::guild_member_status_vanilla_tbc_wrath::GuildMemberStatus;

pub use crate::shared::initial_spell_vanilla_tbc::InitialSpell;

pub use crate::shared::inventory_type_vanilla_tbc_wrath::InventoryType;

pub use crate::shared::item_damage_type_vanilla_tbc_wrath::ItemDamageType;

pub use crate::shared::item_quality_vanilla_tbc::ItemQuality;

pub use crate::shared::item_spells_vanilla_tbc_wrath::ItemSpells;

pub use crate::shared::item_stat_vanilla_tbc_wrath::ItemStat;

pub use crate::shared::language_vanilla_vanilla::Language;

pub use crate::shared::log_format_vanilla_tbc_wrath::LogFormat;

pub use crate::shared::logout_result_vanilla_tbc_wrath::LogoutResult;

pub use crate::shared::logout_speed_vanilla_tbc_wrath::LogoutSpeed;

pub use crate::shared::mail_message_type_vanilla_tbc_wrath::MailMessageType;

pub use crate::shared::meeting_stone_status_vanilla_tbc::MeetingStoneStatus;

pub use crate::shared::mount_result_vanilla_tbc_wrath::MountResult;

pub use crate::shared::msg_auction_hello_client_vanilla_tbc_wrath::MSG_AUCTION_HELLO_Client;

pub use crate::shared::msg_auction_hello_server_vanilla_tbc::MSG_AUCTION_HELLO_Server;

pub use crate::shared::msg_battleground_player_positions_client_vanilla_tbc_wrath::MSG_BATTLEGROUND_PLAYER_POSITIONS_Client;

pub use crate::shared::msg_battleground_player_positions_server_vanilla_tbc_wrath::MSG_BATTLEGROUND_PLAYER_POSITIONS_Server;

pub use crate::shared::msg_corpse_query_client_vanilla_tbc_wrath::MSG_CORPSE_QUERY_Client;

pub use crate::shared::msg_inspect_honor_stats_client_vanilla_tbc_wrath::MSG_INSPECT_HONOR_STATS_Client;

pub use crate::shared::msg_list_stabled_pets_client_vanilla_tbc_wrath::MSG_LIST_STABLED_PETS_Client;

pub use crate::shared::msg_list_stabled_pets_server_vanilla_tbc_wrath::MSG_LIST_STABLED_PETS_Server;

pub use crate::shared::msg_move_teleport_ack_client_vanilla_tbc_wrath::MSG_MOVE_TELEPORT_ACK_Client;

pub use crate::shared::msg_move_worldport_ack_vanilla_tbc_wrath::MSG_MOVE_WORLDPORT_ACK;

pub use crate::shared::msg_petition_decline_vanilla_tbc_wrath::MSG_PETITION_DECLINE;

pub use crate::shared::msg_petition_rename_vanilla_tbc_wrath::MSG_PETITION_RENAME;

pub use crate::shared::msg_pvp_log_data_client_vanilla_tbc_wrath::MSG_PVP_LOG_DATA_Client;

pub use crate::shared::msg_query_next_mail_time_client_vanilla_tbc_wrath::MSG_QUERY_NEXT_MAIL_TIME_Client;

pub use crate::shared::msg_quest_push_result_vanilla_tbc::MSG_QUEST_PUSH_RESULT;

pub use crate::shared::msg_raid_ready_check_client_vanilla_tbc_wrath::MSG_RAID_READY_CHECK_Client;

pub use crate::shared::msg_raid_ready_check_server_vanilla_tbc_wrath::MSG_RAID_READY_CHECK_Server;

pub use crate::shared::msg_random_roll_client_vanilla_tbc_wrath::MSG_RANDOM_ROLL_Client;

pub use crate::shared::msg_random_roll_server_vanilla_tbc_wrath::MSG_RANDOM_ROLL_Server;

pub use crate::shared::msg_save_guild_emblem_client_vanilla_tbc_wrath::MSG_SAVE_GUILD_EMBLEM_Client;

pub use crate::shared::msg_tabardvendor_activate_vanilla_tbc_wrath::MSG_TABARDVENDOR_ACTIVATE;

pub use crate::shared::new_item_chat_alert_vanilla_tbc_wrath::NewItemChatAlert;

pub use crate::shared::new_item_creation_type_vanilla_tbc_wrath::NewItemCreationType;

pub use crate::shared::new_item_source_vanilla_tbc_wrath::NewItemSource;

pub use crate::shared::npc_text_update_emote_vanilla_tbc_wrath::NpcTextUpdateEmote;

pub use crate::shared::pet_command_state_vanilla_tbc_wrath::PetCommandState;

pub use crate::shared::pet_enabled_vanilla_tbc_wrath::PetEnabled;

pub use crate::shared::pet_feedback_vanilla_tbc_wrath::PetFeedback;

pub use crate::shared::pet_react_state_vanilla_tbc_wrath::PetReactState;

pub use crate::shared::pet_talk_reason_vanilla_tbc_wrath::PetTalkReason;

pub use crate::shared::pet_tame_failure_reason_vanilla_tbc::PetTameFailureReason;

pub use crate::shared::petition_signature_vanilla_tbc_wrath::PetitionSignature;

pub use crate::shared::player_chat_tag_vanilla_tbc::PlayerChatTag;

pub use crate::shared::pvp_rank_vanilla_tbc_wrath::PvpRank;

pub use crate::shared::quest_item_reward_vanilla_tbc_wrath::QuestItemReward;

pub use crate::shared::quest_item_vanilla_tbc::QuestItem;

pub use crate::shared::quest_objective_vanilla_tbc_wrath::QuestObjective;

pub use crate::shared::quest_party_message_vanilla_tbc::QuestPartyMessage;

pub use crate::shared::sell_item_result_vanilla_tbc::SellItemResult;

pub use crate::shared::server_message_type_vanilla_tbc::ServerMessageType;

pub use crate::shared::sheath_state_vanilla_tbc_wrath::SheathState;

pub use crate::shared::smsg_account_data_times_vanilla_tbc::SMSG_ACCOUNT_DATA_TIMES;

pub use crate::shared::smsg_activatetaxireply_vanilla_tbc_wrath::SMSG_ACTIVATETAXIREPLY;

pub use crate::shared::smsg_ai_reaction_vanilla_tbc_wrath::SMSG_AI_REACTION;

pub use crate::shared::smsg_area_trigger_message_vanilla_tbc_wrath::SMSG_AREA_TRIGGER_MESSAGE;

pub use crate::shared::smsg_attackstart_vanilla_tbc_wrath::SMSG_ATTACKSTART;

pub use crate::shared::smsg_attackstop_vanilla_tbc_wrath::SMSG_ATTACKSTOP;

pub use crate::shared::smsg_attackswing_badfacing_vanilla_tbc_wrath::SMSG_ATTACKSWING_BADFACING;

pub use crate::shared::smsg_attackswing_cant_attack_vanilla_tbc_wrath::SMSG_ATTACKSWING_CANT_ATTACK;

pub use crate::shared::smsg_attackswing_deadtarget_vanilla_tbc_wrath::SMSG_ATTACKSWING_DEADTARGET;

pub use crate::shared::smsg_attackswing_notinrange_vanilla_tbc_wrath::SMSG_ATTACKSWING_NOTINRANGE;

pub use crate::shared::smsg_attackswing_notstanding_vanilla_tbc::SMSG_ATTACKSWING_NOTSTANDING;

pub use crate::shared::smsg_auction_bidder_notification_vanilla_tbc::SMSG_AUCTION_BIDDER_NOTIFICATION;

pub use crate::shared::smsg_auction_owner_notification_vanilla_tbc::SMSG_AUCTION_OWNER_NOTIFICATION;

pub use crate::shared::smsg_auction_removed_notification_vanilla_tbc_wrath::SMSG_AUCTION_REMOVED_NOTIFICATION;

pub use crate::shared::smsg_auth_challenge_vanilla_tbc::SMSG_AUTH_CHALLENGE;

pub use crate::shared::smsg_battleground_player_joined_vanilla_tbc_wrath::SMSG_BATTLEGROUND_PLAYER_JOINED;

pub use crate::shared::smsg_battleground_player_left_vanilla_tbc_wrath::SMSG_BATTLEGROUND_PLAYER_LEFT;

pub use crate::shared::smsg_binder_confirm_vanilla_tbc::SMSG_BINDER_CONFIRM;

pub use crate::shared::smsg_buy_bank_slot_result_vanilla_tbc_wrath::SMSG_BUY_BANK_SLOT_RESULT;

pub use crate::shared::smsg_buy_failed_vanilla_tbc_wrath::SMSG_BUY_FAILED;

pub use crate::shared::smsg_buy_item_vanilla_tbc_wrath::SMSG_BUY_ITEM;

pub use crate::shared::smsg_cancel_combat_vanilla_tbc_wrath::SMSG_CANCEL_COMBAT;

pub use crate::shared::smsg_channel_list_vanilla_tbc_wrath::SMSG_CHANNEL_LIST;

pub use crate::shared::smsg_chat_player_not_found_vanilla_tbc_wrath::SMSG_CHAT_PLAYER_NOT_FOUND;

pub use crate::shared::smsg_chat_wrong_faction_vanilla_tbc_wrath::SMSG_CHAT_WRONG_FACTION;

pub use crate::shared::smsg_client_control_update_vanilla_tbc_wrath::SMSG_CLIENT_CONTROL_UPDATE;

pub use crate::shared::smsg_destroy_object_vanilla_tbc::SMSG_DESTROY_OBJECT;

pub use crate::shared::smsg_duel_complete_vanilla_tbc_wrath::SMSG_DUEL_COMPLETE;

pub use crate::shared::smsg_duel_countdown_vanilla_tbc_wrath::SMSG_DUEL_COUNTDOWN;

pub use crate::shared::smsg_duel_inbounds_vanilla_tbc_wrath::SMSG_DUEL_INBOUNDS;

pub use crate::shared::smsg_duel_outofbounds_vanilla_tbc_wrath::SMSG_DUEL_OUTOFBOUNDS;

pub use crate::shared::smsg_duel_requested_vanilla_tbc_wrath::SMSG_DUEL_REQUESTED;

pub use crate::shared::smsg_duel_winner_vanilla_tbc_wrath::SMSG_DUEL_WINNER;

pub use crate::shared::smsg_durability_damage_death_vanilla_tbc_wrath::SMSG_DURABILITY_DAMAGE_DEATH;

pub use crate::shared::smsg_environmental_damage_log_vanilla_tbc_wrath::SMSG_ENVIRONMENTAL_DAMAGE_LOG;

pub use crate::shared::smsg_force_run_back_speed_change_vanilla_tbc_wrath::SMSG_FORCE_RUN_BACK_SPEED_CHANGE;

pub use crate::shared::smsg_force_swim_back_speed_change_vanilla_tbc_wrath::SMSG_FORCE_SWIM_BACK_SPEED_CHANGE;

pub use crate::shared::smsg_force_swim_speed_change_vanilla_tbc_wrath::SMSG_FORCE_SWIM_SPEED_CHANGE;

pub use crate::shared::smsg_force_turn_rate_change_vanilla_tbc_wrath::SMSG_FORCE_TURN_RATE_CHANGE;

pub use crate::shared::smsg_force_walk_speed_change_vanilla_tbc_wrath::SMSG_FORCE_WALK_SPEED_CHANGE;

pub use crate::shared::smsg_friend_status_vanilla_tbc::SMSG_FRIEND_STATUS;

pub use crate::shared::smsg_gameobject_custom_anim_vanilla_tbc_wrath::SMSG_GAMEOBJECT_CUSTOM_ANIM;

pub use crate::shared::smsg_gameobject_despawn_anim_vanilla_tbc_wrath::SMSG_GAMEOBJECT_DESPAWN_ANIM;

pub use crate::shared::smsg_gameobject_pagetext_vanilla_tbc_wrath::SMSG_GAMEOBJECT_PAGETEXT;

pub use crate::shared::smsg_gameobject_spawn_anim_vanilla_tbc::SMSG_GAMEOBJECT_SPAWN_ANIM;

pub use crate::shared::smsg_gm_ticket_status_update_vanilla_tbc_wrath::SMSG_GM_TICKET_STATUS_UPDATE;

pub use crate::shared::smsg_gmticket_create_vanilla_tbc_wrath::SMSG_GMTICKET_CREATE;

pub use crate::shared::smsg_gmticket_deleteticket_vanilla_tbc_wrath::SMSG_GMTICKET_DELETETICKET;

pub use crate::shared::smsg_gmticket_systemstatus_vanilla_tbc_wrath::SMSG_GMTICKET_SYSTEMSTATUS;

pub use crate::shared::smsg_gmticket_updatetext_vanilla_tbc_wrath::SMSG_GMTICKET_UPDATETEXT;

pub use crate::shared::smsg_gossip_complete_vanilla_tbc_wrath::SMSG_GOSSIP_COMPLETE;

pub use crate::shared::smsg_gossip_poi_vanilla_tbc_wrath::SMSG_GOSSIP_POI;

pub use crate::shared::smsg_guild_info_vanilla_tbc::SMSG_GUILD_INFO;

pub use crate::shared::smsg_guild_invite_vanilla_tbc_wrath::SMSG_GUILD_INVITE;

pub use crate::shared::smsg_guild_query_response_vanilla_tbc_wrath::SMSG_GUILD_QUERY_RESPONSE;

pub use crate::shared::smsg_initial_spells_vanilla_tbc::SMSG_INITIAL_SPELLS;

pub use crate::shared::smsg_initialize_factions_vanilla_tbc::SMSG_INITIALIZE_FACTIONS;

pub use crate::shared::smsg_item_cooldown_vanilla_tbc_wrath::SMSG_ITEM_COOLDOWN;

pub use crate::shared::smsg_item_enchant_time_update_vanilla_tbc_wrath::SMSG_ITEM_ENCHANT_TIME_UPDATE;

pub use crate::shared::smsg_item_text_query_response_vanilla_tbc::SMSG_ITEM_TEXT_QUERY_RESPONSE;

pub use crate::shared::smsg_item_time_update_vanilla_tbc_wrath::SMSG_ITEM_TIME_UPDATE;

pub use crate::shared::smsg_levelup_info_vanilla_tbc::SMSG_LEVELUP_INFO;

pub use crate::shared::smsg_log_xpgain_vanilla_tbc::SMSG_LOG_XPGAIN;

pub use crate::shared::smsg_login_settimespeed_vanilla_tbc::SMSG_LOGIN_SETTIMESPEED;

pub use crate::shared::smsg_logout_cancel_ack_vanilla_tbc_wrath::SMSG_LOGOUT_CANCEL_ACK;

pub use crate::shared::smsg_logout_complete_vanilla_tbc_wrath::SMSG_LOGOUT_COMPLETE;

pub use crate::shared::smsg_logout_response_vanilla_tbc_wrath::SMSG_LOGOUT_RESPONSE;

pub use crate::shared::smsg_loot_all_passed_vanilla_tbc_wrath::SMSG_LOOT_ALL_PASSED;

pub use crate::shared::smsg_loot_clear_money_vanilla_tbc_wrath::SMSG_LOOT_CLEAR_MONEY;

pub use crate::shared::smsg_loot_master_list_vanilla_tbc_wrath::SMSG_LOOT_MASTER_LIST;

pub use crate::shared::smsg_loot_money_notify_vanilla_tbc::SMSG_LOOT_MONEY_NOTIFY;

pub use crate::shared::smsg_loot_release_response_vanilla_tbc_wrath::SMSG_LOOT_RELEASE_RESPONSE;

pub use crate::shared::smsg_loot_removed_vanilla_tbc_wrath::SMSG_LOOT_REMOVED;

pub use crate::shared::smsg_loot_start_roll_vanilla_tbc::SMSG_LOOT_START_ROLL;

pub use crate::shared::smsg_mountresult_vanilla_tbc_wrath::SMSG_MOUNTRESULT;

pub use crate::shared::smsg_mountspecial_anim_vanilla_tbc_wrath::SMSG_MOUNTSPECIAL_ANIM;

pub use crate::shared::smsg_move_feather_fall_vanilla_tbc_wrath::SMSG_MOVE_FEATHER_FALL;

pub use crate::shared::smsg_move_knock_back_vanilla_tbc_wrath::SMSG_MOVE_KNOCK_BACK;

pub use crate::shared::smsg_move_land_walk_vanilla_tbc_wrath::SMSG_MOVE_LAND_WALK;

pub use crate::shared::smsg_move_normal_fall_vanilla_tbc_wrath::SMSG_MOVE_NORMAL_FALL;

pub use crate::shared::smsg_move_set_hover_vanilla_tbc_wrath::SMSG_MOVE_SET_HOVER;

pub use crate::shared::smsg_move_unset_hover_vanilla_tbc_wrath::SMSG_MOVE_UNSET_HOVER;

pub use crate::shared::smsg_move_water_walk_vanilla_tbc_wrath::SMSG_MOVE_WATER_WALK;

pub use crate::shared::smsg_new_taxi_path_vanilla_tbc_wrath::SMSG_NEW_TAXI_PATH;

pub use crate::shared::smsg_notification_vanilla_tbc_wrath::SMSG_NOTIFICATION;

pub use crate::shared::smsg_page_text_query_response_vanilla_tbc_wrath::SMSG_PAGE_TEXT_QUERY_RESPONSE;

pub use crate::shared::smsg_pet_action_feedback_vanilla_tbc_wrath::SMSG_PET_ACTION_FEEDBACK;

pub use crate::shared::smsg_pet_action_sound_vanilla_tbc_wrath::SMSG_PET_ACTION_SOUND;

pub use crate::shared::smsg_pet_broken_vanilla_tbc_wrath::SMSG_PET_BROKEN;

pub use crate::shared::smsg_pet_dismiss_sound_vanilla_tbc_wrath::SMSG_PET_DISMISS_SOUND;

pub use crate::shared::smsg_pet_mode_vanilla_tbc_wrath::SMSG_PET_MODE;

pub use crate::shared::smsg_pet_tame_failure_vanilla_tbc::SMSG_PET_TAME_FAILURE;

pub use crate::shared::smsg_pet_unlearn_confirm_vanilla_tbc_wrath::SMSG_PET_UNLEARN_CONFIRM;

pub use crate::shared::smsg_petition_show_signatures_vanilla_tbc_wrath::SMSG_PETITION_SHOW_SIGNATURES;

pub use crate::shared::smsg_play_object_sound_vanilla_tbc_wrath::SMSG_PLAY_OBJECT_SOUND;

pub use crate::shared::smsg_pong_vanilla_tbc_wrath::SMSG_PONG;

pub use crate::shared::smsg_procresist_vanilla_tbc_wrath::SMSG_PROCRESIST;

pub use crate::shared::smsg_pvp_credit_vanilla_tbc_wrath::SMSG_PVP_CREDIT;

pub use crate::shared::smsg_quest_confirm_accept_vanilla_tbc_wrath::SMSG_QUEST_CONFIRM_ACCEPT;

pub use crate::shared::smsg_questlog_full_vanilla_tbc_wrath::SMSG_QUESTLOG_FULL;

pub use crate::shared::smsg_questupdate_add_item_vanilla_tbc_wrath::SMSG_QUESTUPDATE_ADD_ITEM;

pub use crate::shared::smsg_questupdate_add_kill_vanilla_tbc_wrath::SMSG_QUESTUPDATE_ADD_KILL;

pub use crate::shared::smsg_questupdate_complete_vanilla_tbc_wrath::SMSG_QUESTUPDATE_COMPLETE;

pub use crate::shared::smsg_questupdate_failed_vanilla_tbc_wrath::SMSG_QUESTUPDATE_FAILED;

pub use crate::shared::smsg_questupdate_failedtimer_vanilla_tbc_wrath::SMSG_QUESTUPDATE_FAILEDTIMER;

pub use crate::shared::smsg_read_item_failed_vanilla_tbc_wrath::SMSG_READ_ITEM_FAILED;

pub use crate::shared::smsg_read_item_ok_vanilla_tbc_wrath::SMSG_READ_ITEM_OK;

pub use crate::shared::smsg_received_mail_vanilla_tbc_wrath::SMSG_RECEIVED_MAIL;

pub use crate::shared::smsg_sell_item_vanilla_tbc::SMSG_SELL_ITEM;

pub use crate::shared::smsg_server_message_vanilla_tbc::SMSG_SERVER_MESSAGE;

pub use crate::shared::smsg_set_faction_visible_vanilla_tbc_wrath::SMSG_SET_FACTION_VISIBLE;

pub use crate::shared::smsg_set_forced_reactions_vanilla_tbc_wrath::SMSG_SET_FORCED_REACTIONS;

pub use crate::shared::smsg_set_rest_start_vanilla_tbc::SMSG_SET_REST_START;

pub use crate::shared::smsg_show_bank_vanilla_tbc_wrath::SMSG_SHOW_BANK;

pub use crate::shared::smsg_showtaxinodes_vanilla_tbc_wrath::SMSG_SHOWTAXINODES;

pub use crate::shared::smsg_spline_move_feather_fall_vanilla_tbc_wrath::SMSG_SPLINE_MOVE_FEATHER_FALL;

pub use crate::shared::smsg_spline_move_land_walk_vanilla_tbc_wrath::SMSG_SPLINE_MOVE_LAND_WALK;

pub use crate::shared::smsg_spline_move_normal_fall_vanilla_tbc_wrath::SMSG_SPLINE_MOVE_NORMAL_FALL;

pub use crate::shared::smsg_spline_move_root_vanilla_tbc_wrath::SMSG_SPLINE_MOVE_ROOT;

pub use crate::shared::smsg_spline_move_set_hover_vanilla_tbc_wrath::SMSG_SPLINE_MOVE_SET_HOVER;

pub use crate::shared::smsg_spline_move_set_run_mode_vanilla_tbc_wrath::SMSG_SPLINE_MOVE_SET_RUN_MODE;

pub use crate::shared::smsg_spline_move_set_walk_mode_vanilla_tbc_wrath::SMSG_SPLINE_MOVE_SET_WALK_MODE;

pub use crate::shared::smsg_spline_move_start_swim_vanilla_tbc_wrath::SMSG_SPLINE_MOVE_START_SWIM;

pub use crate::shared::smsg_spline_move_stop_swim_vanilla_tbc_wrath::SMSG_SPLINE_MOVE_STOP_SWIM;

pub use crate::shared::smsg_spline_move_unroot_vanilla_tbc_wrath::SMSG_SPLINE_MOVE_UNROOT;

pub use crate::shared::smsg_spline_move_unset_hover_vanilla_tbc_wrath::SMSG_SPLINE_MOVE_UNSET_HOVER;

pub use crate::shared::smsg_spline_move_water_walk_vanilla_tbc_wrath::SMSG_SPLINE_MOVE_WATER_WALK;

pub use crate::shared::smsg_spline_set_run_back_speed_vanilla_tbc_wrath::SMSG_SPLINE_SET_RUN_BACK_SPEED;

pub use crate::shared::smsg_spline_set_run_speed_vanilla_tbc_wrath::SMSG_SPLINE_SET_RUN_SPEED;

pub use crate::shared::smsg_spline_set_swim_back_speed_vanilla_tbc_wrath::SMSG_SPLINE_SET_SWIM_BACK_SPEED;

pub use crate::shared::smsg_spline_set_swim_speed_vanilla_tbc_wrath::SMSG_SPLINE_SET_SWIM_SPEED;

pub use crate::shared::smsg_spline_set_turn_rate_vanilla_tbc_wrath::SMSG_SPLINE_SET_TURN_RATE;

pub use crate::shared::smsg_spline_set_walk_speed_vanilla_tbc_wrath::SMSG_SPLINE_SET_WALK_SPEED;

pub use crate::shared::smsg_stable_result_vanilla_tbc::SMSG_STABLE_RESULT;

pub use crate::shared::smsg_standstate_update_vanilla_tbc_wrath::SMSG_STANDSTATE_UPDATE;

pub use crate::shared::smsg_taxinode_status_vanilla_tbc_wrath::SMSG_TAXINODE_STATUS;

pub use crate::shared::smsg_tutorial_flags_vanilla_tbc_wrath::SMSG_TUTORIAL_FLAGS;

pub use crate::shared::smsg_update_world_state_vanilla_tbc_wrath::SMSG_UPDATE_WORLD_STATE;

pub use crate::shared::smsg_whois_vanilla_tbc_wrath::SMSG_WHOIS;

pub use crate::shared::spell_cast_target_flags_vanilla_tbc::SpellCastTargetFlags;

pub use crate::shared::spell_cast_targets_vanilla_tbc::SpellCastTargets;

pub use crate::shared::stable_result_vanilla_tbc::StableResult;

pub use crate::shared::stabled_pet_vanilla_tbc_wrath::StabledPet;

pub use crate::shared::transport_info_vanilla_tbc::TransportInfo;

pub use crate::shared::unit_stand_state_vanilla_tbc_wrath::UnitStandState;

pub use crate::shared::update_flag_vanilla_tbc::UpdateFlag;

pub use crate::shared::update_type_vanilla_tbc::UpdateType;

pub use crate::shared::world_state_vanilla_tbc_wrath::WorldState;

pub use wow_world_base::vanilla::Class;

pub use wow_world_base::vanilla::Gender;

pub use wow_world_base::vanilla::Power;

pub use wow_world_base::vanilla::Vector2d;

pub use wow_world_base::vanilla::Vector3d;

pub(crate) mod quest_completable;
pub use quest_completable::*;
pub(crate) mod quest_details_emote;
pub use quest_details_emote::*;
pub(crate) mod quest_failed_reason;
pub use quest_failed_reason::*;
pub(crate) mod quest_giver_status;
pub use quest_giver_status::*;
pub(crate) mod quest_item_requirement;
pub use quest_item_requirement::*;
pub(crate) mod race;
pub use race::*;
pub(crate) mod raid_group_error;
pub use raid_group_error::*;
pub(crate) mod raid_info;
pub use raid_info::*;
pub(crate) mod raid_instance_message;
pub use raid_instance_message::*;
pub(crate) mod raid_target_index;
pub use raid_target_index::*;
pub(crate) mod raid_target_update;
pub use raid_target_update::*;
pub(crate) mod raid_target_update_type;
pub use raid_target_update_type::*;
pub(crate) mod roll_vote;
pub use roll_vote::*;
pub(crate) mod simple_spell_cast_result;
pub use simple_spell_cast_result::*;
pub(crate) mod skill;
pub use skill::*;
pub(crate) mod smsg_action_buttons;
pub use smsg_action_buttons::*;
pub(crate) mod smsg_addon_info;
pub use smsg_addon_info::*;
pub(crate) mod smsg_area_spirit_healer_time;
pub use smsg_area_spirit_healer_time::*;
pub(crate) mod smsg_attackerstateupdate;
pub use smsg_attackerstateupdate::*;
pub(crate) mod smsg_auction_bidder_list_result;
pub use smsg_auction_bidder_list_result::*;
pub(crate) mod smsg_auction_command_result;
pub use smsg_auction_command_result::*;
pub(crate) mod smsg_auction_list_result;
pub use smsg_auction_list_result::*;
pub(crate) mod smsg_auction_owner_list_result;
pub use smsg_auction_owner_list_result::*;
pub(crate) mod smsg_auth_response;
pub use smsg_auth_response::*;
pub(crate) mod smsg_battlefield_list;
pub use smsg_battlefield_list::*;
pub(crate) mod smsg_battlefield_status;
pub use smsg_battlefield_status::*;
pub(crate) mod smsg_bindpointupdate;
pub use smsg_bindpointupdate::*;
pub(crate) mod smsg_cancel_auto_repeat;
pub use smsg_cancel_auto_repeat::*;
pub(crate) mod smsg_cast_result;
pub use smsg_cast_result::*;
pub(crate) mod smsg_channel_notify;
pub use smsg_channel_notify::*;
pub(crate) mod smsg_char_create;
pub use smsg_char_create::*;
pub(crate) mod smsg_char_delete;
pub use smsg_char_delete::*;
pub(crate) mod smsg_char_enum;
pub use smsg_char_enum::*;
pub(crate) mod smsg_char_rename;
pub use smsg_char_rename::*;
pub(crate) mod smsg_character_login_failed;
pub use smsg_character_login_failed::*;
pub(crate) mod smsg_chat_restricted;
pub use smsg_chat_restricted::*;
pub(crate) mod smsg_clear_cooldown;
pub use smsg_clear_cooldown::*;
pub(crate) mod smsg_compressed_moves;
pub use smsg_compressed_moves::*;
pub(crate) mod smsg_compressed_update_object;
pub use smsg_compressed_update_object::*;
pub(crate) mod smsg_cooldown_event;
pub use smsg_cooldown_event::*;
pub(crate) mod smsg_corpse_reclaim_delay;
pub use smsg_corpse_reclaim_delay::*;
pub(crate) mod smsg_creature_query_response;
pub use smsg_creature_query_response::*;
pub(crate) mod smsg_defense_message;
pub use smsg_defense_message::*;
pub(crate) mod smsg_dismountresult;
pub use smsg_dismountresult::*;
pub(crate) mod smsg_dispel_failed;
pub use smsg_dispel_failed::*;
pub(crate) mod smsg_emote;
pub use smsg_emote::*;
pub(crate) mod smsg_enchantmentlog;
pub use smsg_enchantmentlog::*;
pub(crate) mod smsg_expected_spam_records;
pub use smsg_expected_spam_records::*;
pub(crate) mod smsg_exploration_experience;
pub use smsg_exploration_experience::*;
pub(crate) mod smsg_feign_death_resisted;
pub use smsg_feign_death_resisted::*;
pub(crate) mod smsg_fish_escaped;
pub use smsg_fish_escaped::*;
pub(crate) mod smsg_fish_not_hooked;
pub use smsg_fish_not_hooked::*;
pub(crate) mod smsg_force_move_root;
pub use smsg_force_move_root::*;
pub(crate) mod smsg_force_move_unroot;
pub use smsg_force_move_unroot::*;
pub(crate) mod smsg_force_run_speed_change;
pub use smsg_force_run_speed_change::*;
pub(crate) mod smsg_friend_list;
pub use smsg_friend_list::*;
pub(crate) mod smsg_gameobject_query_response;
pub use smsg_gameobject_query_response::*;
pub(crate) mod smsg_gameobject_reset_state;
pub use smsg_gameobject_reset_state::*;
pub(crate) mod smsg_gmticket_getticket;
pub use smsg_gmticket_getticket::*;
pub(crate) mod smsg_gossip_message;
pub use smsg_gossip_message::*;
pub(crate) mod smsg_group_decline;
pub use smsg_group_decline::*;
pub(crate) mod smsg_group_destroyed;
pub use smsg_group_destroyed::*;
pub(crate) mod smsg_group_invite;
pub use smsg_group_invite::*;
pub(crate) mod smsg_group_joined_battleground;
pub use smsg_group_joined_battleground::*;
pub(crate) mod smsg_group_list;
pub use smsg_group_list::*;
pub(crate) mod smsg_group_set_leader;
pub use smsg_group_set_leader::*;
pub(crate) mod smsg_group_uninvite;
pub use smsg_group_uninvite::*;
pub(crate) mod smsg_guild_command_result;
pub use smsg_guild_command_result::*;
pub(crate) mod smsg_guild_event;
pub use smsg_guild_event::*;
pub(crate) mod smsg_guild_roster;
pub use smsg_guild_roster::*;
pub(crate) mod smsg_ignore_list;
pub use smsg_ignore_list::*;
pub(crate) mod smsg_init_world_states;
pub use smsg_init_world_states::*;
pub(crate) mod smsg_inspect;
pub use smsg_inspect::*;
pub(crate) mod smsg_instance_reset;
pub use smsg_instance_reset::*;
pub(crate) mod smsg_instance_reset_failed;
pub use smsg_instance_reset_failed::*;
pub(crate) mod smsg_instance_save_created;
pub use smsg_instance_save_created::*;
pub(crate) mod smsg_invalidate_player;
pub use smsg_invalidate_player::*;
pub(crate) mod smsg_inventory_change_failure;
pub use smsg_inventory_change_failure::*;
pub(crate) mod smsg_item_name_query_response;
pub use smsg_item_name_query_response::*;
pub(crate) mod smsg_item_push_result;
pub use smsg_item_push_result::*;
pub(crate) mod smsg_item_query_single_response;
pub use smsg_item_query_single_response::*;
pub(crate) mod smsg_learned_spell;
pub use smsg_learned_spell::*;
pub(crate) mod smsg_list_inventory;
pub use smsg_list_inventory::*;
pub(crate) mod smsg_login_verify_world;
pub use smsg_login_verify_world::*;
pub(crate) mod smsg_loot_response;
pub use smsg_loot_response::*;
pub(crate) mod smsg_loot_roll;
pub use smsg_loot_roll::*;
pub(crate) mod smsg_loot_roll_won;
pub use smsg_loot_roll_won::*;
pub(crate) mod smsg_mail_list_result;
pub use smsg_mail_list_result::*;
pub(crate) mod smsg_meetingstone_complete;
pub use smsg_meetingstone_complete::*;
pub(crate) mod smsg_meetingstone_in_progress;
pub use smsg_meetingstone_in_progress::*;
pub(crate) mod smsg_meetingstone_joinfailed;
pub use smsg_meetingstone_joinfailed::*;
pub(crate) mod smsg_meetingstone_member_added;
pub use smsg_meetingstone_member_added::*;
pub(crate) mod smsg_meetingstone_setqueue;
pub use smsg_meetingstone_setqueue::*;
pub(crate) mod smsg_messagechat;
pub use smsg_messagechat::*;
pub(crate) mod smsg_monster_move;
pub use smsg_monster_move::*;
pub(crate) mod smsg_monster_move_transport;
pub use smsg_monster_move_transport::*;
pub(crate) mod smsg_name_query_response;
pub use smsg_name_query_response::*;
pub(crate) mod smsg_new_world;
pub use smsg_new_world::*;
pub(crate) mod smsg_npc_text_update;
pub use smsg_npc_text_update::*;
pub(crate) mod smsg_open_container;
pub use smsg_open_container::*;
pub(crate) mod smsg_party_command_result;
pub use smsg_party_command_result::*;
pub(crate) mod smsg_party_member_stats;
pub use smsg_party_member_stats::*;
pub(crate) mod smsg_party_member_stats_full;
pub use smsg_party_member_stats_full::*;
pub(crate) mod smsg_partykilllog;
pub use smsg_partykilllog::*;
pub(crate) mod smsg_pause_mirror_timer;
pub use smsg_pause_mirror_timer::*;
pub(crate) mod smsg_periodicauralog;
pub use smsg_periodicauralog::*;
pub(crate) mod smsg_pet_cast_failed;
pub use smsg_pet_cast_failed::*;
pub(crate) mod smsg_pet_name_invalid;
pub use smsg_pet_name_invalid::*;
pub(crate) mod smsg_pet_name_query_response;
pub use smsg_pet_name_query_response::*;
pub(crate) mod smsg_pet_spells;
pub use smsg_pet_spells::*;
pub(crate) mod smsg_petition_query_response;
pub use smsg_petition_query_response::*;
pub(crate) mod smsg_petition_showlist;
pub use smsg_petition_showlist::*;
pub(crate) mod smsg_petition_sign_results;
pub use smsg_petition_sign_results::*;
pub(crate) mod smsg_play_music;
pub use smsg_play_music::*;
pub(crate) mod smsg_play_sound;
pub use smsg_play_sound::*;
pub(crate) mod smsg_play_spell_impact;
pub use smsg_play_spell_impact::*;
pub(crate) mod smsg_play_spell_visual;
pub use smsg_play_spell_visual::*;
pub(crate) mod smsg_played_time;
pub use smsg_played_time::*;
pub(crate) mod smsg_player_skinned;
pub use smsg_player_skinned::*;
pub(crate) mod smsg_playerbound;
pub use smsg_playerbound::*;
pub(crate) mod smsg_query_time_response;
pub use smsg_query_time_response::*;
pub(crate) mod smsg_quest_query_response;
pub use smsg_quest_query_response::*;
pub(crate) mod smsg_questgiver_offer_reward;
pub use smsg_questgiver_offer_reward::*;
pub(crate) mod smsg_questgiver_quest_complete;
pub use smsg_questgiver_quest_complete::*;
pub(crate) mod smsg_questgiver_quest_details;
pub use smsg_questgiver_quest_details::*;
pub(crate) mod smsg_questgiver_quest_failed;
pub use smsg_questgiver_quest_failed::*;
pub(crate) mod smsg_questgiver_quest_invalid;
pub use smsg_questgiver_quest_invalid::*;
pub(crate) mod smsg_questgiver_quest_list;
pub use smsg_questgiver_quest_list::*;
pub(crate) mod smsg_questgiver_request_items;
pub use smsg_questgiver_request_items::*;
pub(crate) mod smsg_questgiver_status;
pub use smsg_questgiver_status::*;
pub(crate) mod smsg_raid_group_only;
pub use smsg_raid_group_only::*;
pub(crate) mod smsg_raid_instance_info;
pub use smsg_raid_instance_info::*;
pub(crate) mod smsg_raid_instance_message;
pub use smsg_raid_instance_message::*;
pub(crate) mod smsg_removed_spell;
pub use smsg_removed_spell::*;
pub(crate) mod smsg_resistlog;
pub use smsg_resistlog::*;
pub(crate) mod smsg_resurrect_request;
pub use smsg_resurrect_request::*;
pub(crate) mod smsg_send_mail_result;
pub use smsg_send_mail_result::*;
pub(crate) mod smsg_set_faction_standing;
pub use smsg_set_faction_standing::*;
pub(crate) mod smsg_set_flat_spell_modifier;
pub use smsg_set_flat_spell_modifier::*;
pub(crate) mod smsg_set_pct_spell_modifier;
pub use smsg_set_pct_spell_modifier::*;
pub(crate) mod smsg_set_proficiency;
pub use smsg_set_proficiency::*;
pub(crate) mod smsg_spell_cooldown;
pub use smsg_spell_cooldown::*;
pub(crate) mod smsg_spell_delayed;
pub use smsg_spell_delayed::*;
pub(crate) mod smsg_spell_failed_other;
pub use smsg_spell_failed_other::*;
pub(crate) mod smsg_spell_failure;
pub use smsg_spell_failure::*;
pub(crate) mod smsg_spell_go;
pub use smsg_spell_go::*;
pub(crate) mod smsg_spell_start;
pub use smsg_spell_start::*;
pub(crate) mod smsg_spell_update_chain_targets;
pub use smsg_spell_update_chain_targets::*;
pub(crate) mod smsg_spelldamageshield;
pub use smsg_spelldamageshield::*;
pub(crate) mod smsg_spelldispellog;
pub use smsg_spelldispellog::*;
pub(crate) mod smsg_spellenergizelog;
pub use smsg_spellenergizelog::*;
pub(crate) mod smsg_spellheallog;
pub use smsg_spellheallog::*;
pub(crate) mod smsg_spellinstakilllog;
pub use smsg_spellinstakilllog::*;
pub(crate) mod smsg_spelllogexecute;
pub use smsg_spelllogexecute::*;
pub(crate) mod smsg_spelllogmiss;
pub use smsg_spelllogmiss::*;
pub(crate) mod smsg_spellnonmeleedamagelog;
pub use smsg_spellnonmeleedamagelog::*;
pub(crate) mod smsg_spellordamage_immune;
pub use smsg_spellordamage_immune::*;
pub(crate) mod smsg_spirit_healer_confirm;
pub use smsg_spirit_healer_confirm::*;
pub(crate) mod smsg_start_mirror_timer;
pub use smsg_start_mirror_timer::*;
pub(crate) mod smsg_stop_mirror_timer;
pub use smsg_stop_mirror_timer::*;
pub(crate) mod smsg_summon_request;
pub use smsg_summon_request::*;
pub(crate) mod smsg_superceded_spell;
pub use smsg_superceded_spell::*;
pub(crate) mod smsg_text_emote;
pub use smsg_text_emote::*;
pub(crate) mod smsg_trade_status;
pub use smsg_trade_status::*;
pub(crate) mod smsg_trade_status_extended;
pub use smsg_trade_status_extended::*;
pub(crate) mod smsg_trainer_buy_failed;
pub use smsg_trainer_buy_failed::*;
pub(crate) mod smsg_trainer_buy_succeeded;
pub use smsg_trainer_buy_succeeded::*;
pub(crate) mod smsg_trainer_list;
pub use smsg_trainer_list::*;
pub(crate) mod smsg_transfer_aborted;
pub use smsg_transfer_aborted::*;
pub(crate) mod smsg_transfer_pending;
pub use smsg_transfer_pending::*;
pub(crate) mod smsg_trigger_cinematic;
pub use smsg_trigger_cinematic::*;
pub(crate) mod smsg_turn_in_petition_results;
pub use smsg_turn_in_petition_results::*;
pub(crate) mod smsg_update_aura_duration;
pub use smsg_update_aura_duration::*;
pub(crate) mod smsg_update_instance_ownership;
pub use smsg_update_instance_ownership::*;
pub(crate) mod smsg_update_last_instance;
pub use smsg_update_last_instance::*;
pub(crate) mod smsg_update_object;
pub use smsg_update_object::*;
pub(crate) mod smsg_warden_data;
pub use smsg_warden_data::*;
pub(crate) mod smsg_weather;
pub use smsg_weather::*;
pub(crate) mod smsg_who;
pub use smsg_who::*;
pub(crate) mod smsg_zone_under_attack;
pub use smsg_zone_under_attack::*;
pub(crate) mod spell_cast_result;
pub use spell_cast_result::*;
pub(crate) mod spell_cooldown_status;
pub use spell_cooldown_status::*;
pub(crate) mod spell_effect;
pub use spell_effect::*;
pub(crate) mod spell_log;
pub use spell_log::*;
pub(crate) mod spell_miss;
pub use spell_miss::*;
pub(crate) mod spell_miss_info;
pub use spell_miss_info::*;
pub(crate) mod spell_school;
pub use spell_school::*;
pub(crate) mod spline_flag;
pub use spline_flag::*;
pub(crate) mod status_id;
pub use status_id::*;
pub(crate) mod timer_type;
pub use timer_type::*;
pub(crate) mod trade_slot;
pub use trade_slot::*;
pub(crate) mod trade_status;
pub use trade_status::*;
pub(crate) mod trainer_spell;
pub use trainer_spell::*;
pub(crate) mod trainer_spell_state;
pub use trainer_spell_state::*;
pub(crate) mod training_failure_reason;
pub use training_failure_reason::*;
pub(crate) mod transfer_abort_reason;
pub use transfer_abort_reason::*;
pub(crate) mod url_info;
pub use url_info::*;
pub(crate) mod weather_change_type;
pub use weather_change_type::*;
pub(crate) mod weather_type;
pub use weather_type::*;
pub(crate) mod who_player;
pub use who_player::*;
pub(crate) mod world_result;
pub use world_result::*;
