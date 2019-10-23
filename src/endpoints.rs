
// This file is automatically generated.
// Do not directly edit.
// Generated on 2019-10-23T01:41:03.165Z

// http://www.mingweisamuel.com/riotapi-schema/tool/
// Version 0c74167e0eaaeb6de1c7e8219fecaabcf8386d1f

mod dto;
pub use dto::*;

use std::future::Future;
use std::vec::Vec;

use url::form_urlencoded::Serializer;

use crate::Result;
use crate::consts::Region;
use crate::riot_api::RiotApi;

impl RiotApi {
    /// Handle for ChampionMasteryV4 endpoints. This method is automatically generated.
    /// # Official API Reference
    /// <a href="https://developer.riotgames.com/api-methods/#champion-mastery-v4">https://developer.riotgames.com/api-methods/#champion-mastery-v4</a>
    #[inline]
    pub fn champion_mastery_v4(&self) -> ChampionMasteryV4 {
        ChampionMasteryV4 { base: self }
    }
    /// Handle for ChampionV3 endpoints. This method is automatically generated.
    /// # Official API Reference
    /// <a href="https://developer.riotgames.com/api-methods/#champion-v3">https://developer.riotgames.com/api-methods/#champion-v3</a>
    #[inline]
    pub fn champion_v3(&self) -> ChampionV3 {
        ChampionV3 { base: self }
    }
    /// Handle for LeagueExpV4 endpoints. This method is automatically generated.
    /// # Official API Reference
    /// <a href="https://developer.riotgames.com/api-methods/#league-exp-v4">https://developer.riotgames.com/api-methods/#league-exp-v4</a>
    #[inline]
    pub fn league_exp_v4(&self) -> LeagueExpV4 {
        LeagueExpV4 { base: self }
    }
    /// Handle for LeagueV4 endpoints. This method is automatically generated.
    /// # Official API Reference
    /// <a href="https://developer.riotgames.com/api-methods/#league-v4">https://developer.riotgames.com/api-methods/#league-v4</a>
    #[inline]
    pub fn league_v4(&self) -> LeagueV4 {
        LeagueV4 { base: self }
    }
    /// Handle for LolStatusV3 endpoints. This method is automatically generated.
    /// # Official API Reference
    /// <a href="https://developer.riotgames.com/api-methods/#lol-status-v3">https://developer.riotgames.com/api-methods/#lol-status-v3</a>
    #[inline]
    pub fn lol_status_v3(&self) -> LolStatusV3 {
        LolStatusV3 { base: self }
    }
    /// Handle for MatchV4 endpoints. This method is automatically generated.
    /// # Official API Reference
    /// <a href="https://developer.riotgames.com/api-methods/#match-v4">https://developer.riotgames.com/api-methods/#match-v4</a>
    #[inline]
    pub fn match_v4(&self) -> MatchV4 {
        MatchV4 { base: self }
    }
    /// Handle for SpectatorV4 endpoints. This method is automatically generated.
    /// # Official API Reference
    /// <a href="https://developer.riotgames.com/api-methods/#spectator-v4">https://developer.riotgames.com/api-methods/#spectator-v4</a>
    #[inline]
    pub fn spectator_v4(&self) -> SpectatorV4 {
        SpectatorV4 { base: self }
    }
    /// Handle for SummonerV4 endpoints. This method is automatically generated.
    /// # Official API Reference
    /// <a href="https://developer.riotgames.com/api-methods/#summoner-v4">https://developer.riotgames.com/api-methods/#summoner-v4</a>
    #[inline]
    pub fn summoner_v4(&self) -> SummonerV4 {
        SummonerV4 { base: self }
    }
    /// Handle for ThirdPartyCodeV4 endpoints. This method is automatically generated.
    /// # Official API Reference
    /// <a href="https://developer.riotgames.com/api-methods/#third-party-code-v4">https://developer.riotgames.com/api-methods/#third-party-code-v4</a>
    #[inline]
    pub fn third_party_code_v4(&self) -> ThirdPartyCodeV4 {
        ThirdPartyCodeV4 { base: self }
    }
    /// Handle for TournamentStubV4 endpoints. This method is automatically generated.
    /// # Official API Reference
    /// <a href="https://developer.riotgames.com/api-methods/#tournament-stub-v4">https://developer.riotgames.com/api-methods/#tournament-stub-v4</a>
    #[inline]
    pub fn tournament_stub_v4(&self) -> TournamentStubV4 {
        TournamentStubV4 { base: self }
    }
    /// Handle for TournamentV4 endpoints. This method is automatically generated.
    /// # Official API Reference
    /// <a href="https://developer.riotgames.com/api-methods/#tournament-v4">https://developer.riotgames.com/api-methods/#tournament-v4</a>
    #[inline]
    pub fn tournament_v4(&self) -> TournamentV4 {
        TournamentV4 { base: self }
    }
}

/// ChampionMasteryV4 endpoints. This struct is automatically generated.
/// # Official API Reference
/// <a href="https://developer.riotgames.com/api-methods/#champion-mastery-v4">https://developer.riotgames.com/api-methods/#champion-mastery-v4</a>
pub struct ChampionMasteryV4<'a> {
    base: &'a RiotApi,
}
impl<'a> ChampionMasteryV4<'a> {
        /// Get all champion mastery entries sorted by number of champion points descending,
        /// # Official API Reference
        /// <a href="https://developer.riotgames.com/api-methods/#champion-mastery-v4/GET_getAllChampionMasteries">https://developer.riotgames.com/api-methods/#champion-mastery-v4/GET_getAllChampionMasteries</a>
        /// # Parameters
        /// * `region` - Region to query.
        /// * `encryptedSummonerId` - Summoner ID associated with the player
        pub fn get_all_champion_masteries(&self, region: Region, encrypted_summoner_id: &str)
            -> impl Future<Output = Result<Option<Vec<champion_mastery_v4::ChampionMastery>>>> + 'a
        {
            let path_string = format!("/lol/champion-mastery/v4/champion-masteries/by-summoner/{}", encrypted_summoner_id);
            self.base.get::<Vec<champion_mastery_v4::ChampionMastery>>("champion-mastery-v4.getAllChampionMasteries", region, path_string, None)
        }

        /// Get a champion mastery by player ID and champion ID.
        /// # Official API Reference
        /// <a href="https://developer.riotgames.com/api-methods/#champion-mastery-v4/GET_getChampionMastery">https://developer.riotgames.com/api-methods/#champion-mastery-v4/GET_getChampionMastery</a>
        /// # Parameters
        /// * `region` - Region to query.
        /// * `championId` - Champion ID to retrieve Champion Mastery for
        /// * `encryptedSummonerId` - Summoner ID associated with the player
        pub fn get_champion_mastery(&self, region: Region, encrypted_summoner_id: &str, champion_id: i64)
            -> impl Future<Output = Result<Option<champion_mastery_v4::ChampionMastery>>> + 'a
        {
            let path_string = format!("/lol/champion-mastery/v4/champion-masteries/by-summoner/{}/by-champion/{}", encrypted_summoner_id, champion_id);
            self.base.get::<champion_mastery_v4::ChampionMastery>("champion-mastery-v4.getChampionMastery", region, path_string, None)
        }

        /// Get a player's total champion mastery score, which is the sum of individual champion mastery levels.
        /// # Official API Reference
        /// <a href="https://developer.riotgames.com/api-methods/#champion-mastery-v4/GET_getChampionMasteryScore">https://developer.riotgames.com/api-methods/#champion-mastery-v4/GET_getChampionMasteryScore</a>
        /// # Parameters
        /// * `region` - Region to query.
        /// * `encryptedSummonerId` - Summoner ID associated with the player
        pub fn get_champion_mastery_score(&self, region: Region, encrypted_summoner_id: &str)
            -> impl Future<Output = Result<Option<i32>>> + 'a
        {
            let path_string = format!("/lol/champion-mastery/v4/scores/by-summoner/{}", encrypted_summoner_id);
            self.base.get::<i32>("champion-mastery-v4.getChampionMasteryScore", region, path_string, None)
        }

}

/// ChampionV3 endpoints. This struct is automatically generated.
/// # Official API Reference
/// <a href="https://developer.riotgames.com/api-methods/#champion-v3">https://developer.riotgames.com/api-methods/#champion-v3</a>
pub struct ChampionV3<'a> {
    base: &'a RiotApi,
}
impl<'a> ChampionV3<'a> {
        /// Returns champion rotations, including free-to-play and low-level free-to-play rotations (REST)
        /// # Official API Reference
        /// <a href="https://developer.riotgames.com/api-methods/#champion-v3/GET_getChampionInfo">https://developer.riotgames.com/api-methods/#champion-v3/GET_getChampionInfo</a>
        /// # Parameters
        /// * `region` - Region to query.
        pub fn get_champion_info(&self, region: Region)
            -> impl Future<Output = Result<Option<champion_v3::ChampionInfo>>> + 'a
        {
            let path_string = "/lol/platform/v3/champion-rotations".to_owned();
            self.base.get::<champion_v3::ChampionInfo>("champion-v3.getChampionInfo", region, path_string, None)
        }

}

/// LeagueExpV4 endpoints. This struct is automatically generated.
/// # Official API Reference
/// <a href="https://developer.riotgames.com/api-methods/#league-exp-v4">https://developer.riotgames.com/api-methods/#league-exp-v4</a>
pub struct LeagueExpV4<'a> {
    base: &'a RiotApi,
}
impl<'a> LeagueExpV4<'a> {
        /// Get all the league entries.
        /// # Official API Reference
        /// <a href="https://developer.riotgames.com/api-methods/#league-exp-v4/GET_getLeagueEntries">https://developer.riotgames.com/api-methods/#league-exp-v4/GET_getLeagueEntries</a>
        /// # Parameters
        /// * `region` - Region to query.
        /// * `queue` - Note that the queue value must be a valid ranked queue.
        /// * `tier`
        /// * `division`
        /// * `page` (optional) - Starts with page 1.
        pub fn get_league_entries(&self, region: Region, division: &str, tier: &str, queue: &str, page: Option<i32>)
            -> impl Future<Output = Result<Option<Vec<league_exp_v4::LeagueEntry>>>> + 'a
        {
            let mut query_params = Serializer::new(String::new());
            if let Some(page) = page { query_params.append_pair("page", &*page.to_string()); };
            let query_string = query_params.finish();
            let path_string = format!("/lol/league-exp/v4/entries/{}/{}/{}", division, tier, queue);
            self.base.get::<Vec<league_exp_v4::LeagueEntry>>("league-exp-v4.getLeagueEntries", region, path_string, Some(query_string))
        }

}

/// LeagueV4 endpoints. This struct is automatically generated.
/// # Official API Reference
/// <a href="https://developer.riotgames.com/api-methods/#league-v4">https://developer.riotgames.com/api-methods/#league-v4</a>
pub struct LeagueV4<'a> {
    base: &'a RiotApi,
}
impl<'a> LeagueV4<'a> {
        /// Get the challenger league for given queue.
        /// # Official API Reference
        /// <a href="https://developer.riotgames.com/api-methods/#league-v4/GET_getChallengerLeague">https://developer.riotgames.com/api-methods/#league-v4/GET_getChallengerLeague</a>
        /// # Parameters
        /// * `region` - Region to query.
        /// * `queue`
        pub fn get_challenger_league(&self, region: Region, queue: &str)
            -> impl Future<Output = Result<Option<league_v4::LeagueList>>> + 'a
        {
            let path_string = format!("/lol/league/v4/challengerleagues/by-queue/{}", queue);
            self.base.get::<league_v4::LeagueList>("league-v4.getChallengerLeague", region, path_string, None)
        }

        /// Get league entries in all queues for a given summoner ID.
        /// # Official API Reference
        /// <a href="https://developer.riotgames.com/api-methods/#league-v4/GET_getLeagueEntriesForSummoner">https://developer.riotgames.com/api-methods/#league-v4/GET_getLeagueEntriesForSummoner</a>
        /// # Parameters
        /// * `region` - Region to query.
        /// * `encryptedSummonerId`
        pub fn get_league_entries_for_summoner(&self, region: Region, encrypted_summoner_id: &str)
            -> impl Future<Output = Result<Option<Vec<league_v4::LeagueEntry>>>> + 'a
        {
            let path_string = format!("/lol/league/v4/entries/by-summoner/{}", encrypted_summoner_id);
            self.base.get::<Vec<league_v4::LeagueEntry>>("league-v4.getLeagueEntriesForSummoner", region, path_string, None)
        }

        /// Get all the league entries.
        /// # Official API Reference
        /// <a href="https://developer.riotgames.com/api-methods/#league-v4/GET_getLeagueEntries">https://developer.riotgames.com/api-methods/#league-v4/GET_getLeagueEntries</a>
        /// # Parameters
        /// * `region` - Region to query.
        /// * `division`
        /// * `tier`
        /// * `queue` - Note that the queue value must be a valid ranked queue.
        /// * `page` (optional) - Starts with page 1.
        pub fn get_league_entries(&self, region: Region, queue: &str, tier: &str, division: &str, page: Option<i32>)
            -> impl Future<Output = Result<Option<Vec<league_v4::LeagueEntry>>>> + 'a
        {
            let mut query_params = Serializer::new(String::new());
            if let Some(page) = page { query_params.append_pair("page", &*page.to_string()); };
            let query_string = query_params.finish();
            let path_string = format!("/lol/league/v4/entries/{}/{}/{}", queue, tier, division);
            self.base.get::<Vec<league_v4::LeagueEntry>>("league-v4.getLeagueEntries", region, path_string, Some(query_string))
        }

        /// Get the grandmaster league of a specific queue.
        /// # Official API Reference
        /// <a href="https://developer.riotgames.com/api-methods/#league-v4/GET_getGrandmasterLeague">https://developer.riotgames.com/api-methods/#league-v4/GET_getGrandmasterLeague</a>
        /// # Parameters
        /// * `region` - Region to query.
        /// * `queue`
        pub fn get_grandmaster_league(&self, region: Region, queue: &str)
            -> impl Future<Output = Result<Option<league_v4::LeagueList>>> + 'a
        {
            let path_string = format!("/lol/league/v4/grandmasterleagues/by-queue/{}", queue);
            self.base.get::<league_v4::LeagueList>("league-v4.getGrandmasterLeague", region, path_string, None)
        }

        /// Get league with given ID, including inactive entries.
        /// # Official API Reference
        /// <a href="https://developer.riotgames.com/api-methods/#league-v4/GET_getLeagueById">https://developer.riotgames.com/api-methods/#league-v4/GET_getLeagueById</a>
        /// # Parameters
        /// * `region` - Region to query.
        /// * `leagueId` - The UUID of the league.
        pub fn get_league_by_id(&self, region: Region, league_id: &str)
            -> impl Future<Output = Result<Option<league_v4::LeagueList>>> + 'a
        {
            let path_string = format!("/lol/league/v4/leagues/{}", league_id);
            self.base.get::<league_v4::LeagueList>("league-v4.getLeagueById", region, path_string, None)
        }

        /// Get the master league for given queue.
        /// # Official API Reference
        /// <a href="https://developer.riotgames.com/api-methods/#league-v4/GET_getMasterLeague">https://developer.riotgames.com/api-methods/#league-v4/GET_getMasterLeague</a>
        /// # Parameters
        /// * `region` - Region to query.
        /// * `queue`
        pub fn get_master_league(&self, region: Region, queue: &str)
            -> impl Future<Output = Result<Option<league_v4::LeagueList>>> + 'a
        {
            let path_string = format!("/lol/league/v4/masterleagues/by-queue/{}", queue);
            self.base.get::<league_v4::LeagueList>("league-v4.getMasterLeague", region, path_string, None)
        }

}

/// LolStatusV3 endpoints. This struct is automatically generated.
/// # Official API Reference
/// <a href="https://developer.riotgames.com/api-methods/#lol-status-v3">https://developer.riotgames.com/api-methods/#lol-status-v3</a>
pub struct LolStatusV3<'a> {
    base: &'a RiotApi,
}
impl<'a> LolStatusV3<'a> {
        /// Get League of Legends status for the given shard.
        /// ## Rate Limit Notes
        /// Requests to this API are not counted against the application Rate Limits.
        /// # Official API Reference
        /// <a href="https://developer.riotgames.com/api-methods/#lol-status-v3/GET_getShardData">https://developer.riotgames.com/api-methods/#lol-status-v3/GET_getShardData</a>
        /// # Parameters
        /// * `region` - Region to query.
        pub fn get_shard_data(&self, region: Region)
            -> impl Future<Output = Result<Option<lol_status_v3::ShardStatus>>> + 'a
        {
            let path_string = "/lol/status/v3/shard-data".to_owned();
            self.base.get::<lol_status_v3::ShardStatus>("lol-status-v3.getShardData", region, path_string, None)
        }

}

/// MatchV4 endpoints. This struct is automatically generated.
/// # Official API Reference
/// <a href="https://developer.riotgames.com/api-methods/#match-v4">https://developer.riotgames.com/api-methods/#match-v4</a>
pub struct MatchV4<'a> {
    base: &'a RiotApi,
}
impl<'a> MatchV4<'a> {
        /// Get match IDs by tournament code.
        /// # Official API Reference
        /// <a href="https://developer.riotgames.com/api-methods/#match-v4/GET_getMatchIdsByTournamentCode">https://developer.riotgames.com/api-methods/#match-v4/GET_getMatchIdsByTournamentCode</a>
        /// # Parameters
        /// * `region` - Region to query.
        /// * `tournamentCode` - The tournament code.
        pub fn get_match_ids_by_tournament_code(&self, region: Region, tournament_code: &str)
            -> impl Future<Output = Result<Option<Vec<i64>>>> + 'a
        {
            let path_string = format!("/lol/match/v4/matches/by-tournament-code/{}/ids", tournament_code);
            self.base.get::<Vec<i64>>("match-v4.getMatchIdsByTournamentCode", region, path_string, None)
        }

        /// Get match by match ID.
        /// # Official API Reference
        /// <a href="https://developer.riotgames.com/api-methods/#match-v4/GET_getMatch">https://developer.riotgames.com/api-methods/#match-v4/GET_getMatch</a>
        /// # Parameters
        /// * `region` - Region to query.
        /// * `matchId` - The match ID.
        pub fn get_match(&self, region: Region, match_id: i64)
            -> impl Future<Output = Result<Option<match_v4::Match>>> + 'a
        {
            let path_string = format!("/lol/match/v4/matches/{}", match_id);
            self.base.get::<match_v4::Match>("match-v4.getMatch", region, path_string, None)
        }

        /// Get match by match ID and tournament code.
        /// # Official API Reference
        /// <a href="https://developer.riotgames.com/api-methods/#match-v4/GET_getMatchByTournamentCode">https://developer.riotgames.com/api-methods/#match-v4/GET_getMatchByTournamentCode</a>
        /// # Parameters
        /// * `region` - Region to query.
        /// * `tournamentCode` - The tournament code.
        /// * `matchId` - The match ID.
        pub fn get_match_by_tournament_code(&self, region: Region, match_id: i64, tournament_code: &str)
            -> impl Future<Output = Result<Option<match_v4::Match>>> + 'a
        {
            let path_string = format!("/lol/match/v4/matches/{}/by-tournament-code/{}", match_id, tournament_code);
            self.base.get::<match_v4::Match>("match-v4.getMatchByTournamentCode", region, path_string, None)
        }

        /// Get matchlist for games played on given account ID and platform ID and filtered using given filter parameters, if any.
        /// ## Implementation Notes
        /// A number of optional parameters are provided for filtering. It is up to the caller to ensure that the combination of filter parameters provided is valid for the requested account, otherwise, no matches may be returned.
        ///
        /// If beginIndex is specified, but not endIndex, then endIndex defaults to beginIndex+100. If endIndex is specified, but not beginIndex, then beginIndex defaults to 0. If both are specified, then endIndex must be greater than beginIndex. The maximum range allowed is 100, otherwise a 400 error code is returned.
        ///
        /// If beginTime is specified, but not endTime, then endTime defaults to the the current unix timestamp in milliseconds (the maximum time range limitation is not observed in this specific case). If endTime is specified, but not beginTime, then beginTime defaults to the start of the account's match history returning a 400 due to the maximum time range limitation. If both are specified, then endTime should be greater than beginTime. The maximum time range allowed is one week, otherwise a 400 error code is returned.
        /// # Official API Reference
        /// <a href="https://developer.riotgames.com/api-methods/#match-v4/GET_getMatchlist">https://developer.riotgames.com/api-methods/#match-v4/GET_getMatchlist</a>
        /// # Parameters
        /// * `region` - Region to query.
        /// * `encryptedAccountId` - The account ID.
        /// * `champion` (optional) - Set of champion IDs for filtering the matchlist.
        /// * `queue` (optional) - Set of queue IDs for filtering the matchlist.
        /// * `season` (optional) - Set of season IDs for filtering the matchlist.
        /// * `endTime` (optional) - The end time to use for filtering matchlist specified as epoch milliseconds. If beginTime is specified, but not endTime, then endTime defaults to the the current unix timestamp in milliseconds (the maximum time range limitation is not observed in this specific case). If endTime is specified, but not beginTime, then beginTime defaults to the start of the account's match history returning a 400 due to the maximum time range limitation. If both are specified, then endTime should be greater than beginTime. The maximum time range allowed is one week, otherwise a 400 error code is returned.
        /// * `beginTime` (optional) - The begin time to use for filtering matchlist specified as epoch milliseconds. If beginTime is specified, but not endTime, then endTime defaults to the the current unix timestamp in milliseconds (the maximum time range limitation is not observed in this specific case). If endTime is specified, but not beginTime, then beginTime defaults to the start of the account's match history returning a 400 due to the maximum time range limitation. If both are specified, then endTime should be greater than beginTime. The maximum time range allowed is one week, otherwise a 400 error code is returned.
        /// * `endIndex` (optional) - The end index to use for filtering matchlist. If beginIndex is specified, but not endIndex, then endIndex defaults to beginIndex+100. If endIndex is specified, but not beginIndex, then beginIndex defaults to 0. If both are specified, then endIndex must be greater than beginIndex. The maximum range allowed is 100, otherwise a 400 error code is returned.
        /// * `beginIndex` (optional) - The begin index to use for filtering matchlist.  If beginIndex is specified, but not endIndex, then endIndex defaults to beginIndex+100. If endIndex is specified, but not beginIndex, then beginIndex defaults to 0. If both are specified, then endIndex must be greater than beginIndex. The maximum range allowed is 100, otherwise a 400 error code is returned.
        pub fn get_matchlist(&self, region: Region, encrypted_account_id: &str, champion: Option<std::vec::Vec<i32>>, queue: Option<std::vec::Vec<i32>>, season: Option<std::vec::Vec<i32>>, end_time: Option<i64>, begin_time: Option<i64>, end_index: Option<i32>, begin_index: Option<i32>)
            -> impl Future<Output = Result<Option<match_v4::Matchlist>>> + 'a
        {
            let mut query_params = Serializer::new(String::new());
            if let Some(champion) = champion { query_params.extend_pairs(champion.iter().map(|w| ("champion", w.to_string()))); };
            if let Some(queue) = queue { query_params.extend_pairs(queue.iter().map(|w| ("queue", w.to_string()))); };
            if let Some(season) = season { query_params.extend_pairs(season.iter().map(|w| ("season", w.to_string()))); };
            if let Some(end_time) = end_time { query_params.append_pair("endTime", &*end_time.to_string()); };
            if let Some(begin_time) = begin_time { query_params.append_pair("beginTime", &*begin_time.to_string()); };
            if let Some(end_index) = end_index { query_params.append_pair("endIndex", &*end_index.to_string()); };
            if let Some(begin_index) = begin_index { query_params.append_pair("beginIndex", &*begin_index.to_string()); };
            let query_string = query_params.finish();
            let path_string = format!("/lol/match/v4/matchlists/by-account/{}", encrypted_account_id);
            self.base.get::<match_v4::Matchlist>("match-v4.getMatchlist", region, path_string, Some(query_string))
        }

        /// Get match timeline by match ID.
        /// ## Implementation Notes
        /// Not all matches have timeline data.
        /// # Official API Reference
        /// <a href="https://developer.riotgames.com/api-methods/#match-v4/GET_getMatchTimeline">https://developer.riotgames.com/api-methods/#match-v4/GET_getMatchTimeline</a>
        /// # Parameters
        /// * `region` - Region to query.
        /// * `matchId` - The match ID.
        pub fn get_match_timeline(&self, region: Region, match_id: i64)
            -> impl Future<Output = Result<Option<match_v4::MatchTimeline>>> + 'a
        {
            let path_string = format!("/lol/match/v4/timelines/by-match/{}", match_id);
            self.base.get::<match_v4::MatchTimeline>("match-v4.getMatchTimeline", region, path_string, None)
        }

}

/// SpectatorV4 endpoints. This struct is automatically generated.
/// # Official API Reference
/// <a href="https://developer.riotgames.com/api-methods/#spectator-v4">https://developer.riotgames.com/api-methods/#spectator-v4</a>
pub struct SpectatorV4<'a> {
    base: &'a RiotApi,
}
impl<'a> SpectatorV4<'a> {
        /// Get current game information for the given summoner ID.
        /// # Official API Reference
        /// <a href="https://developer.riotgames.com/api-methods/#spectator-v4/GET_getCurrentGameInfoBySummoner">https://developer.riotgames.com/api-methods/#spectator-v4/GET_getCurrentGameInfoBySummoner</a>
        /// # Parameters
        /// * `region` - Region to query.
        /// * `encryptedSummonerId` - The ID of the summoner.
        pub fn get_current_game_info_by_summoner(&self, region: Region, encrypted_summoner_id: &str)
            -> impl Future<Output = Result<Option<spectator_v4::CurrentGameInfo>>> + 'a
        {
            let path_string = format!("/lol/spectator/v4/active-games/by-summoner/{}", encrypted_summoner_id);
            self.base.get::<spectator_v4::CurrentGameInfo>("spectator-v4.getCurrentGameInfoBySummoner", region, path_string, None)
        }

        /// Get list of featured games.
        /// # Official API Reference
        /// <a href="https://developer.riotgames.com/api-methods/#spectator-v4/GET_getFeaturedGames">https://developer.riotgames.com/api-methods/#spectator-v4/GET_getFeaturedGames</a>
        /// # Parameters
        /// * `region` - Region to query.
        pub fn get_featured_games(&self, region: Region)
            -> impl Future<Output = Result<Option<spectator_v4::FeaturedGames>>> + 'a
        {
            let path_string = "/lol/spectator/v4/featured-games".to_owned();
            self.base.get::<spectator_v4::FeaturedGames>("spectator-v4.getFeaturedGames", region, path_string, None)
        }

}

/// SummonerV4 endpoints. This struct is automatically generated.
/// # Official API Reference
/// <a href="https://developer.riotgames.com/api-methods/#summoner-v4">https://developer.riotgames.com/api-methods/#summoner-v4</a>
pub struct SummonerV4<'a> {
    base: &'a RiotApi,
}
impl<'a> SummonerV4<'a> {
        /// Get a summoner by account ID.
        /// # Official API Reference
        /// <a href="https://developer.riotgames.com/api-methods/#summoner-v4/GET_getByAccountId">https://developer.riotgames.com/api-methods/#summoner-v4/GET_getByAccountId</a>
        /// # Parameters
        /// * `region` - Region to query.
        /// * `encryptedAccountId`
        pub fn get_by_account_id(&self, region: Region, encrypted_account_id: &str)
            -> impl Future<Output = Result<Option<summoner_v4::Summoner>>> + 'a
        {
            let path_string = format!("/lol/summoner/v4/summoners/by-account/{}", encrypted_account_id);
            self.base.get::<summoner_v4::Summoner>("summoner-v4.getByAccountId", region, path_string, None)
        }

        /// Get a summoner by summoner name.
        /// # Official API Reference
        /// <a href="https://developer.riotgames.com/api-methods/#summoner-v4/GET_getBySummonerName">https://developer.riotgames.com/api-methods/#summoner-v4/GET_getBySummonerName</a>
        /// # Parameters
        /// * `region` - Region to query.
        /// * `summonerName` - Summoner Name
        pub fn get_by_summoner_name(&self, region: Region, summoner_name: &str)
            -> impl Future<Output = Result<Option<summoner_v4::Summoner>>> + 'a
        {
            let path_string = format!("/lol/summoner/v4/summoners/by-name/{}", summoner_name);
            self.base.get::<summoner_v4::Summoner>("summoner-v4.getBySummonerName", region, path_string, None)
        }

        /// Get a summoner by PUUID.
        /// # Official API Reference
        /// <a href="https://developer.riotgames.com/api-methods/#summoner-v4/GET_getByPUUID">https://developer.riotgames.com/api-methods/#summoner-v4/GET_getByPUUID</a>
        /// # Parameters
        /// * `region` - Region to query.
        /// * `encryptedPUUID` - Summoner ID
        pub fn get_by_puuid(&self, region: Region, encrypted_puuid: &str)
            -> impl Future<Output = Result<Option<summoner_v4::Summoner>>> + 'a
        {
            let path_string = format!("/lol/summoner/v4/summoners/by-puuid/{}", encrypted_puuid);
            self.base.get::<summoner_v4::Summoner>("summoner-v4.getByPUUID", region, path_string, None)
        }

        /// Get a summoner by summoner ID.
        /// # Official API Reference
        /// <a href="https://developer.riotgames.com/api-methods/#summoner-v4/GET_getBySummonerId">https://developer.riotgames.com/api-methods/#summoner-v4/GET_getBySummonerId</a>
        /// # Parameters
        /// * `region` - Region to query.
        /// * `encryptedSummonerId` - Summoner ID
        pub fn get_by_summoner_id(&self, region: Region, encrypted_summoner_id: &str)
            -> impl Future<Output = Result<Option<summoner_v4::Summoner>>> + 'a
        {
            let path_string = format!("/lol/summoner/v4/summoners/{}", encrypted_summoner_id);
            self.base.get::<summoner_v4::Summoner>("summoner-v4.getBySummonerId", region, path_string, None)
        }

}

/// ThirdPartyCodeV4 endpoints. This struct is automatically generated.
/// # Official API Reference
/// <a href="https://developer.riotgames.com/api-methods/#third-party-code-v4">https://developer.riotgames.com/api-methods/#third-party-code-v4</a>
pub struct ThirdPartyCodeV4<'a> {
    base: &'a RiotApi,
}
impl<'a> ThirdPartyCodeV4<'a> {
        /// Get third party code for a given summoner ID.
        /// # Official API Reference
        /// <a href="https://developer.riotgames.com/api-methods/#third-party-code-v4/GET_getThirdPartyCodeBySummonerId">https://developer.riotgames.com/api-methods/#third-party-code-v4/GET_getThirdPartyCodeBySummonerId</a>
        /// # Parameters
        /// * `region` - Region to query.
        /// * `encryptedSummonerId`
        pub fn get_third_party_code_by_summoner_id(&self, region: Region, encrypted_summoner_id: &str)
            -> impl Future<Output = Result<Option<String>>> + 'a
        {
            let path_string = format!("/lol/platform/v4/third-party-code/by-summoner/{}", encrypted_summoner_id);
            self.base.get::<String>("third-party-code-v4.getThirdPartyCodeBySummonerId", region, path_string, None)
        }

}

/// TournamentStubV4 endpoints. This struct is automatically generated.
/// # Official API Reference
/// <a href="https://developer.riotgames.com/api-methods/#tournament-stub-v4">https://developer.riotgames.com/api-methods/#tournament-stub-v4</a>
pub struct TournamentStubV4<'a> {
    base: &'a RiotApi,
}
impl<'a> TournamentStubV4<'a> {
        /// Gets a mock list of lobby events by tournament code.
        /// # Official API Reference
        /// <a href="https://developer.riotgames.com/api-methods/#tournament-stub-v4/GET_getLobbyEventsByCode">https://developer.riotgames.com/api-methods/#tournament-stub-v4/GET_getLobbyEventsByCode</a>
        /// # Parameters
        /// * `region` - Region to query.
        /// * `tournamentCode` - The short code to look up lobby events for
        pub fn get_lobby_events_by_code(&self, region: Region, tournament_code: &str)
            -> impl Future<Output = Result<Option<tournament_stub_v4::LobbyEventWrapper>>> + 'a
        {
            let path_string = format!("/lol/tournament-stub/v4/lobby-events/by-code/{}", tournament_code);
            self.base.get::<tournament_stub_v4::LobbyEventWrapper>("tournament-stub-v4.getLobbyEventsByCode", region, path_string, None)
        }

}

/// TournamentV4 endpoints. This struct is automatically generated.
/// # Official API Reference
/// <a href="https://developer.riotgames.com/api-methods/#tournament-v4">https://developer.riotgames.com/api-methods/#tournament-v4</a>
pub struct TournamentV4<'a> {
    base: &'a RiotApi,
}
impl<'a> TournamentV4<'a> {
        /// Returns the tournament code DTO associated with a tournament code string.
        /// # Official API Reference
        /// <a href="https://developer.riotgames.com/api-methods/#tournament-v4/GET_getTournamentCode">https://developer.riotgames.com/api-methods/#tournament-v4/GET_getTournamentCode</a>
        /// # Parameters
        /// * `region` - Region to query.
        /// * `tournamentCode` - The tournament code string.
        pub fn get_tournament_code(&self, region: Region, tournament_code: &str)
            -> impl Future<Output = Result<Option<tournament_v4::TournamentCode>>> + 'a
        {
            let path_string = format!("/lol/tournament/v4/codes/{}", tournament_code);
            self.base.get::<tournament_v4::TournamentCode>("tournament-v4.getTournamentCode", region, path_string, None)
        }

        /// Gets a list of lobby events by tournament code.
        /// # Official API Reference
        /// <a href="https://developer.riotgames.com/api-methods/#tournament-v4/GET_getLobbyEventsByCode">https://developer.riotgames.com/api-methods/#tournament-v4/GET_getLobbyEventsByCode</a>
        /// # Parameters
        /// * `region` - Region to query.
        /// * `tournamentCode` - The short code to look up lobby events for
        pub fn get_lobby_events_by_code(&self, region: Region, tournament_code: &str)
            -> impl Future<Output = Result<Option<tournament_v4::LobbyEventWrapper>>> + 'a
        {
            let path_string = format!("/lol/tournament/v4/lobby-events/by-code/{}", tournament_code);
            self.base.get::<tournament_v4::LobbyEventWrapper>("tournament-v4.getLobbyEventsByCode", region, path_string, None)
        }

}
