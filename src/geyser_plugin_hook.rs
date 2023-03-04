use {
    log::*,
    solana_geyser_plugin_interface::geyser_plugin_interface::{
        GeyserPlugin, GeyserPluginError, ReplicaAccountInfoVersions, ReplicaBlockInfoVersions,
        ReplicaTransactionInfoVersions, SlotStatus,
    },
    std::fmt::{Debug, Formatter},
};

pub struct GeyserPluginHook {}

impl GeyserPlugin for GeyserPluginHook {
    fn name(&self) -> &'static str {
        "GeyserPluginHook"
    }

    fn on_load(
        &mut self,
        config_file: &str,
    ) -> solana_geyser_plugin_interface::geyser_plugin_interface::Result<()> {
        solana_logger::setup_with_default("info");
        info!("[on_load] - config_file: {:#?}", config_file);

        Ok(())
    }

    fn notify_transaction(
        &mut self,
        transaction: ReplicaTransactionInfoVersions,
        slot: u64,
    ) -> solana_geyser_plugin_interface::geyser_plugin_interface::Result<()> {
        match transaction {
            ReplicaTransactionInfoVersions::V0_0_1(tx) => {
                info!("tx signature {:?}, {:?}", tx.signature.to_string(), tx.transaction_status_meta.status);
                // info!("tx status {:?}", tx.transaction_status_meta);
            }
        }

        Ok(())
    }

    fn account_data_notifications_enabled(&self) -> bool {
        true
    }

    fn transaction_notifications_enabled(&self) -> bool {
        true
    }
}

/// Also required by GeyserPlugin trait
impl Debug for GeyserPluginHook {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "GeyserPluginHook")
    }
}
