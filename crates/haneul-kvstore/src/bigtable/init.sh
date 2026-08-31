# Copyright (c) Mysten Labs, Inc.
# Modifications Copyright (c) 2026 Geunhwa Jeong
# SPDX-License-Identifier: Apache-2.0
INSTANCE_ID=${1:-haneul}
command=(
  cbt
  -instance
  "$INSTANCE_ID"
)
if [[ -n $BIGTABLE_EMULATOR_HOST ]]; then
  command+=(-project emulator)
fi

for table in transaction_bitmap_index event_bitmap_index checkpoints checkpoints_by_digest \
    transactions objects epochs watermark_alt protocol_configs packages packages_by_id \
    packages_by_checkpoint system_packages tx_seq_digest; do
  (
    set -x
    "${command[@]}" createtable $table
    "${command[@]}" createfamily $table haneul
    "${command[@]}" setgcpolicy $table haneul maxversions=1
  )
done
