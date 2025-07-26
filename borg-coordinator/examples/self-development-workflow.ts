import { Orchestrator } from '../src/orchestrator/Orchestrator';
import { InstanceRole, TaskType, TaskPriority } from '../src/types';
import winston from 'winston';

/**
 * Example: Using Borg Coordinator to develop features on itself
 * This demonstrates the "meta-development" workflow
 */

async function selfDevelopmentWorkflow() {
  const logger = winston.createLogger({
    level: 'info',
    format: winston.format.simple(),
    transports: [new winston.transports.Console()]
  });

  const orchestrator = new Orchestrator(logger);
  await orchestrator.initialize();

  try {
    console.log('🤖 Creating AI development team for self-improvement...\n');

    // Create a specialized team for developing the Borg Coordinator itself
    const supervisor = await orchestrator.createInstance(
      InstanceRole.SUPERVISOR, 
      ['typescript', 'architecture', 'project-management', 'node.js']
    );

    const backendDev = await orchestrator.createInstance(
      InstanceRole.DEVELOPER, 
      ['typescript', 'node.js', 'express', 'redis', 'docker', 'backend']
    );

    const frontendDev = await orchestrator.createInstance(
      InstanceRole.DEVELOPER, 
      ['typescript', 'javascript', 'html', 'css', 'socket.io', 'frontend']
    );

    const tester = await orchestrator.createInstance(
      InstanceRole.TESTER, 
      ['jest', 'testing', 'typescript', 'integration-testing']
    );

    const researcher = await orchestrator.createInstance(
      InstanceRole.RESEARCHER, 
      ['analysis', 'documentation', 'research', 'optimization']
    );

    console.log('✅ Team assembled:');
    console.log(`  - Supervisor: ${supervisor.id}`);
    console.log(`  - Backend Dev: ${backendDev.id}`);
    console.log(`  - Frontend Dev: ${frontendDev.id}`);
    console.log(`  - Tester: ${tester.id}`);
    console.log(`  - Researcher: ${researcher.id}\\n`);

    // Example: Develop a new feature - "Instance Performance Monitoring"
    console.log('🚀 Starting feature development: Instance Performance Monitoring\\n');

    // Step 1: Research Phase
    const researchTask = await orchestrator.assignTask({
      type: TaskType.RESEARCH,
      description: `Research requirements for Instance Performance Monitoring feature:
        - Analyze current system architecture in /Users/edd/Code/borg/borg-coordinator
        - Study how instances are currently managed in src/instances/
        - Research performance metrics we should track (CPU, memory, task completion times)
        - Identify best practices for real-time monitoring in Node.js applications
        - Recommend implementation approach and architecture changes needed`,
      priority: TaskPriority.HIGH
    }, researcher.id);

    // Step 2: Architecture Planning
    const planningTask = await orchestrator.assignTask({
      type: TaskType.FEATURE_IMPLEMENTATION,
      description: `Create architectural plan for Instance Performance Monitoring:
        - Review research findings and create implementation plan
        - Design database schema additions for performance metrics
        - Plan API endpoints for metrics collection and retrieval
        - Design dashboard UI components for performance visualization
        - Create task breakdown for backend and frontend development
        - Identify integration points with existing InstanceManager and Dashboard`,
      priority: TaskPriority.HIGH,
      dependencies: [researchTask.id]
    }, supervisor.id);

    // Step 3: Backend Implementation
    const backendTask = await orchestrator.assignTask({
      type: TaskType.FEATURE_IMPLEMENTATION,
      description: `Implement backend for Instance Performance Monitoring:
        - Add performance metrics collection to src/instances/InstanceManager.ts
        - Create new endpoints in src/dashboard/DashboardServer.ts for metrics API
        - Implement Redis storage for performance data
        - Add real-time metrics broadcasting via Socket.IO
        - Update TypeScript types in src/types/index.ts for performance data
        - Ensure proper error handling and logging`,
      priority: TaskPriority.HIGH,
      dependencies: [planningTask.id]
    }, backendDev.id);

    // Step 4: Frontend Implementation  
    const frontendTask = await orchestrator.assignTask({
      type: TaskType.FEATURE_IMPLEMENTATION,
      description: `Implement frontend for Instance Performance Monitoring:
        - Add performance charts to src/dashboard/public/index.html
        - Update src/dashboard/public/app.js with metrics visualization
        - Create real-time performance graphs using Chart.js or similar
        - Add performance alerts and notifications
        - Update dashboard UI to display instance health and performance
        - Ensure responsive design and good UX`,
      priority: TaskPriority.HIGH,
      dependencies: [planningTask.id]
    }, frontendDev.id);

    // Step 5: Testing
    const testingTask = await orchestrator.assignTask({
      type: TaskType.TEST_CREATION,
      description: `Create comprehensive tests for Instance Performance Monitoring:
        - Write unit tests for new performance collection methods
        - Create integration tests for metrics API endpoints
        - Test real-time data broadcasting functionality
        - Add performance regression tests
        - Test dashboard performance visualization components
        - Ensure all new code has proper test coverage`,
      priority: TaskPriority.MEDIUM,
      dependencies: [backendTask.id, frontendTask.id]
    }, tester.id);

    // Step 6: Final Review and Integration
    const reviewTask = await orchestrator.assignTask({
      type: TaskType.CODE_REVIEW,
      description: `Final review and integration of Instance Performance Monitoring:
        - Review all implemented code for quality and consistency
        - Ensure proper TypeScript typing throughout
        - Verify integration with existing Docker setup
        - Check performance impact of new monitoring features
        - Update documentation and README
        - Approve feature for production deployment`,
      priority: TaskPriority.HIGH,
      dependencies: [testingTask.id]
    }, supervisor.id);

    console.log('📋 Tasks assigned:');
    console.log(`  1. Research: ${researchTask.id}`);
    console.log(`  2. Planning: ${planningTask.id}`);
    console.log(`  3. Backend: ${backendTask.id}`);
    console.log(`  4. Frontend: ${frontendTask.id}`);
    console.log(`  5. Testing: ${testingTask.id}`);
    console.log(`  6. Review: ${reviewTask.id}\\n`);

    // Monitor progress
    orchestrator.on('taskCompleted', ({ task, instance, result }) => {
      console.log(`✅ Task completed by ${instance.role}: ${task.description.substring(0, 80)}...`);
      if (result.filesModified && result.filesModified.length > 0) {
        console.log(`   📁 Files modified: ${result.filesModified.join(', ')}`);
      }
      console.log('');
    });

    orchestrator.on('instanceError', ({ instance, error }) => {
      console.error(`❌ Instance ${instance.role} error: ${error.message}`);
    });

    console.log('🔄 Development workflow started...');
    console.log('💡 This demonstrates how AI agents can collaborate to develop features on their own codebase!');

  } catch (error) {
    console.error('Error in self-development workflow:', error);
  }
}

// Example of simpler tasks for immediate improvements
async function quickImprovements() {
  const logger = winston.createLogger({
    level: 'info',
    format: winston.format.simple(),
    transports: [new winston.transports.Console()]
  });

  const orchestrator = new Orchestrator(logger);
  await orchestrator.initialize();

  console.log('🛠️  Quick improvement tasks...\\n');

  const developer = await orchestrator.createInstance(InstanceRole.DEVELOPER, ['typescript', 'documentation']);
  const tester = await orchestrator.createInstance(InstanceRole.TESTER, ['testing', 'typescript']);

  // Task 1: Add better error handling
  await orchestrator.assignTask({
    type: TaskType.BUG_FIX,
    description: `Improve error handling in src/orchestrator/Orchestrator.ts:
      - Add try-catch blocks around async operations
      - Implement proper error logging with context
      - Add graceful degradation for Redis connection failures
      - Return meaningful error messages to dashboard`,
    priority: TaskPriority.HIGH
  });

  // Task 2: Add input validation
  await orchestrator.assignTask({
    type: TaskType.FEATURE_IMPLEMENTATION,
    description: `Add input validation to API endpoints in src/dashboard/DashboardServer.ts:
      - Validate task creation inputs (type, description, priority)
      - Validate instance creation inputs (role, capabilities)
      - Add proper HTTP status codes for validation errors
      - Sanitize user inputs to prevent XSS`,
    priority: TaskPriority.MEDIUM
  });

  // Task 3: Add more tests
  await orchestrator.assignTask({
    type: TaskType.TEST_CREATION,
    description: `Add missing test coverage:
      - Unit tests for TaskQueue class
      - Integration tests for CommunicationHub
      - API endpoint tests for DashboardServer
      - Mock Redis for testing
      - Add test npm script to package.json`,
    priority: TaskPriority.MEDIUM
  });

  console.log('🎯 Quick improvement tasks assigned for immediate value!');
}

if (require.main === module) {
  const workflow = process.argv[2];
  
  if (workflow === 'quick') {
    quickImprovements().catch(console.error);
  } else {
    selfDevelopmentWorkflow().catch(console.error);
  }
}