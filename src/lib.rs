// License
/*
 *The MIT License
 *Copyright (c) 2017 souryo <dev.souryo@gmail.com>.
 *
 *Permission is hereby granted, free of charge, to any person obtaining a copy
 *of this software and associated documentation files (the "Software"), to deal
 *in the Software without restriction, including without limitation the rights
 *to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 *copies of the Software, and to permit persons to whom the Software is
 *furnished to do so, subject to the following conditions:
 *
 *The above copyright notice and this permission notice shall be included in
 *all copies or substantial portions of the Software.
 *
 *THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 *IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 *FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 *AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 *LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 *OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
 *THE SOFTWARE.
 */
// extern crates
#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate hyper;
extern crate hyper_native_tls;
// extern mods
// intern mods
mod tests;
pub mod models;
pub mod error;
#[macro_use] mod macros;
pub mod files;
// std uses
use std::io::Read;
// crate uses
use hyper::Client;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use hyper::header::*;
// intern uses
use ::error::*;
// consts or statics
static BASE_URL: &'static str = "https://api.dropboxapi.com";
static UPLOAD_URL: &'static str = "https://content.dropboxapi.com";
static API_VERSION: &'static str = "/2";
// etc
pub struct Dropbox
{
	client: hyper::Client,
	header: hyper::header::Headers,
}

impl Dropbox
{
	pub fn new(token: String)
	-> Result<Self>
	{
		let mut h = Headers::new();
		h.set(Authorization(Bearer { token: token }));
		h.set(ContentType::json());

		let ssl = NativeTlsClient::new().unwrap();
		let connector = HttpsConnector::new(ssl);
		let client = Client::with_connector(connector);

		Ok(Dropbox
		{
			client: client,
			header: h,
		})
	}

	fn send_request(&self, uri: String, body: String)
	-> Result<String>
	{
		debug!("uri: {:?}, {:?}", &uri, &body);
		let header = self.header.clone();
		info!("{:?}", header);
		let mut resp = self.client.post(&uri)
			.headers(header)
			.body(&body)
			.send()?;
		trace!("{:?}", &resp);
		let mut body = String::new();
		resp.read_to_string(&mut body)?;
		Ok(body)
	}
}
