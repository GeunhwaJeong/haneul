/*
// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0
*/

import React from 'react';
import {MDXProvider} from '@mdx-js/react';
import MDXComponents from '@theme/MDXComponents';
export default function MDXContent({children}) {
  return <MDXProvider components={MDXComponents}>{children}</MDXProvider>;
}