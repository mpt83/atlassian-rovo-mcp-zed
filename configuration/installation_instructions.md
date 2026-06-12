# Atlassian Rovo MCP Server

This extension connects Zed to Atlassian Rovo MCP Server through Atlassian's recommended `mcp-remote` proxy.

## Prerequisites

- Node.js 18 or later
- `npx` available on your `PATH`
- Access to an Atlassian Cloud site with Jira, Confluence, and/or Compass
- Atlassian Rovo MCP enabled for your organization

## How It Runs

The extension launches:

```sh
npx -y mcp-remote@latest https://mcp.atlassian.com/v1/mcp/authv2
```

The first run should open or prompt an Atlassian OAuth flow. Do not paste bearer tokens into Zed settings.

## Test Prompt

After the context server is active, open Zed's Agent Panel and try:

```text
Use Atlassian Rovo MCP to list the Jira projects I can access.
```

## Troubleshooting

If the server does not start, test the proxy in a terminal:

```sh
NPM_CONFIG_CACHE=.npm-cache npx -y mcp-remote@latest https://mcp.atlassian.com/v1/mcp/authv2
```

If that fails, verify Node.js and `npx` are installed and that your Atlassian organization allows Rovo MCP access.
