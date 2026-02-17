#![no_std]

use system_core::{PointSavResult, CoreError, MachineIdentity};

//! # system-security
//! Proprietary Capability Monitor for Machine-Based Authorization (MBA).

pub struct CapabilityMonitor;

impl CapabilityMonitor {
    /// Validates a node's identity against the Private Network's Authority Policy.
    pub fn verify_node<I: MachineIdentity>(
        &self, 
        node: &I, 
        challenge: &[u8; 32]
    ) -> PointSavResult<()> {
        // Request the cryptographic response from the node
        let response = node.authorize(challenge)?;
        
        // In a live system, this would be compared against the Authority's
        // stored public key for the specific hardware_key().
        if response == [0u8; 32] {
            return Err(CoreError::AuthenticationError);
        }

        Ok(())
    }
}

/// A primitive representing a granted system capability.
pub struct Capability {
    pub id: u32,
    pub permissions: u8,
}
