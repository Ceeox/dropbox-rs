// License
/*
 * The MIT License
 * Copyright (c) 2017 souryo <dev.souryo@gmail.com>.
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
#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
#[macro_use] extern crate hyper;
extern crate hyper_native_tls;
#[macro_use] extern crate mime;
// extern mods
// intern mods
#[cfg(test)] mod tests;
pub mod models;
pub mod error;
#[macro_use] mod macros;
pub mod files;
// std uses
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
// crate uses
use hyper::Client;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use hyper::header::*;
// intern uses
use ::error::*;
// consts or statics
static BASE_URL: &str = "https://api.dropboxapi.com";
static UPLOAD_URL: &str = "https://content.dropboxapi.com";
static API_VERSION: &str = "/2";
static USER_AGENT: &str = concat!("dropbox-rs (https://github.com/souryo/dropbox-rs, ",
	env!("CARGO_PKG_VERSION"), ")");
// etc
pub struct Dropbox
{
	client: hyper::Client,
	token: String,
}

impl Dropbox
{
	pub fn new(token: String)
	-> Result<Self>
	{
		let ssl = NativeTlsClient::new().unwrap();
		let connector = HttpsConnector::new(ssl);
		let client = Client::with_connector(connector);

		Ok(Dropbox
		{
			client: client,
			token: token,
		})
	}

	fn send_request(&self, uri: &str, body: &str)
	-> Result<String>
	{
		let header = self.create_headers();
		debug!("{:?}", &header);
		let mut resp = self.client.post(uri)
			.headers(header)
			.body(body)
			.send()?;
		let mut body = String::new();
		resp.read_to_string(&mut body)?;
		trace!("{:?}", &body);
		Ok(body)
	}

	fn download(&self, uri: &str, arg: &str, file_path: &Path)
	-> Result<String>
	{
		let header = self.create_content_headers(&arg);
		debug!("{:?}", &header);
		let mut resp = self.client.post(uri)
			.headers(header)
			.send()?;
		let api_resp = match resp.headers.iter()
			.find(|i| i.name() == "dropbox-api-result")
			.map(|i| i.value_string())
		{
			None => return Err(DropboxError::MissingDropboxApiResult),
			Some(r) => r,
		};
		trace!("{:?}", &api_resp);
		let mut buffer = vec![];
		resp.read_to_end(&mut buffer)?;
		let mut file = File::create(file_path)?;
		file.write_all(&buffer)?;
		Ok(api_resp)
	}

	fn upload(&self, uri: &str, arg: &str, file_path: &Path)
	-> Result<String>
	{
		let mut file = File::open(file_path)?;
		let mut contents = String::new();
		file.read_to_string(&mut contents)?;
		let mut header = self.create_content_headers(&arg);
		header.set(ContentType(mime!(Application/OctetStream)));
		debug!("{:?}", &header);
		let mut resp = self.client.post(uri)
			.headers(header)
			.body(&contents)
			.send()?;
		let mut body = String::new();
		resp.read_to_string(&mut body)?;
		trace!("{:?}", &body);
		Ok(body)
	}

	fn create_headers(&self)
	-> Headers
	{
		let mut header = Headers::new();
		header.set(Authorization(Bearer { token: self.token.clone() }));
		header.set(UserAgent(USER_AGENT.to_owned()));
		header.set(ContentType::json());
		header
	}

	fn create_content_headers(&self, arg: &str)
	-> Headers
	{
		header!{ (DropboxApiArg, "Dropbox-API-Arg") => [String] };
		let mut header = Headers::new();
		header.set(Authorization(Bearer { token: self.token.clone() }));
		header.set(UserAgent(USER_AGENT.to_owned()));
		header.set(DropboxApiArg(arg.to_owned()));
		header
	}
}
