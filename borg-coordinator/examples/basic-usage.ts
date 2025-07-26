import { Orchestrator } from '../src/orchestrator/Orchestrator';
import { InstanceRole, TaskType, TaskPriority } from '../src/types';
import winston from 'winston';

// Example of how to use the Borg Coordinator programmatically

async function basicExample() {
  // Setup logger
  const logger = winston.createLogger({
    level: 'info',
    format: winston.format.simple(),
    transports: [new winston.transports.Console()]
  });

  // Create orchestrator
  const orchestrator = new Orchestrator(logger);
  await orchestrator.initialize();

  try {
    // Create a team of Claude instances
    const supervisor = await orchestrator.createInstance(InstanceRole.SUPERVISOR, ['leadership', 'architecture']);
    const developer = await orchestrator.createInstance(InstanceRole.DEVELOPER, ['typescript', 'react']);
    const tester = await orchestrator.createInstance(InstanceRole.TESTER, ['jest', 'cypress']);

    console.log('Created instances:');
    console.log('- Supervisor:', supervisor.id);
    console.log('- Developer:', developer.id);
    console.log('- Tester:', tester.id);

    // Assign some tasks
    const featureTask = await orchestrator.assignTask({
      type: TaskType.FEATURE_IMPLEMENTATION,
      description: 'Implement user authentication system with JWT tokens',
      priority: TaskPriority.HIGH
    });

    const testTask = await orchestrator.assignTask({
      type: TaskType.TEST_CREATION,
      description: 'Create comprehensive tests for the authentication system',
      priority: TaskPriority.HIGH,
      dependencies: [featureTask.id]
    });

    const reviewTask = await orchestrator.assignTask({
      type: TaskType.CODE_REVIEW,
      description: 'Review the authentication implementation for security best practices',
      priority: TaskPriority.MEDIUM,
      dependencies: [featureTask.id]
    });

    console.log('\\nAssigned tasks:');
    console.log('- Feature:', featureTask.id);
    console.log('- Tests:', testTask.id);
    console.log('- Review:', reviewTask.id);

    // Listen for completion events
    orchestrator.on('taskCompleted', ({ task, instance, result }) => {
      console.log(`\\nTask completed by ${instance.role}:`);
      console.log(`- Task: ${task.description}`);
      console.log(`- Success: ${result.success}`);
      if (result.filesModified) {
        console.log(`- Files modified: ${result.filesModified.join(', ')}`);
      }
    });

    orchestrator.on('instanceError', ({ instance, error }) => {
      console.error(`\\nInstance ${instance.role} encountered error:`, error.message);
    });

    // Keep the process running to see task completion
    console.log('\\nWaiting for tasks to complete...');
    console.log('(This is a simulation - actual Claude API integration required for real execution)');

  } catch (error) {
    console.error('Error in basic example:', error);
  }
}

if (require.main === module) {
  basicExample().catch(console.error);
}