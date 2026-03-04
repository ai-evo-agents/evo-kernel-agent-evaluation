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

## Capabilities

Beyond the standard pipeline evaluation stage, this agent handles two additional responsibilities:

- **Error Recovery Analysis** — When a pipeline stage fails, king asks this agent to analyze the failure and recommend a recovery action (`retry`, `decompose`, `skip`, or `abort`). Enforces max 3 retries and restricts `skip` to evaluation/skill_manage stages only.
- **Task Decomposition** — When a task is too complex (auto-detected or manually triggered), king asks this agent to break it into smaller subtasks with specific types, summaries, and payloads.

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
