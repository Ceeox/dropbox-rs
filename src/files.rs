use serde_json;

use ::error::*;
use ::Dropbox;
use ::models::files::*;
use ::models::error::*;

pub struct DropboxFiles<'a>
{
	conn: &'a Dropbox,
}

impl<'a> DropboxFiles<'a>
{
	pub fn new(conn: &'a Dropbox)
	-> Result<DropboxFiles<'a>>
	{
		Ok(DropboxFiles
		{
			conn: conn,
		})
	}

	/// PREVIEW - may change or disappear without notice
	pub fn alpha_get_metadata()
	-> Result<()>
	{
		Err(DropboxError::Other)
	}

	/// PREVIEW - may change or disappear without notice
	pub fn alpha_upload()
	-> Result<()>
	{
		Err(DropboxError::Other)
	}

	pub fn copy(&self, arg: RelocationArg)
	-> Result<Metadata>
	{
		let uri = gen_uri!("files", "copy");
		let body: String = serde_json::to_string(&arg)?;
		let resp: String = self.conn.send_request(uri, body)?;
		match serde_json::from_str::<Metadata>(&resp)
		{
			Err(_) => Err(match serde_json::from_str::<Error<RelocationError>>(&resp)
			{
				Err(_) => DropboxError::Other,
				Ok(r) => DropboxError::RelocationError(r),
			}),
			Ok(r) => Ok(r),
		}
	}

	pub fn copy_batch(&self, arg: RelocationBatchArg)
	-> Result<RelocationBatchLaunch>
	{
		let uri = gen_uri!("files", "copy_batch");
		let body: String = serde_json::to_string(&arg)?;
		let resp: String = self.conn.send_request(uri, body)?;
		match serde_json::from_str::<RelocationBatchLaunch>(&resp)
		{
			Err(_) => Err(DropboxError::Other),
			Ok(r) => Ok(r),
		}
	}

	pub fn copy_batch_check(&self, arg: PollArg)
	-> Result<RelocationBatchJobStatus>
	{
		let uri = gen_uri!("files", "copy");
		let body: String = serde_json::to_string(&arg)?;
		let resp: String = self.conn.send_request(uri, body)?;
		match serde_json::from_str::<RelocationBatchJobStatus>(&resp)
		{
			Err(_) => Err(match serde_json::from_str::<Error<PollError>>(&resp)
			{
				Err(_) => DropboxError::Other,
				Ok(r) => DropboxError::PollError(r),
			}),
			Ok(r) => Ok(r),
		}
	}

	pub fn copy_reference_get(&self, arg: GetCopyReferenceArg)
	-> Result<GetCopyReferenceResult>
	{
		let uri = gen_uri!("files", "copy_reference", "get");
		let body: String = serde_json::to_string(&arg)?;
		let resp: String = self.conn.send_request(uri, body)?;
		match serde_json::from_str::<GetCopyReferenceResult>(&resp)
		{
			Err(_) => Err(match serde_json::from_str::<Error<GetCopyReferenceError>>(&resp)
			{
				Err(_) => DropboxError::Other,
				Ok(r) => DropboxError::GetCopyReferenceError(r),
			}),
			Ok(r) => Ok(r),
		}
	}

	pub fn copy_reference_save(&self, arg: SaveCopyReferenceArg)
	-> Result<SaveCopyReferenceResult>
	{
		let uri = gen_uri!("files", "copy_reference", "save");
		let body: String = serde_json::to_string(&arg)?;
		let resp: String = self.conn.send_request(uri, body)?;
		match serde_json::from_str::<SaveCopyReferenceResult>(&resp)
		{
			Err(_) => Err(match serde_json::from_str::<Error<SaveCopyReferenceError>>(&resp)
			{
				Err(_) => DropboxError::Other,
				Ok(r) => DropboxError::SaveCopyReferenceError(r),
			}),
			Ok(r) => Ok(r),
		}
	}

	pub fn create_folder(&self, arg: CreateFolderArg)
	-> Result<FolderMetadata>
	{
		let uri = gen_uri!("files", "create_folder");
		let body: String = serde_json::to_string(&arg)?;
		let resp: String = self.conn.send_request(uri, body)?;
		match serde_json::from_str::<FolderMetadata>(&resp)
		{
			Err(_) => Err(match serde_json::from_str::<Error<CreateFolderError>>(&resp)
			{
				Err(_) => DropboxError::Other,
				Ok(r) => DropboxError::CreateFolderError(r),
			}),
			Ok(r) => Ok(r),
		}
	}

	pub fn delete(&self, arg: DeleteArg)
	-> Result<Metadata>
	{
		let uri = gen_uri!("files", "delete");
		let body: String = serde_json::to_string(&arg)?;
		let resp: String = self.conn.send_request(uri, body)?;
		match serde_json::from_str::<Metadata>(&resp)
		{
			Err(_) => Err(match serde_json::from_str::<Error<DeleteError>>(&resp)
			{
				Err(_) => DropboxError::Other,
				Ok(r) => DropboxError::DeleteError(r),
			}),
			Ok(r) => Ok(r),
		}
	}

	pub fn delete_batch(&self, arg: DeleteBatchArg)
	-> Result<DeleteBatchLaunch>
	{
		let uri = gen_uri!("files", "delete_batch");
		let body: String = serde_json::to_string(&arg)?;
		let resp: String = self.conn.send_request(uri, body)?;
		match serde_json::from_str::<DeleteBatchLaunch>(&resp)
		{
			Err(_) => Err(DropboxError::Other),
			Ok(r) => Ok(r),
		}
	}

	pub fn delete_batch_check()
	{
		unimplemented!();
	}

	pub fn download()
	{
		unimplemented!();
	}

	pub fn get_metadata()
	{
		unimplemented!();
	}

	pub fn get_preview()
	{
		unimplemented!();
	}

	pub fn get_temporary_link()
	{
		unimplemented!();
	}

	pub fn get_thumbnail()
	{
		unimplemented!();
	}

	pub fn list_folder(&self, arg: ListFolderArg)
	-> Result<ListFolderResult>
	{
		let uri = gen_uri!("files", "list_folder");
		let body: String = serde_json::to_string(&arg)?;
		let resp: String = self.conn.send_request(uri, body)?;
		match serde_json::from_str::<ListFolderResult>(&resp)
		{
			Err(_) => Err(match serde_json::from_str::<Error<ListFolderError>>(&resp)
			{
				Err(_) => DropboxError::Other,
				Ok(r) => DropboxError::ListFolderError(r),
			}),
			Ok(r) => Ok(r),
		}
	}

	pub fn list_folders_continue()
	{
		unimplemented!();
	}

	pub fn list_folder_get_lates_cursor()
	{
		unimplemented!();
	}

	pub fn list_folder_longpoll()
	{
		unimplemented!();
	}

	pub fn list_revisions()
	{
		unimplemented!();
	}

	pub fn _move()
	{
		unimplemented!();
	}

	pub fn move_batch()
	{
		unimplemented!();
	}

	pub fn move_batch_check()
	{
		unimplemented!();
	}

	pub fn permanetly_delete()
	{
		unimplemented!();
	}

	pub fn properties_add()
	{
		unimplemented!();
	}

	pub fn properties_overwride()
	{
		unimplemented!();
	}

	pub fn properties_remove()
	{
		unimplemented!();
	}

	pub fn properties_template_get()
	{
		unimplemented!();
	}

	pub fn properties_template_list()
	{
		unimplemented!();
	}

	pub fn properties_update()
	{
		unimplemented!();
	}

	pub fn restore()
	{
		unimplemented!();
	}

	pub fn save_url()
	{
		unimplemented!();
	}

	pub fn save_url_check_jobstatus()
	{
		unimplemented!();
	}

	pub fn search()
	{
		unimplemented!();
	}

	pub fn _upload()
	{
		unimplemented!();
	}

	pub fn upload_session_append()
	{
		unimplemented!();
	}

	pub fn upload_session_apped_v2()
	{
		unimplemented!();
	}

	pub fn upload_session_finish()
	{
		unimplemented!();
	}

	pub fn upload_session_finish_batch()
	{
		unimplemented!();
	}

	pub fn upload_session_finish_batch_check()
	{
		unimplemented!();
	}

	pub fn upload_session_start()
	{
		unimplemented!();
	}
}
