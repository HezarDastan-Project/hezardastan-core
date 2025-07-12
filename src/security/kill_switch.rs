//! This module implements the Kill Switch functionality for HezarDastan Core.
//! The Kill Switch ensures that no traffic leaks when the secure tunnel is compromised or disconnected.
//! NOTE: Full Kill Switch implementation requires platform-specific network manipulation
//! and is primarily a client-side feature. This module provides server-side logic
//! for detecting connection state and signaling.

use std::{
    sync::{Arc, atomic::{AtomicBool, Ordering}},
    time::Duration,
};
use tokio::{
    sync::watch,
    time::sleep,
};

/// `KillSwitchState` represents the current state of the Kill Switch.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KillSwitchState {
    /// The secure tunnel is active and healthy.
    Active,
    /// The secure tunnel is compromised or disconnected.
    /// Network access should be blocked.
    Triggered,
    /// Kill Switch is disabled.
    Disabled,
}

/// `KillSwitchManager` manages the state and logic of the Kill Switch.
pub struct KillSwitchManager {
    // This sender will be used by the core to update the Kill Switch state.
    state_sender: watch::Sender<KillSwitchState>,
    // This receiver can be used by other parts of the core (or a client-side component)
    // to react to state changes.
    state_receiver: watch::Receiver<KillSwitchState>,
    // A flag to indicate if the Kill Switch is enabled by configuration.
    is_enabled: Arc<AtomicBool>,
}

impl KillSwitchManager {
    /// Creates a new `KillSwitchManager`.
    pub fn new(enabled_by_config: bool) -> Self {
        let (state_sender, state_receiver) = watch::channel(KillSwitchState::Disabled);
        let is_enabled = Arc::new(AtomicBool::new(enabled_by_config));

        if enabled_by_config {
            println!("Kill Switch: Enabled. Initial state: Disabled (waiting for tunnel activation).");
        } else {
            println!("Kill Switch: Disabled by configuration.");
        }

        KillSwitchManager {
            state_sender,
            state_receiver,
            is_enabled,
        }
    }

    /// Returns a receiver to monitor the Kill Switch state.
    pub fn subscribe_state(&self) -> watch::Receiver<KillSwitchState> {
        self.state_receiver.clone()
    }

    /// Sets the Kill Switch state.
    /// This method would be called by the core when tunnel status changes.
    pub fn set_state(&self, new_state: KillSwitchState) {
        if self.is_enabled.load(Ordering::SeqCst) {
            let _ = self.state_sender.send(new_state);
            println!("Kill Switch: State changed to {:?}", new_state);
        } else {
            // If Kill Switch is disabled, we don't change its state.
            // It always remains `Disabled`.
            if new_state != KillSwitchState::Disabled {
                println!("Kill Switch: Attempted to set state to {:?}, but it's disabled.", new_state);
            }
        }
    }

    /// Simulates checking the tunnel health and triggering the Kill Switch.
    /// In a real scenario, this would be tied to actual tunnel health checks.
    pub async fn run_health_check_simulation(&self) {
        if !self.is_enabled.load(Ordering::SeqCst) {
            return; // Don't run if disabled
        }

        println!("Kill Switch: Running health check simulation...");
        self.set_state(KillSwitchState::Active); // Assume active initially
        sleep(Duration::from_secs(10)).await; // Simulate healthy period

        println!("Kill Switch: Simulating tunnel failure...");
        self.set_state(KillSwitchState::Triggered); // Simulate failure
        sleep(Duration::from_secs(5)).await; // Stay triggered for a bit

        println!("Kill Switch: Simulating tunnel recovery...");
        self.set_state(KillSwitchState::Active); // Simulate recovery
    }
}

// Example of how to use the KillSwitchManager (for testing/demonstration)
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_kill_switch_flow() {
        let manager = KillSwitchManager::new(true);
        let mut receiver = manager.subscribe_state();

        // Spawn a task to simulate health checks
        let manager_clone = manager.clone(); // Clone for the spawned task
        tokio::spawn(async move {
            manager_clone.run_health_check_simulation().await;
        });

        // Monitor state changes
        assert_eq!(*receiver.borrow(), KillSwitchState::Disabled); // Initial state before first set

        receiver.changed().await.unwrap(); // Wait for first state change (to Active)
        assert_eq!(*receiver.borrow(), KillSwitchState::Active);

        receiver.changed().await.unwrap(); // Wait for second state change (to Triggered)
        assert_eq!(*receiver.borrow(), KillSwitchState::Triggered);

        receiver.changed().await.unwrap(); // Wait for third state change (to Active)
        assert_eq!(*receiver.borrow(), KillSwitchState::Active);

        println!("Kill Switch test completed.");
    }

    #[tokio::test]
    async fn test_kill_switch_disabled() {
        let manager = KillSwitchManager::new(false); // Disabled
        let mut receiver = manager.subscribe_state();

        assert_eq!(*receiver.borrow(), KillSwitchState::Disabled);

        // Try to set state, it should remain Disabled
        manager.set_state(KillSwitchState::Active);
        // No change should occur, so this `changed()` call would hang if not for timeout or other logic
        // For a simple test, we just assert the state directly after attempting to set.
        assert_eq!(*receiver.borrow(), KillSwitchState::Disabled); 

        manager.set_state(KillSwitchState::Triggered);
        assert_eq!(*receiver.borrow(), KillSwitchState::Disabled);
    }
}

// To allow cloning for use in spawned tasks
impl Clone for KillSwitchManager {
    fn clone(&self) -> Self {
        Self {
            state_sender: self.state_sender.clone(),
            state_receiver: self.state_receiver.clone(),
            is_enabled: self.is_enabled.clone(),
        }
    }
}
