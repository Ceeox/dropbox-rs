#[derive(Serialize, Debug, Clone, Default, PartialEq)]
pub struct GetAccountArg
{
	///  A user's account identifier.
	pub account_id: String,
}

/// Basic information about any account.
#[derive(Deserialize, Debug, Clone, Default, PartialEq)]
pub struct BasicAccount
{
	/// The user's unique Dropbox ID.
	pub account_id: String,
	/// Details of a user's name.
	pub name: Name,
	/// The user's e-mail address. Do not rely on this without checking the email_verified field.
	/// Even then, it's possible that the user has since lost access to their e-mail.
	pub email: String,
	/// Whether the user has verified their e-mail address.
	pub email_verified: bool,
	/// Whether the user has been disabled.
	pub disabled: bool,
	/// Whether this user is a teammate of the current user.
	/// If this account is the current user's account, then this will be true.
	pub is_teammate: bool,
	/// URL for the photo representing the user, if one is set. This field is optional.
	pub profile_photo_url: Option<String>,
	/// The user's unique team member id.
	/// This field will only be present if the user is part of a team and is_teammate is true.
	pub team_member_id: Option<String>,
}

/// Representations for a person's name to assist with internationalization.
#[derive(Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Name
{
	/// Also known as a first name.
	pub given_name: String,
	/// Also known as a last name or family name.
	pub surname: String,
	/// Locale-dependent name. In the US, a person's familiar name is their given_name,
	// but elsewhere, it could be any combination of a person's given_name and surname.
	pub familiar_name: String,
	/// A name that can be used directly to represent the name of a user's Dropbox account.
	pub display_name: String,
	/// An abbreviated form of the person's name. Their initials in most locales.
	pub abbreviated_name: String,
}

#[derive(Serialize, Debug, Clone, Default, PartialEq)]
pub struct GetAccountBatchArg
{
	/// List of user account identifiers. Should not contain any duplicate account IDs.
	pub account_ids: Vec<String>
}

/// Detailed information about the current user's account.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct FullAccount
{
	/// The user's unique Dropbox ID.
	pub account_id: String,
	/// Details of a user's name.
	pub name: Name,
	/// The user's e-mail address. Do not rely on this without checking the email_verified field.
	/// Even then, it's possible that the user has since lost access to their e-mail.
	pub email: String,
	/// Whether the user has verified their e-mail address.
	pub email_verified: bool,
	/// Whether the user has been disabled.
	pub disabled: bool,
	/// The language that the user specified. Locale tags will be IETF language tags.
	pub locale: String,
	/// The user's referral link.
	pub referral_link: String,
	/// Whether the user has a personal and work account. If the current account is personal,
	/// then team will always be None, but is_paired will indicate if a work account is linked.
	pub is_paired: bool,
	/// What type of account this user has.
	pub account_type: AccountType,
	/// URL for the photo representing the user, if one is set.
	pub profile_photo_url: Option<String>,
	/// The user's two-letter country code, if available. Country codes are based on ISO 3166-1.
	pub country: Option<String>,
	///  If this account is a member of a team, information about that team.
	pub team: Option<FullTeam>,
	/// This account's unique team member id. This field will only be present if team is present.
	pub team_member_id: Option<String>,
}

/// What type of account this user has.
/// This datatype comes from an imported namespace originally defined in the users_common namespace.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum AccountType
{
	/// The basic account type.
	#[serde(rename="basic")]
	Basic,
	/// The Dropbox Pro account type.
	#[serde(rename="pro")]
	Pro,
	/// The Dropbox Business account type.
	#[serde(rename="business")]
	Business,
}

/// Detailed information about a team.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct FullTeam
{
	/// The team's unique ID.
	pub id: String,
	/// The name of the team.
	pub name: String,
	/// Team policies governing sharing.
	pub sharing_policies: TeamSharingPolicies,
}

/// Policies governing sharing within and outside of the team.
/// This datatype comes from an imported namespace originally defined in the team_policies namespace.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct TeamSharingPolicies
{
	/// Who can join folders shared by team members.
	pub shared_folder_member_policy: SharedFolderMemberPolicy,
	/// Which shared folders team members can join.
	pub shared_folder_join_policy: SharedFolderJoinPolicy,
	/// Who can view shared links owned by team members.
	pub shared_link_create_policy: SharedLinkCreatePolicy,
}

/// Policy governing who can be a member of a folder shared by a team member.
/// This datatype comes from an imported namespace originally defined in the team_policies namespace.
#[derive( Deserialize, Debug, Clone, PartialEq)]
pub enum SharedFolderMemberPolicy
{
	/// Only a teammate can be a member of a folder shared by a team member.
	#[serde(rename="team")]
	Team,
	/// Anyone can be a member of a folder shared by a team member.
	#[serde(rename="anyone")]
	Anyone,
}

/// Policy governing which shared folders a team member can join.
/// This datatype comes from an imported namespace originally defined in the team_policies namespace.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum SharedFolderJoinPolicy
{
	/// Team members can only join folders shared by teammates.
	#[serde(rename="from_team_only")]
	FromTeamOnly,
	/// Team members can join any shared folder, including those shared by users outside the team.
	#[serde(rename="from_anyone")]
	FromAnyone,
}

/// Policy governing the visibility of shared links. This policy can apply to newly created shared links,
/// or all shared links.
/// This datatype comes from an imported namespace originally defined in the team_policies namespace.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum SharedLinkCreatePolicy
{
	/// By default, anyone can access newly created shared links.
	/// No login will be required to access the shared links unless overridden.
	#[serde(rename="default_public")]
	DefaultPublic,
	/// By default, only members of the same team can access newly created shared links.
	/// Login will be required to access the shared links unless overridden.
	#[serde(rename="default_team_only")]
	DefaultTeamOnly,
	/// Only members of the same team can access all shared links.
	/// Login will be required to access all shared links.
	#[serde(rename="team_only")]
	TeamOnly,
}

/// Information about a user's space usage and quota.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct SpaceUsage
{
	/// The user's total space usage (bytes).
	pub used: u64,
	/// The user's space allocation.
	pub allocation: SpaceAllocation,
}

/// Space is allocated differently based on the type of account.
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(tag=".tag")]
pub enum SpaceAllocation
{
	/// The user's space allocation applies only to their individual account.
	#[serde(rename="individual")]
	Individual(IndividualSpaceAllocation),
	/// The user shares space with other members of their team.
	#[serde(rename="team")]
	Team(TeamSpaceAllocation),
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct IndividualSpaceAllocation
{
	/// The total space allocated to the user's account (bytes).
	pub allocated: u64,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct TeamSpaceAllocation
{
	/// The total space currently used by the user's team (bytes).
	pub used: u64,
	/// The total space allocated to the user's team (bytes).
	pub allocated: u64,
}
