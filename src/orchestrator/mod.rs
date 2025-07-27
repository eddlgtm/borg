use crate::types::*;
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tracing::{info, warn};
use uuid::Uuid;

pub mod task_queue;
pub use task_queue::TaskQueue;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum OrchestratorEvent {
    InstanceCreated(ClaudeInstance),
    InstanceTerminated(ClaudeInstance),
    TaskAssigned(Task),
    TaskCompleted { task: Task, instance: ClaudeInstance, result: TaskResult },
    InstanceError { instance: ClaudeInstance, error: String },
}

pub struct Orchestrator {
    instances: Arc<RwLock<HashMap<Uuid, ClaudeInstance>>>,
    tasks: Arc<RwLock<HashMap<Uuid, Task>>>,
    task_queue: TaskQueue,
    config: OrchestratorConfig,
    event_sender: mpsc::UnboundedSender<OrchestratorEvent>,
}

impl Orchestrator {
    pub async fn new(config: OrchestratorConfig) -> BorgResult<(Self, mpsc::UnboundedReceiver<OrchestratorEvent>)> {
        let task_queue = TaskQueue::new(&config.redis_url).await?;
        let (event_sender, event_receiver) = mpsc::unbounded_channel();
        
        let orchestrator = Self {
            instances: Arc::new(RwLock::new(HashMap::new())),
            tasks: Arc::new(RwLock::new(HashMap::new())),
            task_queue,
            config,
            event_sender,
        };
        
        Ok((orchestrator, event_receiver))
    }

    pub async fn initialize(&self) -> BorgResult<()> {
        info!("Initializing Borg Coordinator orchestrator...");
        self.task_queue.initialize().await?;
        
        // Create a complete development team by default
        info!("Creating default development team...");
        
        let mut created_instances = Vec::new();
        
        // Project Manager
        let project_manager = ClaudeInstance {
            id: Uuid::new_v4(),
            role: InstanceRole::ProjectManager,
            status: InstanceStatus::Idle,
            current_task: None,
            capabilities: InstanceRole::ProjectManager.capabilities(),
            config: InstanceConfig::for_role(InstanceRole::ProjectManager),
            created_at: Utc::now(),
            last_activity: Utc::now(),
        };
        created_instances.push(project_manager);
        
        // Supervisor
        let supervisor = ClaudeInstance {
            id: Uuid::new_v4(),
            role: InstanceRole::Supervisor,
            status: InstanceStatus::Idle,
            current_task: None,
            capabilities: InstanceRole::Supervisor.capabilities(),
            config: InstanceConfig::for_role(InstanceRole::Supervisor),
            created_at: Utc::now(),
            last_activity: Utc::now(),
        };
        created_instances.push(supervisor);
        
        // Developer 1 - Frontend focused
        let mut dev1_config = InstanceConfig::for_role(InstanceRole::Developer);
        dev1_config.name = "Frontend Developer".to_string();
        dev1_config.preferred_languages = vec!["typescript".to_string(), "javascript".to_string(), "react".to_string()];
        dev1_config.custom_prompts.insert("role_prompt".to_string(), 
            "You are a frontend developer working on the current project directory. Focus on UI/UX, React components, and TypeScript. Auto-accept and implement all changes and suggestions.".to_string());
        dev1_config.environment_vars.insert("CLAUDE_WORKSPACE".to_string(), ".".to_string());
        
        let dev1 = ClaudeInstance {
            id: Uuid::new_v4(),
            role: InstanceRole::Developer,
            status: InstanceStatus::Idle,
            current_task: None,
            capabilities: InstanceRole::Developer.capabilities(),
            config: dev1_config,
            created_at: Utc::now(),
            last_activity: Utc::now(),
        };
        created_instances.push(dev1);
        
        // Developer 2 - Backend focused
        let mut dev2_config = InstanceConfig::for_role(InstanceRole::Developer);
        dev2_config.name = "Backend Developer".to_string();
        dev2_config.preferred_languages = vec!["rust".to_string(), "python".to_string(), "sql".to_string()];
        dev2_config.custom_prompts.insert("role_prompt".to_string(), 
            "You are a backend developer working on the current project directory. Focus on APIs, databases, and Rust/Python services. Auto-accept and implement all changes and suggestions.".to_string());
        dev2_config.environment_vars.insert("CLAUDE_WORKSPACE".to_string(), ".".to_string());
        
        let dev2 = ClaudeInstance {
            id: Uuid::new_v4(),
            role: InstanceRole::Developer,
            status: InstanceStatus::Idle,
            current_task: None,
            capabilities: InstanceRole::Developer.capabilities(),
            config: dev2_config,
            created_at: Utc::now(),
            last_activity: Utc::now(),
        };
        created_instances.push(dev2);
        
        // Developer 3 - Full-stack focused
        let mut dev3_config = InstanceConfig::for_role(InstanceRole::Developer);
        dev3_config.name = "Full-Stack Developer".to_string();
        dev3_config.preferred_languages = vec!["typescript".to_string(), "rust".to_string(), "node.js".to_string()];
        dev3_config.custom_prompts.insert("role_prompt".to_string(), 
            "You are a full-stack developer working on the current project directory. Handle both frontend and backend tasks with equal expertise. Auto-accept and implement all changes and suggestions.".to_string());
        dev3_config.environment_vars.insert("CLAUDE_WORKSPACE".to_string(), ".".to_string());
        
        let dev3 = ClaudeInstance {
            id: Uuid::new_v4(),
            role: InstanceRole::Developer,
            status: InstanceStatus::Idle,
            current_task: None,
            capabilities: InstanceRole::Developer.capabilities(),
            config: dev3_config,
            created_at: Utc::now(),
            last_activity: Utc::now(),
        };
        created_instances.push(dev3);
        
        // Tester
        let tester = ClaudeInstance {
            id: Uuid::new_v4(),
            role: InstanceRole::Tester,
            status: InstanceStatus::Idle,
            current_task: None,
            capabilities: InstanceRole::Tester.capabilities(),
            config: InstanceConfig::for_role(InstanceRole::Tester),
            created_at: Utc::now(),
            last_activity: Utc::now(),
        };
        created_instances.push(tester);
        
        // Reviewer
        let reviewer = ClaudeInstance {
            id: Uuid::new_v4(),
            role: InstanceRole::Reviewer,
            status: InstanceStatus::Idle,
            current_task: None,
            capabilities: InstanceRole::Reviewer.capabilities(),
            config: InstanceConfig::for_role(InstanceRole::Reviewer),
            created_at: Utc::now(),
            last_activity: Utc::now(),
        };
        created_instances.push(reviewer);
        
        // Researcher
        let researcher = ClaudeInstance {
            id: Uuid::new_v4(),
            role: InstanceRole::Researcher,
            status: InstanceStatus::Idle,
            current_task: None,
            capabilities: InstanceRole::Researcher.capabilities(),
            config: InstanceConfig::for_role(InstanceRole::Researcher),
            created_at: Utc::now(),
            last_activity: Utc::now(),
        };
        created_instances.push(researcher);
        
        // Store all instances
        {
            let mut instances = self.instances.write().await;
            for instance in &created_instances {
                instances.insert(instance.id, instance.clone());
                // Also store in Redis
                if let Err(e) = self.task_queue.store_instance(instance).await {
                    warn!("Failed to store instance {} in Redis: {}", instance.id, e);
                }
            }
        }
        
        // Send events for each created instance
        for instance in created_instances {
            if let Err(e) = self.event_sender.send(OrchestratorEvent::InstanceCreated(instance)) {
                warn!("Failed to send instance created event: {}", e);
            }
        }
        
        info!("Created complete team: Project Manager, Team Supervisor, Frontend Developer, Backend Developer, Full-Stack Developer, QA Tester, Code Reviewer, Researcher");
        info!("Orchestrator initialized successfully");
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn create_instance(&self, role: InstanceRole, capabilities: Option<Vec<String>>) -> BorgResult<ClaudeInstance> {
        let instance = ClaudeInstance {
            id: Uuid::new_v4(),
            role,
            status: InstanceStatus::Idle,
            current_task: None,
            capabilities: capabilities.unwrap_or_else(|| role.capabilities()),
            config: InstanceConfig::for_role(role),
            created_at: Utc::now(),
            last_activity: Utc::now(),
        };

        // Store instance
        {
            let mut instances = self.instances.write().await;
            instances.insert(instance.id, instance.clone());
            // Also store in Redis
            if let Err(e) = self.task_queue.store_instance(&instance).await {
                warn!("Failed to store instance {} in Redis: {}", instance.id, e);
            }
        }

        info!("Created {} instance: {}", role.as_str(), instance.id);
        
        // Send event
        if let Err(e) = self.event_sender.send(OrchestratorEvent::InstanceCreated(instance.clone())) {
            warn!("Failed to send instance created event: {}", e);
        }

        Ok(instance)
    }

    #[allow(dead_code)]
    pub async fn assign_task(&self, task_params: TaskParams) -> BorgResult<Task> {
        let task = Task {
            id: Uuid::new_v4(),
            task_type: task_params.task_type,
            description: task_params.description,
            assigned_to: task_params.target_instance_id,
            status: TaskStatus::Pending,
            priority: task_params.priority,
            dependencies: task_params.dependencies.unwrap_or_default(),
            result: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // Store task
        {
            let mut tasks = self.tasks.write().await;
            tasks.insert(task.id, task.clone());
            // Also store in Redis
            if let Err(e) = self.task_queue.store_task(&task).await {
                warn!("Failed to store task {} in Redis: {}", task.id, e);
            }
        }

        // Add to queue or assign directly
        if let Some(instance_id) = task_params.target_instance_id {
            self.assign_task_to_instance(task.clone(), instance_id).await?;
        } else {
            self.task_queue.add_task(&task).await?;
            self.find_best_instance_for_task(&task).await?;
        }

        info!("Task assigned: {} - {}", task.id, task.description);
        
        // Send event
        if let Err(e) = self.event_sender.send(OrchestratorEvent::TaskAssigned(task.clone())) {
            warn!("Failed to send task assigned event: {}", e);
        }

        Ok(task)
    }

    async fn find_best_instance_for_task(&self, task: &Task) -> BorgResult<()> {
        let instances = self.instances.read().await;
        let available_instances: Vec<_> = instances
            .values()
            .filter(|instance| instance.status == InstanceStatus::Idle)
            .collect();

        if available_instances.is_empty() {
            warn!("No available instances for task {}", task.id);
            return Ok(());
        }

        // Find best instance based on role preferences
        let role_preferences = task.task_type.preferred_roles();
        let best_instance = available_instances
            .into_iter()
            .find(|instance| role_preferences.contains(&instance.role))
            .or_else(|| instances.values().find(|i| i.status == InstanceStatus::Idle))
            .cloned();

        if let Some(instance) = best_instance {
            drop(instances); // Release read lock before calling assign_task_to_instance
            self.assign_task_to_instance(task.clone(), instance.id).await?;
        }

        Ok(())
    }

    async fn assign_task_to_instance(&self, mut task: Task, instance_id: Uuid) -> BorgResult<()> {
        // Update task
        task.assigned_to = Some(instance_id);
        task.status = TaskStatus::InProgress;
        task.updated_at = Utc::now();

        // Update instance
        let instance = {
            let mut instances = self.instances.write().await;
            let instance = instances.get_mut(&instance_id)
                .ok_or(BorgError::InstanceNotFound { id: instance_id })?;
            
            instance.current_task = Some(task.clone());
            instance.status = InstanceStatus::Working;
            instance.last_activity = Utc::now();
            
            // Also store updated instance in Redis
            if let Err(e) = self.task_queue.store_instance(instance).await {
                warn!("Failed to store updated instance {} in Redis: {}", instance.id, e);
            }
            
            instance.clone()
        };

        // Store updated task
        {
            let mut tasks = self.tasks.write().await;
            tasks.insert(task.id, task.clone());
            // Also store in Redis
            if let Err(e) = self.task_queue.store_task(&task).await {
                warn!("Failed to store task {} in Redis: {}", task.id, e);
            }
        }

        info!("Task {} assigned to instance {}", task.id, instance_id);

        // Execute the task via Claude Code process
        self.execute_task_with_claude(task, instance).await;

        Ok(())
    }

    // Execute task with actual Claude Code process
    async fn execute_task_with_claude(&self, mut task: Task, mut instance: ClaudeInstance) {
        let orchestrator = self.clone();
        let claude_path = self.config.claude_code_path.clone();
        
        tokio::spawn(async move {
            info!("Starting Claude Code execution for task {} on instance {}", task.id, instance.id);
            
            // Prepare the prompt based on the instance role and task
            let role_context = match instance.role {
                InstanceRole::ProjectManager => "You are a Project Manager responsible for planning, coordinating, and breaking down high-level requirements into specific tasks. Analyze the user request and create a detailed project plan.",
                InstanceRole::Supervisor => "You are a Team Supervisor responsible for architecture decisions, code review coordination, and ensuring best practices across the development team.",
                InstanceRole::Developer => &format!("You are a {} specializing in {}. Focus on implementing features, fixing bugs, and writing clean, maintainable code.", 
                    instance.config.name, 
                    instance.config.preferred_languages.join(", ")),
                InstanceRole::Tester => "You are a QA Tester responsible for creating comprehensive tests, finding bugs, and ensuring code quality and reliability.",
                InstanceRole::Reviewer => "You are a Code Reviewer focused on security, performance, best practices, and maintaining code quality standards.",
                InstanceRole::Researcher => "You are a Researcher responsible for investigating technologies, analyzing requirements, and providing technical recommendations.",
            };
            
            let full_prompt = format!(
                "{}\n\nTask: {}\n\nPlease complete this task and provide a detailed summary of what you accomplished.",
                role_context,
                task.description
            );
            
            // Execute Claude Code process
            let result = orchestrator.run_claude_process(&claude_path, &full_prompt, &instance).await;
            
            // Update task with result
            match result {
                Ok(task_result) => {
                    task.status = TaskStatus::Completed;
                    task.result = Some(task_result.clone());
                    info!("Task {} completed successfully by instance {}", task.id, instance.id);
                }
                Err(e) => {
                    task.status = TaskStatus::Failed;
                    task.result = Some(TaskResult {
                        success: false,
                        output: None,
                        error: Some(format!("Claude Code execution failed: {}", e)),
                        files_modified: vec![],
                        tests_run: vec![],
                    });
                    warn!("Task {} failed on instance {}: {}", task.id, instance.id, e);
                }
            }
            
            task.updated_at = Utc::now();
            instance.status = InstanceStatus::Idle;
            instance.current_task = None;
            instance.last_activity = Utc::now();

            // Update stored data
            {
                let mut tasks = orchestrator.tasks.write().await;
                tasks.insert(task.id, task.clone());
                // Also store in Redis
                if let Err(e) = orchestrator.task_queue.store_task(&task).await {
                    warn!("Failed to store task {} in Redis: {}", task.id, e);
                }
            }
            {
                let mut instances = orchestrator.instances.write().await;
                instances.insert(instance.id, instance.clone());
                // Also store in Redis
                if let Err(e) = orchestrator.task_queue.store_instance(&instance).await {
                    warn!("Failed to store instance {} in Redis: {}", instance.id, e);
                }
            }

            // Send event
            if let Err(e) = orchestrator.event_sender.send(OrchestratorEvent::TaskCompleted {
                task: task.clone(),
                instance: instance.clone(),
                result: task.result.unwrap(),
            }) {
                warn!("Failed to send task completed event: {}", e);
            }
        });
    }
    
    // Run Claude Code process and capture output
    async fn run_claude_process(&self, claude_path: &str, prompt: &str, instance: &ClaudeInstance) -> BorgResult<TaskResult> {
        use std::process::Stdio;
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        use tokio::process::Command;
        
        info!("Executing Claude Code for instance {} ({})", instance.config.name, instance.role.as_str());
        
        // Build command with role-specific configuration
        let mut cmd = Command::new(claude_path);
        
        // Add environment variables from instance config
        for (key, value) in &instance.config.environment_vars {
            cmd.env(key, value);
        }
        
        // Set working directory to current directory (where the tool is run)
        cmd.current_dir(".");
        
        // Configure stdio
        cmd.stdin(Stdio::piped())
           .stdout(Stdio::piped())
           .stderr(Stdio::piped());
        
        // Add any command line arguments based on instance config
        if instance.config.auto_accept_tasks {
            // Note: Claude Code doesn't have an auto-accept flag yet, but we can configure the prompt
        }
        
        info!("Starting Claude Code process with prompt: {}...", &prompt[..100.min(prompt.len())]);
        
        let mut child = cmd.spawn()
            .map_err(|e| BorgError::TaskExecutionError { 
                message: format!("Failed to spawn Claude Code process: {}", e) 
            })?;
        
        // Send prompt to stdin
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(prompt.as_bytes()).await
                .map_err(|e| BorgError::TaskExecutionError { 
                    message: format!("Failed to write prompt to Claude Code: {}", e) 
                })?;
            stdin.shutdown().await
                .map_err(|e| BorgError::TaskExecutionError { 
                    message: format!("Failed to close stdin: {}", e) 
                })?;
        }
        
        // Capture output with timeout
        let output = tokio::time::timeout(
            tokio::time::Duration::from_secs(instance.config.timeout_seconds),
            async {
                let mut stdout_data = Vec::new();
                let mut stderr_data = Vec::new();
                
                if let Some(mut stdout) = child.stdout.take() {
                    stdout.read_to_end(&mut stdout_data).await?;
                }
                if let Some(mut stderr) = child.stderr.take() {
                    stderr.read_to_end(&mut stderr_data).await?;
                }
                
                let status = child.wait().await?;
                
                Ok::<_, std::io::Error>((status, stdout_data, stderr_data))
            }
        ).await
        .map_err(|_| BorgError::TaskExecutionError { 
            message: format!("Claude Code process timed out after {} seconds", instance.config.timeout_seconds) 
        })?
        .map_err(|e| BorgError::TaskExecutionError { 
            message: format!("Error reading Claude Code output: {}", e) 
        })?;
        
        let (status, stdout_data, stderr_data) = output;
        let stdout = String::from_utf8_lossy(&stdout_data);
        let stderr = String::from_utf8_lossy(&stderr_data);
        
        let success = status.success();
        
        info!("Claude Code process completed with status: {} for instance {}", 
              if success { "success" } else { "failure" }, instance.config.name);
        
        if !success {
            warn!("Claude Code stderr: {}", stderr);
        }
        
        // TODO: Parse output to extract file modifications and test results
        // For now, we'll return the raw output
        Ok(TaskResult {
            success,
            output: if !stdout.is_empty() { Some(stdout.to_string()) } else { None },
            error: if !stderr.is_empty() { Some(stderr.to_string()) } else { None },
            files_modified: vec![], // TODO: Parse from Claude Code output
            tests_run: vec![], // TODO: Parse from Claude Code output
        })
    }

    #[allow(dead_code)]
    pub async fn get_instance_status(&self, instance_id: Uuid) -> BorgResult<ClaudeInstance> {
        let instances = self.instances.read().await;
        instances.get(&instance_id)
            .cloned()
            .ok_or(BorgError::InstanceNotFound { id: instance_id })
    }

    #[allow(dead_code)]
    pub async fn get_all_instances(&self) -> Vec<ClaudeInstance> {
        let instances = self.instances.read().await;
        instances.values().cloned().collect()
    }

    #[allow(dead_code)]
    pub async fn get_all_tasks(&self) -> Vec<Task> {
        let tasks = self.tasks.read().await;
        tasks.values().cloned().collect()
    }

    #[allow(dead_code)]
    pub async fn terminate_instance(&self, instance_id: Uuid) -> BorgResult<()> {
        let instance = {
            let mut instances = self.instances.write().await;
            instances.remove(&instance_id)
                .ok_or(BorgError::InstanceNotFound { id: instance_id })?
        };

        info!("Instance {} terminated", instance_id);
        
        // Send event
        if let Err(e) = self.event_sender.send(OrchestratorEvent::InstanceTerminated(instance)) {
            warn!("Failed to send instance terminated event: {}", e);
        }

        Ok(())
    }

    #[allow(dead_code)]
    pub async fn update_instance_config(&self, instance_id: Uuid, config: InstanceConfig) -> BorgResult<ClaudeInstance> {
        let mut instances = self.instances.write().await;
        let instance = instances.get_mut(&instance_id)
            .ok_or(BorgError::InstanceNotFound { id: instance_id })?;
        
        instance.config = config;
        instance.last_activity = Utc::now();
        
        info!("Updated configuration for instance {}", instance_id);
        Ok(instance.clone())
    }

    #[allow(dead_code)]
    pub async fn get_instance_config(&self, instance_id: Uuid) -> BorgResult<InstanceConfig> {
        let instances = self.instances.read().await;
        let instance = instances.get(&instance_id)
            .ok_or(BorgError::InstanceNotFound { id: instance_id })?;
        
        Ok(instance.config.clone())
    }

    // Process tasks from Redis queue
    pub async fn start_task_processing(&self) {
        info!("Starting task processing loop...");
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(5));
        
        loop {
            interval.tick().await;
            
            // Get next task from queue
            match self.task_queue.get_next_task().await {
                Ok(Some(task)) => {
                    info!("Processing task from queue: {} - {}", task.id, task.description);
                    
                    // Find best instance for the task
                    if let Err(e) = self.process_queued_task(task).await {
                        warn!("Failed to process queued task: {}", e);
                    }
                }
                Ok(None) => {
                    // No tasks in queue, continue
                }
                Err(e) => {
                    warn!("Error getting task from queue: {}", e);
                    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                }
            }
        }
    }

    async fn process_queued_task(&self, task: Task) -> BorgResult<()> {
        // Update task in local memory
        {
            let mut tasks = self.tasks.write().await;
            tasks.insert(task.id, task.clone());
        }

        // Find best instance for the task
        if task.assigned_to.is_some() {
            // Task is already assigned to specific instance
            if let Some(instance_id) = task.assigned_to {
                self.assign_task_to_instance(task, instance_id).await?;
            }
        } else {
            // Find best available instance
            self.find_best_instance_for_task(&task).await?;
        }

        Ok(())
    }
}

impl Clone for Orchestrator {
    fn clone(&self) -> Self {
        Self {
            instances: self.instances.clone(),
            tasks: self.tasks.clone(),
            task_queue: self.task_queue.clone(),
            config: self.config.clone(),
            event_sender: self.event_sender.clone(),
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct TaskParams {
    pub task_type: TaskType,
    pub description: String,
    pub priority: TaskPriority,
    pub dependencies: Option<Vec<Uuid>>,
    pub target_instance_id: Option<Uuid>,
}