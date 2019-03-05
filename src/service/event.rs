use futures::Future;
use std::fmt;

use crate::{context::SessionContext, error::Error, multiaddr::Multiaddr, ProtocolId, SessionId};

/// Error generated by the Service
#[derive(Debug)]
pub enum ServiceError<'a> {
    /// When dial remote error
    DialerError {
        /// Remote address
        address: Multiaddr,
        /// error
        error: Error<ServiceTask>,
    },
    /// When listen error
    ListenError {
        /// Listen address
        address: Multiaddr,
        /// error
        error: Error<ServiceTask>,
    },
    /// Protocol select fail
    ProtocolSelectError {
        /// Protocol name, if none, timeout or other net problem,
        /// if Some, don't support this proto
        proto_name: Option<String>,
        /// Session context
        session_context: &'a SessionContext,
    },
    /// Protocol error during interaction
    ProtocolError {
        /// Session id
        id: SessionId,
        /// Protocol id
        proto_id: ProtocolId,
        /// Codec error
        error: Error<ServiceTask>,
    },
    /// After initializing the connection, the session does not open any protocol,
    /// suspected fd attack
    SessionTimeout {
        /// Session context
        session_context: &'a SessionContext,
    },
}

/// Event generated by the Service
#[derive(Debug)]
pub enum ServiceEvent<'a> {
    /// A session close
    SessionClose {
        /// Session context
        session_context: &'a SessionContext,
    },
    /// A session open
    SessionOpen {
        /// Session context
        session_context: &'a SessionContext,
    },
}

/// Event generated by all protocol
#[derive(Debug)]
pub enum ProtocolEvent<'a> {
    /// Protocol open event
    Connected {
        /// session context
        session_context: &'a SessionContext,
        /// Protocol id
        proto_id: ProtocolId,
        /// Protocol version
        version: String,
    },
    /// Received protocol data
    Received {
        /// Session id
        session_id: SessionId,
        /// Protocol id
        proto_id: ProtocolId,
        /// Protocol version
        data: bytes::Bytes,
    },
    /// Protocol close event
    DisConnected {
        /// Protocol id
        proto_id: ProtocolId,
        /// session context
        session_context: &'a SessionContext,
    },
    /// Service-level notify
    ProtocolNotify {
        /// Protocol id
        proto_id: ProtocolId,
        /// token
        token: u64,
    },
    /// Session-level notify task
    ProtocolSessionNotify {
        /// Session id
        session_id: SessionId,
        /// Protocol id
        proto_id: ProtocolId,
        /// Notify token
        token: u64,
    },
}

/// Task received by the Service.
///
/// An instruction that the outside world can send to the service
pub enum ServiceTask {
    /// Send protocol data task
    ProtocolMessage {
        /// Specify which sessions to send to,
        /// None means broadcast
        session_ids: Option<Vec<SessionId>>,
        /// protocol id
        proto_id: ProtocolId,
        /// data
        data: Vec<u8>,
    },
    /// Service-level notify task
    ProtocolNotify {
        /// Protocol id
        proto_id: ProtocolId,
        /// Notify token
        token: u64,
    },
    /// Session-level notify task
    ProtocolSessionNotify {
        /// Session id
        session_id: SessionId,
        /// Protocol id
        proto_id: ProtocolId,
        /// Notify token
        token: u64,
    },
    /// Future task
    FutureTask {
        /// Future
        task: Box<dyn Future<Item = (), Error = ()> + 'static + Send>,
    },
    /// Disconnect task
    Disconnect {
        /// Session id
        session_id: SessionId,
    },
    /// Dial task
    Dial {
        /// Remote address
        address: Multiaddr,
    },
    /// Listen task
    Listen {
        /// Listen address
        address: Multiaddr,
    },
}

impl fmt::Debug for ServiceTask {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ServiceTask::*;

        match self {
            ProtocolMessage {
                session_ids,
                proto_id,
                data,
            } => write!(
                f,
                "id: {:?}, protoid: {}, message: {:?}",
                session_ids, proto_id, data
            ),
            ProtocolNotify { proto_id, token } => {
                write!(f, "protocol id: {}, token: {}", proto_id, token)
            }
            ProtocolSessionNotify {
                session_id,
                proto_id,
                token,
            } => write!(
                f,
                "session id: {}, protocol id: {}, token: {}",
                session_id, proto_id, token
            ),
            FutureTask { .. } => write!(f, "Future task"),
            Disconnect { session_id } => write!(f, "Disconnect session [{}]", session_id),
            Dial { address } => write!(f, "Dial address: {}", address),
            Listen { address } => write!(f, "Listen address: {}", address),
        }
    }
}
