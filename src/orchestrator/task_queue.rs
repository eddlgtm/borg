use crate::types::*;
use redis::{aio::ConnectionManager, AsyncCommands, Client};
use std::collections::HashMap;
use tracing::{debug, info};

#[derive(Clone)]
pub struct TaskQueue {
    connection: ConnectionManager,
}

impl TaskQueue {
    pub async fn new(redis_url: &str) -> BorgResult<Self> {
        let client = Client::open(redis_url)?;
        let connection = ConnectionManager::new(client).await?;
        
        Ok(Self { connection })
    }

    pub async fn initialize(&self) -> BorgResult<()> {
        // Test connection with a simple GET operation
        let mut conn = self.connection.clone();
        let _: Option<String> = conn.get("borg:test").await?;
        info!("Task queue connected to Redis");
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn add_task(&self, task: &Task) -> BorgResult<()> {
        let queue_key = self.get_priority_queue_key(task.priority);
        let task_data = serde_json::to_string(task)?;
        
        let mut conn = self.connection.clone();
        let _: () = conn.lpush(&queue_key, task_data).await?;
        
        debug!("Task {} added to {} priority queue", task.id, task.priority.as_str());
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn get_next_task(&self) -> BorgResult<Option<Task>> {
        let queue_order = [
            TaskPriority::Critical,
            TaskPriority::High,
            TaskPriority::Medium,
            TaskPriority::Low,
        ];

        let mut conn = self.connection.clone();
        
        for priority in queue_order {
            let queue_key = self.get_priority_queue_key(priority);
            let task_data: Option<String> = conn.rpop(&queue_key, None).await?;
            
            if let Some(data) = task_data {
                let task: Task = serde_json::from_str(&data)?;
                debug!("Retrieved task {} from {} priority queue", task.id, priority.as_str());
                return Ok(Some(task));
            }
        }

        Ok(None)
    }

    #[allow(dead_code)]
    pub async fn get_queue_stats(&self) -> BorgResult<HashMap<TaskPriority, usize>> {
        let mut stats = HashMap::new();
        let mut conn = self.connection.clone();
        
        let priorities = [
            TaskPriority::Critical,
            TaskPriority::High,
            TaskPriority::Medium,
            TaskPriority::Low,
        ];

        for priority in priorities {
            let queue_key = self.get_priority_queue_key(priority);
            let length: usize = conn.llen(&queue_key).await?;
            stats.insert(priority, length);
        }

        Ok(stats)
    }

    #[allow(dead_code)]
    pub async fn clear_queue(&self, priority: Option<TaskPriority>) -> BorgResult<()> {
        let mut conn = self.connection.clone();
        
        if let Some(priority) = priority {
            let queue_key = self.get_priority_queue_key(priority);
            let _: () = conn.del(&queue_key).await?;
            info!("Cleared {} priority queue", priority.as_str());
        } else {
            let priorities = [
                TaskPriority::Critical,
                TaskPriority::High,
                TaskPriority::Medium,
                TaskPriority::Low,
            ];
            
            for priority in priorities {
                let queue_key = self.get_priority_queue_key(priority);
                let _: () = conn.del(&queue_key).await?;
            }
            info!("Cleared all task queues");
        }

        Ok(())
    }

    // Store instance data in Redis
    #[allow(dead_code)]
    pub async fn store_instance(&self, instance: &ClaudeInstance) -> BorgResult<()> {
        let mut conn = self.connection.clone();
        let instance_data = serde_json::to_string(instance)?;
        let key = format!("borg:instances:{}", instance.id);
        let _: () = conn.set(&key, instance_data).await?;
        
        // Also add to instances set
        let _: () = conn.sadd("borg:instances", instance.id.to_string()).await?;
        Ok(())
    }

    // Store task data in Redis
    #[allow(dead_code)]
    pub async fn store_task(&self, task: &Task) -> BorgResult<()> {
        let mut conn = self.connection.clone();
        let task_data = serde_json::to_string(task)?;
        let key = format!("borg:tasks:{}", task.id);
        let _: () = conn.set(&key, task_data).await?;
        
        // Also add to tasks set
        let _: () = conn.sadd("borg:tasks", task.id.to_string()).await?;
        Ok(())
    }

    // Get all instances from Redis
    #[allow(dead_code)]
    pub async fn get_all_instances(&self) -> BorgResult<Vec<ClaudeInstance>> {
        let mut conn = self.connection.clone();
        let instance_ids: Vec<String> = conn.smembers("borg:instances").await?;
        
        let mut instances = Vec::new();
        for id in instance_ids {
            let key = format!("borg:instances:{}", id);
            let instance_data: Option<String> = conn.get(&key).await?;
            if let Some(data) = instance_data {
                if let Ok(instance) = serde_json::from_str::<ClaudeInstance>(&data) {
                    instances.push(instance);
                }
            }
        }
        
        Ok(instances)
    }

    // Get all tasks from Redis
    #[allow(dead_code)]
    pub async fn get_all_tasks(&self) -> BorgResult<Vec<Task>> {
        let mut conn = self.connection.clone();
        let task_ids: Vec<String> = conn.smembers("borg:tasks").await?;
        
        let mut tasks = Vec::new();
        for id in task_ids {
            let key = format!("borg:tasks:{}", id);
            let task_data: Option<String> = conn.get(&key).await?;
            if let Some(data) = task_data {
                if let Ok(task) = serde_json::from_str::<Task>(&data) {
                    tasks.push(task);
                }
            }
        }
        
        Ok(tasks)
    }

    #[allow(dead_code)]
    fn get_priority_queue_key(&self, priority: TaskPriority) -> String {
        format!("borg:tasks:{}", priority.as_str())
    }
}