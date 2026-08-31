// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

import { useHaneulClientContext } from "@haneullabs/dapp-kit";
import { formatAddress } from "@haneullabs/haneul/utils";
import { CheckIcon, CopyIcon } from "@radix-ui/react-icons";
import { useState } from "react";
import toast from "react-hot-toast";

/**
 * A re-usable component for explorer links that offers
 * a copy to clipboard functionality.
 */
export function ExplorerLink({
  id,
  isAddress,
}: {
  id: string;
  isAddress?: boolean;
}) {
  const [copied, setCopied] = useState(false);
  const { network } = useHaneulClientContext();

  const link = `https://haneulexplorer.com/${
    isAddress ? "address" : "object"
  }/${id}?network=${network}`;

  const copy = () => {
    navigator.clipboard.writeText(id);
    setCopied(true);
    setTimeout(() => {
      setCopied(false);
    }, 2000);
    toast.success("Copied to clipboard!");
  };

  return (
    <span className="flex items-center gap-3">
      {copied ? (
        <CheckIcon />
      ) : (
        <CopyIcon
          height={12}
          width={12}
          className="cursor-pointer"
          onClick={copy}
        />
      )}

      <a href={link} target="_blank" rel="noreferrer">
        {formatAddress(id)}
      </a>
    </span>
  );
}
