import { EventEmitter } from 'events';
import { spawn, ChildProcess } from 'child_process';
import { ClaudeInstance, Task, TaskResult, TaskType, InstanceRole } from '../types';
import { RolePromptGenerator } from '../roles/RolePromptGenerator';
import { Logger } from 'winston';
import path from 'path';
import fs from 'fs/promises';

export class ClaudeCodeClient extends EventEmitter {
  private instance: ClaudeInstance;
  private process: ChildProcess | null = null;
  private rolePromptGenerator: RolePromptGenerator;
  private logger: Logger;
  private isActive: boolean = false;
  private workspaceDir: string;
  private sessionFile: string;

  constructor(instance: ClaudeInstance, logger: Logger) {
    super();
    this.instance = instance;
    this.logger = logger;
    this.rolePromptGenerator = new RolePromptGenerator();
    this.workspaceDir = path.join(process.cwd(), 'workspaces', instance.id);
    this.sessionFile = path.join(this.workspaceDir, 'session.md');
  }

  async initialize(): Promise<void> {
    try {
      // Create workspace directory
      await fs.mkdir(this.workspaceDir, { recursive: true });
      
      // Create initial session file with role context
      const rolePrompt = this.rolePromptGenerator.generateRolePrompt(this.instance.role);
      await fs.writeFile(this.sessionFile, `# ${this.instance.role.toUpperCase()} Instance Session\n\n${rolePrompt}\n\n---\n\n`);
      
      this.isActive = true;
      this.logger.info(`Claude Code client initialized for instance ${this.instance.id}`);
    } catch (error) {
      this.logger.error(`Failed to initialize Claude Code client for ${this.instance.id}:`, error);
      throw error;
    }
  }

  async executeTask(task: Task): Promise<void> {
    if (!this.isActive) {
      throw new Error(`Instance ${this.instance.id} is not active`);
    }

    try {
      this.logger.info(`Executing task ${task.id} on instance ${this.instance.id}`);
      
      const prompt = this.buildTaskPrompt(task);
      const result = await this.spawnClaudeCodeSession(prompt);
      
      const taskResult: TaskResult = {
        success: true,
        output: result,
        filesModified: this.extractModifiedFiles(result),
        testsRun: this.extractTestResults(result)
      };

      this.emit('taskCompleted', task, taskResult);
      
    } catch (error) {
      this.logger.error(`Task ${task.id} failed on instance ${this.instance.id}:`, error);
      
      const taskResult: TaskResult = {
        success: false,
        error: error instanceof Error ? error.message : 'Unknown error'
      };

      this.emit('taskCompleted', task, taskResult);
    }
  }

  private buildTaskPrompt(task: Task): string {
    const roleContext = this.rolePromptGenerator.generateRolePrompt(this.instance.role);
    const taskContext = this.generateTaskContext(task);
    
    return `${roleContext}\n\n${taskContext}\n\nTask: ${task.description}`;
  }

  private generateTaskContext(task: Task): string {
    let context = `Task Type: ${task.type}\nPriority: ${task.priority}`;
    
    if (task.dependencies.length > 0) {
      context += `\nDependencies: ${task.dependencies.join(', ')}`;
    }

    switch (task.type) {
      case TaskType.CODE_REVIEW:
        context += '\n\nAs a code reviewer, focus on:\n- Code quality and best practices\n- Security vulnerabilities\n- Performance implications\n- Maintainability';
        break;
      
      case TaskType.TEST_CREATION:
        context += '\n\nAs a tester, focus on:\n- Comprehensive test coverage\n- Edge cases and error conditions\n- Integration test scenarios\n- Performance testing when relevant';
        break;
      
      case TaskType.FEATURE_IMPLEMENTATION:
        context += '\n\nAs a developer, focus on:\n- Clean, maintainable code\n- Following existing patterns\n- Proper error handling\n- Documentation';
        break;
      
      case TaskType.BUG_FIX:
        context += '\n\nAs a bug fixer, focus on:\n- Root cause analysis\n- Minimal, targeted fixes\n- Regression prevention\n- Thorough testing';
        break;
      
      case TaskType.RESEARCH:
        context += '\n\nAs a researcher, focus on:\n- Comprehensive analysis\n- Documentation of findings\n- Recommendations for implementation\n- Consider multiple approaches';
        break;
    }

    return context;
  }

  private async spawnClaudeCodeSession(prompt: string): Promise<string> {
    return new Promise((resolve, reject) => {
      // Append the task to the session file
      const taskEntry = `## Task: ${new Date().toISOString()}\n\n${prompt}\n\n---\n\n`;
      fs.appendFile(this.sessionFile, taskEntry);

      // For now, we'll simulate Claude Code execution
      // In a real implementation, you would:
      // 1. Spawn a new Claude Code process
      // 2. Send the prompt to it
      // 3. Parse the response
      // 4. Extract results and file changes

      const claudeCodePath = process.env.CLAUDE_CODE_PATH || 'claude';
      
      this.process = spawn(claudeCodePath, [
        '--non-interactive',
        '--workspace', this.workspaceDir,
        '--session', this.sessionFile
      ], {
        stdio: ['pipe', 'pipe', 'pipe'],
        env: { ...process.env }
      });

      let output = '';
      let error = '';

      this.process.stdout?.on('data', (data) => {
        output += data.toString();
      });

      this.process.stderr?.on('data', (data) => {
        error += data.toString();
      });

      this.process.on('close', (code) => {
        if (code === 0) {
          resolve(output);
        } else {
          reject(new Error(`Claude Code process exited with code ${code}: ${error}`));
        }
      });

      this.process.on('error', (err) => {
        reject(new Error(`Failed to spawn Claude Code process: ${err.message}`));
      });

      // Send the prompt to the process
      this.process.stdin?.write(prompt);
      this.process.stdin?.end();

      // Set a timeout for the process
      setTimeout(() => {
        if (this.process && !this.process.killed) {
          this.process.kill();
          reject(new Error('Claude Code process timed out'));
        }
      }, 300000); // 5 minutes timeout
    });
  }

  private extractModifiedFiles(output: string): string[] {
    // Simple regex to extract file paths from Claude's output
    const fileMatches = output.match(/(?:modified|created|updated):\s*([^\s\n]+)/gi);
    return fileMatches ? fileMatches.map(match => match.split(':')[1].trim()) : [];
  }

  private extractTestResults(output: string): any[] {
    // Simple extraction of test results from output
    const testMatches = output.match(/test\s+(\w+):\s*(passed|failed)/gi);
    return testMatches ? testMatches.map(match => {
      const [, name, status] = match.match(/test\s+(\w+):\s*(passed|failed)/i) || [];
      return { name, passed: status === 'passed' };
    }) : [];
  }

  async healthCheck(): Promise<boolean> {
    try {
      const healthResult = await this.spawnClaudeCodeSession('Health check - please respond with "OK"');
      return healthResult.toLowerCase().includes('ok');
    } catch (error) {
      this.logger.error(`Health check failed for instance ${this.instance.id}:`, error);
      return false;
    }
  }

  async terminate(): Promise<void> {
    this.isActive = false;
    
    if (this.process && !this.process.killed) {
      this.process.kill();
    }
    
    this.removeAllListeners();
    this.logger.info(`Claude Code client terminated for instance ${this.instance.id}`);
  }
}