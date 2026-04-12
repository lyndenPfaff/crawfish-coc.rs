/* --- IMPORTS --- */

use crate::client::Client;
use crate::model::*;
use crate::Error;

/* --- ------- --- */



/* --- STRUCTS --- */

pub struct WarLogSearch<'a> {
    client: &'a Client,
    clan_tag: String,
    limit: Option<i64>,
    after: Option<String>,
    before: Option<String>,
}

pub struct ClanSearch<'a> {
    client: &'a Client,
    name: Option<String>,
    /// Enum: [ UNKNOWN, ALWAYS, MORE_THAN_ONCE_PER_WEEK, ONCE_PER_WEEK, LESS_THAN_ONCE_PER_WEEK, NEVER, ANY ]
    war_frequency: Option<String>,
    location_id: Option<i64>,
    min_members: Option<i64>,
    max_members: Option<i64>,
    min_clan_points: Option<i64>,
    min_clan_level: Option<i64>,
    limit: Option<i64>,
    after: Option<String>,
    before: Option<String>,
    label_ids: Vec<i64>,
}

pub struct ClanMemberSearch<'a> {
    client: &'a Client,
    clan_tag: String,
    limit: Option<i64>,
    after: Option<String>,
    before: Option<String>,
}

pub struct CapitalRaidSeasonSearch<'a> {
    client: &'a Client,
    clan_tag: String,
    limit: Option<i64>,
    after: Option<String>,
    before: Option<String>,
}

pub struct PlayerBattleLogSearch<'a> {
    client: &'a Client,
    tag: String,
    limit: Option<i64>,
    after: Option<String>,
    before: Option<String>,
}

pub struct LocationalClanRankingsSearch<'a> {
    client: &'a Client,
    location_id: i64,
    limit: Option<i64>,
    after: Option<String>,
    before: Option<String>,
}

pub struct LocationalPlayerRankingsSearch<'a> {
    client: &'a Client,
    location_id: i64,
    limit: Option<i64>,
    after: Option<String>,
    before: Option<String>,
}

pub struct LocationalPlayerBuilderRankingsSearch<'a> {
    client: &'a Client,
    location_id: i64,
    limit: Option<i64>,
    after: Option<String>,
    before: Option<String>,
}

pub struct LocationalClanBuilderRankingsSearch<'a> {
    client: &'a Client,
    location_id: i64,
    limit: Option<i64>,
    after: Option<String>,
    before: Option<String>,
}

pub struct LocationsSearch<'a> {
    client: &'a Client,
    limit: Option<i64>,
    after: Option<String>,
    before: Option<String>,
}

pub struct LocationalClanCapitalRankingsSearch<'a> {
    client: &'a Client,
    location_id: i64,
    limit: Option<i64>,
    after: Option<String>,
    before: Option<String>,
}

/* --- ------- --- */



/* --- IMPLS --- */

impl WarLogSearch<'_> {
    pub (crate) fn new<'a>(client: &'a Client, clan_tag: String) -> WarLogSearch<'a> {
        WarLogSearch { client, clan_tag, limit: None, after: None, before: None }
    }

    pub async fn send(self) -> Result<(Vec<ClanWarLogEntry>, PagingCursors), Error> {
        let mut url = format!("https://api.clashofclans.com/v1/clans/{}/warlog", self.clan_tag);
        if let Some(limit) = self.limit { url.push_str(&format!("?limit={limit}")) }
        if let Some(after) = self.after { url.push_str(&format!("?after={after}")) }
        if let Some(before) = self.before { url.push_str(&format!("?before={before}")) }

        let res: SearchResponse<_> = self.client.get(&url).await?;
        Ok((res.items, res.paging))
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn after(mut self, after: String) -> Self {
        self.after = Some(after);
        self
    }

    pub fn before(mut self, before: String) -> Self {
        self.before = Some(before);
        self
    }
}

impl ClanSearch<'_> {
    pub (crate) fn new<'a>(client: &'a Client) -> ClanSearch<'a> {
        ClanSearch {
            client, name: None, war_frequency: None,
            location_id: None, min_members: None, max_members: None,
            min_clan_points: None, min_clan_level: None, limit: None,
            after: None, before: None, label_ids: Vec::new()
        }
    }

    pub async fn send(self) -> Result<(Vec<Clan>, PagingCursors), Error> {
        let mut url = String::from("https://api.clashofclans.com/v1/clans");
        if let Some(name) = self.name { url.push_str(&format!("?name={name}")) }
        if let Some(wf) = self.war_frequency { url.push_str(&format!("?warFrequency={wf}")) }
        if let Some(li) = self.location_id { url.push_str(&format!("?locationId={li}")) }
        if let Some(mm) = self.min_members { url.push_str(&format!("?minmembers={mm}")) }
        if let Some(mm) = self.max_members { url.push_str(&format!("?maxMembers={mm}")) }
        if let Some(mcp) = self.min_clan_points { url.push_str(&format!("?minClanPoints={mcp}")) }
        if let Some(mcl) = self.min_clan_level { url.push_str(&format!("?minClanLevel={mcl}")) }
        if let Some(limit) = self.limit { url.push_str(&format!("?limit={limit}")) }
        if let Some(after) = self.after { url.push_str(&format!("?after={after}")) }
        if let Some(before) = self.before { url.push_str(&format!("?before={before}")) }
        if !self.label_ids.is_empty() {
            url.push_str(&format!("?labelIds={}",
            self.label_ids.into_iter()
                .map(|l| format!("{l}")).collect::<Vec<_>>()
                .join(",")
            ));
        }

        let res: SearchResponse<_> = self.client.get(&url).await?;
        Ok((res.items, res.paging))
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn war_frequency(mut self, war_frequency: String) -> Self {
        self.war_frequency= Some(war_frequency);
        self
    }

    pub fn location(mut self, location_id: i64) -> Self {
        self.location_id= Some(location_id);
        self
    }

    pub fn min_members(mut self, min: i64) -> Self {
        self.min_members = Some(min);
        self
    }

    pub fn max_members(mut self, max: i64) -> Self {
        self.max_members = Some(max);
        self
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn after(mut self, after: String) -> Self {
        self.after = Some(after);
        self
    }

    pub fn before(mut self, before: String) -> Self {
        self.before = Some(before);
        self
    }

    pub fn add_label_ids(mut self, ids: impl Into<Vec<i64>>) -> Self {
        self.label_ids.append(&mut ids.into());
        self
    }
}

impl ClanMemberSearch<'_> {
    pub (crate) fn new<'a>(client: &'a Client, clan_tag: String) -> ClanMemberSearch<'a> {
        ClanMemberSearch { client, clan_tag, limit: None, before: None, after: None }
    }

    pub async fn send(self) -> Result<(Vec<ClanMember>, PagingCursors), Error> {
        let mut url = format!("https://api.clashofclans.com/v1/clans/{}/members", self.clan_tag);
        if let Some(limit) = self.limit { url.push_str(&format!("?limit={limit}")) }
        if let Some(after) = self.after { url.push_str(&format!("?after={after}")) }
        if let Some(before) = self.before { url.push_str(&format!("?before={before}")) }

        let res: SearchResponse<_> = self.client.get(&url).await?;
        Ok((res.items, res.paging))
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn after(mut self, after: String) -> Self {
        self.after = Some(after);
        self
    }

    pub fn before(mut self, before: String) -> Self {
        self.before = Some(before);
        self
    }
}

impl CapitalRaidSeasonSearch<'_> {
    pub (crate) fn new<'a>(client: &'a Client, clan_tag: String) -> CapitalRaidSeasonSearch<'a> {
        CapitalRaidSeasonSearch { client, clan_tag, limit: None, before: None, after: None }
    }

    pub async fn send(self) -> Result<(Vec<ClanCapitalRaidSeason>, PagingCursors), Error> {
        let mut url = format!("https://api.clashofclans.com/v1/clans/{}/capitalraidseasons", self.clan_tag);
        if let Some(limit) = self.limit { url.push_str(&format!("?limit={limit}")) }
        if let Some(after) = self.after { url.push_str(&format!("?after={after}")) }
        if let Some(before) = self.before { url.push_str(&format!("?before={before}")) }

        let res: SearchResponse<_> = self.client.get(&url).await?;
        Ok((res.items, res.paging))
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn after(mut self, after: String) -> Self {
        self.after = Some(after);
        self
    }

    pub fn before(mut self, before: String) -> Self {
        self.before = Some(before);
        self
    }
}

impl PlayerBattleLogSearch<'_> {
    pub (crate) fn new<'a>(client: &'a Client, tag: String) -> PlayerBattleLogSearch<'a> {
        PlayerBattleLogSearch { client, tag, limit: None, before: None, after: None }
    }

    pub async fn send(self) -> Result<(Vec<BattleLogEntry>, PagingCursors), Error> {
        let mut url = format!("https://api.clashofclans.com/v1/clans/{}/capitalraidseasons", self.tag);
        if let Some(limit) = self.limit { url.push_str(&format!("?limit={limit}")) }
        if let Some(after) = self.after { url.push_str(&format!("?after={after}")) }
        if let Some(before) = self.before { url.push_str(&format!("?before={before}")) }

        let res: SearchResponse<_> = self.client.get(&url).await?;
        Ok((res.items, res.paging))
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn after(mut self, after: String) -> Self {
        self.after = Some(after);
        self
    }

    pub fn before(mut self, before: String) -> Self {
        self.before = Some(before);
        self
    }
}

impl LocationalClanRankingsSearch<'_> {
    pub (crate) fn new<'a>(client: &'a Client, location_id: i64) -> LocationalClanRankingsSearch<'a> {
        LocationalClanRankingsSearch { client, location_id, limit: None, before: None, after: None }
    }

    pub async fn send(self) -> Result<(Vec<ClanRanking>, PagingCursors), Error> {
        let mut url = format!("https://api.clashofclans.com/v1/locations/{}/rankings/clans/", self.location_id);
        if let Some(limit) = self.limit { url.push_str(&format!("?limit={limit}")) }
        if let Some(after) = self.after { url.push_str(&format!("?after={after}")) }
        if let Some(before) = self.before { url.push_str(&format!("?before={before}")) }

        let res: SearchResponse<_> = self.client.get(&url).await?;
        Ok((res.items, res.paging))
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn after(mut self, after: String) -> Self {
        self.after = Some(after);
        self
    }

    pub fn before(mut self, before: String) -> Self {
        self.before = Some(before);
        self
    }
}

impl LocationalPlayerRankingsSearch<'_> {
    pub (crate) fn new<'a>(client: &'a Client, location_id: i64) -> LocationalPlayerRankingsSearch<'a> {
        LocationalPlayerRankingsSearch { client, location_id, limit: None, before: None, after: None }
    }

    pub async fn send(self) -> Result<(Vec<PlayerRanking>, PagingCursors), Error> {
        let mut url = format!("https://api.clashofclans.com/v1/locations/{}/rankings/players/", self.location_id);
        if let Some(limit) = self.limit { url.push_str(&format!("?limit={limit}")) }
        if let Some(after) = self.after { url.push_str(&format!("?after={after}")) }
        if let Some(before) = self.before { url.push_str(&format!("?before={before}")) }

        let res: SearchResponse<_> = self.client.get(&url).await?;
        Ok((res.items, res.paging))
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn after(mut self, after: String) -> Self {
        self.after = Some(after);
        self
    }

    pub fn before(mut self, before: String) -> Self {
        self.before = Some(before);
        self
    }
}

impl LocationalPlayerBuilderRankingsSearch<'_> {
    pub (crate) fn new<'a>(client: &'a Client, location_id: i64) -> LocationalPlayerBuilderRankingsSearch<'a> {
        LocationalPlayerBuilderRankingsSearch { client, location_id, limit: None, before: None, after: None }
    }

    pub async fn send(self) -> Result<(Vec<PlayerBuilderBaseRanking>, PagingCursors), Error> {
        let mut url = format!("https://api.clashofclans.com/v1/locations/{}/rankings/players-builder-base/", self.location_id);
        if let Some(limit) = self.limit { url.push_str(&format!("?limit={limit}")) }
        if let Some(after) = self.after { url.push_str(&format!("?after={after}")) }
        if let Some(before) = self.before { url.push_str(&format!("?before={before}")) }

        let res: SearchResponse<_> = self.client.get(&url).await?;
        Ok((res.items, res.paging))
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn after(mut self, after: String) -> Self {
        self.after = Some(after);
        self
    }

    pub fn before(mut self, before: String) -> Self {
        self.before = Some(before);
        self
    }
}

impl LocationalClanBuilderRankingsSearch<'_> {
    pub (crate) fn new<'a>(client: &'a Client, location_id: i64) -> LocationalClanBuilderRankingsSearch<'a> {
        LocationalClanBuilderRankingsSearch { client, location_id, limit: None, before: None, after: None }
    }

    pub async fn send(self) -> Result<(Vec<ClanBuilderBaseRanking>, PagingCursors), Error> {
        let mut url = format!("https://api.clashofclans.com/v1/locations/{}/rankings/clans-builder-base/", self.location_id);
        if let Some(limit) = self.limit { url.push_str(&format!("?limit={limit}")) }
        if let Some(after) = self.after { url.push_str(&format!("?after={after}")) }
        if let Some(before) = self.before { url.push_str(&format!("?before={before}")) }

        let res: SearchResponse<_> = self.client.get(&url).await?;
        Ok((res.items, res.paging))
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn after(mut self, after: String) -> Self {
        self.after = Some(after);
        self
    }

    pub fn before(mut self, before: String) -> Self {
        self.before = Some(before);
        self
    }
}

impl LocationsSearch<'_> {
    pub (crate) fn new<'a>(client: &'a Client) -> LocationsSearch<'a> {
        LocationsSearch { client, limit: None, before: None, after: None }
    }

    pub async fn send(self) -> Result<(Vec<Location>, PagingCursors), Error> {
        let mut url = String::from("https://api.clashofclans.com/v1/locations/");
        if let Some(limit) = self.limit { url.push_str(&format!("?limit={limit}")) }
        if let Some(after) = self.after { url.push_str(&format!("?after={after}")) }
        if let Some(before) = self.before { url.push_str(&format!("?before={before}")) }

        let res: SearchResponse<_> = self.client.get(&url).await?;
        Ok((res.items, res.paging))
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn after(mut self, after: String) -> Self {
        self.after = Some(after);
        self
    }

    pub fn before(mut self, before: String) -> Self {
        self.before = Some(before);
        self
    }
}

impl LocationalClanCapitalRankingsSearch<'_> {
    pub (crate) fn new<'a>(client: &'a Client, location_id: i64) -> LocationalClanCapitalRankingsSearch<'a> {
        LocationalClanCapitalRankingsSearch { client, location_id, limit: None, before: None, after: None }
    }

    pub async fn send(self) -> Result<(Vec<ClanCapitalRanking>, PagingCursors), Error> {
        let mut url = format!("https://api.clashofclans.com/v1/locations/{}/rankings/clans-builder-base/", self.location_id);
        if let Some(limit) = self.limit { url.push_str(&format!("?limit={limit}")) }
        if let Some(after) = self.after { url.push_str(&format!("?after={after}")) }
        if let Some(before) = self.before { url.push_str(&format!("?before={before}")) }

        let res: SearchResponse<_> = self.client.get(&url).await?;
        Ok((res.items, res.paging))
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn after(mut self, after: String) -> Self {
        self.after = Some(after);
        self
    }

    pub fn before(mut self, before: String) -> Self {
        self.before = Some(before);
        self
    }
}

/* --- ----- --- */
