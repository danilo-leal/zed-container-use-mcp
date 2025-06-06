# Container Use MCP Extension for Zed

This is a Zed extension that integrates the [Container Use](https://github.com/dagger/container-use) MCP server, enabling containerized environments for coding agents within the Zed editor.

## Installation

While Container Use is in development, you'll need to build the `cu` binary from source.

Visit [the Container Use repository](https://github.com/dagger/container-use?tab=readme-ov-file#installing) for the most up-to-date information on how to install it.

## Configuration

Once installed, the extension will be available automatically. No additional configuration is required if `cu` is in your PATH.

### Custom Binary Path

If you installed `cu` in a custom location, add this to your Zed settings.json:

```json
{
  "context_servers": {
    "container-use-mcp": {
      "settings": {
        "cu_path": "/path/to/cu"
      }
    }
  }
}
```

### Agent Rules

To ensure agents use containerized environments, save the following rules file in your project root:

```bash
curl -o .rules https://raw.githubusercontent.com/dagger/container-use/main/rules/agent.md
```
