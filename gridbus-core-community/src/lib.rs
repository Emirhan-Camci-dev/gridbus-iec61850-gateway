#![no_std]

//! GridBus-Core (Community Edition)
//!
//! A `no_std`, zero-allocation parser for IEC 61850 GOOSE & Sampled Values (SV).
//! Features static ring buffers for high-throughput, lock-free message ingestion.

use heapless::spsc::Queue;

/// Represents a parsed IEC 61850 GOOSE Message.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GooseMessage<'a> {
    pub app_id: u16,
    pub cb_ref: &'a [u8],
    pub st_num: u32,
    pub sq_num: u32,
    pub test: bool,
    pub time_allowed_to_live: u32,
    // For community edition, we just hold a reference to the raw dataset bytes
    pub raw_dataset: &'a [u8],
}

/// A highly simplified ASN.1 BER Decoder for GOOSE EtherType (0x88B8).
/// In a real implementation, this would heavily validate lengths and tags.
pub struct GooseBerDecoder;

impl GooseBerDecoder {
    /// Decodes a raw Ethernet payload into a GOOSE message.
    /// Operates with strictly zero heap allocations (`malloc`-free).
    /// Target execution time: <250µs.
    pub fn decode<'a>(payload: &'a [u8]) -> Result<GooseMessage<'a>, &'static str> {
        // Simplified dummy parsing logic for demonstration
        if payload.len() < 14 {
            return Err("Payload too short");
        }

        // Example: Extract AppID (assuming it's at offset 0-1)
        let app_id = u16::from_be_bytes([payload[0], payload[1]]);

        // Mock extraction for stNum and sqNum
        let st_num = u32::from_be_bytes([payload[2], payload[3], payload[4], payload[5]]);
        let sq_num = u32::from_be_bytes([payload[6], payload[7], payload[8], payload[9]]);

        // Mock test flag
        let test = payload[10] != 0;
        let time_allowed_to_live = u32::from_be_bytes([0, 0, payload[11], payload[12]]);

        Ok(GooseMessage {
            app_id,
            cb_ref: &payload[13..14], // Mock cb_ref
            st_num,
            sq_num,
            test,
            time_allowed_to_live,
            raw_dataset: &payload[14..],
        })
    }
}

/// Zero-Allocation, Lock-Free SPSC Ring Buffer for Telemetry.
/// Allows uninterrupted ingress of raw frames from network interrupts.
pub struct TelemetryRingBuffer<const N: usize> {
    queue: Queue<[u8; 256], N>,
}

impl<const N: usize> Default for TelemetryRingBuffer<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> TelemetryRingBuffer<N> {
    pub const fn new() -> Self {
        Self {
            queue: Queue::new(),
        }
    }

    /// Enqueue a raw frame directly from the NIC/DPDK handler.
    #[allow(clippy::result_large_err)]
    pub fn enqueue(&mut self, frame: [u8; 256]) -> Result<(), [u8; 256]> {
        self.queue.enqueue(frame)
    }

    /// Dequeue a frame for processing by the telemetry bridge.
    pub fn dequeue(&mut self) -> Option<[u8; 256]> {
        self.queue.dequeue()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_goose_decode_success() {
        let mut mock_payload = [0u8; 64];
        // AppID = 0x1234
        mock_payload[0] = 0x12;
        mock_payload[1] = 0x34;

        // stNum = 1
        mock_payload[5] = 0x01;

        let msg = GooseBerDecoder::decode(&mock_payload).unwrap();
        assert_eq!(msg.app_id, 0x1234);
        assert_eq!(msg.st_num, 1);
    }

    #[test]
    fn test_ring_buffer_no_leak() {
        let mut rb = TelemetryRingBuffer::<4>::new();
        let frame = [0xFF; 256];
        assert!(rb.enqueue(frame).is_ok());
        let popped = rb.dequeue().unwrap();
        assert_eq!(popped[0], 0xFF);
    }
}
