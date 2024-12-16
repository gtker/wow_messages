pub use crate::manual::shared::*;
pub use wow_world_base::shared::*;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod addon_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod addon_info_vanilla_tbc;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod auction_enchantment_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod auction_sort_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod bank_tab_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod battleground_player_position_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod channel_member_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cooldown_spell_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod damage_info_vanilla_tbc;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod dispelled_spell_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod faction_initializer_vanilla_tbc;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod gm_survey_question_vanilla_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod gossip_item_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod guild_bank_rights_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod guild_bank_socket_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod guild_bank_tab_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod guild_log_event_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod guild_rights_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod initial_spell_vanilla_tbc;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod list_inventory_item_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod loot_item_vanilla_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod mail_item_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod mail_list_item_enchant_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod money_log_item_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod npc_text_update_emote_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod pet_spell_cooldown_vanilla_tbc;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod petition_showlist_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod petition_signature_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod quest_details_emote_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod quest_item_vanilla_tbc;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod quest_item_requirement_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod quest_item_reward_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod quest_objective_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod raid_target_update_vanilla_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod received_mail_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod spell_cooldown_status_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod spell_log_miss_vanilla_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod spell_miss_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod spell_steal_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod stabled_pet_vanilla_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod trade_slot_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod transport_info_vanilla_tbc;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod world_state_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_accept_trade_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_activatetaxi_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod cmsg_activatetaxiexpress_vanilla_tbc;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod cmsg_add_friend_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_add_ignore_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_areatrigger_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_area_spirit_healer_query_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_area_spirit_healer_queue_vanilla_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod cmsg_arena_team_accept_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod cmsg_arena_team_decline_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod cmsg_arena_team_disband_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod cmsg_arena_team_invite_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod cmsg_arena_team_leader_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod cmsg_arena_team_leave_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod cmsg_arena_team_remove_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod cmsg_arena_team_roster_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_attackstop_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_attackswing_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_auction_list_bidder_items_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_auction_list_owner_items_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_auction_place_bid_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_auction_remove_item_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod cmsg_auction_sell_item_vanilla_tbc;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod cmsg_auth_session_vanilla_tbc;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_autobank_item_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_autoequip_item_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_autoequip_item_slot_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_autostore_bag_item_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_autostore_bank_item_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_autostore_loot_item_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_banker_activate_vanilla_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod cmsg_battlefield_port_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_battlefield_status_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_battlemaster_hello_vanilla_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod cmsg_battlemaster_join_arena_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_begin_trade_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_binder_activate_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_bootme_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_bug_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_busy_trade_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_buyback_item_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_buy_bank_slot_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod cmsg_buy_item_vanilla_tbc;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod cmsg_buy_item_in_slot_vanilla_tbc;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_buy_stable_slot_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_cancel_aura_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_cancel_auto_repeat_spell_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_cancel_cast_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_cancel_channelling_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_cancel_growth_aura_vanilla_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod cmsg_cancel_mount_aura_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod cmsg_cancel_temp_enchantment_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_cancel_trade_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_channel_announcements_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_channel_ban_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_channel_invite_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_channel_kick_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_channel_list_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_channel_moderate_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_channel_moderator_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_channel_mute_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_channel_owner_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_channel_password_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_channel_set_owner_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_channel_unban_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_channel_unmoderator_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_channel_unmute_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_char_delete_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_char_enum_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_char_rename_vanilla_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod cmsg_chat_ignored_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_clear_trade_item_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_complete_cinematic_vanilla_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod cmsg_contact_list_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_creature_query_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_dblookup_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_del_friend_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_del_ignore_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_destroyitem_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_duel_accepted_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_duel_cancelled_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_far_sight_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_gameobject_query_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_gameobj_use_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_get_mail_list_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_gmsurvey_submit_vanilla_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod cmsg_gmticketsystem_toggle_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_gmticket_deleteticket_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_gmticket_getticket_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_gmticket_systemstatus_vanilla_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod cmsg_gmticket_updatetext_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_gossip_hello_vanilla_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod cmsg_gossip_select_option_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod cmsg_group_accept_vanilla_tbc;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_group_assistant_leader_vanilla_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod cmsg_group_cancel_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_group_change_sub_group_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_group_decline_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_group_disband_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod cmsg_group_invite_vanilla_tbc;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_group_raid_convert_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_group_set_leader_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_group_swap_sub_group_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_group_uninvite_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod cmsg_group_uninvite_guid_vanilla_tbc;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_guild_accept_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_guild_add_rank_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_guild_create_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_guild_decline_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_guild_del_rank_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_guild_demote_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_guild_disband_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_guild_info_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_guild_info_text_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_guild_invite_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_guild_leader_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_guild_leave_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_guild_motd_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_guild_promote_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_guild_query_vanilla_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod cmsg_guild_rank_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_guild_remove_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_guild_roster_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_guild_set_officer_note_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_guild_set_public_note_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_ignore_trade_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_initiate_trade_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_inspect_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_item_name_query_vanilla_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod cmsg_item_query_single_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod cmsg_item_text_query_vanilla_tbc;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod cmsg_join_channel_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod cmsg_leave_channel_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_list_inventory_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_logout_cancel_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_logout_request_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_loot_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_loot_master_give_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_loot_money_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_loot_release_vanilla_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod cmsg_loot_roll_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod cmsg_mail_create_text_item_vanilla_tbc;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod cmsg_mail_delete_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_mail_mark_as_read_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_mail_return_to_sender_vanilla_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod cmsg_mail_take_item_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_mail_take_money_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod cmsg_meetingstone_info_vanilla_tbc;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_mountspecial_anim_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_move_set_raw_position_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod cmsg_move_time_skipped_vanilla_tbc;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_name_query_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_next_cinematic_camera_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_npc_text_query_vanilla_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod cmsg_offer_petition_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_open_item_vanilla_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod cmsg_page_text_query_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod cmsg_petition_buy_vanilla_tbc;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_petition_query_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_petition_showlist_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_petition_show_signatures_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_petition_sign_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_pet_abandon_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_pet_action_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_pet_cancel_aura_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_pet_name_query_vanilla_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod cmsg_pet_rename_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_pet_set_action_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_pet_spell_autocast_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_pet_stop_attack_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_pet_unlearn_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_ping_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod cmsg_played_time_vanilla_tbc;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_player_login_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_player_logout_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_pushquesttoparty_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_query_time_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_questgiver_accept_quest_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_questgiver_cancel_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_questgiver_choose_reward_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_questgiver_complete_quest_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_questgiver_hello_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod cmsg_questgiver_query_quest_vanilla_tbc;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_questgiver_quest_autolaunch_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_questgiver_request_reward_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_questgiver_status_query_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_questlog_remove_quest_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_questlog_swap_quest_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_quest_confirm_accept_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_quest_query_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_read_item_vanilla_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod cmsg_realm_split_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_reclaim_corpse_vanilla_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod cmsg_repair_item_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_repop_request_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_request_account_data_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_request_party_member_stats_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_request_pet_info_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_request_raid_info_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_reset_instances_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_resurrect_response_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_self_res_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_sell_item_vanilla_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod cmsg_send_mail_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_setsheathed_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_set_actionbar_toggles_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_set_action_button_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_set_active_mover_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_set_ammo_vanilla_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod cmsg_set_contact_notes_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod cmsg_set_lfg_comment_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_set_selection_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod cmsg_set_target_obsolete_vanilla_tbc;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod cmsg_set_taxi_benchmark_mode_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod cmsg_set_title_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_set_trade_gold_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_set_trade_item_vanilla_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod cmsg_socket_gems_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_spirit_healer_activate_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod cmsg_split_item_vanilla_tbc;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_stable_pet_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_stable_swap_pet_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_standstatechange_vanilla_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod cmsg_summon_response_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_swap_item_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_taxinode_status_query_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_taxiqueryavailablenodes_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_teleport_to_unit_vanilla_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod cmsg_time_sync_resp_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_toggle_cloak_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_toggle_helm_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_toggle_pvp_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_trainer_buy_spell_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_trainer_list_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_turn_in_petition_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_tutorial_clear_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_tutorial_flag_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_tutorial_reset_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_unaccept_trade_vanilla_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod cmsg_unlearn_talents_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_unstable_pet_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod cmsg_update_account_data_vanilla_tbc;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod cmsg_voice_session_enable_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_warden_data_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_who_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_whois_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod cmsg_wrap_item_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod msg_auction_hello_client_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod msg_auction_hello_server_vanilla_tbc;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod msg_battleground_player_positions_client_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod msg_battleground_player_positions_server_vanilla_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod msg_channel_start_server_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod msg_channel_update_server_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod msg_corpse_query_client_vanilla_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod msg_inspect_arena_teams_client_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod msg_inspect_arena_teams_server_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod msg_inspect_honor_stats_client_vanilla_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod msg_inspect_honor_stats_server_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod msg_list_stabled_pets_client_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod msg_list_stabled_pets_server_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod msg_minimap_ping_client_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod msg_minimap_ping_server_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod msg_move_teleport_ack_client_vanilla_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod msg_move_teleport_cheat_server_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod msg_move_time_skipped_server_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod msg_move_worldport_ack_vanilla_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod msg_party_assignment_client_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod msg_petition_decline_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod msg_petition_rename_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod msg_pvp_log_data_client_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod msg_query_next_mail_time_client_vanilla_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod msg_query_next_mail_time_server_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod msg_quest_push_result_vanilla_tbc;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod msg_raid_ready_check_confirm_client_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod msg_raid_ready_check_confirm_server_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod msg_raid_ready_check_client_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod msg_raid_ready_check_server_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod msg_raid_target_update_client_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod msg_raid_target_update_server_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod msg_random_roll_client_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod msg_random_roll_server_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod msg_save_guild_emblem_client_vanilla_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod msg_set_dungeon_difficulty_client_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod msg_set_dungeon_difficulty_server_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod msg_tabardvendor_activate_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod msg_talent_wipe_confirm_client_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod msg_talent_wipe_confirm_server_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod smsg_account_data_times_vanilla_tbc;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_activatetaxireply_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_ai_reaction_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_area_spirit_healer_time_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_area_trigger_message_vanilla_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod smsg_arena_error_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod smsg_arena_team_command_result_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod smsg_arena_team_event_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod smsg_arena_team_invite_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod smsg_arena_team_query_response_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod smsg_arena_team_stats_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_attackstart_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_attackstop_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_attackswing_badfacing_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_attackswing_cant_attack_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_attackswing_deadtarget_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_attackswing_notinrange_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod smsg_attackswing_notstanding_vanilla_tbc;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod smsg_auction_bidder_notification_vanilla_tbc;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod smsg_auction_owner_notification_vanilla_tbc;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_auction_removed_notification_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod smsg_auth_challenge_vanilla_tbc;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_battleground_player_joined_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_battleground_player_left_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod smsg_binder_confirm_vanilla_tbc;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_buy_bank_slot_result_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_buy_failed_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_buy_item_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod smsg_cancel_auto_repeat_vanilla_tbc;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_cancel_combat_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_channel_list_vanilla_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod smsg_channel_notify_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod smsg_chat_player_ambiguous_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_chat_player_not_found_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_chat_wrong_faction_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_clear_cooldown_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_client_control_update_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_cooldown_event_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_corpse_reclaim_delay_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod smsg_destroy_object_vanilla_tbc;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod smsg_dismount_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_dispel_failed_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_duel_complete_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_duel_countdown_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_duel_inbounds_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_duel_outofbounds_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_duel_requested_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_duel_winner_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_durability_damage_death_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod smsg_enchantmentlog_vanilla_tbc;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_environmental_damage_log_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_feign_death_resisted_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_fish_escaped_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_fish_not_hooked_vanilla_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod smsg_forced_death_update_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod smsg_force_flight_back_speed_change_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod smsg_force_flight_speed_change_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_force_run_back_speed_change_vanilla_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod smsg_force_run_speed_change_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_force_swim_back_speed_change_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_force_swim_speed_change_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_force_turn_rate_change_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_force_walk_speed_change_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod smsg_friend_status_vanilla_tbc;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_gameobject_custom_anim_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_gameobject_despawn_anim_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_gameobject_pagetext_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod smsg_gameobject_spawn_anim_vanilla_tbc;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_gmticket_create_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_gmticket_deleteticket_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_gmticket_systemstatus_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_gmticket_updatetext_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_gm_ticket_status_update_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_gossip_complete_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_gossip_poi_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_group_decline_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_group_destroyed_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod smsg_group_invite_vanilla_tbc;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_group_set_leader_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_group_uninvite_vanilla_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod smsg_guild_decline_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod smsg_guild_event_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod smsg_guild_info_vanilla_tbc;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_guild_invite_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_guild_query_response_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod smsg_initialize_factions_vanilla_tbc;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod smsg_initial_spells_vanilla_tbc;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod smsg_instance_difficulty_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_instance_save_created_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_invalidate_player_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_item_cooldown_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_item_enchant_time_update_vanilla_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod smsg_item_name_query_response_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod smsg_item_push_result_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod smsg_item_text_query_response_vanilla_tbc;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_item_time_update_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod smsg_levelup_info_vanilla_tbc;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod smsg_lfg_disabled_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod smsg_list_inventory_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod smsg_login_settimespeed_vanilla_tbc;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_logout_cancel_ack_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_logout_complete_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_logout_response_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod smsg_log_xpgain_vanilla_tbc;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_loot_all_passed_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_loot_clear_money_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_loot_master_list_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod smsg_loot_money_notify_vanilla_tbc;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_loot_release_response_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_loot_removed_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_loot_response_vanilla_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod smsg_loot_roll_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod smsg_loot_roll_won_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod smsg_loot_start_roll_vanilla_tbc;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod smsg_monster_move_vanilla_tbc;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod smsg_monster_move_transport_vanilla_tbc;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod smsg_motd_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_mountresult_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_mountspecial_anim_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_move_feather_fall_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_move_knock_back_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_move_land_walk_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_move_normal_fall_vanilla_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod smsg_move_set_can_fly_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_move_set_hover_vanilla_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod smsg_move_unset_can_fly_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_move_unset_hover_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_move_water_walk_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_new_taxi_path_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_notification_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_page_text_query_response_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_partykilllog_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_pause_mirror_timer_vanilla_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod smsg_petition_showlist_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_petition_show_signatures_vanilla_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod smsg_petition_sign_results_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_pet_action_feedback_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_pet_action_sound_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_pet_broken_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_pet_dismiss_sound_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_pet_mode_vanilla_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod smsg_pet_name_invalid_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod smsg_pet_name_query_response_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod smsg_pet_spells_vanilla_tbc;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod smsg_pet_tame_failure_vanilla_tbc;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_pet_unlearn_confirm_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod smsg_played_time_vanilla_tbc;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_player_skinned_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_play_music_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_play_object_sound_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_play_sound_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_play_spell_impact_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_play_spell_visual_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_pong_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_procresist_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_pvp_credit_vanilla_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod smsg_query_time_response_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod smsg_questgiver_quest_list_vanilla_tbc;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_questlog_full_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_questupdate_add_item_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_questupdate_add_kill_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_questupdate_complete_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_questupdate_failed_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_questupdate_failedtimer_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_quest_confirm_accept_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_raid_group_only_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_read_item_failed_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_read_item_ok_vanilla_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod smsg_realm_split_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_received_mail_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod smsg_removed_spell_vanilla_tbc;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod smsg_resurrect_failed_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_resurrect_request_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod smsg_sell_item_vanilla_tbc;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod smsg_server_message_vanilla_tbc;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_set_flat_spell_modifier_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_set_pct_spell_modifier_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod smsg_set_rest_start_vanilla_tbc;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_showtaxinodes_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_show_bank_vanilla_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod smsg_spelldispellog_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod smsg_spellenergizelog_vanilla_tbc;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod smsg_spellinstakilllog_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_spelllogmiss_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_spellordamage_immune_vanilla_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod smsg_spellsteallog_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod smsg_spell_cooldown_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_spell_delayed_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_spell_failed_other_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_spell_update_chain_targets_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_spirit_healer_confirm_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_spline_move_feather_fall_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_spline_move_land_walk_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_spline_move_normal_fall_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_spline_move_root_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_spline_move_set_hover_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_spline_move_set_run_mode_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_spline_move_set_walk_mode_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_spline_move_start_swim_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_spline_move_stop_swim_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_spline_move_unroot_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_spline_move_unset_hover_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_spline_move_water_walk_vanilla_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod smsg_spline_set_flight_back_speed_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod smsg_spline_set_flight_speed_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_spline_set_run_back_speed_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_spline_set_run_speed_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_spline_set_swim_back_speed_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_spline_set_swim_speed_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_spline_set_turn_rate_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_spline_set_walk_speed_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod smsg_stable_result_vanilla_tbc;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_standstate_update_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_start_mirror_timer_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_stop_mirror_timer_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod smsg_superceded_spell_vanilla_tbc;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_taxinode_status_vanilla_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod smsg_time_sync_req_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod smsg_title_earned_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod smsg_trade_status_extended_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_trainer_buy_failed_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_trainer_buy_succeeded_vanilla_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod smsg_turn_in_petition_results_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_tutorial_flags_vanilla_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod smsg_update_account_data_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod smsg_update_aura_duration_vanilla_tbc;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod smsg_update_combo_points_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_update_instance_ownership_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_update_world_state_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_warden_data_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod smsg_whois_vanilla_tbc_wrath;
