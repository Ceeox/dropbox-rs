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
