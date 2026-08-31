# GridBus-Core (SubstationBridge-SDK)

**Enterprise-grade, ultra-low-latency embedded Protocol Gateway & Telemetry Bridge SDK** for high-voltage substations. Ingests raw IEC 61850-9-2 Sampled Values (SV) and IEC 61850-8-1 GOOSE multicast Ethernet frames with **zero packet loss** and **<1ms processing latency**, bridging them into PTP-timestamped modern SCADA/Cloud telemetry (MQTT/gRPC/JSON).

[![License: AGPLv3](https://img.shields.io/badge/License-AGPLv3-blue.svg)](LICENSE-COMMUNITY)
[![License: Commercial](https://img.shields.io/badge/License-Commercial-red.svg)](https://polar.sh/GridBus/subscriptions)

---

## ⚡ 3-Line Quickstart

Embed GridBus directly into your Rust power equipment controller:

```rust
// 1. Initialize the Enterprise Telemetry Bridge (zero-allocation ring buffer)
let mut bridge = gridbus_enterprise::EnterpriseTelemetryBridge::<1024>::new();

// 2. Ingest raw AF_PACKET/DPDK GOOSE frame directly from NIC
bridge.ingest_dpdk_frame(raw_ethernet_bytes).expect("Buffer full");

// 3. Extract parsed ASN.1 message + IEEE 1588 PTP nanosecond timestamp
let telemetry = bridge.process_next().unwrap();
println!("GOOSE stNum: {}, PTP Sync: {} ns", telemetry.goose.st_num, telemetry.hw_timestamp.nanoseconds);
```

---

## 🏗️ Architecture & Hard Real-Time Latency Benchmarks

Built on **Rust (no-std)** and **C23**, GridBus guarantees bounded worst-case execution times by entirely avoiding dynamic memory allocation (`malloc`).

| Sub-system | Expected Latency | Description |
|---|---|---|
| **Raw L2 Ingestion (DPDK)** | < 10 μs | Bypass Linux kernel networking stack entirely. |
| **GOOSE ASN.1 BER Decoder** | < 250 μs | Deterministic bit-field decoder parsing `stNum`, `sqNum`, datasets. |
| **IEEE 1588 PTP Sync** | < 5 μs | Hardware NIC timestamp extraction. |
| **Telemetry Serialization** | < 150 μs | Zero-copy Cap'n Proto / FlatBuffers creation. |
| **Total End-to-End Latency** | **< 1 ms** | **Strictly bounded deterministic pipeline.** |

### IEC Standards Compliance Matrix

- **IEC 61850-8-1:** GOOSE (Generic Object Oriented Substation Events) parsing.
- **IEC 61850-9-2:** Sampled Values (SV) high-rate un-packing.
- **IEEE 1588v2 (IEC 61850-9-3):** Power Utility Profile PTP timestamping.
- **IEC 62351:** Substation Cybersecurity (MAC authentication hooks).

---

## 💎 Dual-Licensing: Community vs. Enterprise Pro

GridBus follows an **Open-Core** model. 

| Feature | Community Edition (Free) | Enterprise Pro Tier |
|---|---|---|
| **License** | AGPLv3 | Proprietary / Commercial B2B |
| **GOOSE Parser** | Basic ASN.1 BER Decoder | Highly-optimized zero-copy decoder |
| **Timestamping** | Software Time (NTP/OS) | **IEEE 1588 PTP Hardware Sync (ns)** |
| **Ingestion** | Standard Sockets (AF_PACKET) | **DPDK / XDP BPF Acceleration** |
| **Sampled Values (SV)** | ❌ No | ✅ IEC 61850-9-2 Multi-Channel Demux |
| **Cybersecurity** | ❌ No | ✅ IEC 62351 MAC Verification |
| **Telemetry Exporter**| Basic MQTT (JSON) | gRPC, Protobuf, FlatBuffers (Zero-Copy) |
| **Validation / Support**| Community Forums | Offline Ed25519 License + SLA |

### 🛒 Ready for Production?
**[Unlock the Enterprise Pro SDK via Polar.sh ($2,400/yr per seat) ➔](https://polar.sh/GridBus/subscriptions)**

*(Provides access to the private `gridbus-core-enterprise` Git repository containing DPDK extensions, SV Demuxer, and Offline License Validator).*
