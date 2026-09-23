// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

import { fileURLToPath } from "url";
import path from "path";
import math from "remark-math";
import katex from "rehype-katex";
import remarkGlossary from "./src/shared/plugins/remark-glossary.js";

const npm2yarn = require("@docusaurus/remark-plugin-npm2yarn");

const effortRemarkPlugin = require("./src/plugins/effort");
const betaRemarkPlugin = require("./src/plugins/betatag");
const graphqlFrontmatterPlugin = require("./src/plugins/graphql-frontmatter");

const lightCodeTheme = require("prism-react-renderer").themes.github;
const darkCodeTheme = require("prism-react-renderer").themes.nightOwl;

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const SIDEBARS_PATH = fileURLToPath(new URL("../content/sidebars.js", import.meta.url));

require("dotenv").config();

/** @type {import('@docusaurus/types').Config} */
const config = {
  title: "Haneul Documentation",
  tagline:
    "Haneul is a next-generation smart contract platform with high throughput, low latency, and an asset-oriented programming model powered by Move",
  favicon: "/img/favicon.ico",
  headTags: [
    {
      tagName: "meta",
      attributes: {
        name: "algolia-site-verification",
        content: "BCA21DA2879818D2",
      },
    },
    {
      tagName: "link",
      attributes: {
        rel: "service-doc",
        href: "/llms.txt",
        type: "text/plain",
        title: "LLM-optimized documentation",
      },
    },
    {
      tagName: "link",
      attributes: {
        rel: "service-doc",
        href: "/references/haneul-api",
        title: "Haneul API Reference",
      },
    },
    {
      tagName: "link",
      attributes: {
        rel: "sitemap",
        href: "/sitemap.xml",
        type: "application/xml",
      },
    },
    {
      tagName: "link",
      attributes: {
        rel: "api-catalog",
        href: "/.well-known/api-catalog",
        type: "application/linkset+json",
      },
    },
  ],
  // Set the production url of your site here
  url: "https://docs.haneul.io",
  // Set the /<baseUrl>/ pathname under which your site is served
  // For GitHub pages deployment, it is often '/<projectName>/'
  baseUrl: "/",

  onBrokenLinks: "throw",
  onBrokenAnchors: "warn",
  onDuplicateRoutes: 'throw',

  staticDirectories: ["static", "src/open-spec"],
  markdown: {
    format: "detect",
    mermaid: true,
    hooks: {
    onBrokenMarkdownLinks: 'throw',
    onBrokenMarkdownImages: 'throw',
  },
  },
  
  clientModules: [
    require.resolve("./src/client/pushfeedback-toc.js"),
    require.resolve("./src/client/webmcp.js"),
    require.resolve("./src/client/kapa-sidebar.js"),
  ],
  plugins: [
    function llmsTxtDirectivePlugin() {
      return {
        name: 'llms-txt-directive-plugin',
        injectHtmlTags() {
          return {
            preBodyTags: [
              {
                tagName: 'link',
                attributes: {
                  rel: 'alternate',
                  type: 'text/plain',
                  href: '/llms.txt',
                  title: 'LLMs.txt',
                },
              },
            ],
          };
        },
      };
    },
    function contentNegotiationPlugin() {
      return {
        name: 'content-negotiation-plugin',
        configureWebpack(config, isServer) {
          if (isServer) return {};
          const fs = require('fs');
          const grayMatter = require('gray-matter');
          const contentDir = path.resolve(__dirname, '../content');

          function cleanForMarkdown(raw) {
            const { content } = grayMatter(raw);
            let cleaned = content;
            cleaned = cleaned.replace(/^\s*import\s+.*?from\s+['"].*?['"];?\s*$/gm, '');
            cleaned = cleaned.replace(/^\s*export\s+(default\s+)?.*$/gm, '');
            cleaned = cleaned.replace(/<\/?(?:Cards|Tabs|ToolGrid)\b[^>]*>/g, '');
            cleaned = cleaned.replace(/<TabItem\b[^>]*label="([^"]*)"[^>]*>([\s\S]*?)<\/TabItem>/g, (_, label, inner) => `\n## ${label.trim()}\n\n${inner.trim()}\n`);
            cleaned = cleaned.replace(/<Admonition\b[^>]*type="([^"]+)"[^>]*>([\s\S]*?)<\/Admonition>/g, (_, type, inner) => `\n:::${type}\n${inner.trim()}\n:::\n`);
            cleaned = cleaned.replace(/<details\b[^>]*>\s*<summary>([\s\S]*?)<\/summary>([\s\S]*?)<\/details>/gi, (_, s, inner) => `\n**${s.trim()}**\n\n${inner.trim()}\n`);
            cleaned = cleaned.replace(/<Badge\b[^>]*\btext="([^"]*)"[^>]*\/>/g, '`$1`');
            cleaned = cleaned.replace(/<Bullet\s*\/>/g, ' ');
            cleaned = cleaned.replace(/<style>\{`[\s\S]*?`\}<\/style>/g, '');
            cleaned = cleaned.replace(/<style\b[^>]*>[\s\S]*?<\/style>/gi, '');
            cleaned = cleaned.replace(/<code\b[^>]*>([\s\S]*?)<\/code>/g, (_, inner) => `\`${inner.replace(/<\/?[a-z][^>]*>/gi, '')}\``);
            cleaned = cleaned.replace(/<[A-Z][A-Za-z0-9]*\b[^>]*\/>/g, '');
            for (let i = 0; i < 3; i++) cleaned = cleaned.replace(/<([A-Z][A-Za-z0-9]*)\b[^>]*>([\s\S]*?)<\/\1>/g, '$2');
            cleaned = cleaned.replace(/\{\/\*[\s\S]*?\*\/\}/g, '');
            cleaned = cleaned.replace(/^\s*\{[A-Z][A-Za-z0-9_.]*\}\s*$/gm, '');
            cleaned = cleaned.replace(/\n{3,}/g, '\n\n');
            return cleaned.trim() + '\n';
          }

          return {
            devServer: {
              setupMiddlewares(middlewares, devServer) {
                devServer.app.use((req, res, next) => {
                  const accept = req.headers.accept || '';
                  if (!accept.includes('text/markdown')) return next();

                  const ext = path.extname(req.path);
                  if (ext && ext !== '.html') return next();

                  let urlPath = req.path;
                  if (urlPath.endsWith('/')) urlPath = urlPath.slice(0, -1);
                  if (!urlPath) urlPath = '';

                  const candidates = [
                    path.join(contentDir, urlPath + '.mdx'),
                    path.join(contentDir, urlPath + '.md'),
                    path.join(contentDir, urlPath, 'index.mdx'),
                    path.join(contentDir, urlPath, 'index.md'),
                  ];

                  for (const filePath of candidates) {
                    if (fs.existsSync(filePath)) {
                      const raw = fs.readFileSync(filePath, 'utf8');
                      const markdown = cleanForMarkdown(raw);
                      const byteLen = Buffer.byteLength(markdown, 'utf8');
                      res.set({
                        'Content-Type': 'text/markdown; charset=utf-8',
                        'Content-Length': String(byteLen),
                        'Vary': 'Accept',
                        'Cache-Control': 'no-cache',
                        'x-markdown-tokens': String(Math.ceil(markdown.length / 4)),
                      });
                      return res.send(markdown);
                    }
                  }
                  next();
                });
                return middlewares;
              },
            },
          };
        },
      };
    },
     function aliasPlugin() {
      return {
        name: 'custom-aliases',
        configureWebpack() {
          return {
            resolve: {
              alias: {
                '@generated-imports': path.resolve(__dirname, '.generated'),
              },
            },
          };
        },
      };
    },
    //require.resolve('./src/plugins/framework'),
    "docusaurus-plugin-copy-page-button",
    require.resolve("./src/plugins/validate-openrpc"),

    [
      require.resolve("./src/shared/plugins/plausible"),
      {
        domain: "docs.haneul.io",
        enableInDev: false,
        trackOutboundLinks: true,
        hashMode: false,
        trackLocalhost: false,
      },
    ],
    function stepHeadingLoader() {
      return {
        name: "step-heading-loader",
        configureWebpack() {
          return {
            module: {
              rules: [
                {
                  test: /\.mdx?$/, // run on .md and .mdx
                  enforce: "pre", // make sure it runs BEFORE @docusaurus/mdx-loader
                  include: [
                    // adjust these to match where your Markdown lives
                    path.resolve(__dirname, "../content"),
                  ],
                  use: [
                    {
                      loader: path.resolve(
                        __dirname,
                        "./src/shared/plugins/inject-code/stepLoader.js",
                      ),
                    },
                    {
                      loader: path.resolve(
                        __dirname,
                        "./src/shared/plugins/inject-code/includeSectionLoader.js",
                      ),
                    },
                  ],
                },
              ],
            },
            resolve: {
              alias: {
                "@repo": path.resolve(__dirname, "../../"),
                "@docs": path.resolve(__dirname, "../content/"),
              },
            },
          };
        },
      };
    },
    [
      "@graphql-markdown/docusaurus",
      {
        id: "beta",
        schema: "../../crates/haneul-indexer-alt-graphql/schema.graphql",
        rootPath: "../content",
        baseURL: "references/haneul-api/haneul-graphql/beta/reference",
        homepage: false,
        docOptions: {
          frontMatter: {
            isGraphQlBeta: true,
            pagination_next: null, // disable page navigation next
            pagination_prev: null, // disable page navigation previous
            hide_table_of_contents: true, // disable page table of content
          },
        },
        loaders: {
          GraphQLFileLoader: "@graphql-tools/graphql-file-loader",
        },
      },
    ],
    //require.resolve("./src/shared/plugins/tabs-md-client/index.mjs"),
    async function myPlugin(context, options) {
      return {
        name: "docusaurus-tailwindcss",
        configurePostCss(postcssOptions) {
          // Appends TailwindCSS and AutoPrefixer.
          postcssOptions.plugins.push(require("tailwindcss"));
          postcssOptions.plugins.push(require("autoprefixer"));
          return postcssOptions;
        },
      };
    },
    path.resolve(__dirname, `./src/shared/plugins/descriptions`),
    path.resolve(__dirname, `./src/plugins/framework`),
    path.resolve(__dirname, `./src/plugins/protocol`),
  ],
  presets: [
    [
      /** @type {import('@docusaurus/preset-classic').Options} */
      "classic",
      {
        docs: {
          path: "../content",
          routeBasePath: "/",
          sidebarPath: SIDEBARS_PATH,
          // the double docs below is a fix for having the path set to ../content
          editUrl: "https://github.com/GeunhwaJeong/haneul/tree/main/docs/docs",
          exclude: [
            "**/snippets/**",
            "**/standards/deepbook-ref/**",
            "**/app-examples/ts-sdk-ref/**",
            "**/app-examples/ts-sdk-ref/**",
          ],
          admonitions: {
            keywords: ["checkpoint"],
            extendDefaults: true,
          },
          beforeDefaultRemarkPlugins: [],
          remarkPlugins: [
            math,
            [npm2yarn, { sync: true, converters: ["yarn", "pnpm"] }],
            effortRemarkPlugin,
            betaRemarkPlugin,
            graphqlFrontmatterPlugin,
            [remarkGlossary, { glossaryFile: path.resolve(__dirname, "static/glossary.json") }],
          ],
          rehypePlugins: [katex],
        },
        theme: {
          customCss: [
            require.resolve("./src/css/fonts.css"),
            require.resolve("./src/css/custom.css"),
            require.resolve("./src/css/details.css"),
          ],
        },
        pages: {
          remarkPlugins: [[remarkGlossary, { glossaryFile: path.resolve(__dirname, "static/glossary.json") }]],
        },
      },
    ],
  ],

  scripts: [
    //{ src: "./src/js/tabs-md.js", defer: true },
    {
      src: "https://widget.kapa.ai/kapa-widget.bundle.js",
      "data-website-id": "b05d8d86-0b10-4eb2-acfe-e9012d75d9db",
      "data-project-name": "Haneul Knowledge",
      "data-project-color": "#298DFF",
      "data-button-hide": "true",
      "data-view-mode": "sidebar",
      "data-modal-title": "Ask Haneul AI",
      "data-modal-ask-ai-input-placeholder": "Ask me anything about Haneul!",
      "data-modal-example-questions":"How do I deploy to Haneul?,What is Mysticeti?,What are object ownership types for Haneul Move?,What are programmable transaction blocks (PTBs)?",
      "data-modal-overlay-hidden": "true",
      "data-modal-lock-scroll": "false",
      "data-modal-image": "/img/logo.svg",
      "data-mcp-enabled": "true",
      "data-mcp-server-url": "https://haneul.mcp.kapa.ai",
      "data-mcp-button-text": "Use Haneul MCP Server",
      "data-chat-disclaimer": "**New:** Install [Haneul Agent Skills](https://docs.haneul.io/skills) to supercharge your AI coding agent with Haneul expertise.",
      async: true,
    },
  ],
  stylesheets: [
    {
      href: "https://fonts.googleapis.com/css2?family=Inter:wght@400;500;700&display=swap",
      type: "text/css",
    },
    {
      href: "https://cdn.jsdelivr.net/npm/katex@0.13.24/dist/katex.min.css",
      type: "text/css",
      integrity:
        "sha384-odtC+0UGzzFL/6PNoE8rX/SPcQDXBJ+uRepguP4QkPCm2LBxH3FA3y+fKSiJ+AmM",
      crossorigin: "anonymous",
    },
    {
      href: "https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.5.1/css/all.min.css",
      type: "text/css",
    },
  ],
  themes: ["@docusaurus/theme-mermaid", "docusaurus-theme-github-codeblock"],
  themeConfig:
    /** @type {import('@docusaurus/preset-classic').ThemeConfig} */
    ({
      image: "img/haneul-doc-og.png",
      mermaid: {
        theme: {
          light: "base",
          dark: "base",
        },
        options: {
          themeVariables: {
            primaryColor: "#000000",
            primaryTextColor: "#FFFFFF",
            primaryBorderColor: "#6C7584",
            secondaryColor: "#6C7584",
            secondaryTextColor: "#FFFFFF",
            tertiaryColor: "#298DFF",
            tertiaryTextColor: "#FFFFFF",
            lineColor: "#298DFF",
            background: "#FFFFFF",
            mainBkg: "#000000",
            secondBkg: "#6C7584",
            noteBkgColor: "#E6F1FB",
            noteTextColor: "#000000",
            noteBorderColor: "#298DFF",
            activationBkgColor: "#298DFF",
            activationBorderColor: "#185FA5",
            fontSize: "14px",
            fontFamily: "Inter, sans-serif",
            signalColor: "#298DFF",
            signalTextColor: "#298DFF",
            labelBoxBkgColor: "#000000",
            labelBoxBorderColor: "#6C7584",
            labelTextColor: "#FFFFFF",
            loopTextColor: "#FFFFFF",
          },
        },
      },
      docs: {
        sidebar: {
          autoCollapseCategories: true,
        },
      },

      navbar: {
        title: "Haneul Documentation",
        logo: {
          alt: "Haneul Docs Logo",
          src: "img/haneul-logo.svg",
        },
        items: [
          {
            type: "dropdown",
            label: "Getting Started",
            to: "getting-started",
            items: [
              { to: "/skills", label: "Skills" },
              { type: "doc", docId: "getting-started/onboarding/index", label: "Hello, World!" },
              { type: "doc", docId: "getting-started/examples/index", label: "Example Apps" },
              { type: "doc", docId: "getting-started/tooling", label: "Developer Tools" },
              { type: "doc", docId: "getting-started/dev-cheat-sheet", label: "Developer Cheat Sheet" },
              { type: "doc", docId: "getting-started/haneul-for-ethereum", label: "Ethereum -> Haneul" },
              { type: "doc", docId: "getting-started/haneul-for-solana", label: "Solana -> Haneul" },
            ],
          },
          {
            type: "dropdown",
            label: "Develop",
            to: "develop",
            items: [
              { type: "doc", docId: "develop/haneul-architecture/index", label: "Haneul Architecture" },
              { type: "doc", docId: "develop/objects/index", label: "Using Objects" },
              { type: "doc", docId: "develop/write-move/index", label: "Writing Move Packages" },
              { type: "doc", docId: "develop/publish-upgrade-packages/index", label: "Deploying and Upgrading Packages" },
              { type: "doc", docId: "develop/manage-packages/index", label: "Managing Packages" },
              { type: "doc", docId: "develop/testing-debugging/index", label: "Testing and Debugging" },
              { type: "doc", docId: "develop/transactions/index", label: "Building Transactions" },
              { type: "doc", docId: "develop/transaction-payment/index", label: "Paying for Transactions" },
              { type: "doc", docId: "develop/accessing-data/index", label: "Accessing Data" },
              { type: "doc", docId: "develop/cryptography/index", label: "Cryptography" },
              { type: "doc", docId: "operators", label: "Node Operators" },
            ],
          },
          {
            type: "dropdown",
            label: "Onchain Finance",
            to: "onchain-finance",
            items: [
              { type: "doc", docId: "onchain-finance/types-of-assets", label: "Types of Assets" },
              { type: "doc", docId: "onchain-finance/asset-custody/index", label: "Asset Custody" },
              { type: "doc", docId: "onchain-finance/fungible-tokens/index", label: "Fungible Tokens" },
              { type: "doc", docId: "onchain-finance/tokenized-assets/index", label: "Tokenized Assets" },
              { type: "doc", docId: "onchain-finance/examples-patterns/index", label: "Example Asset Patterns" },
              { type: "doc", docId: "onchain-finance/closed-loop-token/index", label: "Closed Loop Token" },
              { type: "doc", docId: "onchain-finance/pas/index", label: "Permissioned Asset Standard" },
              { type: "doc", docId: "onchain-finance/deepbook/index", label: "DeepBook" },
              { type: "doc", docId: "onchain-finance/oracles/index", label: "Oracles" },
              { type: "doc", docId: "onchain-finance/kiosk/index", label: "Kiosk" },
              { type: "doc", docId: "onchain-finance/payment-kit", label: "Payment Kit" },
            ],
          },
          {
            type: "dropdown",
            label: "Haneul Stack",
            to: "haneul-stack",
            items: [
              { type: "doc", docId: "haneul-stack/on-chain-primitives/access-time", label: "Onchain Time" },
              { type: "doc", docId: "haneul-stack/on-chain-primitives/randomness-onchain", label: "Onchain Randomness" },
              { type: "doc", docId: "haneul-stack/sagat", label: "Sagat" },
              { type: "doc", docId: "haneul-stack/walrus/index", label: "Walrus" },
              { type: "doc", docId: "haneul-stack/seal/index", label: "Seal" },
              { type: "doc", docId: "haneul-stack/haneulns/index", label: "HaneulNS" },
              { type: "doc", docId: "haneul-stack/enoki/solitaire", label: "Enoki" },
              { type: "doc", docId: "haneul-stack/nautilus/index", label: "Nautilus" },
              { type: "doc", docId: "haneul-stack/zklogin-integration/index", label: "zkLogin" },
              { type: "doc", docId: "haneul-stack/haneulplay0x1/index", label: "HaneulPlay0X1" },
            ],
          },
          {
            type: "dropdown",
            label: "References",
            to: "references",
            items: [
              { type: "doc", docId: "references/haneul-api", label: "Haneul RPC" },
              { type: "doc", docId: "references/cli", label: "Haneul CLI" },
              { type: "doc", docId: "references/ide/index", label: "IDE Support" },
              { type: "doc", docId: "references/haneul-sdks", label: "Haneul SDKs" },             
              { type: "doc", docId: "references/ptb-commands", label: "PTB Commands" },
              { type: "doc", docId: "references/framework", label: "Move Framework" },
              { type: "doc", docId: "references/object-display-syntax", label: "Object Display V2 Syntax" },
              { type: "doc", docId: "references/release-notes", label: "Release Notes" },
              { type: "doc", docId: "references/haneul-glossary", label: "Glossary" },
            ],
          },
        ],
      },
      footer: {
        logo: {
          alt: "Haneul Logo",
          src: "img/haneul-logo-footer.svg",
          href: "https://haneul.io",
        },
        style: "dark",
        copyright: `© ${new Date().getFullYear()} Haneul Foundation | Documentation distributed under <a href="https://github.com/GeunhwaJeong/haneul/blob/main/docs/site/LICENSE">CC BY 4.0</a>`,
      },
      codeblock: {
        showGithubLink: true,
        githubLinkLabel: "View on GitHub",
      },
      prism: {
        theme: lightCodeTheme,
        darkTheme: darkCodeTheme,
        additionalLanguages: ["rust", "typescript", "toml", "json"],
      },
    }),
};

export default config;
