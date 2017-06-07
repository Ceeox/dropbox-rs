use serde_json;

use ::error::*;
use ::Dropbox;
use ::models::users::*;
use ::models::error::*;

pub struct DropboxUsers<'a>
{
	dropbox: &'a Dropbox,
}

impl<'a> DropboxUsers<'a>
{
	pub fn new(dropbox: &'a Dropbox)
	-> DropboxUsers<'a>
	{
		DropboxUsers
		{
			dropbox: dropbox,
		}
	}

	/// Get information about a user's account.
	pub fn get_account(&self, arg: GetAccountArg)
	-> Result<BasicAccount>
	{
		let uri = gen_uri!("users", "get_account");
		let body: String = serde_json::to_string(&arg)?;
		let resp: String = self.dropbox.send_request(&uri, &body)?;
		match serde_json::from_str::<BasicAccount>(&resp)
		{
			Err(_) => Err(match serde_json::from_str::<Error<GetAccountError>>(&resp)
			{
				Err(_) => DropboxError::Other,
				Ok(r) => DropboxError::GetAccountError(r),
			}),
			Ok(r) => Ok(r),
		}
	}

	/// Get information about multiple user accounts. At most 300 accounts may be queried per request.
	pub fn get_account_batch(&self, arg: GetAccountBatchArg)
	-> Result<Vec<BasicAccount>>
	{
		let uri = gen_uri!("users", "get_account_batch");
		let body: String = serde_json::to_string(&arg)?;
		let resp: String = self.dropbox.send_request(&uri, &body)?;
		match serde_json::from_str::<Vec<BasicAccount>>(&resp)
		{
			Err(_) => Err(match serde_json::from_str::<Error<GetAccountError>>(&resp)
			{
				Err(_) => DropboxError::Other,
				Ok(r) => DropboxError::GetAccountError(r),
			}),
			Ok(r) => Ok(r),
		}
	}

	/// Get information about the current user's account.
	pub fn get_current_account(&self)
	-> Result<FullAccount>
	{
		let uri = gen_uri!("users", "get_current_account");
		let resp: String = self.dropbox.send_request(&uri, "")?;
		match serde_json::from_str::<FullAccount>(&resp)
		{
			Err(_) => Err(DropboxError::Other),
			Ok(r) => Ok(r),
		}
	}

	/// Get the space usage information for the current user's account.
	pub fn get_space_usage(&self)
	-> Result<SpaceUsage>
	{
		let uri = gen_uri!("users", "get_space_usage");
		let resp: String = self.dropbox.send_request(&uri, "")?;
		match serde_json::from_str::<SpaceUsage>(&resp)
		{
			Err(_) => Err(DropboxError::Other),
			Ok(r) => Ok(r),
		}
	}
}
