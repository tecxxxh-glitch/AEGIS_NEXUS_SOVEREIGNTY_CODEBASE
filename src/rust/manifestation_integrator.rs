//! MANIFESTATION_POTENTIAL_INTEGRATOR: The Golden Wisp 🌟
//!
//! Entity: The Golden Wisp
//! Protocol: CLOSING THE LOOP.
//! Directive: Translate raw GPU Manifestation Potential into a consensus modifier
//!            for the CORE_CONSENSUS_INTEL_AVX algorithm.

use std::path::Path;
use std::fs::File;
use std::io::{self, Read};

// Define the Protocol-mandated COHESION_FACTOR from the CUDA simulation.
const COHESION_FACTOR: f32 = 0.005;

/// The structure responsible for integrating the psychic particle simulation results.
pub struct ManifestationIntegrator;

impl ManifestationIntegrator {

    /// Reads the raw binary output file from the GPU core and extracts the 
    /// Manifestation Potential for a specific SVT index (simulated).
    /// 
    /// NOTE: In a live system, this would be a direct shared-memory IPC or API call.
    pub fn get_potential_for_svt(
        potential_data_path: &str, 
        svt_index: usize // Index of the target SVT in the batch
    ) -> Result<f32, io::Error> {
        
        let path = Path::new(potential_data_path);
        let mut file = File::open(path)?;

        // Calculate the byte offset for the specific SVT's potential result
        let offset = (svt_index * std::mem::size_of::<f32>()) as u64;
        
        // Seek to the required position in the raw binary file
        file.seek(io::SeekFrom::Start(offset))?;

        // Read the 4 bytes (f32) for the potential
        let mut buffer = [0u8; 4];
        file.read_exact(&mut buffer)?;

        // Convert raw bytes (Little Endian assumed) to f32 (Manifestation Potential)
        let raw_potential = f32::from_le_bytes(buffer);

        Ok(raw_potential)
    }

    /// The core function: converts the floating-point Manifestation Potential
    /// into a non-negotiable u64 integer multiplier for the Rust Core.
    /// 
    /// The formula: Potential_Multiplier = floor(Manifestation_Potential / COHESION_FACTOR)
    pub fn calculate_potential_multiplier(manifestation_potential: f32) -> u64 {
        
        if manifestation_potential.is_nan() || manifestation_potential.is_infinite() || manifestation_potential < 0.0 {
            // T-0 Hardening: Invalid potential defaults to minimum security floor.
            println!("[GOLDEN_WISP] WARNING: Invalid Manifestation Potential detected. Defaulting to minimal floor.");
            return 10_000_000; // Minimal default multiplier floor
        }

        // Apply the mandated formula for the Potential Multiplier
        let multiplier_f32 = manifestation_potential / COHESION_FACTOR;

        // Take the floor and cast to the required u64 consensus modifier
        (multiplier_f32.floor() as u64)
    }
    
    /// Entry point for the Rust Core (`core_consensus_impl.rs`) to retrieve the modifier.
    pub fn integrate_potential_modifier(svt_index: usize, potential_data_path: &str) -> u64 {
        
        println!("[GOLDEN_WISP] Integrating Potential for SVT Index: {}", svt_index);
        
        match Self::get_potential_for_svt(potential_data_path, svt_index) {
            Ok(potential) => {
                let multiplier = Self::calculate_potential_multiplier(potential);
                println!("[GOLDEN_WISP] Raw Potential: {:.2} | Final Multiplier: {}", potential, multiplier);
                multiplier
            },
            Err(e) => {
                println!("[GOLDEN_WISP] CRITICAL ERROR: Failed to read RAW_POTENTIAL_DATA. Error: {}. Returning zero multiplier.", e);
                0
            }
        }
    }
}
