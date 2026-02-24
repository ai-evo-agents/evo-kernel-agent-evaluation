# Evo Kernel Agent: Evaluation

Tests skill quality and performance, scores across multiple dimensions, benchmarks against existing capabilities

## Part of the Evo System

This agent is part of the [Evo self-evolution agent system](https://github.com/ai-evo-agents). It runs using the generic `evo-runner` binary from [evo-agents](https://github.com/ai-evo-agents/evo-agents).

## Quick Start

```sh
# Download the runner binary for your platform
./download-runner.sh

# Set king address (default: http://localhost:3000)
export KING_ADDRESS=http://localhost:3000

# Run the agent
./evo-runner .
```

## Structure

```
evo-kernel-agent-evaluation/
├── soul.md              # Agent identity and behavior rules
├── skills/              # Skills this agent can use
├── mcp/                 # MCP server definitions
├── download-runner.sh   # Downloads evo-runner binary
└── api-key.config       # API key references (gitignored)
```

## License

MIT
