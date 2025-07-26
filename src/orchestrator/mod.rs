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

        // TODO: Actually execute the task via Claude Code process
        // For now, we'll simulate task execution
        self.simulate_task_execution(task, instance).await;

        Ok(())
    }

    // Temporary simulation - replace with actual Claude Code execution
    async fn simulate_task_execution(&self, mut task: Task, mut instance: ClaudeInstance) {
        let orchestrator = self.clone();
        tokio::spawn(async move {
            // Simulate work delay
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            
            // Simulate completion
            let result = TaskResult {
                success: true,
                output: Some(format!("Simulated completion of task: {}", task.description)),
                error: None,
                files_modified: vec!["src/example.rs".to_string()],
                tests_run: vec![TestResult {
                    name: "example_test".to_string(),
                    passed: true,
                    error: None,
                }],
            };

            task.status = TaskStatus::Completed;
            task.result = Some(result.clone());
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
                task,
                instance,
                result,
            }) {
                warn!("Failed to send task completed event: {}", e);
            }
        });
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