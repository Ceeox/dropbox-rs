#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Error<T>
{
	error_summary: String,
	error: T,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum AlphaGetMetadataError
{
	#[serde(rename="path")]
	Path(LookupError),
	#[serde(rename="properties_error")]
	PropertiesError(LookUpPropertiesError),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum RelocationError
{
	#[serde(rename="from_lookup")]
	FromLookup(LookupError),
	#[serde(rename="from_write")]
	FromWrite(WriteError),
	#[serde(rename="to")]
	To(WriteError),
	#[serde(rename="cant_copy_shared_folder")]
	CantCopySharedFolder,
	#[serde(rename="cant_nest_shared_folder")]
	CantNestSharedFolder,
	#[serde(rename="cant_move_folder_into_itself")]
	CantMoveFolderInfoItself,
	#[serde(rename="too_many_files")]
	TooManyFiles,
	#[serde(rename="duplicated_or_nested_paths")]
	DuplicatedOrNestedPaths,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum WriteError
{
	#[serde(rename="malformed_path")]
	MalformedPath(Option<String>),
	#[serde(rename="conflict")]
	Conflict(WriteConflictError),
	#[serde(rename="no_write_permission")]
	NoWritePermission,
	#[serde(rename="insufficient_space")]
	InsufficientSpace,
	#[serde(rename="disallowed_name")]
	DisallowedName,
	#[serde(rename="team_folder")]
	TeamFolder,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum WriteConflictError
{
	#[serde(rename="file")]
	File,
	#[serde(rename="folder")]
	Folder,
	#[serde(rename="file_ancestor")]
	FileAncestor,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum RelocationBatchError
{
	#[serde(rename="from_lookup")]
	FromLookup(LookupError),
	#[serde(rename="from_write")]
	FromWrite(WriteError),
	#[serde(rename="to")]
	To(WriteError),
	#[serde(rename="cant_copy_shared_folder")]
	CantCopySharedFolder,
	#[serde(rename="cant_nest_shared_folder")]
	CantNestSharedFolder,
	#[serde(rename="too_many_files")]
	TooManyFiles,
	#[serde(rename="duplicated_or_nested_paths")]
	DuplicatedOrNestedPaths,
	#[serde(rename="too_many_write_operations")]
	TooManyWriteOperations,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum PollError
{
	#[serde(rename="invalid_async_job_id")]
	InvalidAsyncJobId,
	#[serde(rename="internal_error")]
	InternalError
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum GetCopyReferenceError
{
	#[serde(rename="path")]
	Path{ path: LookupError},
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum LookUpPropertiesError
{
	#[serde(rename="property_group_not_found")]
	PropertyGroupNotFound,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag=".tag")]
pub enum ListFolderError
{
	#[serde(rename="path")]
	Path{ path: LookupError},
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag=".tag")]
pub enum LookupError
{
	#[serde(rename="malformed_path")]
	MalformedPath(Option<String>),
	#[serde(rename="not_found")]
	NotFound,
	#[serde(rename="not_file")]
	NotFile,
	#[serde(rename="not_folder")]
	NotFolder,
	#[serde(rename="restricted_content")]
	RestrictedContent,
	#[serde(rename="invalid_path_root")]
	InvalidPathRoot(PathRootError)
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct PathRootError
{
	path_root: Option<String>,
}
