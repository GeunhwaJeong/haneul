// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

let action = (runtime) => {
    // Step into `observe`, where all three u256 parameters are bound, and
    // snapshot their reconstructed values.
    runtime.step(false);
    runtime.step(false);
    runtime.step(false);
    runtime.step(false);
    return runtime.toString();
};
run_spec(__dirname, action);
