#[macro_export]
macro_rules! gen_uri
{
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

#[macro_export]
macro_rules! build_object {
	() => ();
}
