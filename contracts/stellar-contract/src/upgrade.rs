use soroban_sdk::{Address, Env, String, Vec};

use crate::errors::SettleError;
use crate::authorization::Authorization;
use crate::storage::DataKey;
use crate::events::EventBuilder;

/// Contract upgrade and versioning management
pub struct UpgradeManager;

impl UpgradeManager {
    /// Get current contract version
    pub fn get_version(env: &Env) -> u32 {
        env.storage()
            .persistent()
            .get(&DataKey::ContractVersion)
            .unwrap_or(1)
    }
    
    /// Set contract version (admin only)
    pub fn set_version(env: &Env, caller: &Address, version: u32) -> Result<(), SettleError> {
        Authorization::require_admin(env, caller)?;
        
        let current_version = Self::get_version(env);
        if version <= current_version {
            return Err(SettleError::SystemError);
        }
        
        env.storage()
            .persistent()
            .set(&DataKey::ContractVersion, &version);
            
        Ok(())
    }
    
    /// Check if contract supports a specific feature version
    pub fn supports_feature(env: &Env, feature_version: u32) -> bool {
        Self::get_version(env) >= feature_version
    }
    
    /// Get contract metadata
    pub fn get_metadata(env: &Env) -> ContractMetadata {
        ContractMetadata {
            version: Self::get_version(env),
            name: String::from_str(env, "Settle Protocol"),
            description: String::from_str(env, "Programmable Settlement Infrastructure"),
            deployed_at: env.storage()
                .persistent()
                .get(&DataKey::DeployedAt)
                .unwrap_or(env.ledger().timestamp()),
        }
    }
    
    /// Initialize contract deployment metadata
    pub fn initialize_deployment(env: &Env, admin: &Address) -> Result<(), SettleError> {
        Authorization::require_admin(env, admin)?;
        
        // Set deployment timestamp
        env.storage()
            .persistent()
            .set(&DataKey::DeployedAt, &env.ledger().timestamp());
            
        // Set initial version
        env.storage()
            .persistent()
            .set(&DataKey::ContractVersion, &1u32);
            
        Ok(())
    }
    
    /// Pause contract operations (emergency use only)
    pub fn pause_contract(env: &Env, admin: &Address) -> Result<(), SettleError> {
        Authorization::require_admin(env, admin)?;
        
        env.storage()
            .persistent()
            .set(&DataKey::ContractPaused, &true);
            
        Ok(())
    }
    
    /// Resume contract operations
    pub fn resume_contract(env: &Env, admin: &Address) -> Result<(), SettleError> {
        Authorization::require_admin(env, admin)?;
        
        env.storage()
            .persistent()
            .set(&DataKey::ContractPaused, &false);
            
        Ok(())
    }
    
    /// Check if contract is paused
    pub fn is_paused(env: &Env) -> bool {
        env.storage()
            .persistent()
            .get(&DataKey::ContractPaused)
            .unwrap_or(false)
    }
    
    /// Require contract is not paused (for state-changing operations)
    pub fn require_not_paused(env: &Env) -> Result<(), SettleError> {
        if Self::is_paused(env) {
            return Err(SettleError::SystemError);
        }
        Ok(())
    }
}

/// Contract metadata structure
#[derive(Clone, Debug)]
pub struct ContractMetadata {
    pub version: u32,
    pub name: String,
    pub description: String,
    pub deployed_at: u64,
}

/// Migration utilities for contract upgrades
pub struct MigrationManager;

impl MigrationManager {
    /// Run migration for version upgrade
    pub fn migrate_to_version(
        env: &Env,
        admin: &Address,
        target_version: u32,
    ) -> Result<(), SettleError> {
        Authorization::require_admin(env, admin)?;
        
        let current_version = UpgradeManager::get_version(env);
        
        if target_version <= current_version {
            return Err(SettleError::SystemError);
        }
        
        // Run version-specific migrations
        for version in (current_version + 1)..=target_version {
            Self::run_migration_for_version(env, version)?;
        }
        
        // Update version
        UpgradeManager::set_version(env, admin, target_version)?;
        
        Ok(())
    }
    
    /// Run specific version migration logic
    fn run_migration_for_version(env: &Env, version: u32) -> Result<(), SettleError> {
        match version {
            2 => {
                // Example migration for version 2
                // Add new fields, update data structures, etc.
                Ok(())
            },
            3 => {
                // Example migration for version 3
                Ok(())
            },
            _ => {
                // No migration needed for this version
                Ok(())
            }
        }
    }
    
    /// Check if migration is needed
    pub fn needs_migration(env: &Env, target_version: u32) -> bool {
        UpgradeManager::get_version(env) < target_version
    }
    
    /// Get migration steps needed
    pub fn get_migration_steps(env: &Env, target_version: u32) -> Vec<u32> {
        let current_version = UpgradeManager::get_version(env);
        if target_version <= current_version {
            return Vec::new(env);
        }

        let mut steps = Vec::new(env);
        for version in (current_version + 1)..=target_version {
            steps.push_back(version);
        }
        steps
    }
}