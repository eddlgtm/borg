class BorgDashboard {
    constructor() {
        this.socket = io();
        this.instances = new Map();
        this.tasks = new Map();
        
        this.setupSocketListeners();
        this.loadInitialData();
    }

    setupSocketListeners() {
        this.socket.on('connect', () => {
            this.logActivity('Connected to dashboard server');
            this.socket.emit('subscribe', { events: ['instances', 'tasks'] });
        });

        this.socket.on('instanceCreated', (instance) => {
            this.instances.set(instance.id, instance);
            this.updateInstancesDisplay();
            this.updateStats();
            this.logActivity(`Instance created: ${instance.role} (${instance.id})`);
        });

        this.socket.on('instanceTerminated', (instance) => {
            this.instances.delete(instance.id);
            this.updateInstancesDisplay();
            this.updateStats();
            this.logActivity(`Instance terminated: ${instance.role} (${instance.id})`);
        });

        this.socket.on('taskAssigned', (task) => {
            this.tasks.set(task.id, task);
            this.updateTasksDisplay();
            this.updateStats();
            this.logActivity(`Task assigned: ${task.description.substring(0, 50)}...`);
        });

        this.socket.on('taskCompleted', (data) => {
            const task = data.task;
            this.tasks.set(task.id, task);
            this.updateTasksDisplay();
            this.updateStats();
            this.logActivity(`Task completed: ${task.description.substring(0, 50)}...`);
        });

        this.socket.on('taskAssignedToInstance', (data) => {
            const { task, instance } = data;
            this.tasks.set(task.id, task);
            this.instances.set(instance.id, instance);
            this.updateInstancesDisplay();
            this.updateTasksDisplay();
            this.logActivity(`Task assigned to ${instance.role}: ${task.description.substring(0, 50)}...`);
        });

        this.socket.on('instanceError', (data) => {
            const { instance, error } = data;
            this.instances.set(instance.id, instance);
            this.updateInstancesDisplay();
            this.logActivity(`Instance error (${instance.id}): ${error.message}`, 'error');
        });
    }

    async loadInitialData() {
        try {
            const [instancesRes, tasksRes] = await Promise.all([
                fetch('/api/instances'),
                fetch('/api/tasks')
            ]);

            const instances = await instancesRes.json();
            const tasks = await tasksRes.json();

            instances.forEach(instance => this.instances.set(instance.id, instance));
            tasks.forEach(task => this.tasks.set(task.id, task));

            this.updateInstancesDisplay();
            this.updateTasksDisplay();
            this.updateStats();
        } catch (error) {
            this.logActivity(`Failed to load initial data: ${error.message}`, 'error');
        }
    }

    updateStats() {
        const totalInstances = this.instances.size;
        const activeInstances = Array.from(this.instances.values())
            .filter(i => i.status === 'working').length;
        const totalTasks = this.tasks.size;
        const pendingTasks = Array.from(this.tasks.values())
            .filter(t => t.status === 'pending').length;

        document.getElementById('totalInstances').textContent = totalInstances;
        document.getElementById('activeInstances').textContent = activeInstances;
        document.getElementById('totalTasks').textContent = totalTasks;
        document.getElementById('pendingTasks').textContent = pendingTasks;
    }

    updateInstancesDisplay() {
        const container = document.getElementById('instancesList');
        container.innerHTML = '';

        if (this.instances.size === 0) {
            container.innerHTML = '<p style="text-align: center; color: #7f8c8d; padding: 20px;">No instances created yet</p>';
            return;
        }

        Array.from(this.instances.values()).forEach(instance => {
            const item = document.createElement('div');
            item.className = 'instance-item';
            
            const statusClass = `status-${instance.status}`;
            const currentTask = instance.currentTask ? 
                `<p>Task: ${instance.currentTask.description.substring(0, 40)}...</p>` : 
                '<p>No active task</p>';
            
            item.innerHTML = `
                <div class="instance-info">
                    <h4>${instance.role.toUpperCase()} - ${instance.id.substring(0, 8)}</h4>
                    ${currentTask}
                    <p>Created: ${new Date(instance.createdAt).toLocaleDateString()}</p>
                </div>
                <div>
                    <span class="status-badge ${statusClass}">${instance.status.toUpperCase()}</span>
                    <button class="btn btn-danger" style="margin-left: 10px; padding: 5px 10px;" 
                            onclick="dashboard.terminateInstance('${instance.id}')">Terminate</button>
                </div>
            `;
            
            container.appendChild(item);
        });
    }

    updateTasksDisplay() {
        const container = document.getElementById('tasksList');
        container.innerHTML = '';

        if (this.tasks.size === 0) {
            container.innerHTML = '<p style="text-align: center; color: #7f8c8d; padding: 20px;">No tasks created yet</p>';
            return;
        }

        const sortedTasks = Array.from(this.tasks.values())
            .sort((a, b) => new Date(b.createdAt) - new Date(a.createdAt));

        sortedTasks.forEach(task => {
            const item = document.createElement('div');
            item.className = 'task-item';
            
            const priorityClass = `priority-${task.priority}`;
            const assignedTo = task.assignedTo ? 
                `Assigned to: ${task.assignedTo.substring(0, 8)}` : 
                'Unassigned';
            
            item.innerHTML = `
                <div class="task-title">${task.description}</div>
                <div class="task-meta">
                    <span>Type: ${task.type.replace('_', ' ').toUpperCase()}</span>
                    <span class="${priorityClass}">Priority: ${task.priority.toUpperCase()}</span>
                    <span>Status: ${task.status.replace('_', ' ').toUpperCase()}</span>
                    <span>${assignedTo}</span>
                </div>
            `;
            
            container.appendChild(item);
        });
    }

    async createInstance(role, capabilities) {
        try {
            const response = await fetch('/api/instances', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ 
                    role, 
                    capabilities: capabilities.split(',').map(c => c.trim()).filter(c => c) 
                })
            });

            if (!response.ok) {
                throw new Error('Failed to create instance');
            }

            this.logActivity(`Creating ${role} instance...`);
        } catch (error) {
            this.logActivity(`Failed to create instance: ${error.message}`, 'error');
        }
    }

    async createTask(type, description, priority) {
        try {
            const response = await fetch('/api/tasks', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ type, description, priority })
            });

            if (!response.ok) {
                throw new Error('Failed to create task');
            }

            this.logActivity(`Creating ${type} task...`);
        } catch (error) {
            this.logActivity(`Failed to create task: ${error.message}`, 'error');
        }
    }

    async terminateInstance(instanceId) {
        if (!confirm('Are you sure you want to terminate this instance?')) {
            return;
        }

        try {
            const response = await fetch(`/api/instances/${instanceId}`, {
                method: 'DELETE'
            });

            if (!response.ok) {
                throw new Error('Failed to terminate instance');
            }

            this.logActivity(`Terminating instance ${instanceId.substring(0, 8)}...`);
        } catch (error) {
            this.logActivity(`Failed to terminate instance: ${error.message}`, 'error');
        }
    }

    logActivity(message, type = 'info') {
        const log = document.getElementById('activityLog');
        const timestamp = new Date().toLocaleTimeString();
        const color = type === 'error' ? '#e74c3c' : '#ecf0f1';
        
        log.innerHTML += `<span style="color: ${color}">[${timestamp}] ${message}</span><br>`;
        log.scrollTop = log.scrollHeight;
    }
}

// Initialize dashboard
const dashboard = new BorgDashboard();

// Modal functions
function showCreateInstanceModal() {
    document.getElementById('createInstanceModal').style.display = 'block';
}

function hideCreateInstanceModal() {
    document.getElementById('createInstanceModal').style.display = 'none';
}

function showCreateTaskModal() {
    document.getElementById('createTaskModal').style.display = 'block';
}

function hideCreateTaskModal() {
    document.getElementById('createTaskModal').style.display = 'none';
}

function createInstance(event) {
    event.preventDefault();
    
    const role = document.getElementById('instanceRole').value;
    const capabilities = document.getElementById('instanceCapabilities').value;
    
    dashboard.createInstance(role, capabilities);
    hideCreateInstanceModal();
    
    // Clear form
    document.getElementById('instanceCapabilities').value = '';
}

function createTask(event) {
    event.preventDefault();
    
    const type = document.getElementById('taskType').value;
    const description = document.getElementById('taskDescription').value;
    const priority = document.getElementById('taskPriority').value;
    
    dashboard.createTask(type, description, priority);
    hideCreateTaskModal();
    
    // Clear form
    document.getElementById('taskDescription').value = '';
}

// Close modals when clicking outside
window.onclick = function(event) {
    const instanceModal = document.getElementById('createInstanceModal');
    const taskModal = document.getElementById('createTaskModal');
    
    if (event.target === instanceModal) {
        hideCreateInstanceModal();
    }
    if (event.target === taskModal) {
        hideCreateTaskModal();
    }
}