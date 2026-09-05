// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

module bridge::chain_ids;

// Chain IDs
const HANEUL_MAINNET: u8 = 0;
const HANEUL_TESTNET: u8 = 1;
const HANEUL_CUSTOM: u8 = 2;

const ETH_MAINNET: u8 = 10;
const ETH_SEPOLIA: u8 = 11;
const ETH_CUSTOM: u8 = 12;

const EInvalidBridgeRoute: u64 = 0;

//////////////////////////////////////////////////////
// Types
//

public struct BridgeRoute has copy, drop, store {
    source: u8,
    destination: u8,
}

//////////////////////////////////////////////////////
// Public functions
//

public fun haneul_mainnet(): u8 { HANEUL_MAINNET }

public fun haneul_testnet(): u8 { HANEUL_TESTNET }

public fun haneul_custom(): u8 { HANEUL_CUSTOM }

public fun eth_mainnet(): u8 { ETH_MAINNET }

public fun eth_sepolia(): u8 { ETH_SEPOLIA }

public fun eth_custom(): u8 { ETH_CUSTOM }

public use fun route_source as BridgeRoute.source;

public fun route_source(route: &BridgeRoute): &u8 {
    &route.source
}

public use fun route_destination as BridgeRoute.destination;

public fun route_destination(route: &BridgeRoute): &u8 {
    &route.destination
}

public fun is_haneul_chain(id: u8): bool {
    id == HANEUL_MAINNET || id == HANEUL_TESTNET || id == HANEUL_CUSTOM
}

public fun assert_valid_chain_id(id: u8) {
    assert!(
        id == HANEUL_MAINNET ||
        id == HANEUL_TESTNET ||
        id == HANEUL_CUSTOM ||
        id == ETH_MAINNET ||
        id == ETH_SEPOLIA ||
        id == ETH_CUSTOM,
        EInvalidBridgeRoute,
    )
}

public fun valid_routes(): vector<BridgeRoute> {
    vector[
        BridgeRoute { source: HANEUL_MAINNET, destination: ETH_MAINNET },
        BridgeRoute { source: ETH_MAINNET, destination: HANEUL_MAINNET },
        BridgeRoute { source: HANEUL_TESTNET, destination: ETH_SEPOLIA },
        BridgeRoute { source: HANEUL_TESTNET, destination: ETH_CUSTOM },
        BridgeRoute { source: HANEUL_CUSTOM, destination: ETH_CUSTOM },
        BridgeRoute { source: HANEUL_CUSTOM, destination: ETH_SEPOLIA },
        BridgeRoute { source: ETH_SEPOLIA, destination: HANEUL_TESTNET },
        BridgeRoute { source: ETH_SEPOLIA, destination: HANEUL_CUSTOM },
        BridgeRoute { source: ETH_CUSTOM, destination: HANEUL_TESTNET },
        BridgeRoute { source: ETH_CUSTOM, destination: HANEUL_CUSTOM },
    ]
}

public fun is_valid_route(source: u8, destination: u8): bool {
    let route = BridgeRoute { source, destination };
    valid_routes().contains(&route)
}

// Checks and return BridgeRoute if the route is supported by the bridge.
public fun get_route(source: u8, destination: u8): BridgeRoute {
    let route = BridgeRoute { source, destination };
    assert!(valid_routes().contains(&route), EInvalidBridgeRoute);
    route
}

//////////////////////////////////////////////////////
// Test functions
//

#[test]
fun test_chains_ok() {
    assert_valid_chain_id(HANEUL_MAINNET);
    assert_valid_chain_id(HANEUL_TESTNET);
    assert_valid_chain_id(HANEUL_CUSTOM);
    assert_valid_chain_id(ETH_MAINNET);
    assert_valid_chain_id(ETH_SEPOLIA);
    assert_valid_chain_id(ETH_CUSTOM);
}

#[test, expected_failure(abort_code = EInvalidBridgeRoute)]
fun test_chains_error() {
    assert_valid_chain_id(100);
}

#[test, expected_failure(abort_code = EInvalidBridgeRoute)]
fun test_haneul_chains_error() {
    // this will break if we add one more haneul chain id and should be corrected
    assert_valid_chain_id(4);
}

#[test, expected_failure(abort_code = EInvalidBridgeRoute)]
fun test_eth_chains_error() {
    // this will break if we add one more eth chain id and should be corrected
    assert_valid_chain_id(13);
}

#[test]
fun test_routes() {
    let valid_routes = vector[
        BridgeRoute { source: HANEUL_MAINNET, destination: ETH_MAINNET },
        BridgeRoute { source: ETH_MAINNET, destination: HANEUL_MAINNET },
        BridgeRoute { source: HANEUL_TESTNET, destination: ETH_SEPOLIA },
        BridgeRoute { source: HANEUL_TESTNET, destination: ETH_CUSTOM },
        BridgeRoute { source: HANEUL_CUSTOM, destination: ETH_CUSTOM },
        BridgeRoute { source: HANEUL_CUSTOM, destination: ETH_SEPOLIA },
        BridgeRoute { source: ETH_SEPOLIA, destination: HANEUL_TESTNET },
        BridgeRoute { source: ETH_SEPOLIA, destination: HANEUL_CUSTOM },
        BridgeRoute { source: ETH_CUSTOM, destination: HANEUL_TESTNET },
        BridgeRoute { source: ETH_CUSTOM, destination: HANEUL_CUSTOM },
    ];
    let mut size = valid_routes.length();
    while (size > 0) {
        size = size - 1;
        let route = valid_routes[size];
        assert!(is_valid_route(route.source, route.destination)); // sould not assert
    }
}

#[test, expected_failure(abort_code = EInvalidBridgeRoute)]
fun test_routes_err_haneul_1() {
    get_route(HANEUL_MAINNET, HANEUL_MAINNET);
}

#[test, expected_failure(abort_code = EInvalidBridgeRoute)]
fun test_routes_err_haneul_2() {
    get_route(HANEUL_MAINNET, HANEUL_TESTNET);
}

#[test, expected_failure(abort_code = EInvalidBridgeRoute)]
fun test_routes_err_haneul_3() {
    get_route(HANEUL_MAINNET, ETH_SEPOLIA);
}

#[test, expected_failure(abort_code = EInvalidBridgeRoute)]
fun test_routes_err_haneul_4() {
    get_route(HANEUL_MAINNET, ETH_CUSTOM);
}

#[test, expected_failure(abort_code = EInvalidBridgeRoute)]
fun test_routes_err_eth_1() {
    get_route(ETH_MAINNET, ETH_MAINNET);
}

#[test, expected_failure(abort_code = EInvalidBridgeRoute)]
fun test_routes_err_eth_2() {
    get_route(ETH_MAINNET, ETH_CUSTOM);
}

#[test, expected_failure(abort_code = EInvalidBridgeRoute)]
fun test_routes_err_eth_3() {
    get_route(ETH_MAINNET, HANEUL_CUSTOM);
}

#[test, expected_failure(abort_code = EInvalidBridgeRoute)]
fun test_routes_err_eth_4() {
    get_route(ETH_MAINNET, HANEUL_TESTNET);
}
