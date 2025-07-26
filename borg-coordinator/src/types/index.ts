export interface ClaudeInstance {
  id: string;
  role: InstanceRole;
  status: InstanceStatus;
  currentTask?: Task;
  capabilities: string[];
  createdAt: Date;
  lastActivity: Date;
}

export interface Task {
  id: string;
  type: TaskType;
  description: string;
  assignedTo?: string;
  status: TaskStatus;
  priority: TaskPriority;
  dependencies: string[];
  result?: TaskResult;
  createdAt: Date;
  updatedAt: Date;
}

export interface TaskResult {
  success: boolean;
  output?: string;
  error?: string;
  filesModified?: string[];
  testsRun?: TestResult[];
}

export interface TestResult {
  name: string;
  passed: boolean;
  error?: string;
}

export enum InstanceRole {
  SUPERVISOR = 'supervisor',
  DEVELOPER = 'developer',
  TESTER = 'tester',
  REVIEWER = 'reviewer',
  RESEARCHER = 'researcher'
}

export enum InstanceStatus {
  IDLE = 'idle',
  WORKING = 'working',
  ERROR = 'error',
  OFFLINE = 'offline'
}

export enum TaskType {
  CODE_REVIEW = 'code_review',
  FEATURE_IMPLEMENTATION = 'feature_implementation',
  BUG_FIX = 'bug_fix',
  TEST_CREATION = 'test_creation',
  RESEARCH = 'research',
  DOCUMENTATION = 'documentation'
}

export enum TaskStatus {
  PENDING = 'pending',
  IN_PROGRESS = 'in_progress',
  COMPLETED = 'completed',
  FAILED = 'failed',
  CANCELLED = 'cancelled'
}

export enum TaskPriority {
  LOW = 'low',
  MEDIUM = 'medium',
  HIGH = 'high',
  CRITICAL = 'critical'
}

export interface Message {
  id: string;
  from: string;
  to: string;
  type: MessageType;
  content: any;
  timestamp: Date;
}

export enum MessageType {
  TASK_ASSIGNMENT = 'task_assignment',
  TASK_COMPLETION = 'task_completion',
  STATUS_UPDATE = 'status_update',
  COLLABORATION_REQUEST = 'collaboration_request',
  ERROR_REPORT = 'error_report'
}