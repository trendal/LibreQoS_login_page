use tokio::{net::UnixStream, io::{AsyncWriteExt, AsyncReadExt}};
use crate::{BUS_SOCKET_PATH, BusSession, BusRequest, encode_request, decode_response, BusResponse};
use anyhow::Result;

/// Convenient wrapper for accessing the bus
/// 
/// ## Arguments
/// 
/// * `requests` a vector of `BusRequest` requests to make.
/// 
/// **Returns** Either an error, or a vector of `BusResponse` replies
pub async fn bus_request(requests: Vec<BusRequest>) -> Result<Vec<BusResponse>> {
    let mut stream = UnixStream::connect(BUS_SOCKET_PATH).await.unwrap();
    let test = BusSession {
        requests,
    };
    let msg = encode_request(&test)?;
    stream.write(&msg).await?;
    let mut buf = Vec::new();
    let _ = stream.read_to_end(&mut buf).await.unwrap();
    let reply = decode_response(&buf)?;

    Ok(reply.responses)
}