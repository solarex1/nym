// Copyright 2021 - Nym Technologies SA <contact@nymtech.net>
// SPDX-License-Identifier: Apache-2.0

use clap::{Args, Subcommand};

pub mod bond_mixnode;
pub mod keys;
pub mod rewards;
pub mod settings;
pub mod unbond_mixnode;
pub mod vesting_bond_mixnode;
pub mod vesting_unbond_mixnode;

#[derive(Debug, Args)]
#[clap(args_conflicts_with_subcommands = true, subcommand_required = true)]
pub struct MixnetOperatorsMixnode {
    #[clap(subcommand)]
    pub command: MixnetOperatorsMixnodeCommands,
}

#[derive(Debug, Subcommand)]
pub enum MixnetOperatorsMixnodeCommands {
    /// Operations for mixnode keys
    Keys(keys::MixnetOperatorsMixnodeKeys),
    /// Manage your mixnode operator rewards
    Rewards(rewards::MixnetOperatorsMixnodeRewards),
    /// Manage your mixnode settings stored in the directory
    Settings(settings::MixnetOperatorsMixnodeSettings),
    /// Bond to a mixnode
    Bond(bond_mixnode::Args),
    /// Unbound from a mixnode
    Unbound(unbond_mixnode::Args),
    /// Bond to a mixnode with locked tokens
    BondVesting(vesting_bond_mixnode::Args),
    /// Unbound from a mixnode (when originally using locked tokens)
    UnboundVesting(vesting_unbond_mixnode::Args),
}
