//  SERVER.rs
//    by Lut99
//
//  Created:
//    23 Oct 2024, 11:37:44
//  Last edited:
//    24 Oct 2024, 13:47:34
//  Auto updated?
//    Yes
//
//  Description:
//!   Implements some abstraction over something waiting for requests and
//!   serving policies.
//

use std::error::Error;
use std::future::Future;


/***** LIBRARY *****/
/// Abstracts over the "frontend" of the store; i.e., some API or other interface that listens for
/// requests and interacts with the backend as necessary.
pub trait Server {
    /// The type of errors emitted by this server.
    type Error: Error;


    /// Runs this server.
    ///
    /// This will hijack the current codeflow and keep serving the server until the end of the
    /// universe! ...or until the server quits.
    ///
    /// In case of the latter, the thread just returns.
    ///
    /// # Errors
    /// This function may error if the server failed to listen of if a fatal server errors comes
    /// along as it serves. However, client-side errors should not trigger errors at this level.
    fn serve(self) -> impl Future<Output = Result<(), Self::Error>>;
}
