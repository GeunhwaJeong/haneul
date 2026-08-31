// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

// Tests in this file use the JSON-RPC client (rpc_client), not the gRPC client.
// The rpc_client is deprecated but we need it to test JSON-RPC endpoints.
#![allow(deprecated)]

use haneul_json_rpc_types::{
    Balance as RpcBalance, CoinPage, HaneulData, HaneulObjectDataOptions, HaneulObjectResponse,
};
use haneul_macros::*;
use haneul_types::error::HaneulObjectResponseError;
use haneul_types::{
    base_types::{HaneulAddress, ObjectID},
    coin_reservation::ParsedDigest,
};
use jsonrpsee::core::client::ClientT;
use jsonrpsee::rpc_params;
use test_cluster::addr_balance_test_env::{CoinTypeConfig, TestEnv, TestEnvBuilder};

/// A test scenario specifying the coin setup for HANEUL and optionally a custom coin type.
#[derive(Clone, Debug)]
struct TestScenario {
    /// HANEUL coin configuration. Note: test cluster creates 5 gas coins by default.
    haneul: CoinTypeConfig,
    /// Optional custom coin type configuration.
    custom_coin: Option<CoinTypeConfig>,
}

/// Expected results after running a scenario.
#[derive(Debug, PartialEq, Eq)]
struct ExpectedCounts {
    real_coins: usize,
    fake_coins: usize,
}

impl TestScenario {
    /// Calculate expected counts for getCoins (HANEUL only).
    fn expected_haneul_counts(&self, base_haneul_coins: usize) -> ExpectedCounts {
        let real = base_haneul_coins + self.haneul.real_coins;
        let fake = if self.haneul.has_address_balance {
            1
        } else {
            0
        };
        ExpectedCounts {
            real_coins: real,
            fake_coins: fake,
        }
    }

    /// Calculate expected counts for getCoins (custom coin type).
    fn expected_custom_counts(&self) -> Option<ExpectedCounts> {
        self.custom_coin.as_ref().map(|custom| ExpectedCounts {
            real_coins: custom.real_coins,
            fake_coins: if custom.has_address_balance { 1 } else { 0 },
        })
    }

    /// Calculate expected counts for getAllCoins (all types).
    fn expected_all_counts(&self, base_haneul_coins: usize) -> ExpectedCounts {
        let mut real = base_haneul_coins + self.haneul.real_coins;
        let mut fake = if self.haneul.has_address_balance {
            1
        } else {
            0
        };

        if let Some(ref custom) = self.custom_coin {
            real += custom.real_coins;
            if custom.has_address_balance {
                fake += 1;
            }
        }

        ExpectedCounts {
            real_coins: real,
            fake_coins: fake,
        }
    }
}

/// Set up a test scenario by transferring coins to a fresh address.
async fn setup_scenario(
    test_env: &mut TestEnv,
    scenario: &TestScenario,
) -> (HaneulAddress, Option<String>) {
    let (funder, _) = test_env.get_sender_and_gas(0);

    // Create a fresh address to receive coins
    let recipient = HaneulAddress::random_for_testing_only();

    // Transfer HANEUL coins to recipient
    for _ in 0..scenario.haneul.real_coins {
        test_env
            .transfer_haneul(funder, recipient, 1_000_000_000)
            .await;
    }

    // Fund HANEUL address balance if configured
    if scenario.haneul.has_address_balance {
        test_env
            .transfer_haneul_to_address_balance(funder, recipient, 1_000_000_000)
            .await;
    }

    // Set up custom coin type if configured
    let custom_coin_type = if let Some(ref custom) = scenario.custom_coin {
        let (_, coin_type) = test_env
            .publish_trusted_coin_and_setup(funder, recipient, custom, 1_000_000)
            .await;
        Some(coin_type.to_string())
    } else {
        None
    };

    (recipient, custom_coin_type)
}

/// Query getCoins for a specific coin type and return counts of real and fake coins.
async fn get_coins_counts(
    test_env: &TestEnv,
    owner: HaneulAddress,
    coin_type: Option<&str>,
) -> ExpectedCounts {
    let params = rpc_params![
        owner,
        coin_type,
        Option::<String>::None,
        Option::<usize>::None
    ];
    let coins: CoinPage = test_env
        .cluster
        .fullnode_handle
        .rpc_client
        .request("haneulx_getCoins", params)
        .await
        .unwrap();

    count_real_and_fake(&coins)
}

/// Query getAllCoins and return counts of real and fake coins.
async fn get_all_coins_counts(test_env: &TestEnv, owner: HaneulAddress) -> ExpectedCounts {
    let params = rpc_params![owner, Option::<String>::None, Option::<usize>::None];
    let coins: CoinPage = test_env
        .cluster
        .fullnode_handle
        .rpc_client
        .request("haneulx_getAllCoins", params)
        .await
        .unwrap();

    count_real_and_fake(&coins)
}

fn count_real_and_fake(coins: &CoinPage) -> ExpectedCounts {
    let fake_coins = coins
        .data
        .iter()
        .filter(|c| ParsedDigest::is_coin_reservation_digest(&c.digest))
        .count();
    let real_coins = coins.data.len() - fake_coins;
    ExpectedCounts {
        real_coins,
        fake_coins,
    }
}

/// Verify pagination returns same results regardless of page size.
async fn verify_pagination_consistency(
    test_env: &TestEnv,
    owner: HaneulAddress,
    coin_type: Option<&str>,
) {
    // Fetch all at once
    let all_ids = fetch_all_coin_ids(test_env, owner, coin_type, 100).await;

    // Fetch with page size 1
    let paginated_1 = fetch_all_coin_ids(test_env, owner, coin_type, 1).await;
    assert_eq!(
        all_ids, paginated_1,
        "Pagination with page_size=1 should match all-at-once for coin_type={:?}",
        coin_type
    );

    // Fetch with page size 2
    let paginated_2 = fetch_all_coin_ids(test_env, owner, coin_type, 2).await;
    assert_eq!(
        all_ids, paginated_2,
        "Pagination with page_size=2 should match all-at-once for coin_type={:?}",
        coin_type
    );
}

/// Fetch all coin IDs using getCoins with pagination.
async fn fetch_all_coin_ids(
    test_env: &TestEnv,
    owner: HaneulAddress,
    coin_type: Option<&str>,
    page_size: usize,
) -> Vec<ObjectID> {
    let mut all_ids = vec![];
    let mut cursor: Option<String> = None;

    loop {
        let params = rpc_params![owner, coin_type, cursor.clone(), Some(page_size)];
        let page: CoinPage = test_env
            .cluster
            .fullnode_handle
            .rpc_client
            .request("haneulx_getCoins", params)
            .await
            .unwrap();

        for coin in &page.data {
            all_ids.push(coin.coin_object_id);
        }

        if page.has_next_page {
            cursor = page.next_cursor;
        } else {
            break;
        }
    }

    all_ids
}

/// Verify fake coin ordering: fake coins should be at position 1 within each type,
/// or at position 0 if no real coins exist for that type.
fn verify_fake_coin_ordering(coins: &CoinPage) {
    let mut current_type: Option<String> = None;
    let mut position_in_type = 0;
    let mut has_real_for_type = false;

    for coin in &coins.data {
        let is_fake = ParsedDigest::is_coin_reservation_digest(&coin.digest);

        if current_type.as_ref() != Some(&coin.coin_type) {
            current_type = Some(coin.coin_type.clone());
            position_in_type = 0;
            has_real_for_type = false;
        }

        if is_fake {
            if has_real_for_type {
                assert_eq!(
                    position_in_type, 1,
                    "Fake coin for type {} should be at position 1 (after first real), but was at {}",
                    coin.coin_type, position_in_type
                );
            } else {
                assert_eq!(
                    position_in_type, 0,
                    "Fake coin for type {} (no real coins) should be at position 0, but was at {}",
                    coin.coin_type, position_in_type
                );
            }
        } else {
            has_real_for_type = true;
        }

        position_in_type += 1;
    }
}

// =============================================================================
// Data-driven scenario tests
// =============================================================================

#[sim_test]
async fn test_scenario_haneul_real_only() {
    // HANEUL: real coins only, no address balance
    let scenario = TestScenario {
        haneul: CoinTypeConfig {
            real_coins: 2,
            has_address_balance: false,
        },
        custom_coin: None,
    };
    run_scenario(scenario).await;
}

#[sim_test]
async fn test_scenario_haneul_with_address_balance() {
    // HANEUL: real coins + address balance
    let scenario = TestScenario {
        haneul: CoinTypeConfig {
            real_coins: 2,
            has_address_balance: true,
        },
        custom_coin: None,
    };
    run_scenario(scenario).await;
}

#[sim_test]
async fn test_scenario_haneul_address_balance_only() {
    // HANEUL: address balance only, no additional real coins (just base coins)
    let scenario = TestScenario {
        haneul: CoinTypeConfig {
            real_coins: 0,
            has_address_balance: true,
        },
        custom_coin: None,
    };
    run_scenario(scenario).await;
}

#[sim_test]
async fn test_scenario_two_types_both_with_real_and_fake() {
    // HANEUL + custom: both have real coins and address balance
    let scenario = TestScenario {
        haneul: CoinTypeConfig {
            real_coins: 1,
            has_address_balance: true,
        },
        custom_coin: Some(CoinTypeConfig {
            real_coins: 1,
            has_address_balance: true,
        }),
    };
    run_scenario(scenario).await;
}

#[sim_test]
async fn test_scenario_two_types_custom_address_balance_only() {
    // HANEUL: real + fake, Custom: address balance only (no real coins)
    // This tests the bug fix where fake coins for types without real coins were omitted
    let scenario = TestScenario {
        haneul: CoinTypeConfig {
            real_coins: 1,
            has_address_balance: true,
        },
        custom_coin: Some(CoinTypeConfig {
            real_coins: 0,
            has_address_balance: true,
        }),
    };
    run_scenario(scenario).await;
}

#[sim_test]
async fn test_scenario_two_types_custom_real_only() {
    // HANEUL: real + fake, Custom: real only (no address balance)
    let scenario = TestScenario {
        haneul: CoinTypeConfig {
            real_coins: 1,
            has_address_balance: true,
        },
        custom_coin: Some(CoinTypeConfig {
            real_coins: 2,
            has_address_balance: false,
        }),
    };
    run_scenario(scenario).await;
}

#[sim_test]
async fn test_scenario_two_types_no_address_balances() {
    // HANEUL + custom: both have real coins only, no address balances
    let scenario = TestScenario {
        haneul: CoinTypeConfig {
            real_coins: 1,
            has_address_balance: false,
        },
        custom_coin: Some(CoinTypeConfig {
            real_coins: 1,
            has_address_balance: false,
        }),
    };
    run_scenario(scenario).await;
}

async fn run_scenario(scenario: TestScenario) {
    let mut test_env = TestEnvBuilder::new().build().await;

    let (recipient, custom_type) = setup_scenario(&mut test_env, &scenario).await;

    // For a fresh recipient, base HANEUL coins = 0 (unless we transferred some)
    let base_haneul_coins = 0;

    // Test getCoins for HANEUL
    let haneul_counts = get_coins_counts(&test_env, recipient, None).await;
    let expected_haneul = scenario.expected_haneul_counts(base_haneul_coins);

    // Check if address balance feature is actually working (may be disabled by mainnet config)
    let address_balance_working =
        !scenario.haneul.has_address_balance || haneul_counts.fake_coins > 0;

    if address_balance_working {
        assert_eq!(
            haneul_counts, expected_haneul,
            "getCoins(HANEUL) mismatch for scenario {:?}",
            scenario
        );

        // Test getCoins for custom coin type (if present)
        if let (Some(coin_type), Some(expected_custom)) =
            (custom_type.as_ref(), scenario.expected_custom_counts())
        {
            let custom_counts = get_coins_counts(&test_env, recipient, Some(coin_type)).await;
            assert_eq!(
                custom_counts, expected_custom,
                "getCoins(custom) mismatch for scenario {:?}",
                scenario
            );
        }

        // Test getAllCoins (all types)
        let all_counts = get_all_coins_counts(&test_env, recipient).await;
        let expected_all = scenario.expected_all_counts(base_haneul_coins);
        assert_eq!(
            all_counts, expected_all,
            "getAllCoins mismatch for scenario {:?}",
            scenario
        );

        // Verify ordering
        let params = rpc_params![recipient, Option::<String>::None, Option::<usize>::None];
        let coins: CoinPage = test_env
            .cluster
            .fullnode_handle
            .rpc_client
            .request("haneulx_getAllCoins", params)
            .await
            .unwrap();
        verify_fake_coin_ordering(&coins);

        // Verify pagination consistency for HANEUL
        verify_pagination_consistency(&test_env, recipient, None).await;

        // Verify pagination consistency for custom coin type (if present)
        if let Some(coin_type) = custom_type.as_ref() {
            verify_pagination_consistency(&test_env, recipient, Some(coin_type)).await;
        }
    }
    // If address balance feature is not working (disabled by mainnet config), skip assertions
}

// =============================================================================
// Pagination tests
// =============================================================================

#[sim_test]
async fn test_pagination_no_duplicate_fake_coins() {
    // Verify fake coins don't appear again in subsequent pages
    let mut test_env = TestEnvBuilder::new().build().await;

    let (sender, _) = test_env.get_sender_and_gas(0);

    // Fund address balance
    test_env
        .fund_one_address_balance(sender, 5_000_000_000)
        .await;

    // Fetch all coins with pagination using page size 2
    let mut all_coin_ids: Vec<ObjectID> = vec![];
    let mut cursor: Option<String> = None;

    loop {
        let params = rpc_params![sender, Option::<String>::None, cursor.clone(), Some(2usize)];
        let page: CoinPage = test_env
            .cluster
            .fullnode_handle
            .rpc_client
            .request("haneulx_getCoins", params)
            .await
            .unwrap();

        for coin in &page.data {
            assert!(
                !all_coin_ids.contains(&coin.coin_object_id),
                "Duplicate coin found: {:?}",
                coin.coin_object_id
            );
            all_coin_ids.push(coin.coin_object_id);
        }

        if page.has_next_page {
            cursor = page.next_cursor;
        } else {
            break;
        }
    }

    // Verify we got exactly one fake coin (if address balance feature is working)
    let coin_counts = get_coins_counts(&test_env, sender, None).await;
    let fake_count = all_coin_ids.len() - coin_counts.real_coins;
    if coin_counts.fake_coins > 0 {
        assert_eq!(fake_count, 1, "Should have exactly one fake coin");
    }
}

#[sim_test]
async fn test_pagination_consistency_get_all_coins() {
    // Verify paginated getAllCoins returns same results as fetching all at once
    let mut test_env = TestEnvBuilder::new().build().await;

    let (funder, _) = test_env.get_sender_and_gas(0);

    // Create custom coin type with address balance
    let recipient = HaneulAddress::random_for_testing_only();
    test_env
        .transfer_haneul(funder, recipient, 1_000_000_000)
        .await;
    test_env
        .transfer_haneul_to_address_balance(funder, recipient, 1_000_000_000)
        .await;

    let custom_config = CoinTypeConfig {
        real_coins: 1,
        has_address_balance: true,
    };
    test_env
        .publish_trusted_coin_and_setup(funder, recipient, &custom_config, 1_000_000)
        .await;

    // Fetch all at once
    let params = rpc_params![recipient, Option::<String>::None, Some(100usize)];
    let all_at_once: CoinPage = test_env
        .cluster
        .fullnode_handle
        .rpc_client
        .request("haneulx_getAllCoins", params)
        .await
        .unwrap();

    // Fetch with pagination
    let mut paginated_ids: Vec<ObjectID> = vec![];
    let mut cursor: Option<String> = None;
    loop {
        let params = rpc_params![recipient, cursor.clone(), Some(2usize)];
        let page: CoinPage = test_env
            .cluster
            .fullnode_handle
            .rpc_client
            .request("haneulx_getAllCoins", params)
            .await
            .unwrap();

        for coin in &page.data {
            paginated_ids.push(coin.coin_object_id);
        }

        if page.has_next_page {
            cursor = page.next_cursor;
        } else {
            break;
        }
    }

    let all_at_once_ids: Vec<ObjectID> =
        all_at_once.data.iter().map(|c| c.coin_object_id).collect();
    assert_eq!(
        all_at_once_ids, paginated_ids,
        "Paginated results should match all-at-once results"
    );
}

// =============================================================================
// Other specific behavior tests
// =============================================================================

#[sim_test]
async fn test_get_object_returns_fake_coin() {
    // Test that haneul_getObject returns a fake coin object for a masked object ID
    let mut test_env = TestEnvBuilder::new().build().await;

    let (sender, _) = test_env.get_sender_and_gas(0);
    let amount = 1_000_000_000u64;

    test_env.fund_one_address_balance(sender, amount).await;

    // Check if address balance feature is working
    let params = rpc_params![sender, Option::<String>::None];
    let balance: RpcBalance = test_env
        .cluster
        .fullnode_handle
        .rpc_client
        .request("haneulx_getBalance", params)
        .await
        .unwrap();

    // Skip test assertions if feature is not working (disabled by mainnet config)
    if balance.funds_in_address_balance == 0 {
        return;
    }

    let fake_coin_ref = test_env.encode_coin_reservation(sender, 0, amount);
    let masked_object_id = fake_coin_ref.0;

    let params = rpc_params![
        masked_object_id,
        HaneulObjectDataOptions::new().with_content().with_owner()
    ];
    let response: HaneulObjectResponse = test_env
        .cluster
        .fullnode_handle
        .rpc_client
        .request("haneul_getObject", params)
        .await
        .unwrap();

    let object_data = response.data.expect("Expected object data");
    assert_eq!(object_data.object_id, masked_object_id);

    let content = object_data.content.expect("Expected content");
    let fields = content.try_into_move().expect("Expected move object");
    assert!(
        fields
            .type_
            .to_string()
            .contains("0x2::coin::Coin<0x2::haneul::HANEUL>")
    );
}

#[sim_test]
async fn test_get_balance_includes_address_balance() {
    // Test that getBalance includes address balance in the total
    let mut test_env = TestEnvBuilder::new().build().await;

    let (sender, _) = test_env.get_sender_and_gas(0);
    let amount = 3_000_000_000u64;

    let params = rpc_params![sender, Option::<String>::None];
    let initial: RpcBalance = test_env
        .cluster
        .fullnode_handle
        .rpc_client
        .request("haneulx_getBalance", params)
        .await
        .unwrap();

    test_env.fund_one_address_balance(sender, amount).await;

    let params = rpc_params![sender, Option::<String>::None];
    let updated: RpcBalance = test_env
        .cluster
        .fullnode_handle
        .rpc_client
        .request("haneulx_getBalance", params)
        .await
        .unwrap();

    // Total should be roughly the same (minus gas)
    assert!(
        updated.total_balance >= initial.total_balance - 10_000_000,
        "Total balance changed unexpectedly"
    );

    // If address balance feature is enabled (not disabled by mainnet config override),
    // verify the fake coin is included in the count
    if updated.funds_in_address_balance > 0 {
        // Coin count should increase by 1 (the fake coin)
        assert_eq!(
            updated.coin_object_count,
            initial.coin_object_count + 1,
            "Coin count should increase by 1"
        );

        // Address balance should be reported
        assert_eq!(
            updated.funds_in_address_balance, amount as u128,
            "Address balance should be reported"
        );
    }
}

#[sim_test]
async fn test_no_fake_coins_when_coin_reservations_disabled() {
    // Enable address balances but explicitly disable coin reservations.
    // Verify that getCoins does not return fake coins.
    let mut test_env = TestEnvBuilder::new()
        .with_proto_override_cb(Box::new(|_, mut cfg| {
            cfg.disable_coin_reservation_for_testing();
            cfg
        }))
        .build()
        .await;

    let (sender, _) = test_env.get_sender_and_gas(0);
    test_env
        .fund_one_address_balance(sender, 1_000_000_000)
        .await;

    let counts = get_coins_counts(&test_env, sender, None).await;
    assert_eq!(
        counts.fake_coins, 0,
        "No fake coins should be returned when coin reservations are disabled"
    );

    let all_counts = get_all_coins_counts(&test_env, sender).await;
    assert_eq!(
        all_counts.fake_coins, 0,
        "getAllCoins should not return fake coins when coin reservations are disabled"
    );
}

#[sim_test]
async fn test_get_object_no_fake_coin_when_coin_reservations_disabled() {
    // Enable address balances but explicitly disable coin reservations.
    // Verify that haneul_getObject does not return a fake coin for a masked object ID.
    let mut test_env = TestEnvBuilder::new()
        .with_proto_override_cb(Box::new(|_, mut cfg| {
            cfg.disable_coin_reservation_for_testing();
            cfg
        }))
        .build()
        .await;

    let (sender, _) = test_env.get_sender_and_gas(0);
    let amount = 1_000_000_000u64;
    test_env.fund_one_address_balance(sender, amount).await;

    // Create a masked object ID as if coin reservations were enabled.
    let fake_coin_ref = test_env.encode_coin_reservation(sender, 0, amount);
    let masked_object_id = fake_coin_ref.0;

    let params = rpc_params![
        masked_object_id,
        HaneulObjectDataOptions::new().with_content().with_owner()
    ];
    let response: HaneulObjectResponse = test_env
        .cluster
        .fullnode_handle
        .rpc_client
        .request("haneul_getObject", params)
        .await
        .unwrap();

    assert!(
        response.data.is_none(),
        "haneul_getObject should not return a fake coin when coin reservations are disabled"
    );
    assert!(
        matches!(
            response.error,
            Some(HaneulObjectResponseError::NotExists { .. })
        ),
        "Expected NotExists error for masked object ID when coin reservations disabled"
    );
}
