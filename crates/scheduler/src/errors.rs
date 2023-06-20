pub use common_utils::errors::{ParsingError, ValidationError};
pub use redis_interface::errors::RedisError;
pub use storage_models::errors::StorageError;

#[derive(Debug, thiserror::Error)]
pub enum ProcessTrackerError {
    #[error("An unexpected flow was specified")]
    UnexpectedFlow,
    #[error("Failed to serialize object")]
    SerializationFailed,
    #[error("Failed to deserialize object")]
    DeserializationFailed,
    #[error("Missing required field")]
    MissingRequiredField,
    #[error("Failed to insert process batch into stream")]
    BatchInsertionFailed,
    #[error("Failed to insert process into stream")]
    ProcessInsertionFailed,
    #[error("The process batch with the specified details was not found")]
    BatchNotFound,
    #[error("Failed to update process batch in stream")]
    BatchUpdateFailed,
    #[error("Failed to delete process batch from stream")]
    BatchDeleteFailed,
    #[error("An error occurred when trying to read process tracker configuration")]
    ConfigurationError,
    #[error("Failed to update process in database")]
    ProcessUpdateFailed,
    #[error("Failed to fetch processes from database")]
    ProcessFetchingFailed,
    #[error("Failed while fetching: {resource_name}")]
    ResourceFetchingFailed { resource_name: &'static str },
    #[error("Failed while executing: {flow}")]
    FlowExecutionError { flow: &'static str },
    #[error("Not Implemented")]
    NotImplemented,
    #[error("Job not found")]
    JobNotFound,
    #[error("Received Error ApiResponseError")]
    EApiErrorResponse,
    #[error("Received Error StorageError: {0}")]
    EStorageError(error_stack::Report<StorageError>),
    #[error("Received Error RedisError: {0}")]
    ERedisError(error_stack::Report<RedisError>),
    #[error("Received Error ParsingError: {0}")]
    EParsingError(error_stack::Report<ParsingError>),
    #[error("Validation Error Received: {0}")]
    EValidationError(error_stack::Report<ValidationError>),
}

#[macro_export]
macro_rules! error_to_process_tracker_error {
    ($($path: ident)::+ < $st: ident >, $($path2:ident)::* ($($inner_path2:ident)::+ <$st2:ident>) ) => {
        impl From<$($path)::+ <$st>> for ProcessTrackerError {
            fn from(err: $($path)::+ <$st> ) -> Self {
                $($path2)::*(err)
            }
        }
    };

    ($($path: ident)::+  <$($inner_path:ident)::+>, $($path2:ident)::* ($($inner_path2:ident)::+ <$st2:ident>) ) => {
        impl<'a> From< $($path)::+ <$($inner_path)::+> > for ProcessTrackerError {
            fn from(err: $($path)::+ <$($inner_path)::+> ) -> Self {
                $($path2)::*(err)
            }
        }
    };
}

impl From<error_stack::Report<ProcessTrackerError>> for ProcessTrackerError {
    fn from(value: error_stack::Report<ProcessTrackerError>) -> Self {
        ProcessTrackerError::EApiErrorResponse
    }
}

error_to_process_tracker_error!(
    error_stack::Report<StorageError>,
    ProcessTrackerError::EStorageError(error_stack::Report<StorageError>)
);

error_to_process_tracker_error!(
    error_stack::Report<RedisError>,
    ProcessTrackerError::ERedisError(error_stack::Report<RedisError>)
);

error_to_process_tracker_error!(
    error_stack::Report<ParsingError>,
    ProcessTrackerError::EParsingError(error_stack::Report<ParsingError>)
);

error_to_process_tracker_error!(
    error_stack::Report<ValidationError>,
    ProcessTrackerError::EValidationError(error_stack::Report<ValidationError>)
);