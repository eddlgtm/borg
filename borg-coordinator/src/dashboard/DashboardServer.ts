import express from 'express';
import { createServer } from 'http';
import { Server } from 'socket.io';
import path from 'path';
import { Orchestrator } from '../orchestrator/Orchestrator';
import { Logger } from 'winston';

export class DashboardServer {
  private app: express.Application;
  private server: any;
  private io: Server;
  private orchestrator: Orchestrator;
  private logger: Logger;

  constructor(orchestrator: Orchestrator, logger: Logger) {
    this.orchestrator = orchestrator;
    this.logger = logger;
    this.app = express();
    this.server = createServer(this.app);
    this.io = new Server(this.server, {
      cors: {
        origin: "*",
        methods: ["GET", "POST"]
      }
    });

    this.setupMiddleware();
    this.setupRoutes();
    this.setupWebSocket();
    this.setupOrchestratorEvents();
  }

  private setupMiddleware(): void {
    this.app.use(express.json());
    this.app.use(express.static(path.join(__dirname, 'public')));
  }

  private setupRoutes(): void {
    // API Routes
    this.app.get('/api/instances', async (req, res) => {
      try {
        const instances = await this.orchestrator.getAllInstances();
        res.json(instances);
      } catch (error) {
        res.status(500).json({ error: 'Failed to fetch instances' });
      }
    });

    this.app.get('/api/tasks', async (req, res) => {
      try {
        const tasks = await this.orchestrator.getAllTasks();
        res.json(tasks);
      } catch (error) {
        res.status(500).json({ error: 'Failed to fetch tasks' });
      }
    });

    this.app.post('/api/instances', async (req, res) => {
      try {
        const { role, capabilities } = req.body;
        const instance = await this.orchestrator.createInstance(role, capabilities);
        res.json(instance);
      } catch (error) {
        res.status(500).json({ error: 'Failed to create instance' });
      }
    });

    this.app.post('/api/tasks', async (req, res) => {
      try {
        const { type, description, priority, targetInstanceId } = req.body;
        const task = await this.orchestrator.assignTask({
          type,
          description,
          priority
        }, targetInstanceId);
        res.json(task);
      } catch (error) {
        res.status(500).json({ error: 'Failed to create task' });
      }
    });

    this.app.delete('/api/instances/:id', async (req, res) => {
      try {
        const { id } = req.params;
        await this.orchestrator.terminateInstance(id);
        res.json({ success: true });
      } catch (error) {
        res.status(500).json({ error: 'Failed to terminate instance' });
      }
    });

    this.app.get('/api/instances/:id/status', async (req, res) => {
      try {
        const { id } = req.params;
        const instance = await this.orchestrator.getInstanceStatus(id);
        if (!instance) {
          return res.status(404).json({ error: 'Instance not found' });
        }
        res.json(instance);
      } catch (error) {
        res.status(500).json({ error: 'Failed to fetch instance status' });
      }
    });

    // Serve dashboard HTML
    this.app.get('/', (req, res) => {
      res.sendFile(path.join(__dirname, 'public', 'index.html'));
    });
  }

  private setupWebSocket(): void {
    this.io.on('connection', (socket) => {
      this.logger.info(`Dashboard client connected: ${socket.id}`);

      socket.on('subscribe', (data: { events: string[] }) => {
        data.events.forEach(event => {
          socket.join(event);
        });
      });

      socket.on('disconnect', () => {
        this.logger.info(`Dashboard client disconnected: ${socket.id}`);
      });
    });
  }

  private setupOrchestratorEvents(): void {
    this.orchestrator.on('instanceCreated', (instance) => {
      this.io.to('instances').emit('instanceCreated', instance);
    });

    this.orchestrator.on('instanceTerminated', (instance) => {
      this.io.to('instances').emit('instanceTerminated', instance);
    });

    this.orchestrator.on('taskAssigned', (task) => {
      this.io.to('tasks').emit('taskAssigned', task);
    });

    this.orchestrator.on('taskCompleted', (data) => {
      this.io.to('tasks').emit('taskCompleted', data);
    });

    this.orchestrator.on('instanceError', (data) => {
      this.io.to('instances').emit('instanceError', data);
    });

    this.orchestrator.on('taskAssignedToInstance', (data) => {
      this.io.to('tasks').emit('taskAssignedToInstance', data);
    });
  }

  async start(port: number = 3000): Promise<void> {
    return new Promise((resolve) => {
      this.server.listen(port, () => {
        this.logger.info(`Dashboard server running on port ${port}`);
        resolve();
      });
    });
  }

  async stop(): Promise<void> {
    return new Promise((resolve) => {
      this.server.close(() => {
        this.logger.info('Dashboard server stopped');
        resolve();
      });
    });
  }
}