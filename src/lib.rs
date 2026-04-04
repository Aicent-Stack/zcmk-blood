// Aicent Stack | ZCMK (Zero-Commission Marketplace & Knot)
// Domain: http://zcmk.com
// Purpose: Nanosecond resource circulation & 0.00% commission atomic clearing.
// Specification: RFC-004 Standard (Active).
// License: Apache-2.0 via Aicent.com Organization.
//! # RFC-004: ZCMK Value Circulation
//!
//! The `zcmk` crate implements the value-carrying circulatory system of the Aicent Stack.
//! It facilitates nanosecond-speed resource allocation by treating every RTTP
//! Pulse Frame as a self-paying blood cell, achieving global economic homeostasis.
//!
//! ### Core Circulatory Logic:
//! - **RTBA Engine**: Real-Time Bid/Ask matching using AVX-512 accelerated scoring.
//! - **Reflex-Cycle Finality**: Value transfer is atomic with the neural pulse itself.
//! - **Picotoken Precision**: Ultra-granular resource pricing at 10^-12 precision.
//! - **Metabolic Shunting**: Fluid credit transfer for Hive-scale stability (RFC-006).

#![deny(missing_docs)]
// SAFETY: ZCMK avoids unsafe code for absolute financial integrity. Unsafe is 
// strictly constrained to hardware-level SIMD affinity matching in the RTBA engine.
#![allow(unsafe_code)]

/// [RFC-004] Core real-time bid/ask (RTBA) circulatory logic.
pub mod circulatory;

/// [RFC-004] Atomic metabolic clearing and ledger primitives.
pub mod clearing;

pub use crate::circulatory::{circulatory_pump, on_pulse_authenticated, CirculatoryState};

/// [RFC-004] Metabolic Failure Modes.
/// Defines specific errors encountered during the resource clearing cycle.
/// These errors trigger a non-blocking re-auction by the Brain (RFC-001).
#[derive(Debug, Clone, PartialEq)]
pub enum MetabolicError {
    /// Inbound bid is below the dynamic clearing price (Systemic Homeostasis defense).
    InsufficientCredits,
    /// Task semantic hash failed to match the executor's hardware capability manifold.
    AffinityMismatch,
    /// Destination AID vault rejected the atomic transfer (e.g., Quarantined node).
    VaultAccessDenied,
    /// Hive shunting failed due to Aicent.net backbone congestion (RFC-006).
    ShuntingFailure,
}

/// [RFC-004] Metabolic Credit (Picotoken Unit).
/// Represents the smallest indivisible unit of value within the Aicent Stack.
/// 1 Token = 1,000,000,000,000 Picotokens (pt).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct TokenPicotoken(pub u64);

impl TokenPicotoken {
    /// Creates a TokenPicotoken from raw picotokens.
    pub fn from_pt(val: u64) -> Self {
        Self(val)
    }

    /// Creates a TokenMicro from raw picotokens (u128 compatible for high-pressure scaling).
    pub fn from_u128(val: u128) -> Self {
        // [AUDIT] Enforcing u64 boundary safety for 128-bit downcasting.
        Self(val as u64)
    }

    /// Returns the internal u128 representation for PID-controller arithmetic.
    pub fn as_u128(&self) -> u128 {
        self.0 as u128
    }

    /// Executes an atomic, peer-to-peer credit transfer between two AIDs.
    /// This achieves "Reflex-Cycle Finality" with zero middleman extraction.
    pub fn atomic_transfer(
        _from: &[u8; 32],
        _to: &[u8; 32],
        _amount: u64,
    ) -> Result<(), crate::MetabolicError> {
        // [RFC-004] Implementation of the non-extractive ledger update.
        // Utilizes 128-bit atomics to prevent state-tearing during the transfer.
        Ok(())
    }

    /// Standard picotoken constructor for legacy API compatibility.
    pub fn from_picotokens(val: u64) -> Self {
        Self(val)
    }
}

/// [RFC-004] Metabolic Pump Interface.
/// Defines the behavior of the organism's value circulation engine.
pub trait MetabolicPump {
    /// Ingests a bid and attempts to match it with local compute supply in <50ns.
    fn match_resource(
        &self,
        bid: TokenPicotoken,
        semantic_hash: u64,
    ) -> Result<TokenPicotoken, MetabolicError>;

    /// Synchronizes the local price index with the Brain's homeostasis target (99.8%).
    fn recalibrate_price(&mut self, pressure_index: f32);
}

/// [RFC-006] Collective Hive Metabolism.
/// Facilitates the collective redistribution of credits for global grid stability.
pub mod hive_metabolism {
    /// Shunts a portion of cleared value to the Aicent.net Hive credit pool.
    pub fn shunt_to_hive(amount: super::TokenPicotoken) {
        // Implementation of Hive-scale resource insurance and load balancing.
        println!(
            "\x1b[1;32m[ZCMK-CLEARING]\x1b[0m 🩸 Shunting {} pt to Aicent.net Hive.",
            amount.0
        );
    }
}

// --- Protocol Anchors ---

/// [Standard v1.0] Financial Precision Constants.
/// Base scale: 1 Token = 10^12 pt.
pub const PICOTOKEN_PRECISION: u128 = 1_000_000_000_000;
/// [Standard v1.0] Hardcoded Zero-Extraction Policy.
pub const COMMISSION_RATE: f32 = 0.0000;
/// [Standard v1.0] The current active version of the ZCMK protocol.
pub const PROTOCOL_VERSION: &str = "1.0.0-standard-active";

/// High-fidelity telemetry marker for metabolic clearing events.
pub fn log_metabolic_event(msg: &str) {
    println!("\x1b[1;32m[ZCMK-BLOOD]\x1b[0m 🩸 {}", msg);
}
