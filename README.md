# Atlassian Rovo MCP for Zed

Zed extension that exposes the Atlassian Rovo MCP Server as a Zed context server.

It uses Atlassian's remote MCP endpoint through `mcp-remote`:

```sh
npx -y mcp-remote@latest https://mcp.atlassian.com/v1/mcp/authv2
```

## Local Development

1. Open Zed.
2. Run `zed: extensions`.
3. Click `Install Dev Extension`.
4. Select this repository.
5. Open `agent: open settings`.
6. Enable `Atlassian Rovo MCP Server`.

On first use, complete the Atlassian OAuth flow.

## Settings

```jsonc
{
  "context_servers": {
    "atlassian-rovo": {
      "url": "https://mcp.atlassian.com/v1/mcp/authv2",
      "mcp_remote_package": "mcp-remote@latest",
      "extra_args": [],
      "npm_cache": ".npm-cache"
    }
  }
}
```

Do not put tokens or client secrets in settings. Use the OAuth flow.
