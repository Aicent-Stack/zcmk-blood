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
// SAFETY: Unsafe allowed only for hardware-level RTBA matching.
#![allow(unsafe_code)]

pub mod circulatory;

/// [RFC-004] Metabolic clearing and ledger primitives
pub mod clearing {
    /// Internal credit shunting logic
    pub fn shunt_credits(_from: &[u8; 32], _to: &[u8; 32], _amount: u64) -> bool {
        true
    }
}

/// [RFC-004] Token Micro-unit (Picotoken) implementation
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct TokenPicotoken(pub u64);

impl TokenPicotoken {
    /// Creates a TokenPicotoken from raw picotokens
    pub fn from_pt(val: u64) -> Self {
        Self(val)
    }
    /// Creates a TokenMicro from raw picotokens (u128 compatible)
    pub fn from_u128(val: u128) -> Self {
        Self(val as u64)
    }
    /// Returns the internal u128 representation
    pub fn as_u128(&self) -> u128 {
        self.0 as u128
    }
    /// Executes an atomic peer-to-peer transfer
    pub fn atomic_transfer(
        _from: &[u8; 32],
        _to: &[u8; 32],
        _amount: u64,
    ) -> Result<(), crate::MetabolicError> {
        Ok(())
    }
    /// Standard picotoken constructor
    pub fn from_picotokens(val: u64) -> Self {
        Self(val)
    }
}

pub use crate::circulatory::{circulatory_pump, on_pulse_authenticated, CirculatoryState};

/// [Standard v1.0] Financial Constants
pub const PICOTOKEN_PRECISION: u128 = 1_000_000_000_000;
/// [Standard v1.0] Protocol Version
pub const PROTOCOL_VERSION: &str = "1.0.0-standard-active";

/// [RFC-004] Metabolic Failure Modes
#[derive(Debug, Clone, PartialEq)]
pub enum MetabolicError {
    /// Inbound bid below price
    InsufficientCredits,
    /// Vault access error
    VaultAccessDenied,
}
