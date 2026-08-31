// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

import { Transaction } from '@haneullabs/haneul/transactions';
import type { DeepBookMarginClient } from './client.js';

// docs::#borrow
// Borrow against your collateral. This example opens a leveraged long on HANEUL:
// with HANEUL collateral deposited, borrow the quote asset (DBUSDC) so you can buy
// more HANEUL than your own funds cover. A MarginManager borrows from one margin
// pool at a time, base or quote, not both.
//
// The borrow is rejected unless the resulting risk ratio stays at or above the
// pool's Min Borrow Risk Ratio. That ceiling on how much you can borrow per unit
// of collateral is what sets your maximum leverage, so size the borrow from the
// risk parameters you read, not by trial and error.
export function borrowQuote(
	client: DeepBookMarginClient,
	managerKey: string,
	amount: number,
): Transaction {
	const tx = new Transaction();
	tx.add(client.deepbook.marginManager.borrowQuote(managerKey, amount));
	return tx;
}
// docs::/#borrow

// docs::#repay
// Repay borrowed funds plus the interest accrued since you borrowed. Pass an
// amount to repay part of the debt, or `undefined` to repay the full outstanding
// balance. Repaying raises your risk ratio and is what you do before withdrawing
// collateral or closing the position.
export function repayQuote(
	client: DeepBookMarginClient,
	managerKey: string,
	amount?: number,
): Transaction {
	const tx = new Transaction();
	tx.add(client.deepbook.marginManager.repayQuote(managerKey, amount));
	return tx;
}
// docs::/#repay
