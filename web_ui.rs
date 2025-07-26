use borg_coordinator::{orchestrator::TaskQueue, types::*};
// Removed redundant import
use std::sync::Arc;

// Simple HTTP server for task creation
#[tokio::main]
async fn main() -> BorgResult<()> {
    println!("🌐 Starting Borg Coordinator Web UI...");
    
    // Connect to Redis task queue
    let task_queue = TaskQueue::new("redis://localhost:6379").await?;
    let task_queue = Arc::new(task_queue);
    
    // Try to bind to different ports
    let mut port = 8080;
    let listener = loop {
        match std::net::TcpListener::bind(format!("127.0.0.1:{}", port)) {
            Ok(listener) => {
                println!("🚀 Web UI running at: http://localhost:{}", port);
                println!("💡 Open this URL in your browser to create tasks!");
                break listener;
            }
            Err(_) => {
                if port >= 8090 {
                    eprintln!("❌ Could not bind to any port between 8080-8090");
                    eprintln!("💡 Try stopping other services or use a different port");
                    eprintln!("   Check what's using port 8080: lsof -i :8080");
                    std::process::exit(1);
                }
                println!("⚠️  Port {} is in use, trying port {}...", port, port + 1);
                port += 1;
            }
        }
    };
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let queue = task_queue.clone();
                tokio::spawn(async move {
                    handle_connection(&mut stream, queue).await;
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
    
    Ok(())
}

async fn handle_connection(stream: &mut std::net::TcpStream, task_queue: Arc<TaskQueue>) {
    use std::io::{Read, Write};
    
    let mut buffer = [0; 1024];
    if stream.read(&mut buffer).is_err() {
        return;
    }
    
    let request = String::from_utf8_lossy(&buffer[..]);
    let request_line = request.lines().next().unwrap_or("");
    
    if request_line.starts_with("GET / ") {
        // Serve main page
        let response = create_main_page().await;
        let _ = stream.write_all(response.as_bytes());
        let _ = stream.flush();
    } else if request_line.starts_with("POST /create_task") {
        // Handle task creation
        let response = handle_task_creation(&request, task_queue.clone()).await;
        let _ = stream.write_all(response.as_bytes());
        let _ = stream.flush();
    } else if request_line.starts_with("GET /status") {
        // Handle status request
        let response = handle_status_request(task_queue.clone()).await;
        let _ = stream.write_all(response.as_bytes());
        let _ = stream.flush();
    } else {
        // 404
        let response = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        let _ = stream.write_all(response.as_bytes());
        let _ = stream.flush();
    }
}

async fn create_main_page() -> String {
    let html = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Borg Coordinator</title>
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }
        
        :root {
            --bg-primary: #0f0f23;
            --bg-secondary: #1a1a3a;
            --bg-card: #252547;
            --accent: #00c896;
            --accent-hover: #00a47b;
            --text-primary: #e5e5e5;
            --text-secondary: #a0a0a0;
            --border: #404040;
            --success: #00c896;
            --warning: #ff9500;
            --danger: #ff375f;
        }
        
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
            background: linear-gradient(135deg, var(--bg-primary) 0%, var(--bg-secondary) 100%);
            color: var(--text-primary);
            min-height: 100vh;
            line-height: 1.6;
        }
        
        .container {
            max-width: 1200px;
            margin: 0 auto;
            padding: 2rem;
        }
        
        .header {
            text-align: center;
            margin-bottom: 3rem;
        }
        
        .logo {
            font-size: 3rem;
            margin-bottom: 0.5rem;
        }
        
        .title {
            font-size: 2.5rem;
            font-weight: 300;
            color: var(--text-primary);
            margin-bottom: 0.5rem;
        }
        
        .subtitle {
            color: var(--text-secondary);
            font-size: 1.1rem;
        }
        
        .grid {
            display: grid;
            grid-template-columns: 1fr 1fr;
            gap: 2rem;
            margin-bottom: 2rem;
        }
        
        @media (max-width: 768px) {
            .grid {
                grid-template-columns: 1fr;
            }
        }
        
        .card {
            background: var(--bg-card);
            border-radius: 16px;
            padding: 2rem;
            border: 1px solid var(--border);
            backdrop-filter: blur(10px);
        }
        
        .card h2 {
            font-size: 1.5rem;
            font-weight: 500;
            margin-bottom: 1.5rem;
            color: var(--text-primary);
        }
        
        .status-grid {
            display: grid;
            gap: 1rem;
            margin-bottom: 1rem;
        }
        
        .status-item {
            display: flex;
            align-items: center;
            gap: 0.75rem;
            padding: 0.75rem;
            background: rgba(255, 255, 255, 0.05);
            border-radius: 8px;
            font-size: 0.9rem;
        }
        
        .status-icon {
            font-size: 1.2rem;
        }
        
        .refresh-btn {
            background: transparent;
            border: 1px solid var(--border);
            color: var(--text-secondary);
            padding: 0.5rem 1rem;
            border-radius: 8px;
            cursor: pointer;
            font-size: 0.9rem;
            transition: all 0.2s ease;
        }
        
        .refresh-btn:hover {
            border-color: var(--accent);
            color: var(--accent);
        }
        
        .form-group {
            margin-bottom: 1.5rem;
        }
        
        .form-label {
            display: block;
            font-size: 0.9rem;
            font-weight: 500;
            color: var(--text-secondary);
            margin-bottom: 0.5rem;
        }
        
        .form-input, .form-select, .form-textarea {
            width: 100%;
            padding: 0.75rem 1rem;
            background: rgba(255, 255, 255, 0.05);
            border: 1px solid var(--border);
            border-radius: 8px;
            color: var(--text-primary);
            font-size: 1rem;
            transition: all 0.2s ease;
        }
        
        .form-input:focus, .form-select:focus, .form-textarea:focus {
            outline: none;
            border-color: var(--accent);
            box-shadow: 0 0 0 3px rgba(0, 200, 150, 0.1);
        }
        
        .form-textarea {
            resize: vertical;
            min-height: 100px;
        }
        
        .btn-primary {
            width: 100%;
            padding: 1rem;
            background: linear-gradient(135deg, var(--accent) 0%, var(--accent-hover) 100%);
            border: none;
            border-radius: 8px;
            color: white;
            font-size: 1rem;
            font-weight: 500;
            cursor: pointer;
            transition: all 0.2s ease;
            position: relative;
            overflow: hidden;
        }
        
        .btn-primary:hover {
            transform: translateY(-2px);
            box-shadow: 0 8px 25px rgba(0, 200, 150, 0.3);
        }
        
        .btn-primary:disabled {
            opacity: 0.7;
            cursor: not-allowed;
            transform: none;
            box-shadow: none;
        }
        
        .alert {
            padding: 1rem;
            border-radius: 8px;
            margin-top: 1rem;
            display: none;
        }
        
        .alert-success {
            background: rgba(0, 200, 150, 0.1);
            border: 1px solid var(--success);
            color: var(--success);
        }
        
        .alert-error {
            background: rgba(255, 55, 95, 0.1);
            border: 1px solid var(--danger);
            color: var(--danger);
        }
        
        .team-member {
            display: flex;
            align-items: center;
            gap: 0.75rem;
            padding: 0.5rem 0;
            border-bottom: 1px solid rgba(255, 255, 255, 0.05);
        }
        
        .team-member:last-child {
            border-bottom: none;
        }
        
        .member-status {
            width: 8px;
            height: 8px;
            border-radius: 50%;
            flex-shrink: 0;
        }
        
        .status-idle { background: var(--text-secondary); }
        .status-working { background: var(--accent); }
        .status-error { background: var(--danger); }
        
        .member-info {
            flex: 1;
            font-size: 0.9rem;
        }
        
        .member-name {
            font-weight: 500;
            color: var(--text-primary);
        }
        
        .member-task {
            color: var(--text-secondary);
            font-size: 0.8rem;
        }
        
        .pulse {
            animation: pulse 2s infinite;
        }
        
        @keyframes pulse {
            0% { opacity: 1; }
            50% { opacity: 0.5; }
            100% { opacity: 1; }
        }
        
        .loading {
            display: inline-block;
            width: 20px;
            height: 20px;
            border: 2px solid var(--border);
            border-radius: 50%;
            border-top-color: var(--accent);
            animation: spin 1s ease-in-out infinite;
        }
        
        @keyframes spin {
            to { transform: rotate(360deg); }
        }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <div class="logo">🤖</div>
            <h1 class="title">Borg Coordinator</h1>
            <p class="subtitle">AI Development Team Management</p>
        </div>
        
        <div class="grid">
            <div class="card">
                <h2>📊 Team Status</h2>
                <div id="team-info" class="status-grid">
                    <div class="loading"></div>
                </div>
                <button type="button" class="refresh-btn" onclick="loadStatus()">
                    🔄 Refresh
                </button>
            </div>
            
            <div class="card">
                <h2>💬 Request to AI Team</h2>
                <p style="color: var(--text-secondary); margin-bottom: 1.5rem; font-size: 0.9rem;">
                    Describe what you want accomplished. The Project Manager will analyze your request and break it down into specific tasks for the appropriate team members.
                </p>
                <form id="taskForm" onsubmit="createTask(event)">
                    <div class="form-group">
                        <label class="form-label" for="description">What would you like the AI team to work on?</label>
                        <textarea id="description" name="description" class="form-textarea" required 
                            placeholder="Example: 'Implement user authentication with login, registration, and password reset functionality' or 'Fix the performance issues in the data processing module' or 'Add comprehensive tests for the API endpoints'"></textarea>
                    </div>
                    
                    <button type="submit" class="btn-primary">
                        📋 Send to Project Manager
                    </button>
                </form>
                
                <div id="result" class="alert"></div>
                
                <div style="margin-top: 1.5rem; padding: 1rem; background: rgba(255, 255, 255, 0.03); border-radius: 8px; border-left: 3px solid var(--accent);">
                    <div style="font-size: 0.9rem; color: var(--text-secondary); margin-bottom: 0.5rem;">
                        <strong>How it works:</strong>
                    </div>
                    <div style="font-size: 0.8rem; color: var(--text-secondary); line-height: 1.4;">
                        1. You describe your high-level requirement<br>
                        2. Project Manager analyzes and plans the work<br>
                        3. Tasks are automatically assigned to specialists<br>
                        4. Team collaborates to complete your request
                    </div>
                </div>
            </div>
        </div>
    </div>

    <script>
        async function createTask(event) {
            event.preventDefault();
            
            const formData = new FormData(event.target);
            const taskData = {
                description: formData.get('description')
            };
            
            const button = event.target.querySelector('.btn-primary');
            const originalText = button.innerHTML;
            button.disabled = true;
            button.innerHTML = '<div class="loading"></div> Sending to Project Manager...';
            
            try {
                const response = await fetch('/create_task', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify(taskData)
                });
                
                const result = await response.text();
                const resultDiv = document.getElementById('result');
                
                if (response.ok) {
                    resultDiv.className = 'alert alert-success';
                    resultDiv.innerHTML = '✅ ' + result.replace(/<[^>]*>/g, '');
                    event.target.reset();
                    loadStatus();
                } else {
                    resultDiv.className = 'alert alert-error';
                    resultDiv.innerHTML = '❌ ' + result.replace(/<[^>]*>/g, '');
                }
                
                resultDiv.style.display = 'block';
                setTimeout(() => {
                    resultDiv.style.display = 'none';
                }, 5000);
                
            } catch (error) {
                const resultDiv = document.getElementById('result');
                resultDiv.className = 'alert alert-error';
                resultDiv.innerHTML = '❌ Error: ' + error.message;
                resultDiv.style.display = 'block';
            }
            
            button.disabled = false;
            button.innerHTML = originalText;
        }
        
        async function loadStatus() {
            try {
                const response = await fetch('/status');
                const status = await response.text();
                document.getElementById('team-info').innerHTML = status;
            } catch (error) {
                document.getElementById('team-info').innerHTML = 
                    '<div class="status-item"><span class="status-icon">❌</span>Failed to load team status</div>';
            }
        }
        
        // Load initial status
        loadStatus();
        
        // Auto-refresh every 30 seconds
        setInterval(loadStatus, 30000);
    </script>
</body>
</html>
"#;

    format!("HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}", html.len(), html)
}

async fn handle_task_creation(request: &str, task_queue: Arc<TaskQueue>) -> String {
    // Extract JSON from POST body
    let lines: Vec<&str> = request.lines().collect();
    let body_start = lines.iter().position(|&line| line.is_empty()).unwrap_or(0) + 1;
    let body = lines[body_start..].join("\n");
    
    // Clean up the body - remove null bytes and trim
    let clean_body = body.trim_matches('\0').trim();
    
    // Debug: print the body for troubleshooting
    eprintln!("Received body: '{}'", clean_body);
    
    let task_data: Result<serde_json::Value, _> = serde_json::from_str(clean_body);
    
    match task_data {
        Ok(data) => {
            let description = data["description"].as_str().unwrap_or("No description").to_string();
            
            // Find the Project Manager instance
            let instances = match task_queue.get_all_instances().await {
                Ok(instances) => instances,
                Err(e) => {
                    let response_body = format!("<p style='color: red;'>❌ Failed to get instances: {}</p>", e);
                    return format!("HTTP/1.1 500 Internal Server Error\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}", response_body.len(), response_body);
                }
            };
            
            let project_manager_id = instances
                .iter()
                .find(|instance| instance.role == InstanceRole::ProjectManager)
                .map(|instance| instance.id);
            
            // Create task directly in Redis queue and notify orchestrator
            let task = Task {
                id: uuid::Uuid::new_v4(),
                task_type: TaskType::ProjectPlanning,
                description: format!("User Request: {}", description),
                assigned_to: project_manager_id,
                status: TaskStatus::Pending,
                priority: TaskPriority::High,
                dependencies: vec![],
                result: None,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            };
            
            // Store task and add to queue
            match task_queue.store_task(&task).await {
                Ok(_) => {
                    // Also add to priority queue for processing
                    if let Err(e) = task_queue.add_task(&task).await {
                        let response_body = format!("<p style='color: red;'>❌ Failed to queue task: {}</p>", e);
                        return format!("HTTP/1.1 500 Internal Server Error\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}", response_body.len(), response_body);
                    }
                    
                    let response_body = format!(
                        "<p style='color: green;'>✅ Request sent to Project Manager!</p><p><strong>Task ID:</strong> {}</p><p><strong>Your Request:</strong> {}</p><p style='margin-top: 1rem; font-size: 0.9rem; color: #888;'>The Project Manager will analyze your request and break it down into specific tasks for the team.</p>", 
                        task.id, description
                    );
                    format!("HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}", response_body.len(), response_body)
                }
                Err(e) => {
                    let response_body = format!("<p style='color: red;'>❌ Failed to send request to Project Manager: {}</p>", e);
                    format!("HTTP/1.1 500 Internal Server Error\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}", response_body.len(), response_body)
                }
            }
        }
        Err(e) => {
            let response_body = format!("<p style='color: red;'>❌ Invalid request format: {}</p>", e);
            format!("HTTP/1.1 400 Bad Request\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}", response_body.len(), response_body)
        }
    }
}

async fn handle_status_request(task_queue: Arc<TaskQueue>) -> String {
    let instances = task_queue.get_all_instances().await.unwrap_or_default();
    let tasks = task_queue.get_all_tasks().await.unwrap_or_default();
    
    let mut status_html = String::new();
    
    // Team summary
    status_html.push_str(&format!(
        r#"<div class="status-item">
            <span class="status-icon">👥</span>
            <div><strong>{} AI Agents</strong><br><small>{} Active Tasks</small></div>
        </div>"#,
        instances.len(),
        tasks.len()
    ));
    
    // Team members
    if !instances.is_empty() {
        for instance in &instances {
            let (status_class, _status_icon) = match instance.status {
                InstanceStatus::Idle => ("status-idle", "⭕"),
                InstanceStatus::Working => ("status-working", "🟢"),
                InstanceStatus::Error => ("status-error", "🔴"),
                InstanceStatus::Offline => ("status-idle", "⚫"),
            };
            
            let current_task = if let Some(ref task) = instance.current_task {
                format!("Working on: {}...", &task.description[..task.description.len().min(25)])
            } else {
                "Available".to_string()
            };
            
            let role_emoji = match instance.role {
                InstanceRole::ProjectManager => "📋",
                InstanceRole::Supervisor => "👑",
                InstanceRole::Developer => "💻",
                InstanceRole::Tester => "🧪",
                InstanceRole::Reviewer => "👀",
                InstanceRole::Researcher => "🔍",
            };
            
            status_html.push_str(&format!(
                r#"<div class="team-member">
                    <div class="member-status {}"></div>
                    <div class="status-icon">{}</div>
                    <div class="member-info">
                        <div class="member-name">{}</div>
                        <div class="member-task">{}</div>
                    </div>
                </div>"#,
                status_class,
                role_emoji,
                instance.config.name,
                current_task
            ));
        }
    }
    
    // Recent tasks section
    if !tasks.is_empty() {
        status_html.push_str(r#"<div style="margin-top: 1rem; padding-top: 1rem; border-top: 1px solid rgba(255,255,255,0.1);">"#);
        status_html.push_str("<div style='font-size: 0.9rem; color: var(--text-secondary); margin-bottom: 0.5rem;'>Recent Tasks</div>");
        
        for task in tasks.iter().take(3) {
            let status_icon = match task.status {
                TaskStatus::Pending => "⏳",
                TaskStatus::InProgress => "🔄",
                TaskStatus::Completed => "✅",
                TaskStatus::Failed => "❌",
                TaskStatus::Cancelled => "🚫",
            };
            
            let task_icon = match task.task_type {
                TaskType::ProjectPlanning => "📋",
                TaskType::FeatureImplementation => "⚡",
                TaskType::BugFix => "🐛",
                TaskType::CodeReview => "👀",
                TaskType::TestCreation => "🧪",
                TaskType::Research => "🔍",
                TaskType::Documentation => "📚",
            };
            
            status_html.push_str(&format!(
                r#"<div class="status-item" style="margin: 0.25rem 0;">
                    <span>{} {}</span>
                    <small style="color: var(--text-secondary);">{}</small>
                </div>"#,
                status_icon,
                task_icon,
                &task.description[..task.description.len().min(35)]
            ));
        }
        status_html.push_str("</div>");
    }
    
    format!("HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}", status_html.len(), status_html)
}