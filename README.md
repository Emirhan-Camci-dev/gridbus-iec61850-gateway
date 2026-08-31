# DealShield (PitchRedact-Core)

**Client-side Air-Gapped Investment Pitch & IP Redaction SDK** designed for VC partners, M&A Lawyers, and Due Diligence teams. Automatically masks proprietary IP, financial models, and patent snippets inside multi-page PDFs in **<50ms** completely offline.

[![License: AGPLv3](https://img.shields.io/badge/License-AGPLv3-blue.svg)](LICENSE-COMMUNITY)
[![License: Commercial](https://img.shields.io/badge/License-Commercial-red.svg)](https://polar.sh/DealShield/subscriptions)

---

## ⚡ 3-Line Quickstart

Embed DealShield directly into your client-side Web/Electron or Native app:

```rust
// 1. Initialize the Enterprise Redaction Engine & Vault
let mut engine = dealshield_enterprise::FinancialRedactionEngine::new();

// 2. Ingest multi-page Pitch Deck PDF bytes (Sub-50ms processing)
let (sanitized_text, vault_size) = engine.sanitize_pitch_deck(raw_pdf_bytes);

// 3. Export clean tokens for Safe LLM Analysis
println!("Sanitized Text: {}\nVault Items: {}", sanitized_text, vault_size);
```

---

## 🏗️ Architecture & Performance Benchmarks

Built on **Rust / C++** and compiled to **WASM & Native C-ABI**, DealShield guarantees bounded worst-case execution times by operating strictly offline.

| Sub-system | Expected Latency | Description |
|---|---|---|
| **PDF Stream Parser** | < 15 ms | Strips underlying vector text layers & hidden fonts. |
| **Financial / IP NLP Masker** | < 25 ms | Entity recognition for Valuations, Cap Tables, Source Code. |
| **Bounding Box Obfuscator** | < 10 ms | Zero-copy cryptographic pixel blurring. |
| **Total End-to-End Latency** | **< 50 ms** | **Air-gapped execution.** |

---

## 💎 Dual-Licensing: Community vs. Fund Enterprise Tier

DealShield follows an **Open-Core** model. 

| Feature | Community Edition (Free) | Fund Enterprise Tier |
|---|---|---|
| **License** | AGPLv3 | Proprietary / Commercial B2B |
| **Basic Regex PII** | ✅ Yes (Emails, Phones) | ✅ Yes |
| **PDF Vector Sanitization** | ❌ No (Raster Only) | ✅ Multi-page Stream Stripper |
| **Financial/Cap Table NLP** | ❌ No | ✅ EBITDA, Ownership %, Valuations |
| **Tech IP & Patent Masker** | ❌ No | ✅ Code Snippets, Math Formulas |
| **De-anonymization Vault** | ❌ No | ✅ Reversible Vaults for Local Analysis |
| **Batch CLI Processing**| ❌ No | ✅ Yes |
| **Validation / Support**| Community Forums | Offline Ed25519 License + SLA |

### 🛒 Ready for Production?
**[Unlock the Fund Enterprise SDK via Polar.sh ($990/yr per fund) ➔](https://polar.sh/DealShield/subscriptions)**

*(Provides access to the private `dealshield-enterprise` Git repository containing PDF vector stripping, Financial NLP, and Offline License Validator).*
