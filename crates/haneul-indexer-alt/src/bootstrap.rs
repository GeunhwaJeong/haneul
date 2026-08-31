// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

use std::time::Duration;

use anyhow::Context;
use anyhow::Result;
use anyhow::bail;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;
use haneul_indexer_alt_framework::postgres::Db;
use haneul_indexer_alt_framework::types::full_checkpoint_content::Checkpoint;
use haneul_indexer_alt_framework::types::haneul_system_state::HaneulSystemStateTrait;
use haneul_indexer_alt_framework::types::haneul_system_state::get_haneul_system_state;
use haneul_indexer_alt_framework::types::transaction::TransactionKind;
use haneul_indexer_alt_schema::checkpoints::StoredGenesis;
use haneul_indexer_alt_schema::epochs::StoredEpochStart;
use haneul_indexer_alt_schema::schema::kv_epoch_starts;
use haneul_indexer_alt_schema::schema::kv_genesis;
use haneul_types::transaction::TransactionDataAPI;
use tracing::info;

use crate::Indexer;

pub struct BootstrapGenesis {
    pub stored_genesis: StoredGenesis,
    pub stored_epoch_start: StoredEpochStart,
}

/// Ensures the genesis table has been populated before the rest of the indexer is run, and returns
/// the information stored there. If the database has been bootstrapped before, this function will
/// simply read the previously bootstrapped information. Otherwise, it will wait until the first
/// checkpoint is available and extract the necessary information from there.
pub async fn bootstrap(
    indexer: &Indexer<Db>,
    retry_interval: Duration,
    bootstrap_genesis: Option<BootstrapGenesis>,
) -> Result<StoredGenesis> {
    info!("Bootstrapping indexer with genesis information");

    let Ok(mut conn) = indexer.store().connect().await else {
        bail!("Bootstrap failed to get connection for DB");
    };

    // 1. If the row has already been written, return it.
    if let Some(genesis) = kv_genesis::table
        .select(StoredGenesis::as_select())
        .first(&mut conn)
        .await
        .optional()?
    {
        info!(
            chain = genesis.chain()?.as_str(),
            protocol = ?genesis.initial_protocol_version(),
            "Indexer already bootstrapped",
        );

        return Ok(genesis);
    }

    let BootstrapGenesis {
        stored_genesis,
        stored_epoch_start,
    } = match bootstrap_genesis {
        // 2. If genesis is provided, use it to bootstrap.
        Some(bootstrap_genesis) => bootstrap_genesis,
        // 3. Otherwise, extract the necessary information from the genesis checkpoint:
        //
        // - Get the Genesis system transaction from the genesis checkpoint.
        // - Get the system state object that was written out by the system transaction.
        None => {
            let genesis_checkpoint_envelope = indexer
                .ingestion_client()
                .wait_for(0, retry_interval)
                .await
                .context("Failed to fetch genesis checkpoint")?;

            let Checkpoint {
                transactions,
                object_set,
                ..
            } = genesis_checkpoint_envelope.checkpoint.as_ref();

            let Some(genesis_transaction) = transactions
                .iter()
                .find(|tx| matches!(tx.transaction.kind(), TransactionKind::Genesis(_)))
            else {
                bail!("Could not find Genesis transaction");
            };

            let output_objects: Vec<_> = genesis_transaction
                .output_objects(object_set)
                .cloned()
                .collect();

            let haneul_system_state = get_haneul_system_state(&output_objects.as_slice())
                .context("Failed to get Genesis SystemState")?;

            let stored_genesis = StoredGenesis {
                genesis_digest: genesis_checkpoint_envelope.chain_id.as_bytes().to_vec(),
                initial_protocol_version: haneul_system_state.protocol_version() as i64,
            };
            let stored_epoch_start = StoredEpochStart {
                epoch: 0,
                protocol_version: haneul_system_state.protocol_version() as i64,
                cp_lo: 0,
                start_timestamp_ms: haneul_system_state.epoch_start_timestamp_ms() as i64,
                reference_gas_price: haneul_system_state.reference_gas_price() as i64,
                system_state: bcs::to_bytes(&haneul_system_state)
                    .context("Failed to serialize SystemState")?,
            };

            BootstrapGenesis {
                stored_genesis,
                stored_epoch_start,
            }
        }
    };

    info!(
        chain = stored_genesis.chain()?.as_str(),
        protocol = ?stored_genesis.initial_protocol_version(),
        "Bootstrapped indexer",
    );

    diesel::insert_into(kv_genesis::table)
        .values(&stored_genesis)
        .on_conflict_do_nothing()
        .execute(&mut conn)
        .await
        .context("Failed to write genesis record")?;

    diesel::insert_into(kv_epoch_starts::table)
        .values(&stored_epoch_start)
        .on_conflict_do_nothing()
        .execute(&mut conn)
        .await
        .context("Failed to write genesis epoch start record")?;

    Ok(stored_genesis)
}
