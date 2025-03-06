/// Errors that can occur when creating a model from a model number.
#[derive(Debug, PartialEq, derive_more::Display, derive_more::Error)]
pub enum Error {
    /// The model number is not known or is not yet supported.
    UnknownModel,
    /// The model known but the control table is not yet implemented.
    NotImplemented,
}
