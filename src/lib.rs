// License

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
