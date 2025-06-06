# Rover Zed Extension

This is the [Rover](https://getrover.com) extension for the [Zed](https://zed.dev) text editor.

It can be installed through the Zed extension registry.

## Configuration

The Rover extension has a couple of configuration options to enable interaction with
the Rover. You can configure these options under the `"context_servers"` key of your
Zed settings:

```json
"context_servers": {
  "rover": {
    "settings": {
      "api_key": "",
      "host": "https://api.getrover.com"
    }
  }
}
```

The `"api_key"` field is always required. You can generate a personal API key by
clicking [here](https://app.getrover.com/api-keys), or you can generate an
organization API key by heading to [your organization](https://app.getrover.com/org) >
Settings > API keys.

The `"host"` field defaults to `https://api.getrover.com`. You'll know if you need
to change this.

You'll then be able to enable the Rover context server for use with Zed's Agent
mode in your Agent panel settings.
