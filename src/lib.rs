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
extern crate hyper;
extern crate hyper_tls;
#[macro_use]
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;
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
use hyper::client::HttpConnector;
use hyper::rt::{Future, Stream};
use hyper::{Body, Client, Method, Request, Uri};
use hyper_tls::HttpsConnector;

// intern uses
use crate::error::*;
//use files::DropboxFiles;
use crate::users::DropboxUsers;

// consts or statics
static BASE_URL: &str = "https://api.dropboxapi.com";
static CONTENT_URL: &str = "https://content.dropboxapi.com";
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
		let https = HttpsConnector::new(4)?;
		// keep_alive must be set or else the tokio_runtime runs infinitly
		let client = Client::builder().keep_alive(false).build(https);
		let token = Arc::new(token.to_owned());
		Ok(DropboxContext { client, token })
	}

	fn create_request(&self, uri: Uri, body: Option<Vec<u8>>) -> Request<Body> {
		let mut request = Request::builder();
		request
			.uri(uri)
			.method(Method::POST)
			.header("User-Agent", USER_AGENT)
			.header("Authorization", format!("Bearer {}", self.token.clone()));

		match body {
			Some(r) => request
				.header("Content-Type", "application/json")
				.body(Body::from(r))
				.unwrap(),
			None => request.body(Body::empty()).unwrap(),
		}
	}

	fn create_download_request(&self, uri: Uri, download_arg: String) -> Request<Body> {
		let mut request = Request::builder();
		request
			.uri(uri)
			.method(Method::POST)
			.header("User-Agent", USER_AGENT)
			.header("Authorization", format!("Bearer {}", self.token.clone()))
			.header("Dropbox-API-Arg", download_arg)
			.body(Body::empty())
			.expect("Failed to create download request")
	}

	fn create_upload_request(&self, uri: Uri, upload_arg: String, body: Vec<u8>) -> Request<Body> {
		let mut request = Request::builder();
		request
			.uri(uri)
			.method(Method::POST)
			.header("User-Agent", USER_AGENT)
			.header("Authorization", format!("Bearer {}", self.token.clone()))
			.header("Dropbox-API-Arg", upload_arg)
			.header("Content-Type", "application/octet-stream")
			.body(Body::from(body))
			.expect("Failed to create upload request")
	}

	#[inline]
	fn change_token(&mut self, new_token: String) {
		self.token = Arc::new(new_token);
	}

	fn request(
		&self,
		request: Request<Body>,
	) -> impl Future<Item = hyper::Chunk, Error = DropboxError> {
		trace!("Request: {:?}", request);
		self.client
			.request(request)
			.and_then(|res| res.into_body().concat2())
			.from_err::<DropboxError>()
	}

	fn download(
		&self,
		request: Request<Body>,
	//) -> impl Future<Item = Box<(String, hyper::Chunk)>, Error = DropboxError> {
	) -> Result<()> {
	    unimplemented!();
		/*
		trace!("Request: {:?}", request);
		self.client
			.request(request)
			.and_then(|res| {
				let res_info = res
					.headers()
					.remove("dropbox-api-result")
					.unwrap()
					.to_str()
					.unwrap()
					.to_owned();
				let body = res.into_body().concat2();
				Ok((res_info, body))
			}).from_err::<DropboxError>()
		*/
	}

	fn upload(&self, _uri: &str, _arg: &str, _file_path: &Path) -> Result<String> {
		unimplemented!();
	}
}

impl Clone for DropboxContext {
	fn clone(&self) -> Self {
		Self {
			client: self.client.clone(),
			token: Arc::clone(&self.token),
		}
	}
}

#[derive(Clone)]
pub struct Dropbox {
	context: DropboxContext,
	//files: Option<DropboxFiles>,
	users: Option<DropboxUsers>,
}

impl Dropbox {
	pub fn new(token: &str) -> Result<Dropbox> {
		let context = DropboxContext::new(token)?;
		Ok(Dropbox {
			context,
			//files: None,
			users: None,
		})
	}

	#[inline]
	pub fn change_token(&mut self, new_token: String) {
		self.context.change_token(new_token)
	}

	#[inline]
	pub fn users(&mut self) -> &DropboxUsers {
		match self.users {
			Some(ref users) => users,
			None => {
				self.users = Some(DropboxUsers::new(&self.context));
				self.users.as_ref().unwrap()
			}
		}
	}
}
