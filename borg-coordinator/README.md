# Borg Coordinator

A sophisticated system for managing multiple Claude Code instances working collaboratively on software development projects. Create teams of AI agents with specialized roles (supervisor, developer, tester, reviewer, researcher) that can work together on complex tasks.

## Features

- **Multi-Instance Management**: Create and manage multiple Claude instances with different roles
- **Role-Based Assignment**: Specialized prompts and behaviors for each role type
- **Task Queue System**: Priority-based task distribution with Redis backend
- **Real-Time Dashboard**: Web interface for monitoring instances and tasks
- **Inter-Instance Communication**: Message passing and collaboration between instances
- **Event-Driven Architecture**: Real-time updates and notifications

## Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Dashboard     │    │   Orchestrator  │    │ Claude Instances│
│   (Web UI)      │◄──►│   (Core Logic)  │◄──►│ (API Clients)   │
└─────────────────┘    └─────────────────┘    └─────────────────┘
                                │
                       ┌─────────────────┐
                       │   Task Queue    │
                       │   (Redis)       │
                       └─────────────────┘
```

## Quick Start

### Prerequisites

- Docker and Docker Compose
- Claude Code subscription and CLI tool installed

### Installation

#### Option 1: Docker (Recommended)

```bash
cd borg-coordinator

# Quick setup
./scripts/setup.sh

# Or manually:
docker-compose up --build -d
```

#### Option 2: Manual Setup

```bash
cd borg-coordinator
npm install

# Start Redis manually
redis-server

# Configure environment
cp .env.example .env
# Edit .env file with your settings

# Start the system
npm run dev
```

### Configuration

The `.env` file is created automatically from the template. Key settings:

```env
CLAUDE_CODE_PATH=claude
REDIS_URL=redis://localhost:6379
DASHBOARD_PORT=3000
COMMUNICATION_PORT=3001
LOG_LEVEL=info
```

**Important**: Make sure you have Claude Code installed and accessible. If `claude` is not in your PATH, specify the full path to the Claude Code executable in `CLAUDE_CODE_PATH`.

### Docker Commands

```bash
# Start all services
docker-compose up -d

# Development mode with hot reload
./scripts/dev.sh

# View logs
docker-compose logs -f

# Stop services
docker-compose down

# Rebuild and restart
docker-compose up --build -d
```

### Access Dashboard

Open http://localhost:3000 in your browser to access the management dashboard.

## Instance Roles

### Supervisor
- Oversees project progress
- Makes architectural decisions
- Coordinates team members
- Reviews major changes

### Developer
- Implements features and functionality
- Writes clean, maintainable code
- Follows coding best practices
- Collaborates on implementations

### Tester
- Creates comprehensive test suites
- Identifies edge cases and bugs
- Sets up testing infrastructure
- Validates implementations

### Reviewer
- Conducts thorough code reviews
- Ensures adherence to standards
- Checks for security issues
- Provides constructive feedback

### Researcher
- Investigates new technologies
- Analyzes requirements
- Provides technical recommendations
- Documents findings

## API Usage

### Create Instance

```typescript
import { Orchestrator } from './src/orchestrator/Orchestrator';
import { InstanceRole } from './src/types';

const orchestrator = new Orchestrator(logger);
await orchestrator.initialize();

const developer = await orchestrator.createInstance(
  InstanceRole.DEVELOPER, 
  ['typescript', 'react', 'testing']
);
```

### Assign Task

```typescript
import { TaskType, TaskPriority } from './src/types';

const task = await orchestrator.assignTask({
  type: TaskType.FEATURE_IMPLEMENTATION,
  description: 'Implement user authentication system',
  priority: TaskPriority.HIGH
});
```

### Monitor Events

```typescript
orchestrator.on('taskCompleted', ({ task, instance, result }) => {
  console.log(`Task ${task.id} completed by ${instance.role}`);
});

orchestrator.on('instanceError', ({ instance, error }) => {
  console.error(`Instance ${instance.id} error:`, error);
});
```

## REST API Endpoints

- `GET /api/instances` - List all instances
- `POST /api/instances` - Create new instance
- `DELETE /api/instances/:id` - Terminate instance
- `GET /api/tasks` - List all tasks
- `POST /api/tasks` - Create new task
- `GET /api/instances/:id/status` - Get instance status

## Task Types

- `FEATURE_IMPLEMENTATION` - New feature development
- `BUG_FIX` - Bug fixes and corrections
- `CODE_REVIEW` - Code review and quality assurance
- `TEST_CREATION` - Test writing and validation
- `RESEARCH` - Investigation and analysis
- `DOCUMENTATION` - Documentation creation

## Task Priorities

- `CRITICAL` - Highest priority, assigned immediately
- `HIGH` - High priority, assigned before medium/low
- `MEDIUM` - Standard priority (default)
- `LOW` - Lowest priority, assigned when no higher priority tasks

## Communication

Instances can communicate through:
- **Task Assignment Messages** - From orchestrator to instances
- **Status Updates** - From instances to orchestrator
- **Collaboration Requests** - Between instances
- **Error Reports** - Error notifications

## Example Usage

See `examples/basic-usage.ts` for a complete example of creating instances and assigning tasks programmatically.

## Development

### Project Structure

```
src/
├── orchestrator/     # Core coordination logic
├── instances/        # Claude instance management
├── roles/           # Role-specific prompts and behaviors
├── communication/   # Inter-instance messaging
├── dashboard/       # Web dashboard
└── types/          # TypeScript type definitions
```

### Running Tests

```bash
npm test
```

### Building

```bash
npm run build
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

MIT License - see LICENSE file for details.

## Security Notes

- Ensure Claude Code has appropriate permissions in workspaces
- Use HTTPS in production deployments
- Implement proper authentication for the dashboard
- Monitor Claude Code usage and subscription limits
- Review generated code before deployment
- Isolate instance workspaces for security

## Troubleshooting

### Common Issues

1. **Redis Connection Error**: Ensure Redis server is running
2. **Claude Code Not Found**: Verify CLAUDE_CODE_PATH is set correctly and Claude Code is installed
3. **Port Conflicts**: Check DASHBOARD_PORT and COMMUNICATION_PORT settings
4. **Memory Issues**: Monitor instance count and task queue size
5. **Workspace Permissions**: Ensure the system can create and modify workspace directories
6. **Claude Code Process Timeout**: Increase TASK_TIMEOUT if needed (default: 5 minutes)

### Logging

Logs are written to:
- `logs/error.log` - Error messages only
- `logs/combined.log` - All log messages
- Console output for development

Set `LOG_LEVEL` environment variable to control verbosity (error, warn, info, debug).