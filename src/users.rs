use hyper::rt::*;
use serde_json;

use std::path::Path;

use error::*;
use models::error::*;
use models::users::*;
use DropboxContext;

pub struct DropboxUsers {
	ctx: DropboxContext,
}

impl DropboxUsers {
	pub fn new(context: &DropboxContext) -> Self {
		Self {
			ctx: context.clone(),
		}
	}

	/// Get information about a user's account.
	pub fn get_account(
		&self,
		arg: GetAccountArg,
	) -> Result<impl Future<Item = BasicAccount, Error = DropboxError>> {
		let uri = gen_uri!("users", "get_account");
		let body: String = serde_json::to_string(&arg)?;
		Ok(self
			.ctx
			.send_request::<BasicAccount, GetAccountError>(&uri, body))
	}

	/// Get information about multiple user accounts. At most 300 accounts may be queried per request.
	pub fn get_account_batch(
		&self,
		arg: GetAccountBatchArg,
	) -> Result<impl Future<Item = BasicAccount, Error = DropboxError>> {
		let uri = gen_uri!("users", "get_account_batch");
		let body: String = serde_json::to_string(&arg)?;
		Ok(self
			.ctx
			.send_request::<BasicAccount, GetAccountError>(&uri, body))
	}

	/// Get information about the current user's account.
	pub fn get_current_account(&self) -> impl Future<Item = FullAccount, Error = DropboxError> {
		let uri = gen_uri!("users", "get_current_account");
		self.ctx
			.send_request::<FullAccount, ()>(&uri, String::new())
	}

	/// Get the space usage information for the current user's account.
	pub fn get_space_usage(&self) -> impl Future<Item = SpaceUsage, Error = DropboxError> {
		let uri = gen_uri!("users", "get_space_usage");
		self.ctx.send_request::<SpaceUsage, ()>(&uri, String::new())
	}
}
