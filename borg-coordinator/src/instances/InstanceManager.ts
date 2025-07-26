import { EventEmitter } from 'events';
import { ClaudeInstance, Task, TaskResult, InstanceRole } from '../types';
import { Logger } from 'winston';
import { ClaudeCodeClient } from './ClaudeAPIClient';

export class InstanceManager extends EventEmitter {
  private instances: Map<string, ClaudeCodeClient> = new Map();
  private logger: Logger;

  constructor(logger: Logger) {
    super();
    this.logger = logger;
  }

  async createInstance(instance: ClaudeInstance): Promise<void> {
    const client = new ClaudeCodeClient(instance, this.logger);
    
    client.on('taskCompleted', (task: Task, result: TaskResult) => {
      this.emit('taskCompleted', instance.id, task, result);
    });

    client.on('error', (error: Error) => {
      this.emit('instanceError', instance.id, error);
    });

    client.on('statusUpdate', (status: string) => {
      this.emit('statusUpdate', instance.id, status);
    });

    this.instances.set(instance.id, client);
    await client.initialize();

    this.logger.info(`Instance manager created client for ${instance.id}`);
  }

  async assignTask(instanceId: string, task: Task): Promise<void> {
    const client = this.instances.get(instanceId);
    if (!client) {
      throw new Error(`Instance ${instanceId} not found in manager`);
    }

    await client.executeTask(task);
  }

  async terminateInstance(instanceId: string): Promise<void> {
    const client = this.instances.get(instanceId);
    if (!client) {
      throw new Error(`Instance ${instanceId} not found in manager`);
    }

    await client.terminate();
    this.instances.delete(instanceId);

    this.logger.info(`Instance ${instanceId} terminated in manager`);
  }

  async getInstanceHealth(instanceId: string): Promise<boolean> {
    const client = this.instances.get(instanceId);
    if (!client) {
      return false;
    }

    return await client.healthCheck();
  }

  getActiveInstanceCount(): number {
    return this.instances.size;
  }
}