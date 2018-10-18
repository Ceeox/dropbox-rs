// License
/*
 * The MIT License
 * Copyright (c) 2018 Ceeox <ceox4510471@gmail.com>.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
 * THE SOFTWARE.
 */

// extern crates
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
extern crate hyper;
extern crate hyper_tls;
extern crate serde;
extern crate serde_json;

// intern mods
#[macro_use]
mod macros;

pub mod error;
// #[cfg(test)]
// mod tests;
//pub mod files;
pub mod models;
pub mod users;

// std uses
use std::path::Path;
use std::sync::Arc;
// crate uses
use hyper::body::Body;
use hyper::client::HttpConnector;
use hyper::rt::{Future, Stream};
use hyper::{Client, Method, Request};
use hyper_tls::HttpsConnector;
use serde::{Deserialize, Serialize};

// intern uses
use error::*;
//use files::DropboxFiles;
use models::error::Error;
use users::DropboxUsers;

// consts or statics
static BASE_URL: &str = "https://api.dropboxapi.com";
static UPLOAD_URL: &str = "https://content.dropboxapi.com";
static API_VERSION: &str = "/2";
static USER_AGENT: &str = concat!(
	"dropbox-rs (https://github.com/Ceeox/dropbox-rs), ",
	env!("CARGO_PKG_VERSION")
);

pub struct DropboxContext {
	client: hyper::Client<HttpsConnector<HttpConnector>, Body>,
	token: Arc<String>,
}

impl DropboxContext {
	/// Creates a context this holds the http client and the token
	pub fn new(token: &str) -> Result<Self> {
		let https = HttpsConnector::new(2)?;
		let client = Client::builder().build(https);
		let token = Arc::new(token.to_owned());
		Ok(DropboxContext { client, token })
	}

	pub(crate) fn client(&self) -> &Client<HttpsConnector<HttpConnector>, Body> {
		&self.client
	}

	pub(crate) fn chnage_token(&mut self, new_token: &str) {
		self.token = Arc::new(new_token.to_owned());
	}

	fn send_request<'de, I: Serialize, T: Deserialize<'de>, E: Deserialize<'de>>(
		&self,
		uri: &str,
		body: String,
	) -> Result<impl Future<Item = T, Error = DropboxError>>
	where
		error::DropboxError: std::convert::From<models::error::Error<E>>,
	{
		let request = Request::builder()
			.uri(uri)
			.method(Method::POST)
			.header("User-Agent", "application/json")
			.header("Authorization", format!("Bearer {}", &*self.token.clone()))
			.body(Body::from(body))
			.unwrap();

		self.client
			.request(request)
			.and_then(|res| res.into_body().concat2())
			.from_err::<DropboxError>()
			.and_then(|body| match serde_json::from_slice::<T>(body) {
				Err(e1) => {
					error!("Error in prasing the json response: {}", e1);
					Err(match serde_json::from_slice::<Error<E>>(body) {
						Err(e2) => {
							error!("Error in prasing the json error response: {}", e2);
							DropboxError::Other
						}
						Ok(r) => DropboxError::from(r),
					})
				}
				Ok(r) => Ok(r),
			})
	}

	fn download(&self, uri: &str, arg: &str, file_path: &Path) -> Result<String> {
		Err(DropboxError::Other)
	}

	fn upload(&self, uri: &str, arg: &str, file_path: &Path) -> Result<String> {
		Err(DropboxError::Other)
	}
}

impl Clone for DropboxContext {
	fn clone(&self) -> Self {
		DropboxContext {
			client: self.client.clone(),
			token: Arc::clone(&self.token),
		}
	}
}

pub struct Dropbox {
	context: DropboxContext,
	//pub files: DropboxFiles,
	pub users: DropboxUsers,
}

impl Dropbox {
	pub fn new(token: &str) -> Result<Dropbox> {
		let context = DropboxContext::new(token)?;
		//let files = DropboxFiles::new(&context);
		let users = DropboxUsers::new(&context);
		Ok(Dropbox {
			context,
			//files,
			users,
		})
	}
}
/*
impl Dropbox {
	fn create_headers(&self) -> Headers {
		let mut header = Headers::new();
		header.set(Authorization(Bearer {
			token: self.token.clone(),
		}));
		header.set(UserAgent(USER_AGENT.to_owned()));
		header.set(ContentType::json());
		header
	}

	fn create_content_headers(&self, arg: &str) -> Headers {
		header!{ (DropboxApiArg, "Dropbox-API-Arg") => [String] };
		let mut header = Headers::new();
		header.set(Authorization(Bearer {
			token: self.token.clone(),
		}));
		header.set(UserAgent(USER_AGENT.to_owned()));
		header.set(DropboxApiArg(arg.to_owned()));
		header
	}
}
*/
