use std::path::Path;

use serde_json;

use ::error::*;
use ::Dropbox;
use ::models::files::*;
use ::models::error::*;

pub struct DropboxFiles<'a>
{
	dropbox: &'a Dropbox,
}

impl<'a> DropboxFiles<'a>
{
	pub fn new(dropbox: &'a Dropbox)
	-> Result<DropboxFiles<'a>>
	{
		Ok(DropboxFiles
		{
			dropbox: dropbox,
		})
	}

	/// PREVIEW - may change or disappear without notice
	pub fn alpha_get_metadata(&self)
	-> Result<()>
	{
		Err(DropboxError::Other)
	}

	/// PREVIEW - may change or disappear without notice
	pub fn alpha_upload(&self)
	-> Result<()>
	{
		Err(DropboxError::Other)
	}

	pub fn copy(&self, arg: RelocationArg)
	-> Result<Metadata>
	{
		let uri = gen_uri!("files", "copy");
		let body: String = serde_json::to_string(&arg)?;
		let resp: String = self.dropbox.send_request(&uri, &body)?;
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
		let resp: String = self.dropbox.send_request(&uri, &body)?;
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
		let resp: String = self.dropbox.send_request(&uri, &body)?;
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
		let resp: String = self.dropbox.send_request(&uri, &body)?;
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
		let resp: String = self.dropbox.send_request(&uri, &body)?;
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
		let resp: String = self.dropbox.send_request(&uri, &body)?;
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
		let resp: String = self.dropbox.send_request(&uri, &body)?;
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
		let resp: String = self.dropbox.send_request(&uri, &body)?;
		match serde_json::from_str::<DeleteBatchLaunch>(&resp)
		{
			Err(_) => Err(DropboxError::Other),
			Ok(r) => Ok(r),
		}
	}

	pub fn delete_batch_check(&self, arg: PollArg)
	-> Result<DeleteBatchJobStatus>
	{
		let uri = gen_uri!("files", "delete_batch", "check");
		let body: String = serde_json::to_string(&arg)?;
		let resp: String = self.dropbox.send_request(&uri, &body)?;
		match serde_json::from_str::<DeleteBatchJobStatus>(&resp)
		{
			Err(_) => Err(match serde_json::from_str::<Error<PollError>>(&resp)
			{
				Err(_) => DropboxError::Other,
				Ok(r) => DropboxError::PollError(r),
			}),
			Ok(r) => Ok(r),
		}
	}

	pub fn download(&self, arg: DownloadArg, file_path: &Path)
	-> Result<FileMetadata>
	{
		let uri = gen_upload_uri!("files", "download");
		let body: String = serde_json::to_string(&arg)?;
		let file_info = self.dropbox.download(&uri, &body, &file_path)?;
		let file_info: FileMetadata = match serde_json::from_str::<FileMetadata>(&file_info)
		{
			Err(_) => return Err(match serde_json::from_str::<Error<DownloadError>>(&file_info)
			{
				Err(_) => DropboxError::Other,
				Ok(r) => DropboxError::DownloadError(r),
			}),
			Ok(r) => r,
		};
		Ok(file_info)
	}

	pub fn get_metadata(&self, arg: GetMetadataArg)
	-> Result<Metadata>
	{
		let uri = gen_uri!("files", "get_metadata");
		let body: String = serde_json::to_string(&arg)?;
		let resp: String = self.dropbox.send_request(&uri, &body)?;
		match serde_json::from_str::<Metadata>(&resp)
		{
			Err(_) => Err(match serde_json::from_str::<Error<GetMetadataError>>(&resp)
			{
				Err(_) => DropboxError::Other,
				Ok(r) => DropboxError::GetMetadataError(r),
			}),
			Ok(r) => Ok(r),
		}
	}

	pub fn get_preview(&self, arg:  PreviewArg, file_path: &Path)
	-> Result<FileMetadata>
	{
		let uri = gen_upload_uri!("files", "get_preview");
		let body: String = serde_json::to_string(&arg)?;
		let file_info = self.dropbox.download(&uri, &body, file_path)?;
		let file_info: FileMetadata = match serde_json::from_str::<FileMetadata>(&file_info)
		{
			Err(_) => return Err(match serde_json::from_str::<Error<PreviewError>>(&file_info)
			{
				Err(_) => DropboxError::Other,
				Ok(r) => DropboxError::PreviewError(r),
			}),
			Ok(r) => r,
		};
		Ok(file_info)
	}

	pub fn get_temporary_link(&self, arg: GetTemporaryLinkArg)
	-> Result<GetTemporaryLinkResult>
	{
		let uri = gen_uri!("files", "get_temporary_link");
		let body: String = serde_json::to_string(&arg)?;
		let resp: String = self.dropbox.send_request(&uri, &body)?;
		match serde_json::from_str::<GetTemporaryLinkResult>(&resp)
		{
			Err(_) => Err(match serde_json::from_str::<Error<GetTemporaryLinkError>>(&resp)
			{
				Err(_) => DropboxError::Other,
				Ok(r) => DropboxError::GetTemporaryLinkError(r),
			}),
			Ok(r) => Ok(r),
		}
	}

	pub fn get_thumbnail(&self, arg: ThumbnailArg, file_path: &Path)
	-> Result<FileMetadata>
	{
		let uri = gen_upload_uri!("files", "get_thumbnail");
		let body: String = serde_json::to_string(&arg)?;
		let file_info = self.dropbox.download(&uri, &body, file_path)?;
		let file_info: FileMetadata = match serde_json::from_str::<FileMetadata>(&file_info)
		{
			Err(_) => return Err(match serde_json::from_str::<Error<ThumbnailError>>(&file_info)
			{
				Err(_) => DropboxError::Other,
				Ok(r) => DropboxError::ThumbnailError(r),
			}),
			Ok(r) => r,
		};
		Ok(file_info)
	}

	pub fn list_folder(&self, arg: ListFolderArg)
	-> Result<ListFolderResult>
	{
		let uri = gen_uri!("files", "list_folder");
		let body: String = serde_json::to_string(&arg)?;
		let resp: String = self.dropbox.send_request(&uri, &body)?;
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

	pub fn list_folders_continue(&self, arg: ListFolderContinueArg)
	-> Result<ListFolderResult>
	{
		let uri = gen_uri!("files", "list_folders", "continue");
		let body: String = serde_json::to_string(&arg)?;
		let resp: String = self.dropbox.send_request(&uri, &body)?;
		match serde_json::from_str::<ListFolderResult>(&resp)
		{
			Err(_) => Err(match serde_json::from_str::<Error<ListFolderContinueError>>(&resp)
			{
				Err(_) => DropboxError::Other,
				Ok(r) => DropboxError::ListFolderContinueError(r),
			}),
			Ok(r) => Ok(r),
		}
	}

	pub fn list_folder_get_latest_cursor(&self, arg: ListFolderArg)
	-> Result<ListFolderGetLatestCursorResult>
	{
		let uri = gen_uri!("files", "list_folders", "get_latest_cursor");
		let body: String = serde_json::to_string(&arg)?;
		let resp: String = self.dropbox.send_request(&uri, &body)?;
		match serde_json::from_str::<ListFolderGetLatestCursorResult>(&resp)
		{
			Err(_) => Err(match serde_json::from_str::<Error<ListFolderError>>(&resp)
			{
				Err(_) => DropboxError::Other,
				Ok(r) => DropboxError::ListFolderError(r),
			}),
			Ok(r) => Ok(r),
		}
	}

	pub fn list_folder_longpoll(&self, arg: ListFolderLongpollArg)
	-> Result<ListFolderLongpollResult>
	{
		let uri = gen_uri!("files", "list_folders", "longpoll");
		let body: String = serde_json::to_string(&arg)?;
		let resp: String = self.dropbox.send_request(&uri, &body)?;
		match serde_json::from_str::<ListFolderLongpollResult>(&resp)
		{
			Err(_) => Err(match serde_json::from_str::<Error<ListFolderLongpollError>>(&resp)
			{
				Err(_) => DropboxError::Other,
				Ok(r) => DropboxError::ListFolderLongpollError(r),
			}),
			Ok(r) => Ok(r),
		}
	}

	pub fn list_revisions(&self, arg: ListRevisionsArg)
	-> Result<ListRevisionsResult>
	{
		let uri = gen_uri!("files", "list_revisions");
		let body: String = serde_json::to_string(&arg)?;
		let resp: String = self.dropbox.send_request(&uri, &body)?;
		match serde_json::from_str::<ListRevisionsResult>(&resp)
		{
			Err(_) => Err(match serde_json::from_str::<Error<ListRevisionsError>>(&resp)
			{
				Err(_) => DropboxError::Other,
				Ok(r) => DropboxError::ListRevisionsError(r),
			}),
			Ok(r) => Ok(r),
		}
	}

	#[link_name="move"]
	pub fn _move(&self, arg: RelocationArg)
	-> Result<Metadata>
	{
		let uri = gen_uri!("files", "move");
		let body: String = serde_json::to_string(&arg)?;
		let resp: String = self.dropbox.send_request(&uri, &body)?;
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

	pub fn move_batch(&self, arg: RelocationBatchArg)
	-> Result<RelocationBatchLaunch>
	{
		let uri = gen_uri!("files", "move_batch");
		let body: String = serde_json::to_string(&arg)?;
		let resp: String = self.dropbox.send_request(&uri, &body)?;
		match serde_json::from_str::<RelocationBatchLaunch>(&resp)
		{
			Err(_) => Err(DropboxError::Other),
			Ok(r) => Ok(r),
		}
	}

	pub fn move_batch_check(&self, arg: PollArg)
	-> Result<RelocationBatchJobStatus>
	{
		let uri = gen_uri!("files", "move_batch", "check");
		let body: String = serde_json::to_string(&arg)?;
		let resp: String = self.dropbox.send_request(&uri, &body)?;
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

	pub fn permanetly_delete(&self, arg: DeleteArg)
	-> Result<()>
	{
		let uri = gen_uri!("files", "permanetly_delete");
		let body: String = serde_json::to_string(&arg)?;
		let resp: String = self.dropbox.send_request(&uri, &body)?;
		match serde_json::from_str::<Error<DeleteError>>(&resp)
		{
			Err(e) => match e.is_eof()
			{
				false => Err(DropboxError::Other),
				true => Ok(()),
			},
			Ok(r) => Err(DropboxError::DeleteError(r)),
		}
	}

	// PREVIEW - may change or disappear without notice
	pub fn properties_add(&self)
	-> Result<()>
	{
		Err(DropboxError::Other)
	}

	// PREVIEW - may change or disappear without notice
	pub fn properties_overwride(&self)
	-> Result<()>
	{
		Err(DropboxError::Other)
	}

	// PREVIEW - may change or disappear without notice
	pub fn properties_remove(&self)
	-> Result<()>
	{
		Err(DropboxError::Other)
	}

	// PREVIEW - may change or disappear without notice
	pub fn properties_template_get(&self)
	-> Result<()>
	{
		Err(DropboxError::Other)
	}

	// PREVIEW - may change or disappear without notice
	pub fn properties_template_list(&self)
	-> Result<()>
	{
		Err(DropboxError::Other)
	}

	// PREVIEW - may change or disappear without notice
	pub fn properties_update(&self)
	-> Result<()>
	{
		Err(DropboxError::Other)
	}

	pub fn restore(&self, arg: RestoreArg)
	-> Result<FileMetadata>
	{
		let uri = gen_uri!("files", "restore");
		let body: String = serde_json::to_string(&arg)?;
		let resp: String = self.dropbox.send_request(&uri, &body)?;
		match serde_json::from_str::<FileMetadata>(&resp)
		{
			Err(_) => Err(match serde_json::from_str::<Error<RestoreError>>(&resp)
			{
				Err(_) => DropboxError::Other,
				Ok(r) => DropboxError::RestoreError(r),
			}),
			Ok(r) => Ok(r),
		}
	}

	pub fn save_url(&self, arg: SaveUrlArg)
	-> Result<SaveUrlResult>
	{
		let uri = gen_uri!("files", "save_url");
		let body: String = serde_json::to_string(&arg)?;
		let resp: String = self.dropbox.send_request(&uri, &body)?;
		match serde_json::from_str::<SaveUrlResult>(&resp)
		{
			Err(_) => Err(match serde_json::from_str::<Error<SaveUrlError>>(&resp)
			{
				Err(_) => DropboxError::Other,
				Ok(r) => DropboxError::SaveUrlError(r),
			}),
			Ok(r) => Ok(r),
		}
	}

	pub fn save_url_check_jobstatus(&self, arg: PollArg)
	-> Result<SaveUrlJobStatus>
	{
		let uri = gen_uri!("files", "save_url", "check_job_status");
		let body: String = serde_json::to_string(&arg)?;
		let resp: String = self.dropbox.send_request(&uri, &body)?;
		match serde_json::from_str::<SaveUrlJobStatus>(&resp)
		{
			Err(_) => Err(match serde_json::from_str::<Error<PollError>>(&resp)
			{
				Err(_) => DropboxError::Other,
				Ok(r) => DropboxError::PollError(r),
			}),
			Ok(r) => Ok(r),
		}
	}

	pub fn search(&self, arg: SearchArg)
	-> Result<SearchResult>
	{
		let uri = gen_uri!("files", "search");
		let body: String = serde_json::to_string(&arg)?;
		let resp: String = self.dropbox.send_request(&uri, &body)?;
		match serde_json::from_str::<SearchResult>(&resp)
		{
			Err(_) => Err(match serde_json::from_str::<Error<SearchError>>(&resp)
			{
				Err(_) => DropboxError::Other,
				Ok(r) => DropboxError::SearchError(r),
			}),
			Ok(r) => Ok(r),
		}
	}

	pub fn upload(&self, arg: CommitInfo, file_path: &Path)
	-> Result<FileMetadata>
	{
		let uri = gen_uri!("files", "upload");
		let arg: String = serde_json::to_string(&arg)?;
		let resp: String = self.dropbox.upload(&uri, &arg, file_path)?;
		match serde_json::from_str::<FileMetadata>(&resp)
		{
			Err(_) => Err(match serde_json::from_str::<Error<UploadError>>(&resp)
			{
				Err(_) => DropboxError::Other,
				Ok(r) => DropboxError::UploadError(r),
			}),
			Ok(r) => Ok(r),
		}
	}

	pub fn upload_session_append(&self, arg: UploadSessionAppendArg)
	-> Result<()>
	{
		let uri = gen_upload_uri!("files", "upload_session", "append_v2");
		let body: String = serde_json::to_string(&arg)?;
		let resp: String = self.dropbox.send_request(&uri, &body)?;
		match serde_json::from_str::<Error<UploadSessionLookupError>>(&resp)
		{
			Err(e) => match e.is_eof()
			{
				false => Err(DropboxError::Other),
				true => Ok(()),
			},
			Ok(r) => Err(DropboxError::UploadSessionLookupError(r)),
		}
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
