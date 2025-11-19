# LLM-Analytics-Hub Quick Start Guide

This guide will help you get the LLM-Analytics-Hub up and running quickly.

## Table of Contents
1. [Prerequisites](#prerequisites)
2. [Quick Start Options](#quick-start-options)
3. [Local Development Setup](#local-development-setup)
4. [Cloud Deployment](#cloud-deployment)
5. [Integration Examples](#integration-examples)
6. [Verification](#verification)
7. [Next Steps](#next-steps)

---

## Prerequisites

### Required
- Docker 20.10+
- Kubernetes 1.24+ (for production deployment)
- kubectl CLI
- Helm 3.x

### Optional
- Node.js 18+ (for local development)
- PostgreSQL client (for database access)
- Redis client (for cache debugging)

---

## Quick Start Options

### Option 1: Docker Compose (Fastest - Local Development)

**Time to deploy: ~5 minutes**

```bash
# Clone the repository
git clone https://github.com/llmplatform/llm-analytics-hub.git
cd llm-analytics-hub

# Start all services
docker-compose up -d

# Verify services are running
docker-compose ps

# Access the API
curl http://localhost:8080/health/ready
```

**What gets deployed:**
- Analytics API (port 8080)
- TimescaleDB (port 5432)
- Redis (port 6379)
- Kafka (port 9092)

**Access Points:**
- API: http://localhost:8080
- Metrics: http://localhost:9090/metrics
- Database: postgresql://postgres:password@localhost:5432/analytics

---

### Option 2: Kubernetes (Minikube - Local Testing)

**Time to deploy: ~10 minutes**

```bash
# Start minikube
minikube start --cpus=4 --memory=8192

# Create namespace
kubectl create namespace llm-analytics

# Add Helm repository
helm repo add analytics https://charts.analytics.local/
helm repo update

# Install with default values
helm install analytics-hub analytics/llm-analytics-hub \
  --namespace llm-analytics \
  --set architecture=standalone \
  --set postgresql.enabled=true \
  --set redis.enabled=true \
  --set kafka.enabled=true

# Wait for pods to be ready
kubectl wait --for=condition=ready pod \
  -l app=analytics-api \
  -n llm-analytics \
  --timeout=300s

# Port forward to access locally
kubectl port-forward -n llm-analytics svc/analytics-api 8080:8080
```

**Verify deployment:**
```bash
# Check all pods are running
kubectl get pods -n llm-analytics

# Check services
kubectl get svc -n llm-analytics

# Test API
curl http://localhost:8080/health/ready
```

---

### Option 3: Cloud Deployment (AWS/GCP/Azure)

**Time to deploy: ~30 minutes**

#### AWS (EKS)

```bash
# Create EKS cluster (if not exists)
eksctl create cluster \
  --name analytics-cluster \
  --region us-east-1 \
  --nodegroup-name analytics-nodes \
  --node-type m5.2xlarge \
  --nodes 3 \
  --nodes-min 3 \
  --nodes-max 10

# Install Analytics Hub
helm install analytics-hub analytics/llm-analytics-hub \
  --namespace llm-analytics \
  --create-namespace \
  --values values-aws.yaml

# Get load balancer URL
kubectl get svc -n llm-analytics analytics-api-service \
  -o jsonpath='{.status.loadBalancer.ingress[0].hostname}'
```

#### GCP (GKE)

```bash
# Create GKE cluster (if not exists)
gcloud container clusters create analytics-cluster \
  --region us-central1 \
  --num-nodes 3 \
  --machine-type n2-standard-8

# Get credentials
gcloud container clusters get-credentials analytics-cluster --region us-central1

# Install Analytics Hub
helm install analytics-hub analytics/llm-analytics-hub \
  --namespace llm-analytics \
  --create-namespace \
  --values values-gcp.yaml

# Get load balancer IP
kubectl get svc -n llm-analytics analytics-api-service \
  -o jsonpath='{.status.loadBalancer.ingress[0].ip}'
```

#### Azure (AKS)

```bash
# Create AKS cluster (if not exists)
az aks create \
  --resource-group analytics-rg \
  --name analytics-cluster \
  --node-count 3 \
  --node-vm-size Standard_D8s_v3

# Get credentials
az aks get-credentials --resource-group analytics-rg --name analytics-cluster

# Install Analytics Hub
helm install analytics-hub analytics/llm-analytics-hub \
  --namespace llm-analytics \
  --create-namespace \
  --values values-azure.yaml

# Get load balancer IP
kubectl get svc -n llm-analytics analytics-api-service \
  -o jsonpath='{.status.loadBalancer.ingress[0].ip}'
```

---

## Local Development Setup

### 1. Install Dependencies

```bash
# Clone repository
git clone https://github.com/llmplatform/llm-analytics-hub.git
cd llm-analytics-hub

# Install Node.js dependencies
npm install

# Copy environment template
cp .env.example .env

# Edit environment variables
nano .env
```

### 2. Start Infrastructure Services

```bash
# Start PostgreSQL
docker run -d \
  --name analytics-postgres \
  -p 5432:5432 \
  -e POSTGRES_PASSWORD=password \
  -e POSTGRES_DB=analytics \
  timescale/timescaledb:latest-pg14

# Start Redis
docker run -d \
  --name analytics-redis \
  -p 6379:6379 \
  redis:7-alpine

# Start Kafka (with Zookeeper)
docker-compose up -d kafka zookeeper
```

### 3. Initialize Database

```bash
# Run migrations
npm run migrate

# Seed test data (optional)
npm run seed
```

### 4. Start Application

```bash
# Development mode (with hot reload)
npm run dev

# Production mode
npm run build
npm start
```

### 5. Verify Local Setup

```bash
# Health check
curl http://localhost:8080/health/ready

# Get metrics
curl http://localhost:9090/metrics

# Test API
curl http://localhost:8080/api/v1/metrics
```

---

## Integration Examples

### Example 1: Send Event from LLM-Registry

```typescript
import axios from 'axios';

// Registry asset registration event
const registryEvent = {
  eventId: '550e8400-e29b-41d4-a716-446655440000',
  eventType: 'asset.registered',
  source: 'registry',
  timestamp: new Date().toISOString(),
  version: '1.0.0',
  data: {
    assetId: 'model-123',
    assetName: 'GPT-4-Turbo',
    version: '1.2.0',
    owner: 'ml-team',
    category: 'language-model'
  }
};

// Send to Analytics Hub
await axios.post('http://localhost:8080/api/v1/integrations/registry/events', registryEvent);
```

### Example 2: Report Compliance Metrics to Policy Engine

```typescript
const complianceMetrics = {
  reportId: 'comp-2025-11-19-001',
  timestamp: new Date().toISOString(),
  timeRange: {
    start: new Date(Date.now() - 3600000).toISOString(),
    end: new Date().toISOString()
  },
  metrics: {
    totalEvaluations: 5000,
    compliantRequests: 4850,
    violations: 150,
    complianceRate: 0.97
  }
};

await axios.post('http://localhost:8080/api/v1/integrations/policy-engine/compliance/report', complianceMetrics);
```

### Example 3: Query Analytics Data

```typescript
// Get metrics for an asset
const response = await axios.get('http://localhost:8080/api/v1/metrics', {
  params: {
    assetId: 'model-123',
    timeRange: '24h',
    metricType: 'latency'
  }
});

console.log(response.data);
```

### Example 4: Install Analytics Plugin

```typescript
// Discover available extensions
const extensions = await axios.get('http://localhost:8080/api/v1/integrations/marketplace/extensions', {
  params: {
    category: 'analytics',
    verified: true
  }
});

// Install extension
await axios.post('http://localhost:8080/api/v1/integrations/marketplace/extensions/custom-metrics-plugin/install');
```

---

## Verification

### Health Checks

```bash
# Liveness (is the service alive?)
curl http://localhost:8080/health/live

# Readiness (is the service ready to accept traffic?)
curl http://localhost:8080/health/ready

# Startup (has initialization completed?)
curl http://localhost:8080/health/startup
```

**Expected Response:**
```json
{
  "status": "READY",
  "timestamp": "2025-11-19T10:00:00.000Z",
  "checks": {
    "database": "UP",
    "cache": "UP",
    "messageQueue": "UP",
    "externalAPIs": "UP"
  }
}
```

### Metrics Endpoint

```bash
# Prometheus metrics
curl http://localhost:9090/metrics
```

**Sample Metrics:**
```
# TYPE http_requests_total counter
http_requests_total{method="GET",path="/api/v1/metrics",status="200"} 1234

# TYPE http_request_duration_seconds histogram
http_request_duration_seconds_bucket{le="0.1"} 1000
http_request_duration_seconds_bucket{le="0.5"} 1200
http_request_duration_seconds_sum 450.5
http_request_duration_seconds_count 1234

# TYPE events_processed_total counter
events_processed_total{source="registry",type="asset.registered"} 500
```

### Database Connection

```bash
# Connect to database
psql postgresql://postgres:password@localhost:5432/analytics

# Check tables
\dt

# Count metrics
SELECT COUNT(*) FROM metrics;

# Recent events
SELECT * FROM events ORDER BY timestamp DESC LIMIT 10;
```

### Cache Connection

```bash
# Connect to Redis
redis-cli

# Check keys
KEYS analytics:*

# Get cached metadata
GET analytics:asset:model-123:metadata

# Check cache stats
INFO stats
```

---

## Next Steps

### 1. Configure Integrations

Edit configuration to enable integrations with other modules:

```yaml
# config/integrations.yaml
registry:
  enabled: true
  apiUrl: http://registry-service:8080/api/v1

policyEngine:
  enabled: true
  apiUrl: http://policy-engine-service:8080/api/v1

observatory:
  enabled: true
  apiUrl: http://observatory-service:8080/api/v1
```

### 2. Set Up Monitoring

Install Prometheus and Grafana:

```bash
# Install Prometheus
helm install prometheus prometheus-community/kube-prometheus-stack \
  --namespace monitoring \
  --create-namespace

# Import dashboards
kubectl apply -f dashboards/analytics-overview.json
kubectl apply -f dashboards/integration-health.json
kubectl apply -f dashboards/performance-metrics.json
```

### 3. Configure Alerts

Create alert rules:

```yaml
# alerts/rules.yaml
groups:
  - name: analytics_alerts
    rules:
      - alert: HighErrorRate
        expr: rate(http_requests_total{status=~"5.."}[5m]) > 0.05
        for: 5m
        labels:
          severity: critical
        annotations:
          summary: "High error rate detected"

      - alert: HighLatency
        expr: histogram_quantile(0.95, http_request_duration_seconds_bucket) > 1
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "High latency detected (p95 > 1s)"
```

### 4. Review Documentation

- [Integration Patterns](./docs/INTEGRATION_PATTERNS.md) - Learn about integration patterns
- [Deployment Strategies](./docs/DEPLOYMENT_STRATEGIES.md) - Explore deployment options
- [Architecture Diagrams](./docs/ARCHITECTURE_DIAGRAMS.md) - Understand system architecture
- [API Documentation](./docs/API.md) - API reference (if available)

### 5. Run Tests

```bash
# Unit tests
npm test

# Integration tests
npm run test:integration

# E2E tests
npm run test:e2e

# Coverage report
npm run test:coverage
```

---

## Troubleshooting

### Common Issues

#### Issue: Pods not starting

**Symptoms:**
```bash
kubectl get pods -n llm-analytics
# Shows pods in CrashLoopBackOff or Pending
```

**Solutions:**
```bash
# Check pod logs
kubectl logs -n llm-analytics <pod-name>

# Check events
kubectl get events -n llm-analytics --sort-by='.lastTimestamp'

# Check resource availability
kubectl describe pod -n llm-analytics <pod-name>

# Verify node resources
kubectl top nodes
```

#### Issue: Database connection fails

**Symptoms:**
```
Error: connect ECONNREFUSED 127.0.0.1:5432
```

**Solutions:**
```bash
# Check database is running
kubectl get pods -n llm-analytics -l app=timescaledb

# Port forward to test
kubectl port-forward -n llm-analytics svc/timescaledb 5432:5432

# Test connection
psql postgresql://postgres:password@localhost:5432/analytics

# Check credentials in secrets
kubectl get secret -n llm-analytics analytics-secrets -o yaml
```

#### Issue: High memory usage

**Symptoms:**
```bash
kubectl top pods -n llm-analytics
# Shows pods using > 80% of memory limit
```

**Solutions:**
```bash
# Check current limits
kubectl describe pod -n llm-analytics <pod-name> | grep -A 5 Limits

# Increase memory limits
helm upgrade analytics-hub analytics/llm-analytics-hub \
  --namespace llm-analytics \
  --set resources.limits.memory=8Gi

# Check for memory leaks in logs
kubectl logs -n llm-analytics <pod-name> | grep -i "out of memory"
```

#### Issue: Slow queries

**Symptoms:**
- API responses > 1 second
- High database CPU usage

**Solutions:**
```bash
# Check slow queries
psql -c "SELECT query, mean_exec_time FROM pg_stat_statements ORDER BY mean_exec_time DESC LIMIT 10;"

# Add missing indexes
psql -c "SELECT * FROM pg_stat_user_tables WHERE idx_scan = 0;"

# Verify TimescaleDB chunks
psql -c "SELECT * FROM timescaledb_information.chunks;"

# Check cache hit rate
redis-cli INFO stats | grep keyspace_hits
```

---

## Support

### Getting Help

- **Documentation**: [/docs](./docs/)
- **GitHub Issues**: https://github.com/llmplatform/llm-analytics-hub/issues
- **Community Slack**: #analytics-hub
- **Email**: analytics@llmplatform.io

### Reporting Issues

When reporting issues, include:

1. Deployment architecture (standalone/integrated/distributed)
2. Environment (local/AWS/GCP/Azure)
3. Version information
4. Error logs
5. Steps to reproduce

### Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines.

---

## Additional Resources

### Video Tutorials
- Quick Start Tutorial (5 min)
- Integration Deep Dive (30 min)
- Production Deployment Best Practices (45 min)

### Example Applications
- [Example Registry Integration](./examples/registry-integration/)
- [Example Policy Engine Integration](./examples/policy-integration/)
- [Example Custom Plugin](./examples/custom-plugin/)

### Community Resources
- [Analytics Hub Blog](https://blog.llmplatform.io/tag/analytics)
- [Community Plugins](https://marketplace.llmplatform.io/analytics)
- [Stack Overflow](https://stackoverflow.com/questions/tagged/llm-analytics-hub)

---

**Last Updated**: 2025-11-19
**Version**: 1.0.0
