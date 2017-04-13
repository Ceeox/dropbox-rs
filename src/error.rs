// uses
use std::string::FromUtf8Error;
use std::io::Error as StdIoError;

use ::models::error::*;
use hyper::error::Error as HyperErr;
use serde_json::Error as SerdeJsonError;

pub type Result<T> = ::std::result::Result<T, DropboxError>;

#[derive(Debug)]
pub enum DropboxError
{
	HyperError(HyperErr),
	Utf8Error(FromUtf8Error),
	IoError(StdIoError),
	JsonError(SerdeJsonError),

	RelocationError(Error<RelocationError>),
	PollError(Error<PollError>),
	GetCopyReferenceError(Error<GetCopyReferenceError>),
	SaveCopyReferenceError(Error<SaveCopyReferenceError>),
	CreateFolderError(Error<CreateFolderError>),
	DeleteError(Error<DeleteError>),
	ListFolderError(Error<ListFolderError>),
	DownloadError(Error<DownloadError>),

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

impl From<Error<ListFolderError>> for DropboxError
{
	fn from(err: Error<ListFolderError>)
	-> DropboxError
	{
		DropboxError::ListFolderError(err)
	}
}
