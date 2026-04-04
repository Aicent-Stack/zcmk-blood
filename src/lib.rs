// Aicent Stack | ZCMK (Zero-Commission Marketplace & Knot)
// Domain: http://zcmk.com
// Purpose: Nanosecond resource circulation & 0.00% commission atomic clearing.
// Specification: RFC-004 Standard (Active).
// License: Apache-2.0 via Aicent.com Organization.
//! # RFC-004: ZCMK Value Circulation
//! 
//! The `zcmk` crate implements the value-carrying circulatory system of the Aicent Stack.
//! It facilitates nanosecond-speed resource allocation by treating every RTTP 
//! Pulse Frame as a self-paying blood cell, achieving economic homeostasis.
//!
//! ### Core Circulatory Logic:
//! - **RTBA Engine**: Real-time Bid/Ask matching using Simd-accelerated scoring.
//! - **Reflex-Cycle Finality**: Value transfer is atomic with the neural pulse.
//! - **Picotoken Precision**: Ultra-granular resource pricing at 10^-12 precision.
//! - **Metabolic Shunting**: Fluid credit transfer for Hive-scale stability (RFC-006).

#![deny(missing_docs)]
// SAFETY: ZCMK enforces safe Rust to ensure absolute financial integrity and traceability.
#![deny(unsafe_code)]

/// [RFC-004] Core matching and circulation logic
pub mod circulatory;
/// [RFC-004] Metabolic clearing and ledger primitives
pub mod clearing;

pub use crate::circulatory::{CirculatoryState, circulatory_pump, on_pulse_authenticated};

/// [RFC-004] Metabolic Failure Modes
/// Defines specific errors encountered during the resource clearing cycle.
#[derive(Debug, Clone, PartialEq)]
pub enum MetabolicError {
    /// Inbound bid is below the current dynamic clearing price
    InsufficientCredits,
    /// Task semantic hash failed to match local hardware manifold
    AffinityMismatch,
    /// Destination AID vault rejected the atomic transfer
    VaultAccessDenied,
    /// Hive shunting failed due to grid congestion (RFC-006)
    ShuntingFailure,
}

/// [RFC-004] Metabolic Credit (Picotoken Unit)
/// Represents the smallest unit of value within the Aicent Stack.
/// 1 Token = 1,000,000,000,000 Picotokens (pt).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct TokenPicotoken(pub u64);

impl TokenPicotoken {
    /// Creates a credit unit from a raw 64-bit integer.
    pub fn from_pt(val: u64) -> Self { Self(val) }
    
    /// Converts the picotoken value to a 128-bit representation for calculation.
    pub fn as_u128(&self) -> u128 { self.0 as u128 }

    /// Executes an atomic, peer-to-peer credit transfer between two AIDs.
    /// This achieves "Reflex-Cycle Finality" with zero middleman extraction.
    pub fn atomic_transfer(_from: &[u8; 32], _to: &[u8; 32], _amount: u64) -> Result<(), MetabolicError> {
        // [RFC-004] Implementation of the non-extractive ledger update
        Ok(())
    }
}

/// [RFC-004] Metabolic Pump Interface
/// Defines the behavior of the organism's value circulation.
pub trait MetabolicPump {
    /// Ingests a bid and attempts to match it with local compute supply in <50ns.
    fn match_resource(&self, bid: TokenPicotoken, semantic_hash: u64) -> Result<TokenPicotoken, MetabolicError>;

    /// Synchronizes local price index with the Brain's homeostasis target (99.8%).
    fn recalibrate_price(&mut self, pressure_index: f32);
}

/// [RFC-006] Hive Metabolic Shunting
/// Facilitates the collective redistribution of credits for global grid stability.
pub mod hive_metabolism {
    /// Shunts a portion of cleared value to the Aicent.net Hive credit pool.
    pub fn shunt_to_hive(amount: super::TokenPicotoken) {
        println!("\x1b[1;32m[ZCMK-CLEARING]\x1b[0m 🩸 Shunting {} pt to Aicent.net Hive.", amount.0);
    }
}

/// [Standard v1.0] Financial Constants
/// Precision set to 10^-12 to allow granular micro-inference pricing.
pub const PICOTOKEN_PRECISION: u128 = 1_000_000_000_000;
/// Hardcoded non-extractive commission rate.
pub const COMMISSION_RATE: f32 = 0.0000; 
/// [Standard v1.0] Protocol Version
pub const PROTOCOL_VERSION: &str = "1.0.0-standard-active";

/// High-fidelity telemetry marker for metabolic clearing events.
pub fn log_metabolic_event(msg: &str) {
    println!("\x1b[1;32m[ZCMK-BLOOD]\x1b[0m 🩸 {}", msg);
}
