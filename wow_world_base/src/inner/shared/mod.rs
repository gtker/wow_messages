pub use crate::manual::shared::*;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub use crate::manual::shared::datetime_vanilla_tbc_wrath::*;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub use crate::manual::shared::gold_vanilla_tbc_wrath::*;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub use crate::manual::shared::guid_vanilla_tbc_wrath::*;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub use crate::manual::shared::level_vanilla_tbc_wrath::*;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub use crate::manual::shared::player_gender_vanilla_tbc_wrath::*;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub use crate::manual::shared::player_race_tbc_wrath::*;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc"))]
pub mod account_data_type_vanilla_tbc;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod activate_taxi_reply_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod ai_reaction_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "tbc", feature = "wrath"))]
pub mod arena_faction_tbc_wrath;
#[cfg(any(feature = "shared", feature = "tbc", feature = "wrath"))]
pub mod arena_team_command_tbc_wrath;
#[cfg(any(feature = "shared", feature = "tbc", feature = "wrath"))]
pub mod arena_team_command_error_tbc_wrath;
#[cfg(any(feature = "shared", feature = "tbc", feature = "wrath"))]
pub mod arena_team_event_tbc_wrath;
#[cfg(any(feature = "shared", feature = "tbc", feature = "wrath"))]
pub mod arena_team_role_tbc_wrath;
#[cfg(any(feature = "shared", feature = "tbc", feature = "wrath"))]
pub mod arena_type_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod auction_command_action_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod auction_command_result_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod auction_command_result_two_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod auction_house_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "tbc", feature = "wrath"))]
pub mod bank_swap_source_tbc_wrath;
#[cfg(any(feature = "shared", feature = "tbc", feature = "wrath"))]
pub mod bank_swap_store_mode_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod battlefield_port_action_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod bonding_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod buy_bank_slot_result_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod buy_result_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod buyback_slot_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "tbc", feature = "wrath"))]
pub mod charter_type_tbc_wrath;
#[cfg(any(feature = "shared", feature = "tbc", feature = "wrath"))]
pub mod chat_notify_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc"))]
pub mod class_vanilla_tbc;
#[cfg(any(feature = "shared", feature = "tbc", feature = "wrath"))]
pub mod commentator_enable_option_tbc_wrath;
#[cfg(any(feature = "shared", feature = "tbc", feature = "wrath"))]
pub mod complain_result_window_tbc_wrath;
#[cfg(any(feature = "shared", feature = "tbc", feature = "wrath"))]
pub mod complaint_status_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod corpse_query_result_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "tbc", feature = "wrath"))]
pub mod declined_names_tbc_wrath;
#[cfg(any(feature = "shared", feature = "tbc", feature = "wrath"))]
pub mod declined_pet_name_included_tbc_wrath;
#[cfg(any(feature = "shared", feature = "tbc", feature = "wrath"))]
pub mod dispel_method_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod duel_winner_reason_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "tbc", feature = "wrath"))]
pub mod dungeon_difficulty_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod environmental_damage_type_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod experience_award_type_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod far_sight_operation_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc"))]
pub mod friend_result_vanilla_tbc;
#[cfg(any(feature = "shared", feature = "tbc", feature = "wrath"))]
pub mod friend_status_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod gender_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod gm_ticket_escalation_status_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod gm_ticket_queue_status_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod gm_ticket_response_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod gm_ticket_status_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod gm_ticket_status_response_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod group_loot_setting_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc"))]
pub mod group_type_vanilla_tbc;
#[cfg(any(feature = "shared", feature = "tbc", feature = "wrath"))]
pub mod guild_bank_tab_result_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc"))]
pub mod guild_command_vanilla_tbc;
#[cfg(any(feature = "shared", feature = "tbc", feature = "wrath"))]
pub mod guild_emblem_result_tbc_wrath;
#[cfg(any(feature = "shared", feature = "tbc", feature = "wrath"))]
pub mod guild_event_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod guild_member_status_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc"))]
pub mod hit_info_vanilla_tbc;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod instance_reset_failed_reason_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod inventory_type_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc"))]
pub mod item_class_vanilla_tbc;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc"))]
pub mod item_quality_vanilla_tbc;
#[cfg(any(feature = "shared", feature = "tbc", feature = "wrath"))]
pub mod join_arena_type_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla"))]
pub mod language_vanilla_vanilla;
#[cfg(any(feature = "shared", feature = "tbc", feature = "wrath"))]
pub mod language_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod log_format_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod logout_result_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod logout_speed_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod loot_method_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod loot_method_error_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod loot_slot_type_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod mail_action_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod mail_message_type_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc"))]
pub mod mail_result_vanilla_tbc;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc"))]
pub mod mail_result_two_vanilla_tbc;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod mail_type_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc"))]
pub mod meeting_stone_status_vanilla_tbc;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod monster_move_type_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod mount_result_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod new_item_chat_alert_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod new_item_creation_type_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod new_item_source_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "tbc", feature = "wrath"))]
pub mod page_text_material_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod party_operation_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "tbc", feature = "wrath"))]
pub mod party_result_tbc_wrath;
#[cfg(any(feature = "shared", feature = "tbc", feature = "wrath"))]
pub mod party_role_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod pet_command_state_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod pet_enabled_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod pet_feedback_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "tbc", feature = "wrath"))]
pub mod pet_name_invalid_reason_tbc_wrath;
#[cfg(any(feature = "shared", feature = "tbc", feature = "wrath"))]
pub mod pet_query_disabled_names_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod pet_react_state_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod pet_talk_reason_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc"))]
pub mod pet_tame_failure_reason_vanilla_tbc;
#[cfg(any(feature = "shared", feature = "tbc", feature = "wrath"))]
pub mod petition_result_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc"))]
pub mod player_chat_tag_vanilla_tbc;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc"))]
pub mod power_vanilla_tbc;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod pvp_rank_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod quest_completable_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc"))]
pub mod quest_party_message_vanilla_tbc;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod raid_group_error_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc"))]
pub mod raid_instance_message_vanilla_tbc;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod raid_target_index_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod raid_target_update_type_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "tbc", feature = "wrath"))]
pub mod realm_split_state_tbc_wrath;
#[cfg(any(feature = "shared", feature = "tbc", feature = "wrath"))]
pub mod refer_a_friend_error_tbc_wrath;
#[cfg(any(feature = "shared", feature = "tbc", feature = "wrath"))]
pub mod roll_vote_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc"))]
pub mod sell_item_result_vanilla_tbc;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc"))]
pub mod server_message_type_vanilla_tbc;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod sheath_state_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod sheathe_type_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod size_class_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod skill_category_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "tbc", feature = "wrath"))]
pub mod spam_type_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod spell_miss_info_vanilla_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod spell_school_vanilla_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "tbc", feature = "wrath"))]
pub mod spell_steal_action_tbc_wrath;
#[cfg(any(feature = "shared", feature = "tbc", feature = "wrath"))]
pub mod spell_trigger_type_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc"))]
pub mod stable_result_vanilla_tbc;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod status_id_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod timer_type_vanilla_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "tbc", feature = "wrath"))]
pub mod title_earn_status_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod trade_status_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod trainer_spell_state_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod training_failure_reason_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod unit_stand_state_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc"))]
pub mod update_type_vanilla_tbc;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod weather_change_type_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc"))]
pub mod allowed_class_vanilla_tbc;
#[cfg(any(feature = "shared", feature = "tbc", feature = "wrath"))]
pub mod bag_family_tbc_wrath;
#[cfg(any(feature = "shared", feature = "tbc", feature = "wrath"))]
pub mod billing_plan_flags_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc"))]
pub mod cast_flags_vanilla_tbc;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod channel_flags_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod channel_member_flags_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc"))]
pub mod faction_flag_vanilla_tbc;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod group_member_online_status_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "tbc", feature = "wrath"))]
pub mod relation_type_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc"))]
pub mod spline_flag_vanilla_tbc;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc"))]
pub mod update_flag_vanilla_tbc;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod item_damage_type_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "tbc", feature = "wrath"))]
pub mod item_socket_tbc_wrath;
#[cfg(any(feature = "shared", feature = "tbc", feature = "wrath"))]
pub mod item_spells_tbc_wrath;
#[cfg(any(feature = "shared", feature = "tbc", feature = "wrath"))]
pub mod item_stat_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod vector2d_vanilla_tbc_wrath;
#[cfg(any(feature = "shared", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod vector3d_vanilla_tbc_wrath;
