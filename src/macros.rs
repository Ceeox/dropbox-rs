// maybe check of valid uris
// for example files/create_folder_v2 is ok
// asdf/create_asdf_v2 should create and gen_uri Error
macro_rules! gen_uri {
	( $api_class:expr, $($api_func:expr),* ) =>
	({
		use crate::API_VERSION;
		use crate::BASE_URL;
		let mut func_calls = String::new();
		$(
			func_calls.push_str(&format!("/{}", $api_func));
		)*
		format!("{}{}/{}{}", BASE_URL, API_VERSION,
			$api_class, func_calls).parse::<hyper::Uri>()?
	});
}

macro_rules! gen_upload_uri {
	( $api_class:expr, $($api_func:expr),* ) =>({
		use crate::API_VERSION;
		use crate::BASE_URL;
		let mut func_calls = String::new();
		$(
			func_calls.push_str(&format!("/{}", $api_func));
		)*
		format!("{}{}/{}{}", UPLOAD_URL, API_VERSION,
			$api_class, func_calls).parse::<hyper::Uri>()?
	});
}

// TODO: Add documentaion for this two marcos and tests
#[inline]
macro_rules! check {
	( $result_class:ty, $error_class:ty, $body:ident ) => {{
		match serde_json::from_slice::<$result_class>(&$body) {
			Err(e) => {
				error!("Error in parsing: {:?}", e);
				match serde_json::from_slice::<Error<$error_class>>(&$body) {
					Err(_) => Err(DropboxError::Other(String::from_utf8($body.to_vec())?)),
					Ok(r) => Err(DropboxError::from(r)),
				}
				}
			Ok(r) => Ok(r),
			}
		}};
}

#[inline]
macro_rules! simple_check {
	( $result_class:ty, $body:ident ) => {{
		match serde_json::from_slice::<$result_class>(&$body) {
			Err(_e) => Err(DropboxError::Other(String::from_utf8($body.to_vec())?)),
			Ok(r) => Ok(r),
			}
		}};
}

#[cfg(test)]
mod tests {
	#[test]
	fn gen_uri() {
		let ref_1 = "https://api.dropboxapi.com/2/files/create_folder_v2";
		assert_eq!(ref_1, gen_uri!("files", "create_folder_v2"));

		let ref_2 = "https://api.dropboxapi.com/2/files/upload_session/finish_batch/check";
		assert_eq!(
			ref_2,
			gen_uri!("files", "upload_session", "finish_batch", "check")
		);
	}
}
