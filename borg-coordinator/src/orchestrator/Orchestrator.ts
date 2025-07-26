import { EventEmitter } from 'events';
import { v4 as uuidv4 } from 'uuid';
import { ClaudeInstance, Task, TaskType, TaskStatus, TaskPriority, InstanceRole, InstanceStatus } from '../types';
import { InstanceManager } from '../instances/InstanceManager';
import { TaskQueue } from './TaskQueue';
import { CommunicationHub } from '../communication/CommunicationHub';
import { Logger } from 'winston';

export class Orchestrator extends EventEmitter {
  private instances: Map<string, ClaudeInstance> = new Map();
  private tasks: Map<string, Task> = new Map();
  private instanceManager: InstanceManager;
  private taskQueue: TaskQueue;
  private communicationHub: CommunicationHub;
  private logger: Logger;

  constructor(logger: Logger) {
    super();
    this.logger = logger;
    this.instanceManager = new InstanceManager(logger);
    this.taskQueue = new TaskQueue(logger);
    this.communicationHub = new CommunicationHub(logger);
    
    this.setupEventHandlers();
  }

  async initialize(): Promise<void> {
    this.logger.info('Initializing Orchestrator...');
    await this.taskQueue.initialize();
    await this.communicationHub.initialize();
    this.logger.info('Orchestrator initialized successfully');
  }

  async createInstance(role: InstanceRole, capabilities: string[] = []): Promise<ClaudeInstance> {
    const instance: ClaudeInstance = {
      id: uuidv4(),
      role,
      status: InstanceStatus.IDLE,
      capabilities,
      createdAt: new Date(),
      lastActivity: new Date()
    };

    this.instances.set(instance.id, instance);
    await this.instanceManager.createInstance(instance);
    
    this.logger.info(`Created ${role} instance: ${instance.id}`);
    this.emit('instanceCreated', instance);
    
    return instance;
  }

  async assignTask(task: Partial<Task>, targetInstanceId?: string): Promise<Task> {
    const fullTask: Task = {
      id: uuidv4(),
      type: task.type || TaskType.FEATURE_IMPLEMENTATION,
      description: task.description || '',
      status: TaskStatus.PENDING,
      priority: task.priority || TaskPriority.MEDIUM,
      dependencies: task.dependencies || [],
      createdAt: new Date(),
      updatedAt: new Date(),
      ...task
    };

    this.tasks.set(fullTask.id, fullTask);

    if (targetInstanceId) {
      await this.assignTaskToInstance(fullTask, targetInstanceId);
    } else {
      await this.taskQueue.addTask(fullTask);
      await this.findBestInstanceForTask(fullTask);
    }

    this.logger.info(`Task assigned: ${fullTask.id} - ${fullTask.description}`);
    this.emit('taskAssigned', fullTask);

    return fullTask;
  }

  private async findBestInstanceForTask(task: Task): Promise<void> {
    const availableInstances = Array.from(this.instances.values())
      .filter(instance => instance.status === InstanceStatus.IDLE);

    if (availableInstances.length === 0) {
      this.logger.warn(`No available instances for task ${task.id}`);
      return;
    }

    // Simple role-based assignment logic
    const rolePreferences = this.getRolePreferencesForTask(task.type);
    const bestInstance = availableInstances.find(instance => 
      rolePreferences.includes(instance.role)
    ) || availableInstances[0];

    await this.assignTaskToInstance(task, bestInstance.id);
  }

  private getRolePreferencesForTask(taskType: TaskType): InstanceRole[] {
    switch (taskType) {
      case TaskType.CODE_REVIEW:
        return [InstanceRole.REVIEWER, InstanceRole.SUPERVISOR];
      case TaskType.TEST_CREATION:
        return [InstanceRole.TESTER, InstanceRole.DEVELOPER];
      case TaskType.RESEARCH:
        return [InstanceRole.RESEARCHER, InstanceRole.SUPERVISOR];
      case TaskType.FEATURE_IMPLEMENTATION:
      case TaskType.BUG_FIX:
        return [InstanceRole.DEVELOPER, InstanceRole.SUPERVISOR];
      default:
        return [InstanceRole.DEVELOPER];
    }
  }

  private async assignTaskToInstance(task: Task, instanceId: string): Promise<void> {
    const instance = this.instances.get(instanceId);
    if (!instance) {
      throw new Error(`Instance ${instanceId} not found`);
    }

    task.assignedTo = instanceId;
    task.status = TaskStatus.IN_PROGRESS;
    task.updatedAt = new Date();

    instance.currentTask = task;
    instance.status = InstanceStatus.WORKING;
    instance.lastActivity = new Date();

    this.tasks.set(task.id, task);
    this.instances.set(instanceId, instance);

    await this.instanceManager.assignTask(instanceId, task);
    await this.communicationHub.sendTaskAssignment(instanceId, task);

    this.logger.info(`Task ${task.id} assigned to instance ${instanceId}`);
    this.emit('taskAssignedToInstance', { task, instance });
  }

  async getInstanceStatus(instanceId: string): Promise<ClaudeInstance | undefined> {
    return this.instances.get(instanceId);
  }

  async getAllInstances(): Promise<ClaudeInstance[]> {
    return Array.from(this.instances.values());
  }

  async getAllTasks(): Promise<Task[]> {
    return Array.from(this.tasks.values());
  }

  async terminateInstance(instanceId: string): Promise<void> {
    const instance = this.instances.get(instanceId);
    if (!instance) {
      throw new Error(`Instance ${instanceId} not found`);
    }

    await this.instanceManager.terminateInstance(instanceId);
    this.instances.delete(instanceId);

    this.logger.info(`Instance ${instanceId} terminated`);
    this.emit('instanceTerminated', instance);
  }

  private setupEventHandlers(): void {
    this.instanceManager.on('taskCompleted', (instanceId: string, task: Task, result: any) => {
      this.handleTaskCompletion(instanceId, task, result);
    });

    this.instanceManager.on('instanceError', (instanceId: string, error: Error) => {
      this.handleInstanceError(instanceId, error);
    });
  }

  private async handleTaskCompletion(instanceId: string, task: Task, result: any): Promise<void> {
    const instance = this.instances.get(instanceId);
    if (!instance) return;

    task.status = TaskStatus.COMPLETED;
    task.result = result;
    task.updatedAt = new Date();

    instance.status = InstanceStatus.IDLE;
    instance.currentTask = undefined;
    instance.lastActivity = new Date();

    this.tasks.set(task.id, task);
    this.instances.set(instanceId, instance);

    this.logger.info(`Task ${task.id} completed by instance ${instanceId}`);
    this.emit('taskCompleted', { task, instance, result });

    // Check for next task
    await this.assignNextTask(instanceId);
  }

  private async handleInstanceError(instanceId: string, error: Error): Promise<void> {
    const instance = this.instances.get(instanceId);
    if (!instance) return;

    instance.status = InstanceStatus.ERROR;
    instance.lastActivity = new Date();

    if (instance.currentTask) {
      instance.currentTask.status = TaskStatus.FAILED;
      this.tasks.set(instance.currentTask.id, instance.currentTask);
    }

    this.instances.set(instanceId, instance);
    
    this.logger.error(`Instance ${instanceId} encountered error:`, error);
    this.emit('instanceError', { instance, error });
  }

  private async assignNextTask(instanceId: string): Promise<void> {
    const nextTask = await this.taskQueue.getNextTask();
    if (nextTask) {
      await this.assignTaskToInstance(nextTask, instanceId);
    }
  }
}