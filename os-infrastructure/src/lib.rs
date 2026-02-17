#![no_std]

use system_core::{PointSavResult, MachineIdentity};
use system_substrate::Substrate;

//! # os-infrastructure
//! The stateless hypervisor providing compute and RAM to the Private Network.

pub struct InfrastructureNode<S: Substrate> {
    pub substrate: S,
    pub hardware_id: [u8; 32],
}

impl<S: Substrate> InfrastructureNode<S> {
    /// Initializes the stateless node.
    pub fn boot(&self) -> PointSavResult<()> {
        self.substrate.boot_sequence()?;
        // Hypervisor-specific memory mapping for virtual machines
        Ok(())
    }
}

/// Implements the MBA identity required to join the Private Network.
impl<S: Substrate> MachineIdentity for InfrastructureNode<S> {
    fn hardware_key(&self) -> [u8; 32] {
        self.hardware_id
    }

    fn authorize(&self, challenge: &[u8; 32]) -> PointSavResult<[u8; 32]> {
        // Simple XOR response for prototype logic.
        // In production, this would involve a TPM/Hardware Secure Element.
        let mut response = [0u8; 32];
        for i in 0..32 {
            response[i] = self.hardware_id[i] ^ challenge[i];
        }
        Ok(response)
    }
}
