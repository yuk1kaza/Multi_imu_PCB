use heapless::{String, Vec};
use serde::{Deserialize, Serialize};

use crate::bus::{BusId, BusProfile};
use crate::sample::RawSample;
use crate::types::{ImuDescriptor, ImuError, ImuId, ImuKind, Quaternion};

pub const PROTOCOL_VERSION: u8 = 1;
pub const MAX_IMUS_PER_SYSTEM: usize = 16;
pub const MAX_BUSES_PER_SYSTEM: usize = 8;
pub const MAX_LABEL_LEN: usize = 32;
pub const MAX_MESSAGE_LEN: usize = 96;

#[cfg(feature = "binary")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BinaryCodecError {
    BufferTooSmall,
    Postcard,
    CobsDecode,
    CrcMismatch,
    Truncated,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum WireFormat {
    Binary,
    Json,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WireHeader {
    pub protocol_version: u8,
    pub format: WireFormat,
    pub system_id: u16,
    pub session_id: u32,
    pub seq: u32,
    pub uptime_ms: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BusDescriptor {
    pub bus_id: BusId,
    pub label: String<MAX_LABEL_LEN>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HelloFrame {
    pub header: WireHeader,
    pub system_label: String<MAX_LABEL_LEN>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopologyFrame {
    pub header: WireHeader,
    pub buses: Vec<BusDescriptor, MAX_BUSES_PER_SYSTEM>,
    pub imus: Vec<ImuDescriptor, MAX_IMUS_PER_SYSTEM>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProbeResultFrame {
    pub header: WireHeader,
    pub imu_id: ImuId,
    pub driver_name: String<MAX_LABEL_LEN>,
    pub detected_kind: ImuKind,
    pub success: bool,
    pub error: Option<ImuError>,
    pub profile: Option<BusProfile>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SampleFrame {
    pub header: WireHeader,
    pub imu_id: ImuId,
    pub imu_kind: ImuKind,
    pub sample_index: u32,
    pub timestamp_us: u64,
    pub sample: RawSample,
    pub status_bits: u16,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrientationFrame {
    pub header: WireHeader,
    pub imu_id: ImuId,
    pub imu_kind: ImuKind,
    pub sample_index: u32,
    pub timestamp_us: u64,
    pub quaternion: Quaternion,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ErrorFrame {
    pub header: WireHeader,
    pub imu_id: Option<ImuId>,
    pub error: ImuError,
    pub message: String<MAX_MESSAGE_LEN>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeartbeatFrame {
    pub header: WireHeader,
    pub active_imus: u16,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum WireFrame {
    Hello(HelloFrame),
    Topology(TopologyFrame),
    ProbeResult(ProbeResultFrame),
    Sample(SampleFrame),
    Orientation(OrientationFrame),
    Error(ErrorFrame),
    Heartbeat(HeartbeatFrame),
}

#[cfg(feature = "binary")]
pub fn encode_binary<const N: usize>(frame: &WireFrame) -> Result<Vec<u8, N>, postcard::Error> {
    postcard::to_vec::<WireFrame, N>(frame)
}

#[cfg(feature = "binary")]
pub fn decode_binary(bytes: &[u8]) -> Result<WireFrame, postcard::Error> {
    postcard::from_bytes(bytes)
}

#[cfg(feature = "binary")]
pub fn encode_binary_packet<const N: usize>(
    frame: &WireFrame,
) -> Result<Vec<u8, N>, BinaryCodecError> {
    let mut raw = postcard::to_vec::<WireFrame, N>(frame).map_err(|_| BinaryCodecError::Postcard)?;
    let crc = crc32fast::hash(raw.as_slice()).to_le_bytes();
    for byte in crc {
        raw.push(byte).map_err(|_| BinaryCodecError::BufferTooSmall)?;
    }

    let encoded_len = cobs::max_encoding_length(raw.len());
    if encoded_len + 1 > N {
        return Err(BinaryCodecError::BufferTooSmall);
    }

    let mut scratch = [0u8; N];
    let used = cobs::encode(raw.as_slice(), &mut scratch);
    let mut framed = Vec::new();
    for byte in &scratch[..used] {
        framed.push(*byte).map_err(|_| BinaryCodecError::BufferTooSmall)?;
    }
    framed.push(0).map_err(|_| BinaryCodecError::BufferTooSmall)?;
    Ok(framed)
}

#[cfg(feature = "binary")]
pub fn decode_binary_packet<const N: usize>(
    packet: &[u8],
) -> Result<WireFrame, BinaryCodecError> {
    if packet.is_empty() {
        return Err(BinaryCodecError::Truncated);
    }

    let encoded = if packet.last() == Some(&0) {
        &packet[..packet.len() - 1]
    } else {
        packet
    };

    if encoded.is_empty() {
        return Err(BinaryCodecError::Truncated);
    }

    let mut decoded = [0u8; N];
    let used = cobs::decode(encoded, &mut decoded).map_err(|_| BinaryCodecError::CobsDecode)?;
    if used < 4 {
        return Err(BinaryCodecError::Truncated);
    }

    let payload_len = used - 4;
    let payload = &decoded[..payload_len];
    let crc_bytes: [u8; 4] = decoded[payload_len..used]
        .try_into()
        .map_err(|_| BinaryCodecError::Truncated)?;
    let expected_crc = u32::from_le_bytes(crc_bytes);
    let actual_crc = crc32fast::hash(payload);
    if expected_crc != actual_crc {
        return Err(BinaryCodecError::CrcMismatch);
    }

    postcard::from_bytes(payload).map_err(|_| BinaryCodecError::Postcard)
}

#[cfg(feature = "json")]
pub fn encode_json<const N: usize>(
    frame: &WireFrame,
) -> Result<String<N>, serde_json_core::ser::Error> {
    let mut output = [0u8; N];
    let written = serde_json_core::to_slice(frame, &mut output)?;
    let mut s = String::new();
    for byte in &output[..written] {
        s.push(*byte as char)
            .map_err(|_| serde_json_core::ser::Error::BufferFull)?;
    }
    Ok(s)
}

#[cfg(feature = "std-json")]
pub fn decode_json(line: &str) -> Result<WireFrame, serde_json::Error> {
    serde_json::from_str(line)
}
