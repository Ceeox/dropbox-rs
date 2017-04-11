use std::io::Read;

use hyper;
use hyper::Client;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use hyper::header::*;

use ::error::*;

pub struct DropboxConnection
{
	client: hyper::Client,
	header: hyper::header::Headers,
}

pub enum ConnState
{
	SendMessage(String, String),
	RevMessage(String),
	Pong,
	Waiting,
}

impl DropboxConnection
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

		Ok(DropboxConnection
		{
			client: client,
			header: h,
		})
	}

	pub fn send_request(&self, uri: String, body: String)
	-> Result<String>
	{
		debug!("uri: {:?}, {:?}", &uri, &body);
		let header = self.header.clone();
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
