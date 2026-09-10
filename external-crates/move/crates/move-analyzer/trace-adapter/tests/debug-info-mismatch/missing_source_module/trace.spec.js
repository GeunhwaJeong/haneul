// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

let action = (runtime) => {
    // Models a source-mapped caller entering a module absent from source debug info.
    let res = '';

    res += snapshot(runtime);

    runtime.step(false);
    res += snapshot(runtime);

    // Return from the bytecode-only module to source-level debugging in the caller.
    runtime.step(false);
    res += snapshot(runtime);
    return res;
};
run_spec(__dirname, action);
