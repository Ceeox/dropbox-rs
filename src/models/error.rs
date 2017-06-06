#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Error<T>
{
	error_summary: String,
	error: T,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum AlphaGetMetadataError
{
	#[serde(rename="path")]
	Path(LookupError),
	#[serde(rename="properties_error")]
	PropertiesError(LookUpPropertiesError),
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum RelocationError
{
	#[serde(rename="from_lookup")]
	FromLookup(LookupError),
	#[serde(rename="from_write")]
	FromWrite(WriteError),
	#[serde(rename="to")]
	To(WriteError),
	/// Shared folders can't be copied.
	#[serde(rename="cant_copy_shared_folder")]
	CantCopySharedFolder,
	/// Your move operation would result in nested shared folders. This is not allowed.
	#[serde(rename="cant_nest_shared_folder")]
	CantNestSharedFolder,
	/// You cannot move a folder into itself.
	#[serde(rename="cant_move_folder_into_itself")]
	CantMoveFolderInfoItself,
	/// The operation would involve more than 10,000 files and folders.
	#[serde(rename="too_many_files")]
	TooManyFiles,
	/// There are duplicated/nested paths among RelocationArg.from_path and RelocationArg.to_path.
	#[serde(rename="duplicated_or_nested_paths")]
	DuplicatedOrNestedPaths,
	/// Your move operation would result in an ownership transfer.
	/// You may reissue the request with the field RelocationArg.allow_ownership_transfer to true.
	#[serde(rename="cant_transfer_ownership")]
	CantTransferOwnership,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum WriteError
{
	/// This field is optional.
	#[serde(rename="malformed_path")]
	MalformedPath(Option<String>),
	/// Couldn't write to the target path because there was something in the way.
	#[serde(rename="conflict")]
	Conflict(WriteConflictError),
	/// The user doesn't have permissions to write to the target location.
	#[serde(rename="no_write_permission")]
	NoWritePermission,
	/// The user doesn't have enough available space (bytes) to write more data.
	#[serde(rename="insufficient_space")]
	InsufficientSpace,
	/// Dropbox will not save the file or folder because of its name.
	#[serde(rename="disallowed_name")]
	DisallowedName,
	/// This endpoint cannot modify or delete team folders.
	#[serde(rename="team_folder")]
	TeamFolder,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum WriteConflictError
{
	/// There's a file in the way.
	#[serde(rename="file")]
	File,
	/// There's a folder in the way.
	#[serde(rename="folder")]
	Folder,
	/// There's a file at an ancestor path, so we couldn't create the required parent folders.
	#[serde(rename="file_ancestor")]
	FileAncestor,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum RelocationBatchError
{
	#[serde(rename="from_lookup")]
	FromLookup(LookupError),
	#[serde(rename="from_write")]
	FromWrite(WriteError),
	#[serde(rename="to")]
	To(WriteError),
	/// Shared folders can't be copied.
	#[serde(rename="cant_copy_shared_folder")]
	CantCopySharedFolder,
	/// Your move operation would result in nested shared folders. This is not allowed.
	#[serde(rename="cant_nest_shared_folder")]
	CantNestSharedFolder,
	/// You cannot move a folder into itself.
	#[serde(rename="too_many_files")]
	TooManyFiles,
	/// The operation would involve more than 10,000 files and folders.
	#[serde(rename="duplicated_or_nested_paths")]
	DuplicatedOrNestedPaths,
	/// Your move operation would result in an ownership transfer.
	/// You may reissue the request with the field RelocationArg.allow_ownership_transfer to true.
	#[serde(rename="cant_transfer_ownership")]
	CantTransferOwnership,
	/// There are too many write operations in user's Dropbox. Please retry this request.
	#[serde(rename="too_many_write_operations ")]
	TooManyWriteOperations,
}

/// Error returned by methods for polling the status of asynchronous job.
/// This datatype comes from an imported namespace originally defined in the async namespace
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum PollError
{
	/// The job ID is invalid.
	#[serde(rename="invalid_async_job_id")]
	InvalidAsyncJobId,
	/// Something went wrong with the job on Dropbox's end.
	/// You'll need to verify that the action you were taking succeeded,
	/// and if not, try again. This should happen very rarely.
	#[serde(rename="internal_error")]
	InternalError
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum GetCopyReferenceError
{
	#[serde(rename="path")]
	Path(LookupError),
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(tag=".tag")]
pub enum SaveCopyReferenceError
{
	#[serde(rename="path")]
	Path(WriteError),
	/// The copy reference is invalid.
	#[serde(rename="invalid_copy_reference")]
	InvalidCopyReference,
	/// You don't have permission to save the given copy reference.
	/// Please make sure this app is same app which created the copy reference and
	/// the source user is still linked to the app.
	#[serde(rename="no_permission")]
	NoPermission,
	/// The file referenced by the copy reference cannot be found.
	#[serde(rename="not_found")]
	NotFound,
	/// The operation would involve more than 10,000 files and folders.
	#[serde(rename="too_many_files")]
	TooManyFiles,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(tag=".tag")]
pub enum DownloadError
{
	#[serde(rename="path")]
	Path(LookupError)
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(tag=".tag")]
pub enum GetMetadataError
{
	#[serde(rename="path")]
	Path(LookupError),
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(tag=".tag")]
pub enum PreviewError
{
	/// An error occurs when downloading metadata for the file.
	#[serde(rename="path")]
	Path(LookupError),
	/// This preview generation is still in progress and the file is not ready for preview yet.
	#[serde(rename="in_prpgress")]
	InProgress,
	/// The file extension is not supported preview generation.
	#[serde(rename="unsupported_extension")]
	UnsupportedExtension,
	/// The file content is not supported for preview generation.
	#[serde(rename="unsupported_content")]
	UnsupportedContent,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(tag=".tag")]
pub enum GetTemporaryLinkError
{
	#[serde(rename="path")]
	Path(LookupError),
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(tag=".tag")]
pub enum ThumbnailError
{
	/// An error occurs when downloading metadata for the image.
	Path(LookupError),
	/// The file extension doesn't allow conversion to a thumbnail.
	#[serde(rename="unsupported_extension")]
	UnsupportedExtension,
	/// The image cannot be converted to a thumbnail.
	#[serde(rename="unsupported_image")]
	UnsupportedImage,
	/// An error occurs during thumbnail conversion.
	#[serde(rename="conversion_error")]
	ConversionError,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(tag=".tag")]
pub enum ListFolderContinueError
{
	#[serde(rename="path")]
	Path(LookupError),
	/// Indicates that the cursor has been invalidated. Call list_folder to obtain a new cursor.
	#[serde(rename="reset")]
	Reset,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(tag=".tag")]
pub enum ListFolderLongpollError
{
	/// Indicates that the cursor has been invalidated. Call list_folder to obtain a new cursor.
	#[serde(rename="reset")]
	Reset,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(tag=".tag")]
pub enum ListRevisionsError
{
	#[serde(rename="path")]
	Path(LookupError),
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(tag=".tag")]
pub enum CreateFolderError
{
	#[serde(rename="path")]
	Path(WriteError),
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(tag=".tag")]
pub enum DeleteError
{
	#[serde(rename="path_lookup")]
	PathLookup(LookupError),
	#[serde(rename="path_write")]
	PathWrite(WriteError),
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum LookUpPropertiesError
{
	/// his property group does not exist for this file.
	#[serde(rename="property_group_not_found")]
	PropertyGroupNotFound,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(tag=".tag")]
pub enum ListFolderError
{
	#[serde(rename="path")]
	Path{ path: LookupError},
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(tag=".tag")]
pub enum LookupError
{
	/// This field is optional.
	#[serde(rename="malformed_path")]
	MalformedPath(Option<String>),
	/// There is nothing at the given path.
	#[serde(rename="not_found")]
	NotFound,
	/// We were expecting a file, but the given path refers to something that isn't a file.
	#[serde(rename="not_file")]
	NotFile,
	/// We were expecting a folder, but the given path refers to something that isn't a folder.
	#[serde(rename="not_folder")]
	NotFolder,
	/// The file cannot be transferred because the content is restricted.
	/// For example, sometimes there are legal restrictions due to copyright claims.
	#[serde(rename="restricted_content")]
	RestrictedContent,
	// I have no idea where the heck I found this, but it seems it dosn't belong anywhere
	// so I commented it out.
	// #[serde(rename="invalid_path_root")]
	// InvalidPathRoot(PathRootError)
}

// #[derive(Deserialize, Debug, Clone, Default, PartialEq)]
// pub struct PathRootError
// {
// 	pub path_root: Option<String>,
// }

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(tag=".tag")]
pub enum RestoreError
{
	/// An error occurs when downloading metadata for the file.
	#[serde(rename="path_lookup")]
	PathLookup(LookupError),
	/// An error occurs when trying to restore the file to that path.
	#[serde(rename="path_write")]
	PathWrite(WriteError),
	/// The revision is invalid. It may point to a different file.
	#[serde(rename="invalid_revision")]
	InvalidRevision,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(tag=".tag")]
pub enum SaveUrlError
{
	#[serde(rename="path")]
	Path(WriteError),
	/// Failed downloading the given URL.
	#[serde(rename="download_failed")]
	DownloadFailed,
	/// The given URL is invalid.
	#[serde(rename="invalid_url")]
	InvalidUrl,
	/// The file where the URL is saved to no longer exists.
	#[serde(rename="not_found")]
	NotFound,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(tag=".tag")]
pub enum SearchError
{
	#[serde(rename="path")]
	Path(LookupError),
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(tag=".tag")]
pub enum UploadError
{
	///Unable to save the uploaded contents to a file.
	#[serde(rename="path")]
	Path(UploadWriteFailed),
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct UploadWriteFailed
{
	/// The reason why the file couldn't be saved.
	pub reason: WriteError,
	/// The upload session ID; this may be used to retry the commit.
	pub upload_session_id: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum UploadSessionLookupError
{
	/// The upload session ID was not found or has expired. Upload sessions are valid for 48 hours.
	#[serde(rename="not_found")]
	NotFound,
	/// The specified offset was incorrect.
	/// See the value for the correct offset.
	/// This error may occur when a previous request was received and processed successfully but
	/// the client did not receive the response, e.g. due to a network error.
	#[serde(rename="incorrect_offset")]
	IncorrectOffset(UploadSessionOffsetError),
	/// You are attempting to append data to an upload session that has alread been closed (i.e. committed).
	#[serde(rename="closed")]
	Closed,
	/// The session must be closed before calling upload_session/finish_batch.
	#[serde(rename="not_closed")]
	NotClosed,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct UploadSessionOffsetError
{
	/// The offset up to which data has been collected.
	pub correct_offset: u64,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum UploadSessionFinishError
{
	/// The session arguments are incorrect; the value explains the reason.
	#[serde(rename="lookup_failed")]
	LookupFailed(UploadSessionLookupError),
	/// Unable to save the uploaded contents to a file.
	#[serde(rename="path")]
	Path(WriteError),
	/// The batch request commits files into too many different shared folders.
	/// Please limit your batch request to files contained in a single shared folder.
	#[serde(rename="too_many_shared_folder_targets")]
	TooManySharedFolderTargets,
	/// There are too many write operations happening in the user's Dropbox. You should retry uploading this file.
	#[serde(rename="too_many_write_operations")]
	TooManyWriteOperations,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(tag=".tag")]
pub enum GetAccountError
{
	#[serde(rename="no_account")]
	NoAccount,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(tag=".tag")]
pub enum GetAccountBatchError
{
	#[serde(rename="no_account")]
	NoAccount(String),
}
