import winston from 'winston';
import { Orchestrator } from './orchestrator/Orchestrator';
import { DashboardServer } from './dashboard/DashboardServer';

// Configure logging
const logger = winston.createLogger({
  level: process.env.LOG_LEVEL || 'info',
  format: winston.format.combine(
    winston.format.timestamp(),
    winston.format.errors({ stack: true }),
    winston.format.json()
  ),
  defaultMeta: { service: 'borg-coordinator' },
  transports: [
    new winston.transports.File({ filename: 'logs/error.log', level: 'error' }),
    new winston.transports.File({ filename: 'logs/combined.log' }),
    new winston.transports.Console({
      format: winston.format.combine(
        winston.format.colorize(),
        winston.format.simple()
      )
    })
  ]
});

async function main() {
  try {
    logger.info('Starting Borg Coordinator...');

    // Initialize orchestrator
    const orchestrator = new Orchestrator(logger);
    await orchestrator.initialize();

    // Initialize dashboard server
    const dashboardServer = new DashboardServer(orchestrator, logger);
    const dashboardPort = parseInt(process.env.DASHBOARD_PORT || '3000');
    await dashboardServer.start(dashboardPort);

    logger.info(`Borg Coordinator started successfully`);
    logger.info(`Dashboard available at http://localhost:${dashboardPort}`);

    // Graceful shutdown
    process.on('SIGINT', async () => {
      logger.info('Received SIGINT, shutting down gracefully...');
      await dashboardServer.stop();
      process.exit(0);
    });

    process.on('SIGTERM', async () => {
      logger.info('Received SIGTERM, shutting down gracefully...');
      await dashboardServer.stop();
      process.exit(0);
    });

  } catch (error) {
    logger.error('Failed to start Borg Coordinator:', error);
    process.exit(1);
  }
}

// Handle uncaught exceptions
process.on('uncaughtException', (error) => {
  logger.error('Uncaught Exception:', error);
  process.exit(1);
});

process.on('unhandledRejection', (reason, promise) => {
  logger.error('Unhandled Rejection at:', promise, 'reason:', reason);
  process.exit(1);
});

main();