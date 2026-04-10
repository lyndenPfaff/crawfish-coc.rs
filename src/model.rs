/* --- IMPORTS --- */

use std::collections::HashMap;
use serde::Deserialize;

/* --- ------- --- */



/* --- STRUCTS --- */

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct WarLeague {
    pub name: String,
    pub id: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct ServiceVersion {
    pub major: i64,
    pub minor: i64,
    pub content: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Label {
    pub name: String,
    pub id: i64,
    pub icon_urls: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct ClanCapitalRaidSeason {
    pub attack_log: Vec<ClanCapitalRaidSeasonAttackLogEntry>,
    pub defense_log: Vec<ClanCapitalRaidSeasonDefenseLogEntry>,
    pub state: String,
    pub start_time: String,
    pub end_time: String,
    pub capital_total_loot: i64,
    pub raids_completed: i64,
    pub total_attacks: i64,
    pub enemy_districts_destroyed: i64,
    pub offensive_rewards: i64,
    pub defensive_rewards: i64,
    pub members: Vec<ClanCapitalRaidSeasonMember>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct ClanCapitalRaidSeasonMember {
    pub tag: String,
    pub name: String,
    pub attacks: i64,
    pub attack_limit: i64,
    pub bonus_attack_limit: i64,
    pub capital_resources_looted: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct ClanCapitalRaidSeasonDefenseLogEntry {
    pub attacker: ClanCapitalRaidSeasonClanInfo,
    pub attack_count: i64,
    pub district_count: i64,
    pub districts_destroyed: i64,
    pub districts: Vec<ClanCapitalRaidSeasonDistrict>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct ClanCapitalRaidSeasonDistrict {
    pub stars: i64,
    pub name: String,
    pub id: i64,
    pub destruction_percent: i64,
    pub attack_count: i64,
    pub total_looted: i64,
    pub attacks: Vec<ClanCapitalRaidSeasonAttack>,
    pub district_hall_level: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct ClanCapitalRaidSeasonAttack {
    pub attacker: ClanCapitalRaidSeasonAttacker,
    pub destruction_percent: i64,
    pub stars: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct ClanCapitalRaidSeasonAttacker {
    pub tag: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct ClanCapitalRaidSeasonClanInfo {
    pub tag: String,
    pub name: String,
    pub level: i64,
    pub badge_urls: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct ClanCapitalRaidSeasonAttackLogEntry {
    pub defender: ClanCapitalRaidSeasonClanInfo,
    pub attack_count: i64,
    pub district_count: i64,
    pub districts_destroyed: i64,
    pub districts: Vec<ClanCapitalRaidSeasonDistrict>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Location {
    pub localized_name: String,
    pub id: i64,
    pub name: String,
    pub is_country: bool,
    pub country_code: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Replay {
    pub replay_data: HashMap<String, String>,
    pub replay_tag: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct ClanCapitalRanking {
    pub clan_points: i64,
    pub clan_capital_points: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct WarStatus {
    pub status_code: i64,
    pub clan_tag: String,
    pub enemy_clan_tag: String,
    /// Enum: [ CLAN_NOT_FOUND, ACCESS_DENIED, NOT_IN_WAR, IN_MATCHMAKING, ENTER_WAR, MATCHED, PREPARATION, WAR, IN_WAR, ENDED ]
    pub war_state: String,
    pub timestamp: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct ClanMember {
    pub league: League,
    pub league_tier: LeagueTier,
    pub builder_base_league: BuilderBaseLeague,
    pub tag: String,
    pub name: String,
    /// Enum: [ NOT_MEMBER, MEMBER, LEADER, ADMIN, COLEADER ]
    pub role: String,
    pub town_hall_level: i64,
    pub exp_level: i64,
    pub clan_rank: i64,
    pub previous_clan_rank: i64,
    pub donations: i64,
    pub donations_received: i64,
    pub trophies: i64,
    pub builder_base_trophies: i64,
    pub player_house: PlayerHouse,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct PlayerHouse {
    pub elements: Vec<PlayerHouseElement>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct PlayerHouseElement {
    pub id: i64,
    /// Enum: [ GROUND, ROOF, FOOT, DECO ]
    pub r#type: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct BuilderBaseLeague {
    pub name: String,
    pub id: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct LeagueTier {
    pub name: String,
    pub id: i64,
    pub icon_urls: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct League {
    pub name: String,
    pub id: i64,
    pub icon_urls: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct LeagueSeasonResult {
    pub league_season_id: i128,
    pub league_trophies: i64,
    pub league_tier_id: i64,
    pub placement: i64,
    pub attack_wins: i64,
    pub attack_losses: i64,
    pub attack_stars: i64,
    pub defense_wins: i64,
    pub defense_losses: i64,
    pub defense_stars: i64,
    pub max_battles: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct LeagueSeason {
    pub id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct LeagueGroup {
    pub members: Vec<LeagueGroupMember>,
    pub attack_logs: Vec<LeagueBattleLogEntry>,
    pub defense_logs: Vec<LeagueBattleLogEntry>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct LeagueBattleLogEntry {
    pub opponent_player_tag: String,
    pub opponent_name: String,
    pub stars: i64,
    pub destruction_percentage: i64,
    pub trophies: i64,
    pub creation_time: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct LeagueGroupMember {
    pub player_tag: String,
    pub player_name: String,
    pub clan_tag: String,
    pub clan_name: String,
    pub league_trophies: i64,
    pub attack_win_count: i64,
    pub attack_lose_count: i64,
    pub defense_win_count: i64,
    pub defense_lose_count: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct VerifyTokenRequest {
    pub token: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct VerifyTokenResponce {
    pub tag: String,
    pub token: String,
    pub status: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Clan {
    pub member_list: Vec<ClanMember>,
    pub war_league: WarLeague,
    pub capital_league: CapitalLeague,
    pub tag: String,
    pub war_ties: i64,
    pub war_losses: i64,
    pub clan_points: i64,
    pub chat_language: Language,
    pub is_family_friendly: bool,
    pub is_war_log_public: bool,
    /// Enum: [ UNKNOWN, ALWAYS, MORE_THAN_ONCE_PER_WEEK, ONCE_PER_WEEK, LESS_THAN_ONCE_PER_WEEK, NEVER, ANY ]
    pub war_frequency: String,
    pub war_win_streak: i64,
    pub war_wins: i64,
    pub clan_level: i64,
    pub clan_builder_base_points: i64,
    pub clan_capital_points: i64,
    pub required_trophies: i64,
    pub required_builder_base_trophies: i64,
    pub required_townhall_level: i64,
    pub labels: Vec<Label>,
    pub name: String,
    pub location: Location,
    /// Enum: [ OPEN, INVITE_ONLY, CLOSED ]
    pub r#type: String,
    pub members: i64,
    pub description: String,
    pub clan_capital: ClanCapital,
    pub badge_urls: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct ClanCapital {
    pub capital_hall_level: i64,
    pub districts: Vec<ClanDistrictData>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct ClanDistrictData {
    pub name: String,
    pub id: i64,
    pub district_hall_level: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Language {
    pub name: String,
    pub id: i64,
    pub language_code: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct CapitalLeague {
    pub name: String,
    pub id: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct ClanWar {
    pub clan: WarClan,
    pub team_size: i64,
    pub attacks_per_member: i64,
    /// Enum [ NONE, HARD_MODE ]
    pub battle_modifier: String,
    pub opponent: WarClan,
    pub start_time: String,
    /// Enum [ CLAN_NOT_FOUND, ACCESS_DENIED, NOT_IN_WAR, IN_MATCHMAKING, ENTER_WAR, MATCHED, PREPARATION, WAR, IN_WAR, ENDED ]
    pub state: String,
    pub end_time: String,
    pub preparation_start_time: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct WarClan {
    pub destruction_percentage: f64,
    pub tag: String,
    pub name: String,
    pub badge_urls: HashMap<String, String>,
    pub clan_level: i64,
    pub attacks: i64,
    pub stars: i64,
    pub exp_earned: i64,
    pub members: Vec<ClanWarMember>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct ClanWarMember {
    pub tag: String,
    pub name: String,
    pub map_position: i64,
    pub townhall_level: i64,
    pub opponent_attacks: i64,
    pub best_opponent_attack: ClanWarAttack,
    pub attacks: Vec<ClanWarAttack>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct ClanWarAttack {
    pub order: i64,
    pub attacker_tag: String,
    pub defender_tag: String,
    pub stars: i64,
    pub destruction_percentage: i64,
    pub duration: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct BattleLogEntry {
    /// Enum: [ HOME_VILLAGE, RANKED, LEGEND ]
    pub battle_type: String,
    pub attack: bool,
    pub army_share_code: String,
    pub opponent_player_tag: String,
    pub stars: i64,
    pub destruction_percentage: i64,
    pub looted_resources: Vec<Resource>,
    pub extra_looted_resources: Vec<Resource>,
    pub available_loot: Vec<Resource>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Resource {
    pub name: String,
    pub amount: i128,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct ClanBuilderBaseRanking {
    pub clan_points: i64,
    pub clan_builder_base_points: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct PlayerBuilderBaseRanking {
    pub clan: PlayerRankingClan,
    pub builder_base_league: BuilderBaseLeague,
    pub tag: String,
    pub name: String,
    pub exp_level: i64,
    pub rank: i64,
    pub previous_rank: i64,
    pub builder_base_trophies: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct PlayerRankingClan {
    pub tag: String,
    pub name: String,
    pub badge_urls: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct PlayerRanking {
    pub clan: PlayerRankingClan,
    pub league: League,
    pub league_tier: LeagueTier,
    pub attack_wins: i64,
    pub defense_wins: i64,
    pub tag: String,
    pub name: String,
    pub exp_level: i64,
    pub rank: i64,
    pub previous_rank: i64,
    pub trophies: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct ClanWarLogEntry {
    pub clan: WarClan,
    pub team_size: i64,
    pub attacks_per_member: i64,
    /// Enum: [ NONE, HARD_MODE ]
    pub battle_modifier: String,
    pub opponent: WarClan,
    pub end_time: String,
    /// Enum: [ LOSE, WIN, TIE ]
    pub result: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct GoldPassSeason {
    pub start_time: String,
    pub end_time: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct DeepLinkCreationRequest {
    pub player_tags: Vec<String>,
    pub clan_tag: String,
    pub opponent_clan_tag: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct DeepLinkCreationResponse{
    pub link: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct ClanWarLeagueGroup {
    pub tag: String,
    /// Enum: [ GROUP_NOT_FOUND, NOT_IN_WAR, PREPARATION, WAR, ENDED ]
    pub state: String,
    pub season: String,
    pub clans: Vec<ClanWarLeagueClan>,
    pub rounds: Vec<ClanWarLeagueRound>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct ClanWarLeagueRound {
    pub war_tags: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct ClanWarLeagueClan {
    pub tag: String,
    pub clan_level: i64,
    pub name: String,
    pub members: Vec<ClanWarLeagueClanMember>,
    pub badge_urls: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct ClanWarLeagueClanMember {
    pub tag: String,
    pub town_hall_level: i64,
    pub name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct ClanRanking {
    pub clan_points: i64,
    pub clan_level: i64,
    pub location: Location,
    pub members: i64,
    pub tag: String,
    pub name: String,
    pub rank: i64,
    pub previous_rank: i64,
    pub badge_urls: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Player {
    pub clan: PlayerClan,
    pub league: League,
    pub league_tier: LeagueTier,
    pub builder_base_league: BuilderBaseLeague,
    /// Enum: [ NOT_MEMBER, MEMBER, LEADER, ADMIN, COLEADER ]
    pub role: String,
    /// Enum: [ OUT, IN ]
    pub war_preference: String,
    pub attack_wins: i64,
    pub defense_wins: i64,
    pub town_hall_level: i64,
    pub town_hall_weapon_level: i64,
    pub legend_statistics: PlayerLegendStatistics,
    pub troops: Vec<PlayerItemLevel>,
    pub heroes: Vec<PlayerItemLevel>,
    pub hero_equipment: Vec<PlayerItemLevel>,
    pub spells: Vec<PlayerItemLevel>,
    pub labels: Vec<Label>,
    pub tag: String,
    pub name: String,
    pub exp_level: i64,
    pub trophies: i64,
    pub best_trophies: i64,
    pub donations: i64,
    pub donations_received: i64,
    pub builder_hall_level: i64,
    pub builder_base_trophies: i64,
    pub best_builder_base_trophies: i64,
    pub war_stars: i64,
    pub achievements: Vec<PlayerAchievementProgress>,
    pub clan_capital_contributions: i64,
    pub player_house: PlayerHouse,
    pub current_league_group_tag: String,
    pub current_league_season_id: i128,
    pub previous_league_group_tag: String,
    pub previous_league_season_id: i128,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct PlayerAchievementProgress {
    pub stars: i64,
    pub value: i64,
    pub name: String,
    pub target: i64,
    pub info: String,
    pub completion_info: String,
    /// Enum: [ HOME_VILLAGE, BUILDER_BASE, CLAN_CAPITAL ]
    pub village: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct PlayerItemLevel {
    pub level: i64,
    pub name: String,
    pub max_level: i64,
    /// Enum: [ HOME_VILLAGE, BUILDER_BASE, CLAN_CAPITAL ]
    pub village: String,
    pub super_troop_is_active: bool,
    pub equipment: Vec<PlayerItemLevel>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct PlayerLegendStatistics {
    pub best_builder_base_season: LegendLeagueTournamentSeasonResult,
    pub current_season: LegendLeagueTournamentSeasonResult,
    pub legend_trophies: i64,
    pub best_season: LegendLeagueTournamentSeasonResult,
    pub previous_season: LegendLeagueTournamentSeasonResult,
    pub previous_builder_base_season: LegendLeagueTournamentSeasonResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct LegendLeagueTournamentSeasonResult {
    pub trophies: i64,
    pub id: String,
    pub rank: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct PlayerClan {
    pub tag: String,
    pub clan_level: i64,
    pub name: String,
    pub badge_urls: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct ClientError {
    pub reason: String,
    pub message: String,
    pub r#type: String,
}

/* --- ------- --- */
