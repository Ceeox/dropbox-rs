// maybe check of valid uris
// for example files/create_folder_v2 is ok
// asdf/create_asdf_v2 should create and gen_uri Error
macro_rules! gen_uri {
	( $api_class:expr, $($api_func:expr),* ) =>
	({
		use BASE_URL;
		use API_VERSION;
		let mut func_calls = String::new();
		$(
			func_calls.push_str(&format!("/{}", $api_func));
		)*
		format!("{}{}/{}{}", BASE_URL, API_VERSION,
			$api_class, func_calls)
	});
}

macro_rules! gen_upload_uri
{
	( $api_class:expr, $($api_func:expr),* ) =>
	({
		use UPLOAD_URL;
		use API_VERSION;
		let mut func_calls = String::new();
		$(
			func_calls.push_str(&format!("/{}", $api_func));
		)*
		format!("{}{}/{}{}", UPLOAD_URL, API_VERSION,
			$api_class, func_calls)
	});
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
