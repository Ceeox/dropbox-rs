use hyper::rt::*;
use serde_json;

use error::*;
use hyper::Method;
use models::error::*;
use models::users::*;
use DropboxContext;

#[derive(Clone)]
pub struct DropboxUsers {
	ctx: DropboxContext,
}

impl DropboxUsers {
	pub(crate) fn new(context: &DropboxContext) -> Self {
		Self {
			ctx: context.clone(),
		}
	}

	/// Get information about a user's account.
	pub fn get_account<I, E>(&self, arg: GetAccountArg) -> Result<impl Future<Item = I, Error = E>>
	where
		I: BasicAccount,
		E: DropboxError,
	{
		let uri = gen_uri!("users", "get_account");
		let body = serde_json::to_vec(&arg)?;
		let request = self.ctx.create_request(uri, Method::POST, Some(body));
		Ok(self
			.ctx
			.request(request)
			.and_then(|body| check!(BasicAccount, GetAccountError, body))
			.from_err::<DropboxError>())
	}

	/// Get information about multiple user accounts.
	/// At most 300 accounts may be queried per request.
	pub fn get_account_batch(
		&self,
		arg: GetAccountBatchArg,
	) -> Result<impl Future<Item = Vec<BasicAccount>, Error = DropboxError>> {
		let uri = gen_uri!("users", "get_account_batch");
		let body = serde_json::to_vec(&arg)?;
		let request = self.ctx.create_request(uri, Method::POST, Some(body));
		Ok(self
			.ctx
			.request(request)
			.and_then(|body| check!(Vec<BasicAccount>, GetAccountBatchError, body))
			.from_err::<DropboxError>())
	}

	/// Get information about the current user's account.
	pub fn get_current_account(
		&self,
	) -> Result<impl Future<Item = FullAccount, Error = DropboxError>> {
		let uri = gen_uri!("users", "get_current_account");
		let request = self.ctx.create_request(uri, Method::POST, None);
		Ok(self
			.ctx
			.request(request)
			.and_then(|body| simple_check!(FullAccount, body))
			.from_err::<DropboxError>())
	}

	/// Get the space usage information for the current user's account.
	pub fn get_space_usage(&self) -> Result<impl Future<Item = SpaceUsage, Error = DropboxError>> {
		let uri = gen_uri!("users", "get_space_usage");
		let request = self.ctx.create_request(uri, Method::POST, None);
		Ok(self
			.ctx
			.request(request)
			.and_then(|body| simple_check!(SpaceUsage, body))
			.from_err::<DropboxError>())
	}
}
