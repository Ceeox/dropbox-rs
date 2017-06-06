use ::models::error::*;

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct AlphaGetMetadataArg
{
}

#[derive(Serialize, Debug, Clone, Default, PartialEq)]
pub struct RelocationArg
{
	/// Path in the user's Dropbox to be copied or moved.
	pub from_path: String,
	/// Path in the user's Dropbox that is the destination.
	pub to_path: String,
	/// If true, copy will copy contents in shared folder,
	/// otherwise RelocationError.cant_copy_shared_folder will be returned if from_path contains shared folder.
	/// This field is always true for move. The default for this field is False.
	pub allow_shared_folder: bool,
	/// If there's a conflict, have the Dropbox server try to autorename the file to avoid the conflict.
	/// The default for this field is False.
	pub autorename: bool,
	/// Allow moves by owner even if it would result in an ownership transfer for the content being moved.
	/// This does not apply to copies. The default for this field is True.
	pub allow_ownership_transfer: bool,
}

#[derive(Serialize, Debug, Clone, Default, PartialEq)]
pub struct RelocationBatchArg
{
	/// List of entries to be moved or copied. Each entry is RelocationPath.
	pub entries: Vec<RelocationPath>,
	/// If true, copy_batch will copy contents in shared folder,
	/// otherwise RelocationError.cant_copy_shared_folder will be returned if
	/// RelocationPath.from_path contains shared folder.
	/// This field is always true for move_batch. The default for this field is False.
	pub allow_shared_folder: bool,
	///  If there's a conflict with any file, have the Dropbox server try to autorename that file to avoid the conflict.
	/// The default for this field is False.
	pub autorename: bool,
	/// Allow moves by owner even if it would result in an ownership transfer for the content being moved.
	/// This does not apply to copies The default for this field is True.
	pub allow_ownership_transfer: bool,
}

#[derive(Serialize, Debug, Clone, Default, PartialEq)]
pub struct RelocationPath
{
	/// This response indicates that the processing is asynchronous.
	/// The string is an id that can be used to obtain the status of the asynchronous job.
	pub from_path: String,
	/// Path in the user's Dropbox that is the destination.
	pub to_path: String,
}

/// Result returned by copy_batch or move_batch that may either launch an asynchronous job or complete synchronously.
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(tag=".tag")]
pub enum RelocationBatchLaunch
{
	#[serde(rename="async_job_id")]
	AsyncJobId(String),
	#[serde(rename="complete")]
	Complete(RelocationBatchResult),
}

#[derive(Deserialize, Debug, Clone, Default, PartialEq)]
pub struct RelocationBatchResult
{
	pub entries: Vec<RelocationResult>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct RelocationResult
{
	pub metadata: Metadata,
}

/// Arguments for methods that poll the status of an asynchronous job.
/// This datatype comes from an imported namespace originally defined in the async namespace.
#[derive(Serialize, Debug, Clone, Default, PartialEq)]
pub struct PollArg
{
	/// Id of the asynchronous job.
	/// This is the value of a response returned from the method that launched the job.
	pub async_job_id: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum RelocationBatchJobStatus
{
	/// The asynchronous job is still in progress.
	#[serde(rename="in_progress")]
	InProgress,
	/// The batch delete has finished.
	#[serde(rename="complete")]
	Complete(RelocationBatchResult),
	/// The batch delete has failed.
	#[serde(rename="failed")]
	Failed(RelocationBatchError),
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct GetCopyReferenceArg
{
	/// The path to the file or folder you want to get a copy reference to.
	pub path: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct GetCopyReferenceResult
{
	/// Metadata of the file or folder.
	pub metadata: Metadata,
	/// A copy reference to the file or folder.
	pub copy_reference: String,
	/// The expiration date of the copy reference.
	/// This value is currently set to be far enough in the future so that expiration is
	/// effectively not an issue.
	pub expires: String,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct SaveCopyReferenceArg
{
	/// A copy reference returned by copy_reference/get.
	pub copy_reference: String,
	/// Path in the user's Dropbox that is the destination.
	pub path: String
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct SaveCopyReferenceResult
{
	/// The metadata of the saved file or folder in the user's Dropbox.
	pub metadata: Metadata,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct CreateFolderArg
{
	/// Path in the user's Dropbox to create.
	pub path: String,
	///  If there's a conflict, have the Dropbox server try to autorename the folder to avoid the conflict.
	/// The default for this field is False.
	pub autorename: bool,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct DeleteArg
{
	/// Path in the user's Dropbox to delete.
	pub path: String,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct DeleteBatchArg
{
	pub entries: Vec<DeleteArg>
}

/// Result returned by delete_batch that may either launch an asynchronous job or complete synchronously.
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(tag=".tag")]
pub enum DeleteBatchLaunch
{
	/// This response indicates that the processing is asynchronous.
	/// The string is an id that can be used to obtain the status of the asynchronous job.
	#[serde(rename="async_job_id")]
	AsyncJobId{ async_job_id: String },
	#[serde(rename="complete")]
	Complete(DeleteBatchResult),
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct DeleteBatchResult
{
	pub entries: Vec<DeleteBatchResultEntry>
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum DeleteBatchResultEntry
{
	#[serde(rename="success")]
	Success(DeleteResult),
	#[serde(rename="failure")]
	Failure(DeleteError),
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct DeleteResult
{
	pub metadata: Metadata,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(tag=".tag")]
pub enum DeleteBatchJobStatus
{
	/// The asynchronous job is still in progress.
	#[serde(rename="in_progress")]
	InProgress,
	/// The batch delete has finished.
	#[serde(rename="complete")]
	Complete(DeleteBatchResult),
	/// The batch delete has failed.
	#[serde(rename="failed")]
	Failed(DeleteBatchError)
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum DeleteBatchError
{
	/// There are too many write operations in user's Dropbox. Please retry this request.
	#[serde(rename="too_many_write_operations")]
	TooManyWriteOperations,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct DownloadArg
{
	/// The path of the file to download.
	pub path: String,
	/// Deprecated. Please specify revision in path instead. This field is optional.
	pub rev: String,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct GetMetadataArg
{
	/// The path of a file or folder on Dropbox.
	pub path: String,
	/// If true, FileMetadata.media_info is set for photo and video. The default for this field is False.
	pub include_media_info: bool,
	/// If true, DeletedMetadata will be returned for deleted file or folder,
	/// otherwise LookupError.not_found will be returned. The default for this field is False.
	pub include_deleted: bool,
	/// If true, the results will include a flag for each file indicating whether
	/// or not that file has any explicit members. The default for this field is False.
	pub include_has_explicit_shared_members: bool,
}

#[derive(Serialize, Debug, Clone, Default, PartialEq)]
pub struct PreviewArg
{
	/// The path of the file to preview.
	pub path: String,
}

#[derive(Serialize, Debug, Clone, Default, PartialEq)]
pub struct GetTemporaryLinkArg
{
	/// The path to the file you want a temporary link to.
	pub path: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct GetTemporaryLinkResult
{
	/// Metadata of the file.
	pub metadata: Metadata,
	/// The temporary link which can be used to stream content the file.
	pub link: String,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct ThumbnailArg
{
	/// The path to the image file you want to thumbnail.
	pub path: String,
	/// The format for the thumbnail image, jpeg (default) or png. For images that are photos,
	/// jpeg should be preferred, while png is better for screenshots and digital arts.
	/// The default for this union is jpeg.
	pub format: ThumbnailFormat,
	/// The size for the thumbnail image. The default for this union is w64h64.
	pub size: ThumbnailSize,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub enum ThumbnailFormat
{
	#[serde(rename="png")]
	Png,
	#[serde(rename="jpeg")]
	Jpeg,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub enum ThumbnailSize
{
	/// 32 by 32 px.
	#[serde(rename="w32h32")]
	W32h32,
	/// 64 by 64 px.
	#[serde(rename="w64h64")]
	W64h64,
	/// 128 by 128 px.
	#[serde(rename="w128h128")]
	W128h128,
	/// 640 by 480 px.
	#[serde(rename="w640h640")]
	W640h640,
	/// 1024 by 768.
	#[serde(rename="w1024h1024")]
	W1024h1024,
}

#[derive(Serialize, Debug, Clone, Default, PartialEq)]
pub struct ListFolderContinueArg
{
	pub cursor: String,
}

#[derive(Deserialize, Debug, Clone, Default, PartialEq)]
pub struct ListFolderGetLatestCursorResult
{
	/// The cursor returned by your last call to list_folder or list_folder/continue.
	pub cursor: String,
}

#[derive(Serialize, Debug, Clone, Default, PartialEq)]
pub struct ListFolderLongpollArg
{
	/// A cursor as returned by list_folder or list_folder/continue.
	/// Cursors retrieved by setting ListFolderArg.include_media_info to true are not supported.
	pub cursor: String,
	/// A timeout in seconds. The request will block for at most this length of time,
	/// plus up to 90 seconds of random jitter added to avoid the thundering herd problem.
	/// Care should be taken when using this parameter, as some network infrastructure does not support long timeouts.
	/// The default for this field is 30.
	pub timeout: u64,
}

#[derive(Deserialize, Debug, Clone, Default, PartialEq)]
pub struct ListFolderLongpollResult
{
	///  Indicates whether new changes are available.
	/// If true, call list_folder/continue to retrieve the changes.
	pub changes: bool,
	/// If present, backoff for at least this many seconds before calling list_folder/longpoll again.
	/// This field is optional.
	pub backoff: Option<u64>,
}

#[derive(Serialize, Debug, Clone, Default, PartialEq)]
pub struct ListRevisionsArg
{
	/// The path to the file you want to see the revisions of.
	pub path: String,
	/// The maximum number of revision entries returned. The default for this field is 10.
	pub limit: u64,
}

#[derive(Deserialize, Debug, Clone, Default, PartialEq)]
pub struct ListRevisionsResult
{
	/// If the file is deleted.
	pub is_deleted: bool,
	/// The revisions for the file. Only non-delete revisions will show up here.
	pub entries: Vec<FileMetadata>,
}

#[derive(Serialize, Debug, Clone, Default, PartialEq)]
pub struct ListFolderArg
{
	/// The path to the folder you want to see the contents of.
	pub path: String,
	/// If true, the list folder operation will be applied recursively to all subfolders and
	/// the response will contain contents of all subfolders.
	/// The default for this field is False.
	pub recursive: bool,
	/// If true, FileMetadata.media_info is set for photo and video. The default for this field is False.
	pub include_media_info: bool,
	/// If true, the results will include entries for files and folders that used to exist but were deleted.
	/// The default for this field is False.
	pub include_deleted: bool,
	/// If true, the results will include a flag for each file indicating whether or
	/// not that file has any explicit members.
	/// The default for this field is False.
	pub include_has_explicit_shared_members: bool,
}

#[derive(Deserialize, Debug, Clone, Default, PartialEq)]
pub struct ListFolderResult
{
	/// The files and (direct) subfolders in the folder.
	pub entries: Vec<Metadata>,
	/// Pass the cursor into list_folder/continue to see what's changed in the folder since your previous query.
	pub cursor: String,
	/// If true, then there are more entries available.
	/// Pass the cursor to list_folder/continue to retrieve the rest.
	pub has_more: bool,
}

/// Metadata for a file or folder. This datatype will be one of the following subtypes:
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(tag=".tag")]
pub enum Metadata
{
	#[serde(rename="file")]
	File(FileMetadata),
	#[serde(rename="folder")]
	Folder(FolderMetadata),
	#[serde(rename="deleted")]
	Deleted(DeletedMetadata),
}

#[derive(Deserialize, Debug, Clone, Default, PartialEq)]
pub struct FileMetadata
{
	/// The last component of the path (including extension). This never contains a slash.
	pub name: String,
	///  A unique identifier for the file.
	pub id: String,
	/// For files, this is the modification time set by the desktop client when the file was added to Dropbox.
	/// Since this time is not verified (the Dropbox server stores whatever the desktop client sends up),
	/// this should only be used for display purposes (such as sorting) and not,
	/// for example, to determine if a file has changed or not.
	pub client_modified: String,
	/// The last time the file was modified on Dropbox.
	pub server_modified: String,
	/// A unique identifier for the current revision of a file.
	/// This field is the same rev as elsewhere in the API and can be used to detect changes and avoid conflicts.
	pub rev: String,
	/// The file size in bytes.
	pub size: u64,
	/// The lowercased full path in the user's Dropbox.
	/// This always starts with a slash. This field will be null if the file or folder is not mounted.
	/// This field is optional.
	pub path_lower: Option<String>,
	/// The cased path to be used for display purposes only.
	/// In rare instances the casing will not correctly match the user's filesystem,
	/// but this behavior will match the path provided in the Core API v1,
	/// and at least the last path component will have the correct casing.
	/// Changes to only the casing of paths won't be returned by list_folder/continue.
	/// This field will be null if the file or folder is not mounted. This field is optional.
	pub path_display: Option<String>,
	/// Deprecated. Please use FileSharingInfo.parent_shared_folder_id or FolderSharingInfo.parent_shared_folder_id instead.
	/// This field is optional.
	pub parent_shared_folder_id: Option<String>,
	/// Additional information if the file is a photo or video. This field is optional.
	pub media_info: Option<MediaInfo>,
	/// Set if this file is contained in a shared folder. This field is optional.
	pub sharing_info: Option<FileSharingInfo>,
	/// Additional information if the file has custom properties with the property template specified.
	/// This field is optional.
	pub property_groups: Option<Vec<PropertyGroup>>,
	/// This flag will only be present if include_has_explicit_shared_members is true in list_folder or get_metadata.
	/// If this flag is present, it will be true if this file has any explicit shared members.
	/// This is different from sharing_info in that this could be true in the case where a file
	/// has explicit members but is not contained within a shared folder.
	/// This field is optional.
	pub has_explicit_shared_members: Option<bool>,
	/// A hash of the file content. This field can be used to verify data integrity.
	/// For more information see our Content hash page. This field is optional.
	pub content_hash: Option<String>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum MediaInfo
{
	/// Indicate the photo/video is still under processing and metadata is not available yet.
	#[serde(rename="pending")]
	Pending,
	/// The metadata for the photo/video.
	#[serde(rename="metadata")]
	Metadata(MediaMetadata),
}

/// Additional information if the file is a photo or video.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum MediaMetadata
{
	/// Metadata for a photo.
	#[serde(rename="photo")]
	Photo(PhotoMetadata),
	/// Metadata for a video.
	#[serde(rename="video")]
	Video(VideoMetadata),
}

/// Metadata for a photo.
#[derive(Deserialize, Debug, Clone, Default, PartialEq)]
pub struct PhotoMetadata
{
	/// Dimension of the photo/video.
	pub dimensions: Option<Dimensions>,
	/// The GPS coordinate of the photo/video.
	pub location: Option<GpsCoordinates>,
	/// The timestamp when the photo/video is taken.
	pub time_taken: Option<String>,
}

/// Dimension of the photo/video.
#[derive(Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Dimensions
{
	/// Height of the photo/video.
	pub height: u64,
	/// Width of the photo/video.
	pub width: u64,
}

/// The GPS coordinate of the photo/video.
#[derive(Deserialize, Debug, Clone, Default, PartialEq)]
pub struct GpsCoordinates
{
	/// Latitude of the GPS coordinates.
	pub latitude: f64,
	///  Longitude of the GPS coordinates.
	pub longitude: f64,
}

/// Metadata for a video.
#[derive(Deserialize, Debug, Clone, Default, PartialEq)]
pub struct VideoMetadata
{
	/// Dimension of the photo/video.
	pub dimensions: Option<Dimensions>,
	/// The GPS coordinate of the photo/video.
	pub location: Option<GpsCoordinates>,
	/// The timestamp when the photo/video is taken.
	pub time_taken: Option<String>,
	/// The duration of the video in milliseconds.
	pub duration: Option<f64>,
}

/// Sharing info for a file which is contained by a shared folder.
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct FileSharingInfo
{
	/// Additional information if the file has custom properties with the property template specified.
	pub read_only: bool,
	/// This flag will only be present if include_has_explicit_shared_members is true in list_folder or get_metadata.
	/// If this flag is present, it will be true if this file has any explicit shared members.
	/// This is different from sharing_info in that this could be true in the case where a file has
	/// explicit members but is not contained within a shared folder.
	pub parent_shared_folder_id: String,
	/// A hash of the file content. This field can be used to verify data integrity.
	/// For more information see our Content hash page.
	pub modified_by: Option<String>,
}

/// Collection of custom properties in filled property templates.
/// This datatype comes from an imported namespace originally defined in the properties namespace.
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct PropertyGroup
{
	/// A unique identifier for a property template type.
	pub template_id: String,
	/// This is a list of custom properties associated with a file.
	/// There can be up to 32 properties for a template.
	pub fields: Vec<PropertyField>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct FolderMetadata
{
	/// The last component of the path (including extension). This never contains a slash.
	pub name: String,
	/// A unique identifier for the folder.
	pub id: String,
	/// The lowercased full path in the user's Dropbox.
	/// This always starts with a slash. This field will be null if the file or folder is not mounted.
	pub path_lower: Option<String>,
	/// The cased path to be used for display purposes only.
	/// In rare instances the casing will not correctly match the user's filesystem,
	/// but this behavior will match the path provided in the Core API v1,
	/// and at least the last path component will have the correct casing.
	/// Changes to only the casing of paths won't be returned by list_folder/continue.
	/// This field will be null if the file or folder is not mounted.
	pub path_display: Option<String>,
	/// Deprecated. Please use FileSharingInfo.parent_shared_folder_id or FolderSharingInfo.parent_shared_folder_id instead.
	pub parent_shared_folder_id: Option<String>,
	/// Deprecated. Please use sharing_info instead. This field is optional.
	pub shared_folder_id: Option<String>,
	/// Set if the folder is contained in a shared folder or is a shared folder mount point.
	pub sharing_info: Option<FolderSharingInfo>,
	/// Additional information if the file has custom properties with the property template specified.
	pub property_groups: Option<PropertyGroup>,
}

/// Sharing info for a folder which is contained in a shared folder or is a shared folder mount point.
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct FolderSharingInfo
{
	/// True if the file or folder is inside a read-only shared folder.
	pub read_only: bool,
	/// Set if the folder is contained by a shared folder. This field is optional.
	pub parent_shared_folder_id: Option<String>,
	/// If this folder is a shared folder mount point, the ID of the shared folder mounted at this location.
	pub shared_folder_id: Option<String>,
	/// Specifies that the folder can only be traversed and the user can only see a limited subset of
	/// the contents of this folder because they don't have read access to this folder.
	/// They do, however, have access to some sub folder. The default for this field is False.
	pub traverse_only: bool,
	/// Specifies that the folder cannot be accessed by the user. The default for this field is False.
	pub no_access: bool,
}

/// This datatype comes from an imported namespace originally defined in the properties namespace.
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct PropertyField
{
	/// This is the name or key of a custom property in a property template.
	/// File property names can be up to 256 bytes.
	pub name: String,
	/// Value of a custom property attached to a file. Values can be up to 1024 bytes.
	pub value: String,
}

/// Indicates that there used to be a file or folder at this path, but it no longer exists.
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct DeletedMetadata
{
	/// The last component of the path (including extension). This never contains a slash.
	pub name: String,
	/// The lowercased full path in the user's Dropbox.
	/// This always starts with a slash. This field will be null if the file or folder is not mounted.
	/// This field is optional.
	pub path_lower: Option<String>,
	/// The cased path to be used for display purposes only.
	/// In rare instances the casing will not correctly match the user's filesystem,
	/// but this behavior will match the path provided in the Core API v1,
	/// and at least the last path component will have the correct casing.
	/// Changes to only the casing of paths won't be returned by list_folder/continue.
	/// This field will be null if the file or folder is not mounted. This field is optional.
	pub path_display: Option<String>,
	/// Deprecated. Please use FileSharingInfo.parent_shared_folder_id or FolderSharingInfo.parent_shared_folder_id instead.
	/// This field is optional.
	pub parent_shared_folder_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct RestoreArg
{
	/// The path to the file you want to restore.
	pub path: String,
	/// The revision to restore for the file.
	pub rev: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct SaveUrlArg
{
	/// The path in Dropbox where the URL will be saved to.
	pub path: String,
	/// The URL to be saved.
	pub url: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum SaveUrlResult
{
	/// This response indicates that the processing is asynchronous.
	/// The string is an id that can be used to obtain the status of the asynchronous job.
	#[serde(rename="async_job_id")]
	AsyncJobId(String),
	/// Metadata of the file where the URL is saved to.
	#[serde(rename="complete")]
	Complete(FileMetadata),
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum SaveUrlJobStatus
{
	/// The asynchronous job is still in progress.
	#[serde(rename="in_progress")]
	InProgress,
	/// Metadata of the file where the URL is saved to.
	#[serde(rename="complete")]
	Complete(FileMetadata),
	#[serde(rename="failed")]
	Failed(SaveUrlError),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SearchArg
{
	/// The path in the user's Dropbox to search. Should probably be a folder.
	pub path: String,
	/// The string to search for. The search string is split on spaces into multiple tokens.
	/// For file name searching, the last token is used for prefix matching
	/// (i.e. "bat c" matches "bat cave" but not "batman car").
	pub query: String,
	/// The starting index within the search results (used for paging). The default for this field is 0.
	pub start: u64,
	/// The maximum number of search results to return. The default for this field is 100.
	pub max_results: u64,
	/// The search mode (filename, filename_and_content, or deleted_filename).
	/// Note that searching file content is only available for Dropbox Business accounts.
	/// The default for this union is filename.
	pub mode: SearchMode,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum SearchMode
{
	/// Search file and folder names.
	#[serde(rename="filename")]
	Filename,
	/// Search file and folder names as well as file contents.
	#[serde(rename="filename_and_content")]
	FilenameAndContent,
	/// Search for deleted file and folder names.
	#[serde(rename="deleted_filename")]
	DeletedFilename,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct SearchResult
{
	/// A list (possibly empty) of matches for the query.
	pub matches: Vec<SearchMatch>,
	/// Used for paging. If true, indicates there is another page of results available that
	/// can be fetched by calling search again.
	pub more: bool,
	/// Used for paging. Value to set the start argument to when calling search to fetch the next page of results.
	pub start: u64,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct SearchMatch
{
	/// The type of the match.
	pub match_type: SearchMatchType,
	/// The metadata for the matched file or folder.
	pub metadata: Metadata,
}

/// Indicates what type of match was found for a given item.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum SearchMatchType
{
	/// This item was matched on its file or folder name.
	#[serde(rename="filename")]
	Filename,
	/// This item was matched based on its file contents.
	#[serde(rename="content")]
	Content,
	/// This item was matched based on both its contents and its file name.
	#[serde(rename="both")]
	Both,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct CommitInfo
{
	/// Path in the user's Dropbox to save the file.
	pub path: String,
	/// Selects what to do if the file already exists. The default for this union is add.
	pub mode: WriteMode,
	/// If there's a conflict, as determined by mode,
	/// have the Dropbox server try to autorename the file to avoid conflict.
	/// The default for this field is False.
	pub autorename: bool,
	/// The value to store as the client_modified timestamp. Dropbox automatically records the time at
	/// which the file was written to the Dropbox servers.
	/// It can also record an additional timestamp, provided by Dropbox desktop clients,
	/// mobile clients, and API apps of when the file was actually created or modified.
	pub client_modified: String,
	/// Normally, users are made aware of any file modifications in their Dropbox account via notifications
	/// in the client software. If true, this tells the clients that this modification shouldn't
	/// result in a user notification.
	/// The default for this field is False.
	pub mute: bool,
}

/// Your intent when writing a file to some path. This is used to determine what constitutes a conflict and
/// what the autorename strategy is.
/// In some situations, the conflict behavior is identical:
/// (a) If the target path doesn't contain anything, the file is always written; no conflict.
/// (b) If the target path contains a folder, it's always a conflict.
/// (c) If the target path contains a file with identical contents, nothing gets written; no conflict.
/// The conflict checking differs in the case where there's a file at the target path with contents different
/// from the contents you're trying to write.
#[derive(Serialize, Debug, Clone, PartialEq)]
pub enum WriteMode
{
	/// Do not overwrite an existing file if there is a conflict.
	/// The autorename strategy is to append a number to the file name. For example,
	/// "document.txt" might become "document (2).txt".
	#[serde(rename="add")]
	Add,
	/// Always overwrite the existing file. The autorename strategy is the same as it is for add.
	#[serde(rename="overwrite")]
	Overwrite,
	/// Overwrite if the given "rev" matches the existing file's "rev".
	/// The autorename strategy is to append the string "conflicted copy" to the file name. For example,
	/// "document.txt" might become "document (conflicted copy).txt" or "document (Panda's conflicted copy).txt".
	#[serde(rename="update")]
	Update(String),
}

#[derive(Serialize, Debug, Clone, Default, PartialEq)]
pub struct UploadSessionAppendArg
{
	/// Contains the upload session ID and the offset.
	pub cursor: UploadSessionCursor,
	/// If true, the current session will be closed, at which point you won't be able to call upload_session/append_v2
	/// anymore with the current session. The default for this field is False.
	pub close: bool,
}

#[derive(Serialize, Debug, Clone, Default, PartialEq)]
pub struct UploadSessionCursor
{
	/// The upload session ID (returned by upload_session/start).
	pub session_id: String,
	/// The amount of data that has been uploaded so far.
	/// We use this to make sure upload data isn't lost or duplicated in the event of a network error.
	pub offset: u64,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct UploadSessionFinishArg
{
	/// Contains the upload session ID and the offset.
	pub cursor: UploadSessionCursor,
	/// Contains the path and other optional modifiers for the commit.
	pub commit: CommitInfo,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct UploadSessionFinishBatchArg
{
	/// Commit information for each file in the batch.
	pub entries: Vec<UploadSessionFinishArg>,
}

/// Result returned by upload_session/finish_batch that may either launch an asynchronous job or complete synchronously.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum UploadSessionFinishBatchLaunch
{
	/// his response indicates that the processing is asynchronous.
	/// The string is an id that can be used to obtain the status of the asynchronous job.
	#[serde(rename="async_job_id")]
	AsyncJobId(String),
	#[serde(rename="complete")]
	Complete(UploadSessionFinishBatchResult),
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct UploadSessionFinishBatchResult
{
	/// Commit result for each file in the batch.
	pub entries: Vec<UploadSessionFinishBatchResultEntry>
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum UploadSessionFinishBatchResultEntry
{
	#[serde(rename="success")]
	Success(FileMetadata),
	#[serde(rename="failure")]
	Failure(UploadSessionFinishError),
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum UploadSessionFinishBatchJobStatus
{
	/// The asynchronous job is still in progress.
	#[serde(rename="in_progress")]
	InProgress,
	/// The upload_session/finish_batch has finished.
	#[serde(rename="complete")]
	Complete(UploadSessionFinishBatchResult),
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct UploadSessionStartArg
{
	/// If true, the current session will be closed,
	/// at which point you won't be able to call upload_session/append_v2 anymore with the current session.
	/// The default for this field is False.
	pub close: bool,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct UploadSessionStartResult
{
	/// A unique identifier for the upload session.
	/// Pass this to upload_session/append_v2 and upload_session/finish.
	pub session_id: String,
}
