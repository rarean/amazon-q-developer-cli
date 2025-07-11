# Architecture Constraints

## Technical Constraints

| Constraint | Background | Consequences |
|------------|------------|-------------|
| **Rust Language** | ACP crate is implemented in Rust | Must use Rust-compatible MCP libraries |
| **Async Runtime** | ACP uses tokio async runtime | All proxy operations must be async |
| **No Direct Dependencies** | ACP crate cannot depend on chat-cli crate | Must use trait-based abstraction |
| **Protocol Compatibility** | Must maintain ACP protocol compliance | Proxy must not break existing ACP behavior |

## Organizational Constraints

| Constraint | Background |
|------------|------------|
| **Crate Separation** | ACP and chat-cli are separate crates for architectural reasons |
| **Minimal API Surface** | Keep proxy interface minimal to reduce coupling |

## Conventions

- Follow existing ACP crate patterns
- Use standard Rust async/await patterns
- Maintain error handling consistency
- Follow MCP protocol specifications
