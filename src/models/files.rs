use ::models::error::*;

// /files/alpha/get_metadata
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct AlphaGetMetadataArg
{
}

// /files/copy
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct RelocationArg
{
	pub from_path: String,
	pub to_path: String,
	pub allow_shared_folder: bool,
	pub autorename: bool,
}

// /files/copy_batch
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct RelocationBatchArg
{
	pub entries: Vec<RelocationPath>,
	pub allow_shared_folder: bool,
	pub autorename: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct RelocationPath
{
	pub from_path: String,
	pub to_path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct RelocationBatchLaunch
{
	pub async_job_id: String,
	pub complete: RelocationBatchResult,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct RelocationBatchResult
{
	pub entries: Vec<RelocationResult>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RelocationResult
{
	pub metadata: Metadata,
}

// /files/copy_batch/check
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct PollArg
{
	pub async_job_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum RelocationBatchJobStatus
{
	#[serde(rename="in_progress")]
	InProgress,
	#[serde(rename="complete")]
	Complete(RelocationBatchResult),
	#[serde(rename="failed")]
	Failed(RelocationBatchError),
}

// /files/copy_reference/get
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetCopyReferenceArg
{
	pub path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetCopyReferenceResult
{
	pub metadata: Metadata,
	pub copy_reference: String,
	pub expires: String,
}

// /files/copy_reference/save
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SaveCopyReferenceArg
{
	pub copy_reference: String,
	pub path: String
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SaveCopyReferenceResult
{
	pub metadata: Metadata,
}

// /files/create_folder
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CreateFolderArg
{
	pub path: String,
	pub autorename: bool,
}

// /files/delete
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DeleteArg
{
	pub path: String,
}

// /files/delete_batch
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DeleteBatchArg
{
	pub entries: Vec<DeleteArg>
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag=".tag")]
pub enum DeleteBatchLaunch
{
	#[serde(rename="async_job_id")]
	AsyncJobId{ async_job_id: String },
	#[serde(rename="complete")]
	Complete(DeleteBatchResult),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DeleteBatchResult
{
	pub entries: Vec<DeleteBatchResultEntry>
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DeleteBatchResultEntry
{
	#[serde(rename="success")]
	Success(DeleteResult),
	#[serde(rename="failure")]
	Failure(DeleteError),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DeleteResult
{
	pub metadata: Metadata,
}

// /files/delete_batch/check
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag=".tag")]
pub enum DeleteBatchJobStatus
{
	#[serde(rename="in_progress")]
	InProgress,
	#[serde(rename="complete")]
	Complete(DeleteBatchResult),
	#[serde(rename="failed")]
	Failed(DeleteBatchError)
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DeleteBatchError
{
	#[serde(rename="too_many_write_operations")]
	TooManyWriteOperations,
}

// /files/download
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DownloadArg
{
	pub path: String,
}

// /files/get_metadata
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetMetadataArg
{
	pub path: String,
	pub include_media_info: bool,
	pub include_deleted: bool,
	pub include_has_explicit_shared_members: bool,
}

// /files/get_preview
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct PreviewArg
{
	pub path: String,
}

// /files/get_temporary_link
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct GetTemporaryLinkArg
{
	pub path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetTemporaryLinkResult
{
	pub metadata: Metadata,
	pub link: String,
}

// files/get_thumbnail
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ThumbnailArg
{
	pub path: String,
	pub format: ThumbnailFormat,
	pub size: ThumbnailSize,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ThumbnailFormat
{
	#[serde(rename="png")]
	Png,
	#[serde(rename="jpeg")]
	Jpeg,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ThumbnailSize
{
	#[serde(rename="w32h32")]
	W32h32,
	#[serde(rename="w64h64")]
	W64h64,
	#[serde(rename="w128h128")]
	W128h128,
	#[serde(rename="w640h640")]
	W640h640,
	#[serde(rename="w1024h1024")]
	W1024h1024,
}

// /flies/list_folders/continue
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct ListFolderContinueArg
{
	pub cursor: String,
}

// /files/list_folder/get_latest_cursor
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct ListFolderGetLatestCursorResult
{
	pub cursor: String,
}

// /files/list_folder/longpoll
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct ListFolderLongpollArg
{
	pub cursor: String,
	pub timeout: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct ListFolderLongpollResult
{
	pub changes: bool,
	pub backoff: Option<u64>,
}

// /files/list_revisions
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct ListRevisionsArg
{
	pub path: String,
	pub limit: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct ListRevisionsResult
{
	pub is_deleted: bool,
	pub entries: Vec<FileMetadata>,
}

// /files/get_account
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct ListFolderArg
{
	pub path: String,
	pub recursive: bool,
	pub include_media_info: bool,
	pub include_deleted: bool,
	pub include_has_explicit_shared_members: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct ListFolderResult
{
	pub entries: Vec<Metadata>,
	pub cursor: String,
	pub has_more: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct FileMetadata
{
	pub name: String,
	pub id: String,
	pub client_modified: String,
	pub server_modified: String,
	pub rev: String,
	pub size: u64,
	pub path_lower: Option<String>,
	pub path_display: Option<String>,
	pub parent_shared_folder_id: Option<String>,
	pub media_info: Option<MediaInfo>,
	pub sharing_info: Option<FileSharingInfo>,
	pub property_groups: Option<Vec<ProertyGroup>>,
	pub has_explicit_shared_members: Option<bool>,
	pub content_hash: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum MediaInfo
{
	#[serde(rename="pending")]
	Pending,
	#[serde(rename="metadata")]
	Metadata(MediaMetadata),
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct MediaMetadata
{
	pub photo: PhotoMetadata,
	pub video: VideoMetadata,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct PhotoMetadata
{
	pub dimensions: Option<Dimensions>,
	pub location: Option<GpsCoordinates>,
	pub time_taken: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Dimensions
{
	pub height: u64,
	pub width: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct GpsCoordinates
{
	pub latitude: f64,
	pub longitude: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct VideoMetadata
{
	pub dimensions: Option<Dimensions>,
	pub location: Option<GpsCoordinates>,
	pub time_taken: Option<String>,
	pub duration: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct FileSharingInfo
{
	pub read_only: bool,
	pub parent_shared_folder_id: String,
	pub modified_by: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct ProertyGroup
{
	pub template_id: String,
	pub fields: Vec<PropertyField>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct FolderMetadata
{
	pub name: String,
	pub id: String,
	pub path_lower: Option<String>,
	pub path_display: Option<String>,
	pub parent_shared_folder_id: Option<String>,
	pub shared_folder_id: Option<String>,
	pub sharing_info: Option<FolderSharingInfo>,
	pub property_groups: Option<PropertyGroup>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct FolderSharingInfo
{
	pub read_only: bool,
	pub parent_shared_folder_id: Option<String>,
	pub shared_folder_id: Option<String>,
	pub traverse_only: bool,
	pub no_access: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct PropertyGroup
{
	pub template_id: String,
	pub fields: Vec<PropertyField>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct PropertyField
{
	pub name: String,
	pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct DeletedMetadata
{
	pub name: String,
	pub path_lower: Option<String>,
	pub path_display: Option<String>,
	pub parent_shared_folder_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct RestoreArg
{
	pub path: String,
	pub rev: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct SaveUrlArg
{
	pub path: String,
	pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum SaveUrlResult
{
	#[serde(rename="async_job_id")]
	AsyncJobId(String),
	#[serde(rename="complete")]
	Complete(FileMetadata),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum SaveUrlJobStatus
{
	#[serde(rename="in_progress")]
	InProgress,
	#[serde(rename="complete")]
	Complete(FileMetadata),
	#[serde(rename="failed")]
	Failed(SaveUrlError),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SearchArg
{
	pub path: String,
	pub query: String,
	pub start: u64,
	pub max_results: u64,
	pub mode: SearchMode,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum SearchMode
{
	#[serde(rename="filename")]
	Filename,
	#[serde(rename="filename_and_content")]
	FilenameAndContent,
	#[serde(rename="deleted_filename")]
	DeletedFilename,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SearchResult
{
	pub matches: Vec<SearchMatch>,
	pub more: bool,
	pub start: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SearchMatch
{
	pub match_type: SearchMatchType,
	pub metadata: Metadata,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum SearchMatchType
{
	#[serde(rename="filename")]
	Filename,
	#[serde(rename="content")]
	Content,
	#[serde(rename="both")]
	Both,
}
