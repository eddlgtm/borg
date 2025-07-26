import { createClient, RedisClientType } from 'redis';
import { Task, TaskStatus, TaskPriority } from '../types';
import { Logger } from 'winston';

export class TaskQueue {
  private redis: RedisClientType;
  private logger: Logger;
  private readonly QUEUE_KEY = 'borg:tasks';
  private readonly PRIORITY_QUEUES = {
    [TaskPriority.CRITICAL]: 'borg:tasks:critical',
    [TaskPriority.HIGH]: 'borg:tasks:high',
    [TaskPriority.MEDIUM]: 'borg:tasks:medium',
    [TaskPriority.LOW]: 'borg:tasks:low'
  };

  constructor(logger: Logger) {
    this.logger = logger;
    this.redis = createClient({
      url: process.env.REDIS_URL || 'redis://localhost:6379'
    });

    this.redis.on('error', (err) => {
      this.logger.error('Redis Client Error:', err);
    });
  }

  async initialize(): Promise<void> {
    await this.redis.connect();
    this.logger.info('TaskQueue connected to Redis');
  }

  async addTask(task: Task): Promise<void> {
    const queueKey = this.PRIORITY_QUEUES[task.priority];
    const taskData = JSON.stringify(task);
    
    await this.redis.lPush(queueKey, taskData);
    
    this.logger.debug(`Task ${task.id} added to ${task.priority} priority queue`);
  }

  async getNextTask(): Promise<Task | null> {
    // Check queues in priority order
    const queueOrder = [
      TaskPriority.CRITICAL,
      TaskPriority.HIGH,
      TaskPriority.MEDIUM,
      TaskPriority.LOW
    ];

    for (const priority of queueOrder) {
      const queueKey = this.PRIORITY_QUEUES[priority];
      const taskData = await this.redis.rPop(queueKey);
      
      if (taskData) {
        const task = JSON.parse(taskData) as Task;
        this.logger.debug(`Retrieved task ${task.id} from ${priority} priority queue`);
        return task;
      }
    }

    return null;
  }

  async getQueueStats(): Promise<Record<TaskPriority, number>> {
    const stats: Record<TaskPriority, number> = {
      [TaskPriority.CRITICAL]: 0,
      [TaskPriority.HIGH]: 0,
      [TaskPriority.MEDIUM]: 0,
      [TaskPriority.LOW]: 0
    };

    for (const [priority, queueKey] of Object.entries(this.PRIORITY_QUEUES)) {
      const length = await this.redis.lLen(queueKey);
      stats[priority as TaskPriority] = length;
    }

    return stats;
  }

  async clearQueue(priority?: TaskPriority): Promise<void> {
    if (priority) {
      const queueKey = this.PRIORITY_QUEUES[priority];
      await this.redis.del(queueKey);
      this.logger.info(`Cleared ${priority} priority queue`);
    } else {
      for (const queueKey of Object.values(this.PRIORITY_QUEUES)) {
        await this.redis.del(queueKey);
      }
      this.logger.info('Cleared all task queues');
    }
  }

  async disconnect(): Promise<void> {
    await this.redis.disconnect();
    this.logger.info('TaskQueue disconnected from Redis');
  }
}