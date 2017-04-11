#[cfg(test)]
mod tests
{
	use std::io;
	use std::io::prelude::*;
	use std::fs::File;

	use serde_json;
	use ::models::files::*;

	#[test]
	fn files_list_folder_arg()
	{
		let arg = ListFolderArg
		{
			path: "/Homework/math".to_string(),
			recursive: false,
			include_media_info: false,
			include_deleted: false,
			include_has_explicit_shared_members: false,
		};
		let mut file = File::open("tests_json/files/list_folder_arg.json").unwrap();
		let file: ListFolderArg = serde_json::from_reader(file).unwrap();
		assert!(arg == file)
	}

	// TODO
	#[test]
	fn files_list_folder_ret()
	{
		let arg = ListFolderResult
		{
			entries: vec![
				File(FileMetadata {
					name: "Prime_Numbers.txt".to_string(),
					id: "id:a4ayc_80_OEAAAAAAAAAXw".to_string(),
					client_modified: "2015-05-12T15:50:38Z".to_string(),
					server_modified: "2015-05-12T15:50:38Z".to_string(),
					rev: "a1c10ce0dd78".to_string(),
					size: 7212,
					path_lower: "/homework/math/prime_numbers.txt".to_string(),
					path_display: "/Homework/math/Prime_Numbers.txt".to_string(),
					sharing_info: {
						read_only: true,
						parent_shared_folder_id: 84528192421,
						modified_by: "dbid:AAH4f99T0taONIb-OurWxbNQ6ywGRopQngc".to_string(),
					},
					property_groups: vec![{
						template_id: "ptid:1a5n2i6d3OYEAAAAAAAAAYa".to_string(),
						fields: vec![{
							name: "Security Policy".to_string(),
							value: "Confidential".to_string(),
						}]
					}],
					has_explicit_shared_members: false,
					content_hash: "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855".to_string(),
				}),
				Folder(FolderMetaData {
					name: "math".to_string(),
					id: "id:a4ayc_80_OEAAAAAAAAAXz".to_string(),
					path_lower: "/homework/math".to_string(),
					path_display: "/Homework/math".to_string(),
					sharing_info: {
						read_only: false,
						parent_shared_folder_id: 84528192421,
						traverse_only: false,
						no_access: false,
					},
					property_groups: [{
						template_id: "ptid:1a5n2i6d3OYEAAAAAAAAAYa".to_string(),
						fields: [{
							name: "Security Policy".to_string(),
							value: "Confidential".to_string(),
						}]
					}]
				})],
			cursor: "ZtkX9_EHj3x7PMkVuFIhwKYXEpwpLwyxp9vMKomUhllil9q7eWiAu".to_string(),
			has_more: false
		};
		let mut file = File::open("tests_json/files/list_folder_arg.json").unwrap();
		let file: ListFolderArg = serde_json::from_reader(file).unwrap();
		assert!(arg == file)
	}
}
