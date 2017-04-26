// std uses
use std::string::FromUtf8Error;
use std::io::Error as StdIoError;
// extern uses
use hyper::error::Error as HyperErr;
use serde_json::Error as SerdeJsonError;
// intern uses
use ::models::error::*;

pub type Result<T> = ::std::result::Result<T, DropboxError>;

#[derive(Debug)]
pub enum DropboxError
{
	HyperError(HyperErr),
	Utf8Error(FromUtf8Error),
	IoError(StdIoError),
	JsonError(SerdeJsonError),

	// Dropbox api errors
	RelocationError(Error<RelocationError>),
	PollError(Error<PollError>),
	GetCopyReferenceError(Error<GetCopyReferenceError>),
	SaveCopyReferenceError(Error<SaveCopyReferenceError>),
	CreateFolderError(Error<CreateFolderError>),
	DeleteError(Error<DeleteError>),
	ListFolderError(Error<ListFolderError>),
	GetMetadataError(Error<GetMetadataError>),
	DownloadError(Error<DownloadError>),
	PreviewError(Error<PreviewError>),
	GetTemporaryLinkError(Error<GetTemporaryLinkError>),
	ThumbnailError(Error<ThumbnailError>),
	ListFolderContinueError(Error<ListFolderContinueError>),
	ListFolderLongpollError(Error<ListFolderLongpollError>),
	ListRevisionsError(Error<ListRevisionsError>),

	Other,
}

impl From<HyperErr> for DropboxError
{
	fn from(err: HyperErr)
	-> DropboxError
	{
		DropboxError::HyperError(err)
	}
}

impl From<FromUtf8Error> for DropboxError
{
	fn from(err: FromUtf8Error)
	-> DropboxError
	{
		DropboxError::Utf8Error(err)
	}
}

impl From<StdIoError> for DropboxError
{
	fn from(err: StdIoError)
	-> DropboxError
	{
		DropboxError::IoError(err)
	}
}

impl From<SerdeJsonError> for DropboxError
{
	fn from(err: SerdeJsonError)
	-> DropboxError
	{
		DropboxError::JsonError(err)
	}
}

impl From<Error<RelocationError>> for DropboxError
{
	fn from(err: Error<RelocationError>)
	-> DropboxError
	{
		DropboxError::RelocationError(err)
	}
}

impl From<Error<PollError>> for DropboxError
{
	fn from(err: Error<PollError>)
	-> DropboxError
	{
		DropboxError::PollError(err)
	}
}

impl From<Error<GetCopyReferenceError>> for DropboxError
{
	fn from(err: Error<GetCopyReferenceError>)
	-> DropboxError
	{
		DropboxError::GetCopyReferenceError(err)
	}
}

impl From<Error<SaveCopyReferenceError>> for DropboxError
{
	fn from(err: Error<SaveCopyReferenceError>)
	-> DropboxError
	{
		DropboxError::SaveCopyReferenceError(err)
	}
}

impl From<Error<CreateFolderError>> for DropboxError
{
	fn from(err: Error<CreateFolderError>)
	-> DropboxError
	{
		DropboxError::CreateFolderError(err)
	}
}

impl From<Error<DeleteError>> for DropboxError
{
	fn from(err: Error<DeleteError>)
	-> DropboxError
	{
		DropboxError::DeleteError(err)
	}
}

impl From<Error<DownloadError>> for DropboxError
{
	fn from(err: Error<DownloadError>)
	-> DropboxError
	{
		DropboxError::DownloadError(err)
	}
}

impl From<Error<GetMetadataError>> for DropboxError
{
	fn from(err: Error<GetMetadataError>)
	-> DropboxError
	{
		DropboxError::GetMetadataError(err)
	}
}

impl From<Error<PreviewError>> for DropboxError
{
	fn from(err: Error<PreviewError>)
	-> DropboxError
	{
		DropboxError::PreviewError(err)
	}
}

impl From<Error<GetTemporaryLinkError>> for DropboxError
{
	fn from(err: Error<GetTemporaryLinkError>)
	-> DropboxError
	{
		DropboxError::GetTemporaryLinkError(err)
	}
}

impl From<Error<ThumbnailError>> for DropboxError
{
	fn from(err: Error<ThumbnailError>)
	-> DropboxError
	{
		DropboxError::ThumbnailError(err)
	}
}

impl From<Error<ListFolderError>> for DropboxError
{
	fn from(err: Error<ListFolderError>)
	-> DropboxError
	{
		DropboxError::ListFolderError(err)
	}
}

impl From<Error<ListFolderContinueError>> for DropboxError
{
	fn from(err: Error<ListFolderContinueError>)
	-> DropboxError
	{
		DropboxError::ListFolderContinueError(err)
	}
}

impl From<Error<ListFolderLongpollError>> for DropboxError
{
	fn from(err: Error<ListFolderLongpollError>)
	-> DropboxError
	{
		DropboxError::ListFolderLongpollError(err)
	}
}

impl From<Error<ListRevisionsError>> for DropboxError
{
	fn from(err: Error<ListRevisionsError>)
	-> DropboxError
	{
		DropboxError::ListRevisionsError(err)
	}
}
