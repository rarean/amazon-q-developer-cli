# Deployment and Operations Guide

## Installation Methods

### NPM Installation
```bash
npm install @zed-industries/claude-code-acp
```

### Global Installation
```bash
npm install -g @zed-industries/claude-code-acp
```

### Development Setup
```bash
git clone <repository>
cd claude-code-acp
npm install
npm run build
```

## Configuration

### Environment Variables
- `ANTHROPIC_API_KEY`: Required Claude API key
- `NODE_ENV`: Environment setting (development/production)
- `DEBUG`: Enable debug logging

### Command Line Usage
```bash
# Basic usage
ANTHROPIC_API_KEY=sk-... claude-code-acp

# With custom configuration
claude-code-acp --port 3000 --host localhost
```

## Integration with ACP Clients

### Zed Editor Integration
Zed includes built-in support for this adapter:
1. Open Agent Panel in Zed
2. Click "New Claude Code Thread"
3. Adapter automatically launches

### Custom Client Integration
```typescript
import { AgentSideConnection } from "@zed-industries/agent-client-protocol";

// Connect to adapter
const connection = new AgentSideConnection(process.stdin, process.stdout);
```

## Monitoring and Logging

### Log Levels
- **Error**: Critical failures and exceptions
- **Warn**: Non-critical issues and deprecations
- **Info**: General operational information
- **Debug**: Detailed execution traces

### Log Destinations
- **stderr**: All application logs (stdout reserved for ACP)
- **File logging**: Optional file-based logging
- **Structured logging**: JSON format for log aggregation

### Key Metrics
- Session count and duration
- Tool execution frequency and latency
- Error rates by category
- Memory and CPU usage

## Troubleshooting

### Common Issues

#### Authentication Failures
```
Error: Invalid API key
Solution: Verify ANTHROPIC_API_KEY environment variable
```

#### Connection Issues
```
Error: Cannot connect to Claude API
Solution: Check network connectivity and API status
```

#### Tool Execution Failures
```
Error: Tool execution timeout
Solution: Increase timeout or check system resources
```

### Debug Mode
```bash
DEBUG=* ANTHROPIC_API_KEY=sk-... claude-code-acp
```

### Health Checks
- Process responsiveness
- API connectivity
- Memory usage monitoring
- Session state validation

## Performance Tuning

### Memory Optimization
- Session cleanup intervals
- File cache size limits
- Stream buffer management
- Garbage collection tuning

### Network Optimization
- Connection pooling
- Request batching
- Compression settings
- Timeout configurations

### Scaling Considerations
- Horizontal scaling via load balancing
- Session affinity requirements
- Resource limits per session
- Rate limiting strategies

## Security Considerations

### API Key Management
- Secure storage of credentials
- Key rotation procedures
- Access logging and auditing
- Environment isolation

### Network Security
- TLS encryption for API calls
- Input validation and sanitization
- Rate limiting and throttling
- CORS configuration for web clients

### File System Security
- Sandboxed file operations
- Path traversal prevention
- Permission validation
- Temporary file cleanup

## Backup and Recovery

### State Management
- Session persistence options
- Configuration backup
- Log retention policies
- Recovery procedures

### Disaster Recovery
- Service restart procedures
- Data recovery processes
- Failover mechanisms
- Monitoring and alerting

## Maintenance Procedures

### Regular Maintenance
- Log rotation and cleanup
- Cache invalidation
- Performance monitoring
- Security updates

### Update Procedures
```bash
# Update to latest version
npm update @zed-industries/claude-code-acp

# Verify installation
claude-code-acp --version
```

### Health Monitoring
- Process monitoring
- Resource usage tracking
- Error rate monitoring
- Performance benchmarking
