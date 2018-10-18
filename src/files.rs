use std::path::Path;

use serde_json;

use error::*;
use models::error::*;
use models::files::*;
use DropboxContext;

use hyper::rt::{self, Future, Stream};
use hyper::{Method, Request, Uri};
use serde::{Deserialize, Serialize};

pub struct DropboxFiles {
	ctx: DropboxContext,
}

impl DropboxFiles {
	pub(crate) fn new(context: &DropboxContext) -> Self {
		Self {
			ctx: context.clone(),
		}
	}

	/// Copy a file or folder to a different location in the user's Dropbox.
	/// If the source path is a folder all its contents will be copied.
	pub fn copy(&self, arg: RelocationArg) -> Result<Metadata> {
		let uri = gen_uri!("files", "copy");
		let body: String = serde_json::to_string(&arg)?;
		let resp: String = self.ctx.send_request(&uri, &body)?;
		match serde_json::from_str::<Metadata>(&resp) {
			Err(_) => Err(
				match serde_json::from_str::<Error<RelocationError>>(&resp) {
					Err(_) => DropboxError::Other,
					Ok(r) => DropboxError::RelocationError(r),
				},
			),
			Ok(r) => Ok(r),
		}
	}

	/// Copy multiple files or folders to different locations at once in the user's Dropbox.
	/// If RelocationBatchArg.allow_shared_folder is false, this route is atomic.
	/// If on entry failes, the whole transaction will abort.
	/// If RelocationBatchArg.allow_shared_folder is true, not atomicity is guaranteed,
	/// but you will be able to copy the contents of shared folders to new locations.
	/// This route will return job ID immediately and do the async copy job in background.
	/// Please use copy_batch/check to check the job status.
	pub fn copy_batch(&self, arg: RelocationBatchArg) -> Result<RelocationBatchLaunch> {
		let uri = gen_uri!("files", "copy_batch");
		let body: String = serde_json::to_string(&arg)?;
		let resp: String = self.ctx.send_request(&uri, &body)?;
		match serde_json::from_str::<RelocationBatchLaunch>(&resp) {
			Err(_) => Err(DropboxError::Other),
			Ok(r) => Ok(r),
		}
	}

	/// Returns the status of an asynchronous job for copy_batch.
	/// If success, it returns list of results for each entry.
	pub fn copy_batch_check(&self, arg: PollArg) -> Result<RelocationBatchJobStatus> {
		let uri = gen_uri!("files", "copy");
		let body: String = serde_json::to_string(&arg)?;
		let resp: String = self.ctx.send_request(&uri, &body)?;
		match serde_json::from_str::<RelocationBatchJobStatus>(&resp) {
			Err(_) => Err(match serde_json::from_str::<Error<PollError>>(&resp) {
				Err(_) => DropboxError::Other,
				Ok(r) => DropboxError::PollError(r),
			}),
			Ok(r) => Ok(r),
		}
	}

	/// Get a copy reference to a file or folder.
	/// This reference string can be used to save that file or folder to another user's Dropbox
	/// by passing it to copy_reference/save.
	pub fn copy_reference_get(&self, arg: GetCopyReferenceArg) -> Result<GetCopyReferenceResult> {
		let uri = gen_uri!("files", "copy_reference", "get");
		let body: String = serde_json::to_string(&arg)?;
		let resp: String = self.ctx.send_request(&uri, &body)?;
		match serde_json::from_str::<GetCopyReferenceResult>(&resp) {
			Err(_) => Err(
				match serde_json::from_str::<Error<GetCopyReferenceError>>(&resp) {
					Err(_) => DropboxError::Other,
					Ok(r) => DropboxError::GetCopyReferenceError(r),
				},
			),
			Ok(r) => Ok(r),
		}
	}

	/// Save a copy reference returned by copy_reference/get to the user's Dropbox.
	pub fn copy_reference_save(
		&self,
		arg: SaveCopyReferenceArg,
	) -> Result<SaveCopyReferenceResult> {
		let uri = gen_uri!("files", "copy_reference", "save");
		let body: String = serde_json::to_string(&arg)?;
		let resp: String = self.ctx.send_request(&uri, &body)?;
		match serde_json::from_str::<SaveCopyReferenceResult>(&resp) {
			Err(_) => Err(
				match serde_json::from_str::<Error<SaveCopyReferenceError>>(&resp) {
					Err(_) => DropboxError::Other,
					Ok(r) => DropboxError::SaveCopyReferenceError(r),
				},
			),
			Ok(r) => Ok(r),
		}
	}

	/// Create a folder at a given path.
	pub fn create_folder(&self, arg: CreateFolderArg) -> Result<FolderMetadata> {
		let uri = gen_uri!("files", "create_folder");
		let body: String = serde_json::to_string(&arg)?;
		let resp: String = self.ctx.send_request(&uri, &body)?;
		match serde_json::from_str::<FolderMetadata>(&resp) {
			Err(_) => Err(
				match serde_json::from_str::<Error<CreateFolderError>>(&resp) {
					Err(_) => DropboxError::Other,
					Ok(r) => DropboxError::CreateFolderError(r),
				},
			),
			Ok(r) => Ok(r),
		}
	}

	/// Delete the file or folder at a given path.
	/// If the path is a folder, all its contents will be deleted too.
	/// A successful response indicates that the file or folder was deleted.
	/// The returned metadata will be the corresponding FileMetadata or FolderMetadata
	/// for the item at time of deletion, and not a DeletedMetadata object.
	pub fn delete(&self, arg: DeleteArg) -> Result<Metadata> {
		let uri = gen_uri!("files", "delete");
		let body: String = serde_json::to_string(&arg)?;
		let resp: String = self.ctx.send_request(&uri, &body)?;
		match serde_json::from_str::<Metadata>(&resp) {
			Err(_) => Err(match serde_json::from_str::<Error<DeleteError>>(&resp) {
				Err(_) => DropboxError::Other,
				Ok(r) => DropboxError::DeleteError(r),
			}),
			Ok(r) => Ok(r),
		}
	}

	/// Delete multiple files/folders at once.
	/// This route is asynchronous, which returns a job ID immediately and runs the delete batch asynchronously.
	/// Use delete_batch/check to check the job status.
	pub fn delete_batch(&self, arg: DeleteBatchArg) -> Result<DeleteBatchLaunch> {
		let uri = gen_uri!("files", "delete_batch");
		let body: String = serde_json::to_string(&arg)?;
		let resp: String = self.ctx.send_request(&uri, &body)?;
		match serde_json::from_str::<DeleteBatchLaunch>(&resp) {
			Err(_) => Err(DropboxError::Other),
			Ok(r) => Ok(r),
		}
	}

	/// Returns the status of an asynchronous job for delete_batch.
	/// If success, it returns list of result for each entry.
	pub fn delete_batch_check(&self, arg: PollArg) -> Result<DeleteBatchJobStatus> {
		let uri = gen_uri!("files", "delete_batch", "check");
		let body: String = serde_json::to_string(&arg)?;
		let resp: String = self.ctx.send_request(&uri, &body)?;
		match serde_json::from_str::<DeleteBatchJobStatus>(&resp) {
			Err(_) => Err(match serde_json::from_str::<Error<PollError>>(&resp) {
				Err(_) => DropboxError::Other,
				Ok(r) => DropboxError::PollError(r),
			}),
			Ok(r) => Ok(r),
		}
	}

	/// Download a file from a user's Dropbox.
	pub fn _download(&self, arg: DownloadArg, file_path: &Path) -> Result<FileMetadata> {
		let uri = gen_upload_uri!("files", "download");
		let body: String = serde_json::to_string(&arg)?;
		let file_info = self.download(&uri, &body, &file_path)?;
		match serde_json::from_str::<FileMetadata>(&file_info) {
			Err(_) => {
				return Err(
					match serde_json::from_str::<Error<DownloadError>>(&file_info) {
						Err(_) => DropboxError::Other,
						Ok(r) => DropboxError::DownloadError(r),
					},
				)
			}
			Ok(r) => Ok(r),
		}
	}

	/// Returns the metadata for a file or folder.
	/// Note: Metadata for the root folder is unsupported.
	pub fn get_metadata(&self, arg: GetMetadataArg) -> Result<Metadata> {
		let uri = gen_uri!("files", "get_metadata");
		let body: String = serde_json::to_string(&arg)?;
		let resp: String = self.ctx.send_request(&uri, &body)?;
		match serde_json::from_str::<Metadata>(&resp) {
			Err(_) => Err(
				match serde_json::from_str::<Error<GetMetadataError>>(&resp) {
					Err(_) => DropboxError::Other,
					Ok(r) => DropboxError::GetMetadataError(r),
				},
			),
			Ok(r) => Ok(r),
		}
	}

	/// Get a preview for a file.
	/// Currently, PDF previews are generated for files with the following extensions:
	/// .ai, .doc, .docm, .docx, .eps, .odp, .odt, .pps, .ppsm, .ppsx, .ppt, .pptm, .pptx, .rtf.
	/// HTML previews are generated for files with the following extensions: .csv, .ods, .xls, .xlsm, .xlsx.
	/// Other formats will return an unsupported extension error.
	pub fn get_preview(&self, arg: PreviewArg, file_path: &Path) -> Result<FileMetadata> {
		let uri = gen_upload_uri!("files", "get_preview");
		let body: String = serde_json::to_string(&arg)?;
		let file_info = self.download(&uri, &body, file_path)?;
		match serde_json::from_str::<FileMetadata>(&file_info) {
			Err(_) => {
				return Err(
					match serde_json::from_str::<Error<PreviewError>>(&file_info) {
						Err(_) => DropboxError::Other,
						Ok(r) => DropboxError::PreviewError(r),
					},
				)
			}
			Ok(r) => Ok(r),
		}
	}

	/// Get a temporary link to stream content of a file.
	/// This link will expire in four hours and afterwards you will get 410 Gone.
	/// Content-Type of the link is determined automatically by the file's mime type.
	pub fn get_temporary_link(&self, arg: GetTemporaryLinkArg) -> Result<GetTemporaryLinkResult> {
		let uri = gen_uri!("files", "get_temporary_link");
		let body: String = serde_json::to_string(&arg)?;
		let resp: String = self.ctx.send_request(&uri, &body)?;
		match serde_json::from_str::<GetTemporaryLinkResult>(&resp) {
			Err(_) => Err(
				match serde_json::from_str::<Error<GetTemporaryLinkError>>(&resp) {
					Err(_) => DropboxError::Other,
					Ok(r) => DropboxError::GetTemporaryLinkError(r),
				},
			),
			Ok(r) => Ok(r),
		}
	}

	/// Get a thumbnail for an image.
	/// This method currently supports files with the following file extensions:
	/// jpg, jpeg, png, tiff, tif, gif and bmp.
	/// Photos that are larger than 20MB in size won't be converted to a thumbnail.
	pub fn get_thumbnail(&self, arg: ThumbnailArg, file_path: &Path) -> Result<FileMetadata> {
		let uri = gen_upload_uri!("files", "get_thumbnail");
		let body: String = serde_json::to_string(&arg)?;
		let file_info = self.download(&uri, &body, file_path)?;
		match serde_json::from_str::<FileMetadata>(&file_info) {
			Err(_) => {
				return Err(
					match serde_json::from_str::<Error<ThumbnailError>>(&file_info) {
						Err(_) => DropboxError::Other,
						Ok(r) => DropboxError::ThumbnailError(r),
					},
				)
			}
			Ok(r) => Ok(r),
		}
	}

	/// Starts returning the contents of a folder.
	/// If the result's ListFolderResult.has_more field is true,
	/// call list_folder/continue with the returned ListFolderResult.cursor to retrieve more entries.
	/// If you're using ListFolderArg.recursive set to true to keep a local cache of the contents of a Dropbox account,
	/// iterate through each entry in order and process them as follows to keep your local state in sync:
	/// For each FileMetadata, store the new entry at the given path in your local state.
	/// If the required parent folders don't exist yet, create them. If there's already something else at the given path,
	/// replace it and remove all its children.
	/// For each FolderMetadata, store the new entry at the given path in your local state.
	/// If the required parent folders don't exist yet, create them. If there's already something else at the given path,
	/// replace it but leave the children as they are.
	/// Check the new entry's FolderSharingInfo.read_only and set all its children's read-only statuses to match.
	/// For each DeletedMetadata, if your local state has something at the given path, remove it and all its children.
	/// If there's nothing at the given path, ignore this entry.
	/// Note: auth.RateLimitError may be returned if multiple list_folder or list_folder/continue
	/// calls with same parameters are made simultaneously by same API app for same user.
	/// If your app implements retry logic, please hold off the retry until the previous request finishes.
	pub fn list_folder(&self, arg: ListFolderArg) -> Result<ListFolderResult> {
		let uri = gen_uri!("files", "list_folder");
		let body: String = serde_json::to_string(&arg)?;
		let resp: String = self.ctx.send_request(&uri, &body)?;
		match serde_json::from_str::<ListFolderResult>(&resp) {
			Err(_) => Err(
				match serde_json::from_str::<Error<ListFolderError>>(&resp) {
					Err(_) => DropboxError::Other,
					Ok(r) => DropboxError::ListFolderError(r),
				},
			),
			Ok(r) => Ok(r),
		}
	}

	/// Once a cursor has been retrieved from list_folder,
	/// use this to paginate through all files and retrieve updates to the folder,
	/// following the same rules as documented for list_folder.
	pub fn list_folders_continue(&self, arg: ListFolderContinueArg) -> Result<ListFolderResult> {
		let uri = gen_uri!("files", "list_folders", "continue");
		let body: String = serde_json::to_string(&arg)?;
		let resp: String = self.ctx.send_request(&uri, &body)?;
		match serde_json::from_str::<ListFolderResult>(&resp) {
			Err(_) => Err(
				match serde_json::from_str::<Error<ListFolderContinueError>>(&resp) {
					Err(_) => DropboxError::Other,
					Ok(r) => DropboxError::ListFolderContinueError(r),
				},
			),
			Ok(r) => Ok(r),
		}
	}

	/// A way to quickly get a cursor for the folder's state. Unlike list_folder,
	/// list_folder/get_latest_cursor doesn't return any entries.
	/// This endpoint is for app which only needs to know about new files and modifications
	/// and doesn't need to know about files that already exist in Dropbox.
	pub fn list_folder_get_latest_cursor(
		&self,
		arg: ListFolderArg,
	) -> Result<ListFolderGetLatestCursorResult> {
		let uri = gen_uri!("files", "list_folders", "get_latest_cursor");
		let body: String = serde_json::to_string(&arg)?;
		let resp: String = self.ctx.send_request(&uri, &body)?;
		match serde_json::from_str::<ListFolderGetLatestCursorResult>(&resp) {
			Err(_) => Err(
				match serde_json::from_str::<Error<ListFolderError>>(&resp) {
					Err(_) => DropboxError::Other,
					Ok(r) => DropboxError::ListFolderError(r),
				},
			),
			Ok(r) => Ok(r),
		}
	}

	/// A longpoll endpoint to wait for changes on an account.
	/// In conjunction with list_folder/continue, this call gives you a low-latency way to monitor an account
	/// for file changes. The connection will block until there are changes available or a timeout occurs.
	/// This endpoint is useful mostly for client-side apps.
	/// If you're looking for server-side notifications, check out our webhooks documentation.
	pub fn list_folder_longpoll(
		&self,
		arg: ListFolderLongpollArg,
	) -> Result<ListFolderLongpollResult> {
		let uri = gen_uri!("files", "list_folders", "longpoll");
		let body: String = serde_json::to_string(&arg)?;
		let resp: String = self.ctx.send_request(&uri, &body)?;
		match serde_json::from_str::<ListFolderLongpollResult>(&resp) {
			Err(_) => Err(
				match serde_json::from_str::<Error<ListFolderLongpollError>>(&resp) {
					Err(_) => DropboxError::Other,
					Ok(r) => DropboxError::ListFolderLongpollError(r),
				},
			),
			Ok(r) => Ok(r),
		}
	}

	/// Return revisions of a file.
	pub fn list_revisions(&self, arg: ListRevisionsArg) -> Result<ListRevisionsResult> {
		let uri = gen_uri!("files", "list_revisions");
		let body: String = serde_json::to_string(&arg)?;
		let resp: String = self.ctx.send_request(&uri, &body)?;
		match serde_json::from_str::<ListRevisionsResult>(&resp) {
			Err(_) => Err(
				match serde_json::from_str::<Error<ListRevisionsError>>(&resp) {
					Err(_) => DropboxError::Other,
					Ok(r) => DropboxError::ListRevisionsError(r),
				},
			),
			Ok(r) => Ok(r),
		}
	}

	/// Move a file or folder to a different location in the user's Dropbox.
	/// If the source path is a folder all its contents will be moved.
	#[link_name = "move"]
	pub fn _move(&self, arg: RelocationArg) -> Result<Metadata> {
		let uri = gen_uri!("files", "move");
		let body: String = serde_json::to_string(&arg)?;
		let resp: String = self.ctx.send_request(&uri, &body)?;
		match serde_json::from_str::<Metadata>(&resp) {
			Err(_) => Err(
				match serde_json::from_str::<Error<RelocationError>>(&resp) {
					Err(_) => DropboxError::Other,
					Ok(r) => DropboxError::RelocationError(r),
				},
			),
			Ok(r) => Ok(r),
		}
	}

	/// Move multiple files or folders to different locations at once in the user's Dropbox.
	/// This route is 'all or nothing', which means if one entry fails, the whole transaction will abort.
	/// This route will return job ID immediately and do the async moving job in background.
	/// Please use move_batch/check to check the job status.
	pub fn move_batch(&self, arg: RelocationBatchArg) -> Result<RelocationBatchLaunch> {
		let uri = gen_uri!("files", "move_batch");
		let body: String = serde_json::to_string(&arg)?;
		let resp: String = self.ctx.send_request(&uri, &body)?;
		match serde_json::from_str::<RelocationBatchLaunch>(&resp) {
			Err(_) => Err(DropboxError::Other),
			Ok(r) => Ok(r),
		}
	}

	/// Returns the status of an asynchronous job for move_batch.
	/// If success, it returns list of results for each entry.
	pub fn move_batch_check(&self, arg: PollArg) -> Result<RelocationBatchJobStatus> {
		let uri = gen_uri!("files", "move_batch", "check");
		let body: String = serde_json::to_string(&arg)?;
		let resp: String = self.ctx.send_request(&uri, &body)?;
		match serde_json::from_str::<RelocationBatchJobStatus>(&resp) {
			Err(_) => Err(match serde_json::from_str::<Error<PollError>>(&resp) {
				Err(_) => DropboxError::Other,
				Ok(r) => DropboxError::PollError(r),
			}),
			Ok(r) => Ok(r),
		}
	}

	/// Permanently delete the file or folder at a given path (see https://www.dropbox.com/en/help/40).
	/// Note: This endpoint is only available for Dropbox Business apps.
	pub fn permanetly_delete(&self, arg: DeleteArg) -> Result<()> {
		let uri = gen_uri!("files", "permanetly_delete");
		let body: String = serde_json::to_string(&arg)?;
		let resp: String = self.ctx.send_request(&uri, &body)?;
		match serde_json::from_str::<Error<DeleteError>>(&resp) {
			Err(e) => match e.is_eof() {
				false => Err(DropboxError::Other),
				true => Ok(()),
			},
			Ok(r) => Err(DropboxError::DeleteError(r)),
		}
	}

	/// PREVIEW - may change or disappear without notice
	///
	/// Add custom properties to a file using a filled property template.
	/// See properties/template/add to create new property templates.
	pub fn properties_add(&self) -> Result<()> {
		Err(DropboxError::Other)
	}

	/// PREVIEW - may change or disappear without notice
	///
	/// Overwrite custom properties from a specified template associated with a file.
	pub fn properties_overwride(&self) -> Result<()> {
		Err(DropboxError::Other)
	}

	/// PREVIEW - may change or disappear without notice
	///
	/// Remove all custom properties from a specified template associated with a file.
	/// To remove specific property key value pairs, see properties/update.
	/// To update a property template, see properties/template/update.
	/// Property templates can't be removed once created.
	pub fn properties_remove(&self) -> Result<()> {
		Err(DropboxError::Other)
	}

	/// PREVIEW - may change or disappear without notice
	///
	/// Get the schema for a specified template.
	pub fn properties_template_get(&self) -> Result<()> {
		Err(DropboxError::Other)
	}

	/// PREVIEW - may change or disappear without notice
	///
	/// Get the property template identifiers for a user.
	/// To get the schema of each template use properties/template/get.
	pub fn properties_template_list(&self) -> Result<()> {
		Err(DropboxError::Other)
	}

	/// PREVIEW - may change or disappear without notice
	///
	/// Add, update or remove custom properties from a specified template associated with a file.
	/// Fields that already exist and not described in the request will not be modified.
	pub fn properties_update(&self) -> Result<()> {
		Err(DropboxError::Other)
	}

	/// Restore a file to a specific revision.
	pub fn restore(&self, arg: RestoreArg) -> Result<FileMetadata> {
		let uri = gen_uri!("files", "restore");
		let body: String = serde_json::to_string(&arg)?;
		let resp: String = self.ctx.send_request(&uri, &body)?;
		match serde_json::from_str::<FileMetadata>(&resp) {
			Err(_) => Err(match serde_json::from_str::<Error<RestoreError>>(&resp) {
				Err(_) => DropboxError::Other,
				Ok(r) => DropboxError::RestoreError(r),
			}),
			Ok(r) => Ok(r),
		}
	}

	/// Save a specified URL into a file in user's Dropbox.
	/// If the given path already exists, the file will be renamed to avoid the conflict (e.g. myfile (1).txt).
	pub fn save_url(&self, arg: SaveUrlArg) -> Result<SaveUrlResult> {
		let uri = gen_uri!("files", "save_url");
		let body: String = serde_json::to_string(&arg)?;
		let resp: String = self.ctx.send_request(&uri, &body)?;
		match serde_json::from_str::<SaveUrlResult>(&resp) {
			Err(_) => Err(match serde_json::from_str::<Error<SaveUrlError>>(&resp) {
				Err(_) => DropboxError::Other,
				Ok(r) => DropboxError::SaveUrlError(r),
			}),
			Ok(r) => Ok(r),
		}
	}

	/// Check the status of a save_url job.
	pub fn save_url_check_jobstatus(&self, arg: PollArg) -> Result<SaveUrlJobStatus> {
		let uri = gen_uri!("files", "save_url", "check_job_status");
		let body: String = serde_json::to_string(&arg)?;
		let resp: String = self.ctx.send_request(&uri, &body)?;
		match serde_json::from_str::<SaveUrlJobStatus>(&resp) {
			Err(_) => Err(match serde_json::from_str::<Error<PollError>>(&resp) {
				Err(_) => DropboxError::Other,
				Ok(r) => DropboxError::PollError(r),
			}),
			Ok(r) => Ok(r),
		}
	}

	/// Searches for files and folders.
	/// Note: Recent changes may not immediately be reflected in search results due to a short delay in indexing.
	pub fn search(&self, arg: SearchArg) -> Result<SearchResult> {
		let uri = gen_uri!("files", "search");
		let body: String = serde_json::to_string(&arg)?;
		let resp: String = self.ctx.send_request(&uri, &body)?;
		match serde_json::from_str::<SearchResult>(&resp) {
			Err(_) => Err(match serde_json::from_str::<Error<SearchError>>(&resp) {
				Err(_) => DropboxError::Other,
				Ok(r) => DropboxError::SearchError(r),
			}),
			Ok(r) => Ok(r),
		}
	}

	/// Create a new file with the contents provided in the request.
	/// Do not use this to upload a file larger than 150 MB. Instead,
	/// create an upload session with upload_session/start.
	pub fn _upload(&self, arg: CommitInfo, file_path: &Path) -> Result<FileMetadata> {
		let uri = gen_uri!("files", "upload");
		let arg: String = serde_json::to_string(&arg)?;
		let resp: String = self.ctx.upload(&uri, &arg, file_path)?;
		match serde_json::from_str::<FileMetadata>(&resp) {
			Err(_) => Err(match serde_json::from_str::<Error<UploadError>>(&resp) {
				Err(_) => DropboxError::Other,
				Ok(r) => DropboxError::UploadError(r),
			}),
			Ok(r) => Ok(r),
		}
	}

	/// Append more data to an upload session.
	/// When the parameter close is set, this call will close the session.
	/// A single request should not upload more than 150 MB.
	pub fn upload_session_append(
		&self,
		arg: UploadSessionAppendArg,
		file_path: &Path,
	) -> Result<()> {
		let uri = gen_upload_uri!("files", "upload_session", "append_v2");
		let arg: String = serde_json::to_string(&arg)?;
		let resp: String = self.ctx.upload(&uri, &arg, file_path)?;
		match serde_json::from_str::<Error<UploadSessionLookupError>>(&resp) {
			Err(e) => match e.is_eof() {
				false => Err(DropboxError::Other),
				true => Ok(()),
			},
			Ok(r) => Err(DropboxError::UploadSessionLookupError(r)),
		}
	}

	/// Finish an upload session and save the uploaded data to the given file path.
	/// A single request should not upload more than 150 MB.
	pub fn upload_session_finish(
		&self,
		arg: UploadSessionCursor,
		file_path: &Path,
	) -> Result<FileMetadata> {
		let uri = gen_upload_uri!("files", "upload_session", "finish");
		let arg: String = serde_json::to_string(&arg)?;
		let resp: String = self.ctx.upload(&uri, &arg, file_path)?;
		match serde_json::from_str::<FileMetadata>(&resp) {
			Err(_) => Err(
				match serde_json::from_str::<Error<UploadSessionFinishError>>(&resp) {
					Err(_) => DropboxError::Other,
					Ok(r) => DropboxError::UploadSessionFinishError(r),
				},
			),
			Ok(r) => Ok(r),
		}
	}

	/// This route helps you commit many files at once into a user's Dropbox.
	/// Use upload_session/start and upload_session/append_v2 to upload file contents.
	/// We recommend uploading many files in parallel to increase throughput.
	/// Once the file contents have been uploaded, rather than calling upload_session/finish,
	/// use this route to finish all your upload sessions in a single request.
	/// UploadSessionStartArg.close or UploadSessionAppendArg.close needs to be true
	/// for the last upload_session/start or upload_session/append_v2 call.
	/// This route will return a job_id immediately and do the async commit job in background.
	/// Use upload_session/finish_batch/check to check the job status.
	/// For the same account, this route should be executed serially.
	/// That means you should not start the next job before current job finishes.
	/// We allow up to 1000 entries in a single request.
	pub fn upload_session_finish_batch(
		&self,
		arg: UploadSessionFinishBatchArg,
	) -> Result<UploadSessionFinishBatchLaunch> {
		let uri = gen_uri!("files", "upload_session", "finish_batch");
		let body: String = serde_json::to_string(&arg)?;
		let resp: String = self.ctx.send_request(&uri, &body)?;
		match serde_json::from_str::<UploadSessionFinishBatchLaunch>(&resp) {
			Err(_) => Err(DropboxError::Other),
			Ok(r) => Ok(r),
		}
	}

	/// Returns the status of an asynchronous job for upload_session/finish_batch.
	/// If success, it returns list of result for each entry.
	pub fn upload_session_finish_batch_check(
		&self,
		arg: PollArg,
	) -> Result<UploadSessionFinishBatchJobStatus> {
		let uri = gen_upload_uri!("files", "upload_session", "finish_batch", "check");
		let body: String = serde_json::to_string(&arg)?;
		let resp: String = self.ctx.send_request(&uri, &body)?;
		match serde_json::from_str::<UploadSessionFinishBatchJobStatus>(&resp) {
			Err(_) => Err(match serde_json::from_str::<Error<PollError>>(&resp) {
				Err(_) => DropboxError::Other,
				Ok(r) => DropboxError::PollError(r),
			}),
			Ok(r) => Ok(r),
		}
	}

	/// Upload sessions allow you to upload a single file in one or more requests,
	/// for example where the size of the file is greater than 150 MB.
	/// This call starts a new upload session with the given data.
	/// You can then use upload_session/append_v2 to add more data and
	/// upload_session/finish to save all the data to a file in Dropbox.
	/// A single request should not upload more than 150 MB.
	/// An upload session can be used for a maximum of 48 hours.
	/// Attempting to use an UploadSessionStartResult.session_id with upload_session/append_v2 or
	/// upload_session/finish more than 48 hours after its creation will return a UploadSessionLookupError.not_found.
	pub fn upload_session_start(
		&self,
		arg: UploadSessionStartArg,
		file_path: &Path,
	) -> Result<UploadSessionStartResult> {
		let uri = gen_upload_uri!("files", "upload_session", "start");
		let arg: String = serde_json::to_string(&arg)?;
		let resp: String = self.ctx.upload(&uri, &arg, file_path)?;
		match serde_json::from_str::<UploadSessionStartResult>(&resp) {
			Err(_) => Err(DropboxError::Other),
			Ok(r) => Ok(r),
		}
	}
}
