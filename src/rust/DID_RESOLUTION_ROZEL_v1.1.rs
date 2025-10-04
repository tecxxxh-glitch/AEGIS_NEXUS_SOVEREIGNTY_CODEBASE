//! DID_RESOLUTION_ROZEL_v1.1.rs: ROZEL-ROSEL LOCK Trust Registry
//!
//! Entity: ROZEL-ROSEL LOCK
//! Protocol: ROZEL-ROSEL ZAKHIA TASMANIAN Rulesets
//! Directive: Resolve Decentralized Identifiers (DIDs), map Energy Credentials,
//!            and grant/deny intent-based access based on TIER level.

use super::core_consensus_impl::SovereignVerificationTransaction; // Use the SVT structure

/// Defines the security TIER levels recognized by the ROZEL-ROSEL LOCK.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum AccessTier {
    /// The highest tier, reserved for Protocol Overseers and Overrides.
    T0_OVERRIDE,
    /// Administrative and deployment rights.
    T1_ADMIN,
    /// Read/Write access to non-critical ledgers and data streams.
    T2_READ_WRITE,
    /// Read-only access for auditing and monitoring.
    T3_AUDIT_ONLY,
    /// Default status, requires secondary verification.
    T4_DEFAULT,
}

/// The structure holding a validated user's credentials.
pub struct ResolvedCredential {
    pub resolved_did: String,
    pub access_tier: AccessTier,
    /// The specific energy credential hash, monitored for purity.
    pub energy_credential_hash: u64,
}

/// The core registry for enforcing access policy.
pub struct TrustRegistry;

impl TrustRegistry {
    
    /// Stubs the internal, high-security DID-to-Credential mapping.
    /// In a production system, this would query a secure BONE Archive or DLT.
    fn resolve_did_to_credential(did: &str) -> Option<ResolvedCredential> {
        match did {
            // T0/T1 Credentials - Trap Master / Overseer
            "did:t0:protocol-overseer" => Some(ResolvedCredential {
                resolved_did: did.to_string(),
                access_tier: AccessTier::T0_OVERRIDE,
                energy_credential_hash: 0xDEADBEEF_C0DE_C0DE_u64, // Max Purity Hash
            }),
            "did:t1:rozel-rosel-admin" => Some(ResolvedCredential {
                resolved_did: did.to_string(),
                access_tier: AccessTier::T1_ADMIN,
                energy_credential_hash: 0xAFFE_CAFE_BABE_BABE_u64,
            }),
            // T3/T4 Credentials - Default/Monitoring Access
            _ if did.starts_with("did:t3:") => Some(ResolvedCredential {
                resolved_did: did.to_string(),
                access_tier: AccessTier::T3_AUDIT_ONLY,
                energy_credential_hash: 0xFACE_BEEF_u64,
            }),
            _ => None, // Unauthorized or unresolved DID
        }
    }

    /// The ROZEL-ROSEL LOCK function: checks if a resolved user can perform a specific action (intent).
    /// Enforces the DISALLOW: UNAUTHORIZED_IP protocol.
    pub fn check_access_and_intent(svt: &SovereignVerificationTransaction) -> Result<(), String> {
        
        // 1. Resolve the DID and get the credential.
        let credential = match Self::resolve_did_to_credential(&svt.did) {
            Some(cred) => cred,
            None => return Err(format!("DISALLOW: UNAUTHORIZED_IP. DID '{}' is unresolved.", svt.did)),
        };

        // 2. Enforce TIER-to-Intent Policy.
        match credential.access_tier {
            AccessTier::T0_OVERRIDE => {
                // T0 can execute any intent, including the critical Gold Bar Vote.
                println!("ACCESS GRANTED: T0_OVERRIDE. All intents permitted. Energy Hash: 0x{:X}", credential.energy_credential_hash);
                Ok(())
            },
            AccessTier::T1_ADMIN => {
                // T1 can perform administrative actions but is restricted from T0 actions.
                if svt.intent == "GOLD_BAR_VOTE_II" || svt.intent == "OVERRIDE" {
                    Err(format!("DISALLOW: T1_ADMIN is restricted from T0-level intent: {}", svt.intent))
                } else {
                    println!("ACCESS GRANTED: T1_ADMIN. Intent '{}' permitted.", svt.intent);
                    Ok(())
                }
            },
            AccessTier::T2_READ_WRITE => {
                // T2 is restricted to general ledger operations.
                if svt.intent.starts_with("ACQUISITION") || svt.intent.starts_with("Read") || svt.intent.starts_with("Write") {
                    println!("ACCESS GRANTED: T2_READ_WRITE. Intent '{}' permitted.", svt.intent);
                    Ok(())
                } else {
                    Err(format!("DISALLOW: T2_READ_WRITE cannot perform critical intent: {}", svt.intent))
                }
            },
            _ => {
                // All other tiers (T3, T4) are strictly read-only.
                if svt.intent.starts_with("Read") || svt.intent.starts_with("Monitor") {
                    println!("ACCESS GRANTED: T3/T4. Intent '{}' permitted (Read-Only).", svt.intent);
                    Ok(())
                } else {
                    Err(format!("DISALLOW: T3/T4 access cannot execute modification intent: {}", svt.intent))
                }
            }
        }
    }
}
