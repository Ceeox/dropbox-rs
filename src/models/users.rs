// /get_account
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct GetAccountArg
{
	account_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct BasicAccount
{
	account_id: String,
	name: Name,
	email: String,
	email_verified: bool,
	disabled: bool,
	is_teammate: bool,
	profile_photo_url: Option<String>,
	team_member_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Name
{
	given_name: String,
	surname: String,
	familiar_name: String,
	display_name: String,
	abbreviated_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct GetAccountBatchArg
{
	account_ids: Vec<String>
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct FullAccount
{
	account_id: String,
	name: Name,
	email: String,
	email_verified: bool,
	disabled: bool,
	locale: String,
	referral_link: String,
	is_paired: bool,
	account_type: AccountType,
	profile_photo_url: Option<String>,
	country: Option<String>,
	team: Option<FullTeam>,
	team_member_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum AccountType
{
	#[serde(rename="basic")]
	Basic,
	#[serde(rename="pro")]
	Pro,
	#[serde(rename="business")]
	Business,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct FullTeam
{
	id: String,
	name: String,
	sharing_policies: TeamSharingPolicies,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TeamSharingPolicies
{
	shared_folder_member_policy: SharedFolderMemberPolicy,
	shared_folder_join_policy: SharedFolderJoinPolicy,
	shared_link_create_policy: SharedLinkCreatePolicy,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum SharedFolderMemberPolicy
{
	#[serde(rename="team")]
	Team,
	#[serde(rename="anyone")]
	Anyone,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum SharedFolderJoinPolicy
{
	#[serde(rename="from_team_only")]
	FromTeamOnly,
	#[serde(rename="from_anyone")]
	FromAnyone,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum SharedLinkCreatePolicy
{
	#[serde(rename="default_public")]
	DefaultPublic,
	#[serde(rename="default_team_only")]
	DefaultTeamOnly,
	#[serde(rename="team_only")]
	TeamOnly,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct SpaceUsage
{
	used: u64,
	allocation: SpaceAllocation,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct SpaceAllocation
{
	individual: IndividualSpaceAllocation,
	team: TeamSpaceAllocation,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct IndividualSpaceAllocation
{
	allocated: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct TeamSpaceAllocation
{
	used: u64,
	allocated: u64,
}
