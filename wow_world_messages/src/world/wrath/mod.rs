pub(crate) mod action_bar_behavior;
pub use action_bar_behavior::*;
pub(crate) mod action_button;
pub use action_button::*;
pub(crate) mod addon;
pub use addon::*;
pub(crate) mod area;
pub use area::*;
pub(crate) mod auction_list_item;
pub use auction_list_item::*;
pub(crate) mod battlefield_list_location;
pub use battlefield_list_location::*;
pub(crate) mod battleground_type;
pub use battleground_type::*;
pub(crate) mod cache_mask;
pub use cache_mask::*;
pub(crate) mod character;
pub use character::*;
pub(crate) mod character_gear;
pub use character_gear::*;
pub(crate) mod chat_restriction_type;
pub use chat_restriction_type::*;
pub(crate) mod chat_type;
pub use chat_type::*;
pub(crate) mod cinematic_sequence_id;
pub use cinematic_sequence_id::*;
pub(crate) mod class;
pub use class::*;
pub(crate) mod client_cast_flags;
pub use client_cast_flags::*;
pub(crate) mod client_movement_data;
pub use client_movement_data::*;
pub(crate) mod cmsg_activatetaxiexpress;
pub use cmsg_activatetaxiexpress::*;
pub(crate) mod cmsg_auction_list_items;
pub use cmsg_auction_list_items::*;
pub(crate) mod cmsg_auction_sell_item;
pub use cmsg_auction_sell_item::*;
pub(crate) mod cmsg_auth_session;
pub use cmsg_auth_session::*;
pub(crate) mod cmsg_battlefield_list;
pub use cmsg_battlefield_list::*;
pub(crate) mod cmsg_battlemaster_join;
pub use cmsg_battlemaster_join::*;
pub(crate) mod cmsg_buy_item;
pub use cmsg_buy_item::*;
pub(crate) mod cmsg_buy_item_in_slot;
pub use cmsg_buy_item_in_slot::*;
pub(crate) mod cmsg_calendar_get_num_pending;
pub use cmsg_calendar_get_num_pending::*;
pub(crate) mod cmsg_cast_spell;
pub use cmsg_cast_spell::*;
pub(crate) mod cmsg_char_create;
pub use cmsg_char_create::*;
pub(crate) mod cmsg_deleteequipment_set;
pub use cmsg_deleteequipment_set::*;
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
pub(crate) mod cmsg_gmticket_create;
pub use cmsg_gmticket_create::*;
pub(crate) mod cmsg_group_accept;
pub use cmsg_group_accept::*;
pub(crate) mod cmsg_group_invite;
pub use cmsg_group_invite::*;
pub(crate) mod cmsg_group_uninvite_guid;
pub use cmsg_group_uninvite_guid::*;
pub(crate) mod cmsg_item_text_query;
pub use cmsg_item_text_query::*;
pub(crate) mod cmsg_leave_battlefield;
pub use cmsg_leave_battlefield::*;
pub(crate) mod cmsg_lfd_player_lock_info_request;
pub use cmsg_lfd_player_lock_info_request::*;
pub(crate) mod cmsg_lfg_get_status;
pub use cmsg_lfg_get_status::*;
pub(crate) mod cmsg_loot_method;
pub use cmsg_loot_method::*;
pub(crate) mod cmsg_mail_create_text_item;
pub use cmsg_mail_create_text_item::*;
pub(crate) mod cmsg_messagechat;
pub use cmsg_messagechat::*;
pub(crate) mod cmsg_move_chng_transport;
pub use cmsg_move_chng_transport::*;
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
pub(crate) mod cmsg_move_set_fly;
pub use cmsg_move_set_fly::*;
pub(crate) mod cmsg_move_spline_done;
pub use cmsg_move_spline_done::*;
pub(crate) mod cmsg_move_time_skipped;
pub use cmsg_move_time_skipped::*;
pub(crate) mod cmsg_move_water_walk_ack;
pub use cmsg_move_water_walk_ack::*;
pub(crate) mod cmsg_pet_cast_spell;
pub use cmsg_pet_cast_spell::*;
pub(crate) mod cmsg_petition_buy;
pub use cmsg_petition_buy::*;
pub(crate) mod cmsg_played_time;
pub use cmsg_played_time::*;
pub(crate) mod cmsg_questgiver_query_quest;
pub use cmsg_questgiver_query_quest::*;
pub(crate) mod cmsg_ready_for_account_data_times;
pub use cmsg_ready_for_account_data_times::*;
pub(crate) mod cmsg_set_active_voice_channel;
pub use cmsg_set_active_voice_channel::*;
pub(crate) mod cmsg_set_faction_atwar;
pub use cmsg_set_faction_atwar::*;
pub(crate) mod cmsg_split_item;
pub use cmsg_split_item::*;
pub(crate) mod cmsg_text_emote;
pub use cmsg_text_emote::*;
pub(crate) mod cmsg_update_account_data;
pub use cmsg_update_account_data::*;
pub(crate) mod cmsg_use_item;
pub use cmsg_use_item::*;
pub(crate) mod cmsg_voice_session_enable;
pub use cmsg_voice_session_enable::*;
pub(crate) mod cmsg_world_state_ui_timer_update;
pub use cmsg_world_state_ui_timer_update::*;
pub(crate) mod cmsg_world_teleport;
pub use cmsg_world_teleport::*;
pub(crate) mod cmsg_zoneupdate;
pub use cmsg_zoneupdate::*;
pub(crate) mod complaint_status;
pub use complaint_status::*;
pub use crate::helper::wrath::*;
pub(crate) mod declined_names;
pub use declined_names::*;
pub(crate) mod emote;
pub use emote::*;
pub(crate) mod expansion;
pub use expansion::*;
pub(crate) mod extra_movement_flags;
pub use extra_movement_flags::*;
pub(crate) mod faction_flag;
pub use faction_flag::*;
pub(crate) mod faction_initializer;
pub use faction_initializer::*;
pub(crate) mod friend_status;
pub use friend_status::*;
pub(crate) mod guild_command;
pub use guild_command::*;
pub(crate) mod guild_command_result;
pub use guild_command_result::*;
pub(crate) mod guild_member;
pub use guild_member::*;
pub(crate) mod hit_info;
pub use hit_info::*;
pub(crate) mod initial_spell;
pub use initial_spell::*;
pub(crate) mod inventory_result;
pub use inventory_result::*;
pub(crate) mod item_class;
pub use item_class::*;
pub(crate) mod item_quality;
pub use item_quality::*;
pub(crate) mod item_text_query;
pub use item_text_query::*;
pub(crate) mod map;
pub use map::*;
pub(crate) mod movement_block;
pub use movement_block::*;
pub(crate) mod movement_flags;
pub use movement_flags::*;
pub(crate) mod movement_info;
pub use movement_info::*;
pub(crate) mod msg_auction_hello_server;
pub use msg_auction_hello_server::*;
pub(crate) mod msg_corpse_query_server;
pub use msg_corpse_query_server::*;
pub(crate) mod msg_guild_bank_money_withdrawn;
pub use msg_guild_bank_money_withdrawn::*;
pub(crate) mod msg_move_fall_land;
pub use msg_move_fall_land::*;
pub(crate) mod msg_move_heartbeat;
pub use msg_move_heartbeat::*;
pub(crate) mod msg_move_jump;
pub use msg_move_jump::*;
pub(crate) mod msg_move_set_facing;
pub use msg_move_set_facing::*;
pub(crate) mod msg_move_set_pitch;
pub use msg_move_set_pitch::*;
pub(crate) mod msg_move_set_run_mode;
pub use msg_move_set_run_mode::*;
pub(crate) mod msg_move_set_walk_mode;
pub use msg_move_set_walk_mode::*;
pub(crate) mod msg_move_start_ascend;
pub use msg_move_start_ascend::*;
pub(crate) mod msg_move_start_backward;
pub use msg_move_start_backward::*;
pub(crate) mod msg_move_start_descend;
pub use msg_move_start_descend::*;
pub(crate) mod msg_move_start_forward;
pub use msg_move_start_forward::*;
pub(crate) mod msg_move_start_pitch_down;
pub use msg_move_start_pitch_down::*;
pub(crate) mod msg_move_start_pitch_up;
pub use msg_move_start_pitch_up::*;
pub(crate) mod msg_move_start_strafe_left;
pub use msg_move_start_strafe_left::*;
pub(crate) mod msg_move_start_strafe_right;
pub use msg_move_start_strafe_right::*;
pub(crate) mod msg_move_start_swim;
pub use msg_move_start_swim::*;
pub(crate) mod msg_move_start_turn_left;
pub use msg_move_start_turn_left::*;
pub(crate) mod msg_move_start_turn_right;
pub use msg_move_start_turn_right::*;
pub(crate) mod msg_move_stop;
pub use msg_move_stop::*;
pub(crate) mod msg_move_stop_ascend;
pub use msg_move_stop_ascend::*;
pub(crate) mod msg_move_stop_pitch;
pub use msg_move_stop_pitch::*;
pub(crate) mod msg_move_stop_strafe;
pub use msg_move_stop_strafe::*;
pub(crate) mod msg_move_stop_swim;
pub use msg_move_stop_swim::*;
pub(crate) mod msg_move_stop_turn;
pub use msg_move_stop_turn::*;
pub(crate) mod msg_move_teleport_ack_server;
pub use msg_move_teleport_ack_server::*;
pub(crate) mod msg_quest_push_result;
pub use msg_quest_push_result::*;
pub(crate) mod msg_save_guild_emblem_server;
pub use msg_save_guild_emblem_server::*;
pub(crate) mod msg_set_dungeon_difficulty;
pub use msg_set_dungeon_difficulty::*;
pub(crate) mod npc_text_update;
pub use npc_text_update::*;
pub(crate) mod object;
pub use object::*;
pub(crate) mod object_type;
pub use object_type::*;
pub mod opcodes;
pub(crate) mod pet_tame_failure_reason;
pub use pet_tame_failure_reason::*;
pub(crate) mod player_chat_tag;
pub use player_chat_tag::*;
pub(crate) mod power;
pub use power::*;
pub use crate::shared::activate_taxi_reply_vanilla_tbc_wrath::ActivateTaxiReply;

pub use crate::shared::addon_info_vanilla_tbc_wrath::AddonInfo;

pub use crate::shared::ai_reaction_vanilla_tbc_wrath::AiReaction;

pub use crate::shared::auction_enchantment_tbc_wrath::AuctionEnchantment;

pub use crate::shared::auction_sort_tbc_wrath::AuctionSort;

pub use crate::shared::battlefield_port_action_vanilla_tbc_wrath::BattlefieldPortAction;

pub use crate::shared::battleground_player_position_vanilla_tbc_wrath::BattlegroundPlayerPosition;

pub use crate::shared::billing_plan_flags_tbc_wrath::BillingPlanFlags;

pub use crate::shared::buy_bank_slot_result_vanilla_tbc_wrath::BuyBankSlotResult;

pub use crate::shared::buy_result_vanilla_tbc_wrath::BuyResult;

pub use crate::shared::buyback_slot_vanilla_tbc_wrath::BuybackSlot;

pub use crate::shared::channel_flags_vanilla_tbc_wrath::ChannelFlags;

pub use crate::shared::channel_member_flags_vanilla_tbc_wrath::ChannelMemberFlags;

pub use crate::shared::channel_member_vanilla_tbc_wrath::ChannelMember;

pub use crate::shared::chat_notify_tbc_wrath::ChatNotify;

pub use crate::shared::cmsg_accept_trade_vanilla_tbc_wrath::CMSG_ACCEPT_TRADE;

pub use crate::shared::cmsg_activatetaxi_vanilla_tbc_wrath::CMSG_ACTIVATETAXI;

pub use crate::shared::cmsg_add_friend_tbc_wrath::CMSG_ADD_FRIEND;

pub use crate::shared::cmsg_add_ignore_vanilla_tbc_wrath::CMSG_ADD_IGNORE;

pub use crate::shared::cmsg_area_spirit_healer_query_vanilla_tbc_wrath::CMSG_AREA_SPIRIT_HEALER_QUERY;

pub use crate::shared::cmsg_areatrigger_vanilla_tbc_wrath::CMSG_AREATRIGGER;

pub use crate::shared::cmsg_attackstop_vanilla_tbc_wrath::CMSG_ATTACKSTOP;

pub use crate::shared::cmsg_attackswing_vanilla_tbc_wrath::CMSG_ATTACKSWING;

pub use crate::shared::cmsg_auction_list_bidder_items_vanilla_tbc_wrath::CMSG_AUCTION_LIST_BIDDER_ITEMS;

pub use crate::shared::cmsg_auction_list_owner_items_vanilla_tbc_wrath::CMSG_AUCTION_LIST_OWNER_ITEMS;

pub use crate::shared::cmsg_auction_place_bid_vanilla_tbc_wrath::CMSG_AUCTION_PLACE_BID;

pub use crate::shared::cmsg_auction_remove_item_vanilla_tbc_wrath::CMSG_AUCTION_REMOVE_ITEM;

pub use crate::shared::cmsg_autobank_item_vanilla_tbc_wrath::CMSG_AUTOBANK_ITEM;

pub use crate::shared::cmsg_autoequip_item_slot_vanilla_tbc_wrath::CMSG_AUTOEQUIP_ITEM_SLOT;

pub use crate::shared::cmsg_autoequip_item_vanilla_tbc_wrath::CMSG_AUTOEQUIP_ITEM;

pub use crate::shared::cmsg_autostore_bag_item_vanilla_tbc_wrath::CMSG_AUTOSTORE_BAG_ITEM;

pub use crate::shared::cmsg_autostore_bank_item_vanilla_tbc_wrath::CMSG_AUTOSTORE_BANK_ITEM;

pub use crate::shared::cmsg_autostore_loot_item_vanilla_tbc_wrath::CMSG_AUTOSTORE_LOOT_ITEM;

pub use crate::shared::cmsg_banker_activate_vanilla_tbc_wrath::CMSG_BANKER_ACTIVATE;

pub use crate::shared::cmsg_battlefield_port_tbc_wrath::CMSG_BATTLEFIELD_PORT;

pub use crate::shared::cmsg_battlefield_status_vanilla_tbc_wrath::CMSG_BATTLEFIELD_STATUS;

pub use crate::shared::cmsg_battlemaster_hello_vanilla_tbc_wrath::CMSG_BATTLEMASTER_HELLO;

pub use crate::shared::cmsg_begin_trade_vanilla_tbc_wrath::CMSG_BEGIN_TRADE;

pub use crate::shared::cmsg_binder_activate_vanilla_tbc_wrath::CMSG_BINDER_ACTIVATE;

pub use crate::shared::cmsg_bootme_vanilla_tbc_wrath::CMSG_BOOTME;

pub use crate::shared::cmsg_bug_vanilla_tbc_wrath::CMSG_BUG;

pub use crate::shared::cmsg_busy_trade_vanilla_tbc_wrath::CMSG_BUSY_TRADE;

pub use crate::shared::cmsg_buy_bank_slot_vanilla_tbc_wrath::CMSG_BUY_BANK_SLOT;

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

pub use crate::shared::cmsg_chat_ignored_tbc_wrath::CMSG_CHAT_IGNORED;

pub use crate::shared::cmsg_clear_trade_item_vanilla_tbc_wrath::CMSG_CLEAR_TRADE_ITEM;

pub use crate::shared::cmsg_complete_cinematic_vanilla_tbc_wrath::CMSG_COMPLETE_CINEMATIC;

pub use crate::shared::cmsg_contact_list_tbc_wrath::CMSG_CONTACT_LIST;

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

pub use crate::shared::cmsg_gmticket_updatetext_tbc_wrath::CMSG_GMTICKET_UPDATETEXT;

pub use crate::shared::cmsg_gossip_hello_vanilla_tbc_wrath::CMSG_GOSSIP_HELLO;

pub use crate::shared::cmsg_gossip_select_option_tbc_wrath::CMSG_GOSSIP_SELECT_OPTION;

pub use crate::shared::cmsg_group_assistant_leader_vanilla_tbc_wrath::CMSG_GROUP_ASSISTANT_LEADER;

pub use crate::shared::cmsg_group_change_sub_group_vanilla_tbc_wrath::CMSG_GROUP_CHANGE_SUB_GROUP;

pub use crate::shared::cmsg_group_decline_vanilla_tbc_wrath::CMSG_GROUP_DECLINE;

pub use crate::shared::cmsg_group_disband_vanilla_tbc_wrath::CMSG_GROUP_DISBAND;

pub use crate::shared::cmsg_group_raid_convert_vanilla_tbc_wrath::CMSG_GROUP_RAID_CONVERT;

pub use crate::shared::cmsg_group_set_leader_vanilla_tbc_wrath::CMSG_GROUP_SET_LEADER;

pub use crate::shared::cmsg_group_swap_sub_group_vanilla_tbc_wrath::CMSG_GROUP_SWAP_SUB_GROUP;

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

pub use crate::shared::cmsg_guild_rank_tbc_wrath::CMSG_GUILD_RANK;

pub use crate::shared::cmsg_guild_remove_vanilla_tbc_wrath::CMSG_GUILD_REMOVE;

pub use crate::shared::cmsg_guild_roster_vanilla_tbc_wrath::CMSG_GUILD_ROSTER;

pub use crate::shared::cmsg_guild_set_officer_note_vanilla_tbc_wrath::CMSG_GUILD_SET_OFFICER_NOTE;

pub use crate::shared::cmsg_guild_set_public_note_vanilla_tbc_wrath::CMSG_GUILD_SET_PUBLIC_NOTE;

pub use crate::shared::cmsg_ignore_trade_vanilla_tbc_wrath::CMSG_IGNORE_TRADE;

pub use crate::shared::cmsg_initiate_trade_vanilla_tbc_wrath::CMSG_INITIATE_TRADE;

pub use crate::shared::cmsg_inspect_vanilla_tbc_wrath::CMSG_INSPECT;

pub use crate::shared::cmsg_item_name_query_vanilla_tbc_wrath::CMSG_ITEM_NAME_QUERY;

pub use crate::shared::cmsg_item_query_single_tbc_wrath::CMSG_ITEM_QUERY_SINGLE;

pub use crate::shared::cmsg_join_channel_tbc_wrath::CMSG_JOIN_CHANNEL;

pub use crate::shared::cmsg_learn_talent_vanilla_tbc_wrath::CMSG_LEARN_TALENT;

pub use crate::shared::cmsg_leave_channel_tbc_wrath::CMSG_LEAVE_CHANNEL;

pub use crate::shared::cmsg_list_inventory_vanilla_tbc_wrath::CMSG_LIST_INVENTORY;

pub use crate::shared::cmsg_logout_cancel_vanilla_tbc_wrath::CMSG_LOGOUT_CANCEL;

pub use crate::shared::cmsg_logout_request_vanilla_tbc_wrath::CMSG_LOGOUT_REQUEST;

pub use crate::shared::cmsg_loot_master_give_vanilla_tbc_wrath::CMSG_LOOT_MASTER_GIVE;

pub use crate::shared::cmsg_loot_money_vanilla_tbc_wrath::CMSG_LOOT_MONEY;

pub use crate::shared::cmsg_loot_release_vanilla_tbc_wrath::CMSG_LOOT_RELEASE;

pub use crate::shared::cmsg_loot_roll_tbc_wrath::CMSG_LOOT_ROLL;

pub use crate::shared::cmsg_loot_vanilla_tbc_wrath::CMSG_LOOT;

pub use crate::shared::cmsg_mail_delete_tbc_wrath::CMSG_MAIL_DELETE;

pub use crate::shared::cmsg_mail_mark_as_read_vanilla_tbc_wrath::CMSG_MAIL_MARK_AS_READ;

pub use crate::shared::cmsg_mail_return_to_sender_vanilla_tbc_wrath::CMSG_MAIL_RETURN_TO_SENDER;

pub use crate::shared::cmsg_mail_take_item_tbc_wrath::CMSG_MAIL_TAKE_ITEM;

pub use crate::shared::cmsg_mail_take_money_vanilla_tbc_wrath::CMSG_MAIL_TAKE_MONEY;

pub use crate::shared::cmsg_mountspecial_anim_vanilla_tbc_wrath::CMSG_MOUNTSPECIAL_ANIM;

pub use crate::shared::cmsg_move_set_raw_position_vanilla_tbc_wrath::CMSG_MOVE_SET_RAW_POSITION;

pub use crate::shared::cmsg_name_query_vanilla_tbc_wrath::CMSG_NAME_QUERY;

pub use crate::shared::cmsg_next_cinematic_camera_vanilla_tbc_wrath::CMSG_NEXT_CINEMATIC_CAMERA;

pub use crate::shared::cmsg_npc_text_query_vanilla_tbc_wrath::CMSG_NPC_TEXT_QUERY;

pub use crate::shared::cmsg_offer_petition_tbc_wrath::CMSG_OFFER_PETITION;

pub use crate::shared::cmsg_open_item_vanilla_tbc_wrath::CMSG_OPEN_ITEM;

pub use crate::shared::cmsg_page_text_query_tbc_wrath::CMSG_PAGE_TEXT_QUERY;

pub use crate::shared::cmsg_pet_abandon_vanilla_tbc_wrath::CMSG_PET_ABANDON;

pub use crate::shared::cmsg_pet_action_vanilla_tbc_wrath::CMSG_PET_ACTION;

pub use crate::shared::cmsg_pet_cancel_aura_vanilla_tbc_wrath::CMSG_PET_CANCEL_AURA;

pub use crate::shared::cmsg_pet_name_query_vanilla_tbc_wrath::CMSG_PET_NAME_QUERY;

pub use crate::shared::cmsg_pet_rename_tbc_wrath::CMSG_PET_RENAME;

pub use crate::shared::cmsg_pet_set_action_vanilla_tbc_wrath::CMSG_PET_SET_ACTION;

pub use crate::shared::cmsg_pet_spell_autocast_vanilla_tbc_wrath::CMSG_PET_SPELL_AUTOCAST;

pub use crate::shared::cmsg_pet_stop_attack_vanilla_tbc_wrath::CMSG_PET_STOP_ATTACK;

pub use crate::shared::cmsg_pet_unlearn_vanilla_tbc_wrath::CMSG_PET_UNLEARN;

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

pub use crate::shared::cmsg_questgiver_quest_autolaunch_vanilla_tbc_wrath::CMSG_QUESTGIVER_QUEST_AUTOLAUNCH;

pub use crate::shared::cmsg_questgiver_request_reward_vanilla_tbc_wrath::CMSG_QUESTGIVER_REQUEST_REWARD;

pub use crate::shared::cmsg_questgiver_status_query_vanilla_tbc_wrath::CMSG_QUESTGIVER_STATUS_QUERY;

pub use crate::shared::cmsg_questlog_remove_quest_vanilla_tbc_wrath::CMSG_QUESTLOG_REMOVE_QUEST;

pub use crate::shared::cmsg_questlog_swap_quest_vanilla_tbc_wrath::CMSG_QUESTLOG_SWAP_QUEST;

pub use crate::shared::cmsg_read_item_vanilla_tbc_wrath::CMSG_READ_ITEM;

pub use crate::shared::cmsg_realm_split_tbc_wrath::CMSG_REALM_SPLIT;

pub use crate::shared::cmsg_reclaim_corpse_vanilla_tbc_wrath::CMSG_RECLAIM_CORPSE;

pub use crate::shared::cmsg_repair_item_tbc_wrath::CMSG_REPAIR_ITEM;

pub use crate::shared::cmsg_repop_request_vanilla_tbc_wrath::CMSG_REPOP_REQUEST;

pub use crate::shared::cmsg_request_account_data_vanilla_tbc_wrath::CMSG_REQUEST_ACCOUNT_DATA;

pub use crate::shared::cmsg_request_party_member_stats_vanilla_tbc_wrath::CMSG_REQUEST_PARTY_MEMBER_STATS;

pub use crate::shared::cmsg_request_pet_info_vanilla_tbc_wrath::CMSG_REQUEST_PET_INFO;

pub use crate::shared::cmsg_request_raid_info_vanilla_tbc_wrath::CMSG_REQUEST_RAID_INFO;

pub use crate::shared::cmsg_reset_instances_vanilla_tbc_wrath::CMSG_RESET_INSTANCES;

pub use crate::shared::cmsg_resurrect_response_vanilla_tbc_wrath::CMSG_RESURRECT_RESPONSE;

pub use crate::shared::cmsg_sell_item_vanilla_tbc_wrath::CMSG_SELL_ITEM;

pub use crate::shared::cmsg_send_mail_tbc_wrath::CMSG_SEND_MAIL;

pub use crate::shared::cmsg_set_action_button_vanilla_tbc_wrath::CMSG_SET_ACTION_BUTTON;

pub use crate::shared::cmsg_set_actionbar_toggles_vanilla_tbc_wrath::CMSG_SET_ACTIONBAR_TOGGLES;

pub use crate::shared::cmsg_set_active_mover_vanilla_tbc_wrath::CMSG_SET_ACTIVE_MOVER;

pub use crate::shared::cmsg_set_ammo_vanilla_tbc_wrath::CMSG_SET_AMMO;

pub use crate::shared::cmsg_set_selection_vanilla_tbc_wrath::CMSG_SET_SELECTION;

pub use crate::shared::cmsg_set_trade_gold_vanilla_tbc_wrath::CMSG_SET_TRADE_GOLD;

pub use crate::shared::cmsg_set_trade_item_vanilla_tbc_wrath::CMSG_SET_TRADE_ITEM;

pub use crate::shared::cmsg_set_watched_faction_vanilla_tbc_wrath::CMSG_SET_WATCHED_FACTION;

pub use crate::shared::cmsg_setsheathed_vanilla_tbc_wrath::CMSG_SETSHEATHED;

pub use crate::shared::cmsg_spirit_healer_activate_vanilla_tbc_wrath::CMSG_SPIRIT_HEALER_ACTIVATE;

pub use crate::shared::cmsg_stable_pet_vanilla_tbc_wrath::CMSG_STABLE_PET;

pub use crate::shared::cmsg_stable_swap_pet_vanilla_tbc_wrath::CMSG_STABLE_SWAP_PET;

pub use crate::shared::cmsg_standstatechange_vanilla_tbc_wrath::CMSG_STANDSTATECHANGE;

pub use crate::shared::cmsg_swap_inv_item_vanilla_tbc_wrath::CMSG_SWAP_INV_ITEM;

pub use crate::shared::cmsg_swap_item_vanilla_tbc_wrath::CMSG_SWAP_ITEM;

pub use crate::shared::cmsg_taxinode_status_query_vanilla_tbc_wrath::CMSG_TAXINODE_STATUS_QUERY;

pub use crate::shared::cmsg_taxiqueryavailablenodes_vanilla_tbc_wrath::CMSG_TAXIQUERYAVAILABLENODES;

pub use crate::shared::cmsg_teleport_to_unit_vanilla_tbc_wrath::CMSG_TELEPORT_TO_UNIT;

pub use crate::shared::cmsg_time_sync_resp_tbc_wrath::CMSG_TIME_SYNC_RESP;

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

pub use crate::shared::cmsg_unlearn_skill_vanilla_tbc_wrath::CMSG_UNLEARN_SKILL;

pub use crate::shared::cmsg_unstable_pet_vanilla_tbc_wrath::CMSG_UNSTABLE_PET;

pub use crate::shared::cmsg_who_vanilla_tbc_wrath::CMSG_WHO;

pub use crate::shared::cmsg_whois_vanilla_tbc_wrath::CMSG_WHOIS;

pub use crate::shared::cmsg_wrap_item_vanilla_tbc_wrath::CMSG_WRAP_ITEM;

pub use crate::shared::cooldown_spell_vanilla_tbc_wrath::CooldownSpell;

pub use crate::shared::corpse_query_result_vanilla_tbc_wrath::CorpseQueryResult;

pub use crate::shared::declined_pet_name_included_tbc_wrath::DeclinedPetNameIncluded;

pub use crate::shared::duel_winner_reason_vanilla_tbc_wrath::DuelWinnerReason;

pub use crate::shared::dungeon_difficulty_tbc_wrath::DungeonDifficulty;

pub use crate::shared::environmental_damage_type_vanilla_tbc_wrath::EnvironmentalDamageType;

pub use crate::shared::experience_award_type_vanilla_tbc_wrath::ExperienceAwardType;

pub use crate::shared::faction_vanilla_tbc_wrath::Faction;

pub use crate::shared::far_sight_operation_vanilla_tbc_wrath::FarSightOperation;

pub use crate::shared::forced_reaction_vanilla_tbc_wrath::ForcedReaction;

pub use crate::shared::gm_survey_question_vanilla_tbc_wrath::GmSurveyQuestion;

pub use crate::shared::gm_ticket_escalation_status_vanilla_tbc_wrath::GmTicketEscalationStatus;

pub use crate::shared::gm_ticket_queue_status_vanilla_tbc_wrath::GmTicketQueueStatus;

pub use crate::shared::gm_ticket_response_vanilla_tbc_wrath::GmTicketResponse;

pub use crate::shared::gm_ticket_status_response_vanilla_tbc_wrath::GmTicketStatusResponse;

pub use crate::shared::gm_ticket_status_vanilla_tbc_wrath::GmTicketStatus;

pub use crate::shared::gossip_item_tbc_wrath::GossipItem;

pub use crate::shared::group_loot_setting_vanilla_tbc_wrath::GroupLootSetting;

pub use crate::shared::guild_bank_rights_tbc_wrath::GuildBankRights;

pub use crate::shared::guild_emblem_result_tbc_wrath::GuildEmblemResult;

pub use crate::shared::guild_event_tbc_wrath::GuildEvent;

pub use crate::shared::guild_member_status_vanilla_tbc_wrath::GuildMemberStatus;

pub use crate::shared::guild_rights_tbc_wrath::GuildRights;

pub use crate::shared::inventory_type_vanilla_tbc_wrath::InventoryType;

pub use crate::shared::language_tbc_wrath::Language;

pub use crate::shared::list_inventory_item_tbc_wrath::ListInventoryItem;

pub use crate::shared::log_format_vanilla_tbc_wrath::LogFormat;

pub use crate::shared::logout_result_vanilla_tbc_wrath::LogoutResult;

pub use crate::shared::logout_speed_vanilla_tbc_wrath::LogoutSpeed;

pub use crate::shared::mail_item_tbc_wrath::MailItem;

pub use crate::shared::mail_message_type_vanilla_tbc_wrath::MailMessageType;

pub use crate::shared::mount_result_vanilla_tbc_wrath::MountResult;

pub use crate::shared::msg_auction_hello_client_vanilla_tbc_wrath::MSG_AUCTION_HELLO_Client;

pub use crate::shared::msg_battleground_player_positions_client_vanilla_tbc_wrath::MSG_BATTLEGROUND_PLAYER_POSITIONS_Client;

pub use crate::shared::msg_battleground_player_positions_server_vanilla_tbc_wrath::MSG_BATTLEGROUND_PLAYER_POSITIONS_Server;

pub use crate::shared::msg_corpse_query_client_vanilla_tbc_wrath::MSG_CORPSE_QUERY_Client;

pub use crate::shared::msg_inspect_honor_stats_client_vanilla_tbc_wrath::MSG_INSPECT_HONOR_STATS_Client;

pub use crate::shared::msg_inspect_honor_stats_server_tbc_wrath::MSG_INSPECT_HONOR_STATS_Server;

pub use crate::shared::msg_list_stabled_pets_client_vanilla_tbc_wrath::MSG_LIST_STABLED_PETS_Client;

pub use crate::shared::msg_list_stabled_pets_server_vanilla_tbc_wrath::MSG_LIST_STABLED_PETS_Server;

pub use crate::shared::msg_move_teleport_ack_client_vanilla_tbc_wrath::MSG_MOVE_TELEPORT_ACK_Client;

pub use crate::shared::msg_move_worldport_ack_vanilla_tbc_wrath::MSG_MOVE_WORLDPORT_ACK;

pub use crate::shared::msg_petition_decline_vanilla_tbc_wrath::MSG_PETITION_DECLINE;

pub use crate::shared::msg_petition_rename_vanilla_tbc_wrath::MSG_PETITION_RENAME;

pub use crate::shared::msg_pvp_log_data_client_vanilla_tbc_wrath::MSG_PVP_LOG_DATA_Client;

pub use crate::shared::msg_query_next_mail_time_client_vanilla_tbc_wrath::MSG_QUERY_NEXT_MAIL_TIME_Client;

pub use crate::shared::msg_query_next_mail_time_server_tbc_wrath::MSG_QUERY_NEXT_MAIL_TIME_Server;

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

pub use crate::shared::pet_name_invalid_reason_tbc_wrath::PetNameInvalidReason;

pub use crate::shared::pet_query_disabled_names_tbc_wrath::PetQueryDisabledNames;

pub use crate::shared::pet_react_state_vanilla_tbc_wrath::PetReactState;

pub use crate::shared::pet_talk_reason_vanilla_tbc_wrath::PetTalkReason;

pub use crate::shared::petition_result_tbc_wrath::PetitionResult;

pub use crate::shared::petition_showlist_tbc_wrath::PetitionShowlist;

pub use crate::shared::petition_signature_vanilla_tbc_wrath::PetitionSignature;

pub use crate::shared::pvp_rank_vanilla_tbc_wrath::PvpRank;

pub use crate::shared::realm_split_state_tbc_wrath::RealmSplitState;

pub use crate::shared::received_mail_tbc_wrath::ReceivedMail;

pub use crate::shared::roll_vote_tbc_wrath::RollVote;

pub use crate::shared::sheath_state_vanilla_tbc_wrath::SheathState;

pub use crate::shared::smsg_activatetaxireply_vanilla_tbc_wrath::SMSG_ACTIVATETAXIREPLY;

pub use crate::shared::smsg_ai_reaction_vanilla_tbc_wrath::SMSG_AI_REACTION;

pub use crate::shared::smsg_area_trigger_message_vanilla_tbc_wrath::SMSG_AREA_TRIGGER_MESSAGE;

pub use crate::shared::smsg_attackstart_vanilla_tbc_wrath::SMSG_ATTACKSTART;

pub use crate::shared::smsg_attackstop_vanilla_tbc_wrath::SMSG_ATTACKSTOP;

pub use crate::shared::smsg_attackswing_badfacing_vanilla_tbc_wrath::SMSG_ATTACKSWING_BADFACING;

pub use crate::shared::smsg_attackswing_cant_attack_vanilla_tbc_wrath::SMSG_ATTACKSWING_CANT_ATTACK;

pub use crate::shared::smsg_attackswing_deadtarget_vanilla_tbc_wrath::SMSG_ATTACKSWING_DEADTARGET;

pub use crate::shared::smsg_attackswing_notinrange_vanilla_tbc_wrath::SMSG_ATTACKSWING_NOTINRANGE;

pub use crate::shared::smsg_auction_removed_notification_vanilla_tbc_wrath::SMSG_AUCTION_REMOVED_NOTIFICATION;

pub use crate::shared::smsg_battleground_player_joined_vanilla_tbc_wrath::SMSG_BATTLEGROUND_PLAYER_JOINED;

pub use crate::shared::smsg_battleground_player_left_vanilla_tbc_wrath::SMSG_BATTLEGROUND_PLAYER_LEFT;

pub use crate::shared::smsg_buy_bank_slot_result_vanilla_tbc_wrath::SMSG_BUY_BANK_SLOT_RESULT;

pub use crate::shared::smsg_buy_failed_vanilla_tbc_wrath::SMSG_BUY_FAILED;

pub use crate::shared::smsg_buy_item_vanilla_tbc_wrath::SMSG_BUY_ITEM;

pub use crate::shared::smsg_cancel_combat_vanilla_tbc_wrath::SMSG_CANCEL_COMBAT;

pub use crate::shared::smsg_channel_list_vanilla_tbc_wrath::SMSG_CHANNEL_LIST;

pub use crate::shared::smsg_channel_notify_tbc_wrath::SMSG_CHANNEL_NOTIFY;

pub use crate::shared::smsg_chat_player_not_found_vanilla_tbc_wrath::SMSG_CHAT_PLAYER_NOT_FOUND;

pub use crate::shared::smsg_chat_wrong_faction_vanilla_tbc_wrath::SMSG_CHAT_WRONG_FACTION;

pub use crate::shared::smsg_client_control_update_vanilla_tbc_wrath::SMSG_CLIENT_CONTROL_UPDATE;

pub use crate::shared::smsg_duel_complete_vanilla_tbc_wrath::SMSG_DUEL_COMPLETE;

pub use crate::shared::smsg_duel_countdown_vanilla_tbc_wrath::SMSG_DUEL_COUNTDOWN;

pub use crate::shared::smsg_duel_inbounds_vanilla_tbc_wrath::SMSG_DUEL_INBOUNDS;

pub use crate::shared::smsg_duel_outofbounds_vanilla_tbc_wrath::SMSG_DUEL_OUTOFBOUNDS;

pub use crate::shared::smsg_duel_requested_vanilla_tbc_wrath::SMSG_DUEL_REQUESTED;

pub use crate::shared::smsg_duel_winner_vanilla_tbc_wrath::SMSG_DUEL_WINNER;

pub use crate::shared::smsg_durability_damage_death_vanilla_tbc_wrath::SMSG_DURABILITY_DAMAGE_DEATH;

pub use crate::shared::smsg_environmental_damage_log_vanilla_tbc_wrath::SMSG_ENVIRONMENTAL_DAMAGE_LOG;

pub use crate::shared::smsg_force_run_back_speed_change_vanilla_tbc_wrath::SMSG_FORCE_RUN_BACK_SPEED_CHANGE;

pub use crate::shared::smsg_force_run_speed_change_tbc_wrath::SMSG_FORCE_RUN_SPEED_CHANGE;

pub use crate::shared::smsg_force_swim_back_speed_change_vanilla_tbc_wrath::SMSG_FORCE_SWIM_BACK_SPEED_CHANGE;

pub use crate::shared::smsg_force_swim_speed_change_vanilla_tbc_wrath::SMSG_FORCE_SWIM_SPEED_CHANGE;

pub use crate::shared::smsg_force_turn_rate_change_vanilla_tbc_wrath::SMSG_FORCE_TURN_RATE_CHANGE;

pub use crate::shared::smsg_force_walk_speed_change_vanilla_tbc_wrath::SMSG_FORCE_WALK_SPEED_CHANGE;

pub use crate::shared::smsg_gameobject_custom_anim_vanilla_tbc_wrath::SMSG_GAMEOBJECT_CUSTOM_ANIM;

pub use crate::shared::smsg_gameobject_despawn_anim_vanilla_tbc_wrath::SMSG_GAMEOBJECT_DESPAWN_ANIM;

pub use crate::shared::smsg_gameobject_pagetext_vanilla_tbc_wrath::SMSG_GAMEOBJECT_PAGETEXT;

pub use crate::shared::smsg_gm_ticket_status_update_vanilla_tbc_wrath::SMSG_GM_TICKET_STATUS_UPDATE;

pub use crate::shared::smsg_gmticket_create_vanilla_tbc_wrath::SMSG_GMTICKET_CREATE;

pub use crate::shared::smsg_gmticket_deleteticket_vanilla_tbc_wrath::SMSG_GMTICKET_DELETETICKET;

pub use crate::shared::smsg_gmticket_systemstatus_vanilla_tbc_wrath::SMSG_GMTICKET_SYSTEMSTATUS;

pub use crate::shared::smsg_gmticket_updatetext_vanilla_tbc_wrath::SMSG_GMTICKET_UPDATETEXT;

pub use crate::shared::smsg_gossip_complete_vanilla_tbc_wrath::SMSG_GOSSIP_COMPLETE;

pub use crate::shared::smsg_gossip_poi_vanilla_tbc_wrath::SMSG_GOSSIP_POI;

pub use crate::shared::smsg_guild_event_tbc_wrath::SMSG_GUILD_EVENT;

pub use crate::shared::smsg_guild_invite_vanilla_tbc_wrath::SMSG_GUILD_INVITE;

pub use crate::shared::smsg_guild_query_response_vanilla_tbc_wrath::SMSG_GUILD_QUERY_RESPONSE;

pub use crate::shared::smsg_item_cooldown_vanilla_tbc_wrath::SMSG_ITEM_COOLDOWN;

pub use crate::shared::smsg_item_enchant_time_update_vanilla_tbc_wrath::SMSG_ITEM_ENCHANT_TIME_UPDATE;

pub use crate::shared::smsg_item_name_query_response_tbc_wrath::SMSG_ITEM_NAME_QUERY_RESPONSE;

pub use crate::shared::smsg_item_push_result_tbc_wrath::SMSG_ITEM_PUSH_RESULT;

pub use crate::shared::smsg_item_time_update_vanilla_tbc_wrath::SMSG_ITEM_TIME_UPDATE;

pub use crate::shared::smsg_list_inventory_tbc_wrath::SMSG_LIST_INVENTORY;

pub use crate::shared::smsg_logout_cancel_ack_vanilla_tbc_wrath::SMSG_LOGOUT_CANCEL_ACK;

pub use crate::shared::smsg_logout_complete_vanilla_tbc_wrath::SMSG_LOGOUT_COMPLETE;

pub use crate::shared::smsg_logout_response_vanilla_tbc_wrath::SMSG_LOGOUT_RESPONSE;

pub use crate::shared::smsg_loot_all_passed_vanilla_tbc_wrath::SMSG_LOOT_ALL_PASSED;

pub use crate::shared::smsg_loot_clear_money_vanilla_tbc_wrath::SMSG_LOOT_CLEAR_MONEY;

pub use crate::shared::smsg_loot_master_list_vanilla_tbc_wrath::SMSG_LOOT_MASTER_LIST;

pub use crate::shared::smsg_loot_release_response_vanilla_tbc_wrath::SMSG_LOOT_RELEASE_RESPONSE;

pub use crate::shared::smsg_loot_removed_vanilla_tbc_wrath::SMSG_LOOT_REMOVED;

pub use crate::shared::smsg_loot_roll_tbc_wrath::SMSG_LOOT_ROLL;

pub use crate::shared::smsg_loot_roll_won_tbc_wrath::SMSG_LOOT_ROLL_WON;

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

pub use crate::shared::smsg_pet_name_invalid_tbc_wrath::SMSG_PET_NAME_INVALID;

pub use crate::shared::smsg_pet_name_query_response_tbc_wrath::SMSG_PET_NAME_QUERY_RESPONSE;

pub use crate::shared::smsg_pet_unlearn_confirm_vanilla_tbc_wrath::SMSG_PET_UNLEARN_CONFIRM;

pub use crate::shared::smsg_petition_show_signatures_vanilla_tbc_wrath::SMSG_PETITION_SHOW_SIGNATURES;

pub use crate::shared::smsg_petition_sign_results_tbc_wrath::SMSG_PETITION_SIGN_RESULTS;

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

pub use crate::shared::smsg_realm_split_tbc_wrath::SMSG_REALM_SPLIT;

pub use crate::shared::smsg_received_mail_vanilla_tbc_wrath::SMSG_RECEIVED_MAIL;

pub use crate::shared::smsg_set_faction_visible_vanilla_tbc_wrath::SMSG_SET_FACTION_VISIBLE;

pub use crate::shared::smsg_set_forced_reactions_vanilla_tbc_wrath::SMSG_SET_FORCED_REACTIONS;

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

pub use crate::shared::smsg_standstate_update_vanilla_tbc_wrath::SMSG_STANDSTATE_UPDATE;

pub use crate::shared::smsg_taxinode_status_vanilla_tbc_wrath::SMSG_TAXINODE_STATUS;

pub use crate::shared::smsg_time_sync_req_tbc_wrath::SMSG_TIME_SYNC_REQ;

pub use crate::shared::smsg_turn_in_petition_results_tbc_wrath::SMSG_TURN_IN_PETITION_RESULTS;

pub use crate::shared::smsg_tutorial_flags_vanilla_tbc_wrath::SMSG_TUTORIAL_FLAGS;

pub use crate::shared::smsg_update_world_state_vanilla_tbc_wrath::SMSG_UPDATE_WORLD_STATE;

pub use crate::shared::stabled_pet_vanilla_tbc_wrath::StabledPet;

pub use crate::shared::unit_stand_state_vanilla_tbc_wrath::UnitStandState;

pub use crate::shared::world_state_vanilla_tbc_wrath::WorldState;

pub use wow_world_base::wrath::Gender;

pub use wow_world_base::wrath::Vector2d;

pub use wow_world_base::wrath::Vector3d;

pub(crate) mod quest_failed_reason;
pub use quest_failed_reason::*;
pub(crate) mod quest_giver_status;
pub use quest_giver_status::*;
pub(crate) mod quest_item;
pub use quest_item::*;
pub(crate) mod quest_party_message;
pub use quest_party_message::*;
pub(crate) mod race;
pub use race::*;
pub(crate) mod raid_info;
pub use raid_info::*;
pub(crate) mod random_bg;
pub use random_bg::*;
pub(crate) mod relation;
pub use relation::*;
pub(crate) mod relation_type;
pub use relation_type::*;
pub(crate) mod roll_flags;
pub use roll_flags::*;
pub(crate) mod sell_item_result;
pub use sell_item_result::*;
pub(crate) mod server_message_type;
pub use server_message_type::*;
pub(crate) mod smsg_account_data_times;
pub use smsg_account_data_times::*;
pub(crate) mod smsg_action_buttons;
pub use smsg_action_buttons::*;
pub(crate) mod smsg_addon_info;
pub use smsg_addon_info::*;
pub(crate) mod smsg_attackerstateupdate;
pub use smsg_attackerstateupdate::*;
pub(crate) mod smsg_auction_bidder_list_result;
pub use smsg_auction_bidder_list_result::*;
pub(crate) mod smsg_auction_bidder_notification;
pub use smsg_auction_bidder_notification::*;
pub(crate) mod smsg_auction_list_result;
pub use smsg_auction_list_result::*;
pub(crate) mod smsg_auction_owner_list_result;
pub use smsg_auction_owner_list_result::*;
pub(crate) mod smsg_auction_owner_notification;
pub use smsg_auction_owner_notification::*;
pub(crate) mod smsg_auth_challenge;
pub use smsg_auth_challenge::*;
pub(crate) mod smsg_auth_response;
pub use smsg_auth_response::*;
pub(crate) mod smsg_battlefield_list;
pub use smsg_battlefield_list::*;
pub(crate) mod smsg_binder_confirm;
pub use smsg_binder_confirm::*;
pub(crate) mod smsg_bindpointupdate;
pub use smsg_bindpointupdate::*;
pub(crate) mod smsg_calendar_send_num_pending;
pub use smsg_calendar_send_num_pending::*;
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
pub(crate) mod smsg_clientcache_version;
pub use smsg_clientcache_version::*;
pub(crate) mod smsg_contact_list;
pub use smsg_contact_list::*;
pub(crate) mod smsg_creature_query_response;
pub use smsg_creature_query_response::*;
pub(crate) mod smsg_defense_message;
pub use smsg_defense_message::*;
pub(crate) mod smsg_destroy_object;
pub use smsg_destroy_object::*;
pub(crate) mod smsg_emote;
pub use smsg_emote::*;
pub(crate) mod smsg_exploration_experience;
pub use smsg_exploration_experience::*;
pub(crate) mod smsg_feature_system_status;
pub use smsg_feature_system_status::*;
pub(crate) mod smsg_force_move_root;
pub use smsg_force_move_root::*;
pub(crate) mod smsg_force_move_unroot;
pub use smsg_force_move_unroot::*;
pub(crate) mod smsg_gameobject_query_response;
pub use smsg_gameobject_query_response::*;
pub(crate) mod smsg_gm_messagechat;
pub use smsg_gm_messagechat::*;
pub(crate) mod smsg_gmticket_getticket;
pub use smsg_gmticket_getticket::*;
pub(crate) mod smsg_gossip_message;
pub use smsg_gossip_message::*;
pub(crate) mod smsg_guild_command_result;
pub use smsg_guild_command_result::*;
pub(crate) mod smsg_guild_info;
pub use smsg_guild_info::*;
pub(crate) mod smsg_init_world_states;
pub use smsg_init_world_states::*;
pub(crate) mod smsg_initial_spells;
pub use smsg_initial_spells::*;
pub(crate) mod smsg_initialize_factions;
pub use smsg_initialize_factions::*;
pub(crate) mod smsg_inventory_change_failure;
pub use smsg_inventory_change_failure::*;
pub(crate) mod smsg_item_text_query_response;
pub use smsg_item_text_query_response::*;
pub(crate) mod smsg_levelup_info;
pub use smsg_levelup_info::*;
pub(crate) mod smsg_log_xpgain;
pub use smsg_log_xpgain::*;
pub(crate) mod smsg_login_settimespeed;
pub use smsg_login_settimespeed::*;
pub(crate) mod smsg_login_verify_world;
pub use smsg_login_verify_world::*;
pub(crate) mod smsg_loot_money_notify;
pub use smsg_loot_money_notify::*;
pub(crate) mod smsg_loot_start_roll;
pub use smsg_loot_start_roll::*;
pub(crate) mod smsg_messagechat;
pub use smsg_messagechat::*;
pub(crate) mod smsg_name_query_response;
pub use smsg_name_query_response::*;
pub(crate) mod smsg_new_world;
pub use smsg_new_world::*;
pub(crate) mod smsg_npc_text_update;
pub use smsg_npc_text_update::*;
pub(crate) mod smsg_pet_cast_failed;
pub use smsg_pet_cast_failed::*;
pub(crate) mod smsg_pet_tame_failure;
pub use smsg_pet_tame_failure::*;
pub(crate) mod smsg_played_time;
pub use smsg_played_time::*;
pub(crate) mod smsg_playerbound;
pub use smsg_playerbound::*;
pub(crate) mod smsg_query_time_response;
pub use smsg_query_time_response::*;
pub(crate) mod smsg_quest_force_remove;
pub use smsg_quest_force_remove::*;
pub(crate) mod smsg_questgiver_quest_failed;
pub use smsg_questgiver_quest_failed::*;
pub(crate) mod smsg_questgiver_quest_invalid;
pub use smsg_questgiver_quest_invalid::*;
pub(crate) mod smsg_questgiver_status;
pub use smsg_questgiver_status::*;
pub(crate) mod smsg_raid_instance_info;
pub use smsg_raid_instance_info::*;
pub(crate) mod smsg_sell_item;
pub use smsg_sell_item::*;
pub(crate) mod smsg_server_message;
pub use smsg_server_message::*;
pub(crate) mod smsg_set_faction_standing;
pub use smsg_set_faction_standing::*;
pub(crate) mod smsg_set_proficiency;
pub use smsg_set_proficiency::*;
pub(crate) mod smsg_stable_result;
pub use smsg_stable_result::*;
pub(crate) mod smsg_text_emote;
pub use smsg_text_emote::*;
pub(crate) mod smsg_transfer_aborted;
pub use smsg_transfer_aborted::*;
pub(crate) mod smsg_transfer_pending;
pub use smsg_transfer_pending::*;
pub(crate) mod smsg_trigger_cinematic;
pub use smsg_trigger_cinematic::*;
pub(crate) mod smsg_update_account_data;
pub use smsg_update_account_data::*;
pub(crate) mod smsg_update_account_data_complete;
pub use smsg_update_account_data_complete::*;
pub(crate) mod smsg_update_object;
pub use smsg_update_object::*;
pub(crate) mod smsg_world_state_ui_timer_update;
pub use smsg_world_state_ui_timer_update::*;
pub(crate) mod smsg_zone_under_attack;
pub use smsg_zone_under_attack::*;
pub(crate) mod spell_cast_result;
pub use spell_cast_result::*;
pub(crate) mod spell_cast_target_flags;
pub use spell_cast_target_flags::*;
pub(crate) mod spell_cast_targets;
pub use spell_cast_targets::*;
pub(crate) mod spline_flag;
pub use spline_flag::*;
pub(crate) mod stable_result;
pub use stable_result::*;
pub(crate) mod transfer_abort_reason;
pub use transfer_abort_reason::*;
pub(crate) mod transport_info;
pub use transport_info::*;
pub(crate) mod update_flag;
pub use update_flag::*;
pub(crate) mod update_type;
pub use update_type::*;
pub(crate) mod world_result;
pub use world_result::*;
