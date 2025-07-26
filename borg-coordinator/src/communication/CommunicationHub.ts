import { EventEmitter } from 'events';
import { Server } from 'socket.io';
import { createServer } from 'http';
import { Message, MessageType, Task } from '../types';
import { Logger } from 'winston';
import { v4 as uuidv4 } from 'uuid';

export class CommunicationHub extends EventEmitter {
  private io: Server;
  private server: any;
  private logger: Logger;
  private connectedInstances: Map<string, any> = new Map();
  private messageHistory: Message[] = [];

  constructor(logger: Logger) {
    super();
    this.logger = logger;
    this.server = createServer();
    this.io = new Server(this.server, {
      cors: {
        origin: "*",
        methods: ["GET", "POST"]
      }
    });

    this.setupSocketHandlers();
  }

  async initialize(): Promise<void> {
    const port = process.env.COMMUNICATION_PORT || 3001;
    
    return new Promise((resolve) => {
      this.server.listen(port, () => {
        this.logger.info(`Communication hub listening on port ${port}`);
        resolve();
      });
    });
  }

  private setupSocketHandlers(): void {
    this.io.on('connection', (socket) => {
      this.logger.info(`Instance connected: ${socket.id}`);

      socket.on('register', (data: { instanceId: string, role: string }) => {
        this.connectedInstances.set(data.instanceId, {
          socketId: socket.id,
          role: data.role,
          connectedAt: new Date()
        });
        
        socket.join(`instance_${data.instanceId}`);
        this.logger.info(`Instance ${data.instanceId} registered with role ${data.role}`);
      });

      socket.on('message', (messageData: any) => {
        this.handleIncomingMessage(socket, messageData);
      });

      socket.on('task_update', (data: { taskId: string, status: string, progress?: number }) => {
        this.handleTaskUpdate(socket, data);
      });

      socket.on('collaboration_request', (data: { targetInstance: string, requestType: string, data: any }) => {
        this.handleCollaborationRequest(socket, data);
      });

      socket.on('disconnect', () => {
        this.handleInstanceDisconnect(socket);
      });
    });
  }

  async sendTaskAssignment(instanceId: string, task: Task): Promise<void> {
    const message: Message = {
      id: uuidv4(),
      from: 'orchestrator',
      to: instanceId,
      type: MessageType.TASK_ASSIGNMENT,
      content: task,
      timestamp: new Date()
    };

    await this.sendMessage(message);
  }

  async sendMessage(message: Message): Promise<void> {
    this.messageHistory.push(message);
    
    if (message.to === 'broadcast') {
      this.io.emit('message', message);
    } else {
      this.io.to(`instance_${message.to}`).emit('message', message);
    }

    this.logger.debug(`Message sent from ${message.from} to ${message.to}: ${message.type}`);
    this.emit('messageSent', message);
  }

  async broadcastToRole(role: string, message: Omit<Message, 'id' | 'timestamp'>): Promise<void> {
    const roleInstances = Array.from(this.connectedInstances.entries())
      .filter(([_, instance]) => instance.role === role)
      .map(([instanceId, _]) => instanceId);

    for (const instanceId of roleInstances) {
      const fullMessage: Message = {
        ...message,
        id: uuidv4(),
        to: instanceId,
        timestamp: new Date()
      };
      
      await this.sendMessage(fullMessage);
    }
  }

  private handleIncomingMessage(socket: any, messageData: any): void {
    const message: Message = {
      ...messageData,
      id: uuidv4(),
      timestamp: new Date()
    };

    this.messageHistory.push(message);
    this.logger.debug(`Message received: ${message.type} from ${message.from}`);

    // Route message to appropriate handler
    switch (message.type) {
      case MessageType.TASK_COMPLETION:
        this.emit('taskCompleted', message);
        break;
      case MessageType.STATUS_UPDATE:
        this.emit('statusUpdate', message);
        break;
      case MessageType.ERROR_REPORT:
        this.emit('errorReport', message);
        break;
      case MessageType.COLLABORATION_REQUEST:
        this.handleCollaborationMessage(message);
        break;
      default:
        this.emit('message', message);
    }

    // Forward message if it has a specific target
    if (message.to && message.to !== 'orchestrator') {
      this.sendMessage(message);
    }
  }

  private handleTaskUpdate(socket: any, data: { taskId: string, status: string, progress?: number }): void {
    const instanceId = this.getInstanceIdBySocket(socket.id);
    if (!instanceId) return;

    const message: Message = {
      id: uuidv4(),
      from: instanceId,
      to: 'orchestrator',
      type: MessageType.STATUS_UPDATE,
      content: data,
      timestamp: new Date()
    };

    this.emit('taskUpdate', message);
    this.logger.debug(`Task update from ${instanceId}: ${data.taskId} - ${data.status}`);
  }

  private handleCollaborationRequest(socket: any, data: { targetInstance: string, requestType: string, data: any }): void {
    const fromInstanceId = this.getInstanceIdBySocket(socket.id);
    if (!fromInstanceId) return;

    const message: Message = {
      id: uuidv4(),
      from: fromInstanceId,
      to: data.targetInstance,
      type: MessageType.COLLABORATION_REQUEST,
      content: {
        requestType: data.requestType,
        data: data.data
      },
      timestamp: new Date()
    };

    this.sendMessage(message);
  }

  private handleCollaborationMessage(message: Message): void {
    // Handle collaboration requests between instances
    this.emit('collaborationRequest', message);
  }

  private handleInstanceDisconnect(socket: any): void {
    const instanceId = this.getInstanceIdBySocket(socket.id);
    if (instanceId) {
      this.connectedInstances.delete(instanceId);
      this.logger.info(`Instance ${instanceId} disconnected`);
      this.emit('instanceDisconnected', instanceId);
    }
  }

  private getInstanceIdBySocket(socketId: string): string | null {
    for (const [instanceId, instance] of this.connectedInstances.entries()) {
      if (instance.socketId === socketId) {
        return instanceId;
      }
    }
    return null;
  }

  getConnectedInstances(): Map<string, any> {
    return new Map(this.connectedInstances);
  }

  getMessageHistory(limit?: number): Message[] {
    if (limit) {
      return this.messageHistory.slice(-limit);
    }
    return [...this.messageHistory];
  }

  async shutdown(): Promise<void> {
    this.io.close();
    this.server.close();
    this.logger.info('Communication hub shut down');
  }
}