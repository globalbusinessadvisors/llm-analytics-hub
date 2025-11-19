# LLM-Analytics-Hub Deployment Strategies

## Table of Contents
1. [Deployment Architectures](#deployment-architectures)
2. [Deployment Specifications](#deployment-specifications)
3. [Operational Patterns](#operational-patterns)
4. [Deployment Decision Trees](#deployment-decision-trees)
5. [Cloud Provider Configurations](#cloud-provider-configurations)
6. [On-Premise Deployment](#on-premise-deployment)

---

## Deployment Architectures

### Architecture 1: Standalone Analytics Service

#### Overview
Independent service deployment with its own infrastructure, database, and API gateway.

#### Architecture Diagram (Textual)

```
                        [Load Balancer]
                              |
                    +---------+---------+
                    |                   |
            [API Gateway 1]     [API Gateway 2]
                    |                   |
        +-----------+--------+----------+-----------+
        |           |        |          |           |
   [Service 1] [Service 2] [Service 3] [Service 4] [Service 5]
   (Ingestion) (Processing) (Query)    (Alerts)   (Reporting)
        |           |        |          |           |
        +-----+-----+--------+----------+-----+-----+
              |                              |
      [Analytics Database]          [Cache Layer]
      (TimescaleDB/ClickHouse)      (Redis)
              |
      [Message Queue]
      (Kafka/RabbitMQ)
```

#### Components

**API Gateway Layer:**
- Request routing and load balancing
- Authentication and authorization
- Rate limiting and throttling
- API versioning
- Request/response transformation

**Service Layer:**
- **Ingestion Service**: Receives data from external modules
- **Processing Service**: Transforms and enriches data
- **Query Service**: Handles analytical queries
- **Alert Service**: Manages alerting and notifications
- **Reporting Service**: Generates reports and dashboards

**Data Layer:**
- **Analytics Database**: Time-series optimized storage
- **Cache Layer**: High-speed data access
- **Message Queue**: Event streaming and buffering

#### Pros

1. **Isolation**: Complete independence from other platform services
2. **Scalability**: Independent horizontal and vertical scaling
3. **Technology Freedom**: Choose optimal tech stack without constraints
4. **Security**: Dedicated security perimeter and access controls
5. **Deployment Flexibility**: Deploy, update, and rollback independently
6. **Resource Optimization**: Dedicated resource allocation and tuning
7. **Fault Isolation**: Failures don't impact other platform services
8. **Multi-Tenancy**: Easier to implement tenant isolation
9. **Compliance**: Simplified compliance with dedicated infrastructure
10. **Performance Tuning**: Optimize specifically for analytics workloads

#### Cons

1. **Operational Overhead**: Separate monitoring, logging, and alerting infrastructure
2. **Infrastructure Cost**: Dedicated resources may be underutilized
3. **Integration Complexity**: Requires robust API contracts and versioning
4. **Data Duplication**: May duplicate data from other platform services
5. **Network Latency**: Additional network hops for inter-service communication
6. **Authentication Overhead**: Separate auth mechanisms may be needed
7. **Deployment Complexity**: More moving parts to coordinate
8. **Observability**: Need separate observability stack
9. **Maintenance Burden**: More systems to patch and upgrade
10. **Learning Curve**: Teams need to understand separate architecture

#### Use Cases

- **Large-scale deployments**: High volume analytics workloads
- **Multi-tenant SaaS**: Serving multiple organizations
- **Compliance-critical**: Strict data isolation requirements
- **Global distribution**: Analytics nodes in multiple regions
- **Performance-critical**: Sub-second query requirements

#### Configuration Example

```yaml
# standalone-deployment.yaml
deployment:
  architecture: "standalone"

  apiGateway:
    replicas: 3
    resources:
      requests:
        cpu: "500m"
        memory: "512Mi"
      limits:
        cpu: "2000m"
        memory: "2Gi"

  services:
    ingestion:
      replicas: 5
      autoscaling:
        enabled: true
        minReplicas: 5
        maxReplicas: 20
        targetCPUUtilization: 70

    processing:
      replicas: 3
      autoscaling:
        enabled: true
        minReplicas: 3
        maxReplicas: 15
        targetCPUUtilization: 60

    query:
      replicas: 4
      autoscaling:
        enabled: true
        minReplicas: 4
        maxReplicas: 12
        targetCPUUtilization: 75

    alerts:
      replicas: 2
      autoscaling:
        enabled: false

    reporting:
      replicas: 2
      autoscaling:
        enabled: true
        minReplicas: 2
        maxReplicas: 8
        targetCPUUtilization: 65

  database:
    type: "timescaledb"
    replicas: 3
    storage: "500Gi"
    backupEnabled: true
    backupRetention: "30d"

  cache:
    type: "redis"
    mode: "cluster"
    replicas: 6
    memory: "16Gi"

  messageQueue:
    type: "kafka"
    brokers: 5
    partitions: 50
    replicationFactor: 3
```

---

### Architecture 2: Integrated Platform Module

#### Overview
Embedded within the LLM DevOps Platform, sharing infrastructure and communicating directly with other modules.

#### Architecture Diagram (Textual)

```
                [LLM DevOps Platform]
                          |
        +-----------------+------------------+
        |                 |                  |
  [API Gateway]   [Service Mesh]   [Platform Database]
        |                 |                  |
  +-----+-----+-----------+----------+-------+-------+
  |           |           |          |       |       |
[Registry] [Policy]  [Analytics]  [Obs.]  [Cost]  [Gov.]
  |           |           |          |       |       |
  +-----------+-----------+----------+-------+-------+
                          |
              [Shared Infrastructure]
                          |
        +-----------------+------------------+
        |                 |                  |
   [Logging]        [Monitoring]        [Tracing]
   (Shared)          (Shared)            (Shared)
```

#### Components

**Shared Platform Layer:**
- **API Gateway**: Single entry point for all platform services
- **Service Mesh**: Service-to-service communication (Istio/Linkerd)
- **Platform Database**: Shared database infrastructure
- **Authentication**: Unified auth across all services
- **Authorization**: RBAC with shared policy engine

**Analytics Module:**
- **Analytics Core**: Embedded analytics service
- **Data Collectors**: Direct integration with other modules
- **Analytics Store**: Dedicated schema in shared database
- **Event Processors**: Consumes events from platform event bus

**Shared Infrastructure:**
- **Logging**: Centralized log aggregation (ELK/Loki)
- **Monitoring**: Unified metrics (Prometheus/Grafana)
- **Tracing**: Distributed tracing (Jaeger/Zipkin)

#### Pros

1. **Simplified Operations**: Single platform to manage and monitor
2. **Shared Infrastructure**: Reduced operational overhead and cost
3. **Direct Communication**: Low-latency inter-module communication
4. **Unified Auth**: Single authentication and authorization system
5. **Consistent Observability**: Shared logging, monitoring, and tracing
6. **Reduced Network Overhead**: In-process or local network calls
7. **Simplified Deployment**: Deploy as part of platform releases
8. **Data Consistency**: Easier to maintain consistency with other modules
9. **Configuration Management**: Centralized configuration
10. **Developer Experience**: Simpler local development setup

#### Cons

1. **Tight Coupling**: Changes in platform affect analytics module
2. **Scaling Limitations**: Can't scale independently from platform
3. **Technology Constraints**: Must use platform's tech stack
4. **Deployment Coupling**: Platform upgrades affect analytics
5. **Resource Contention**: Competes for resources with other modules
6. **Blast Radius**: Analytics issues may impact entire platform
7. **Testing Complexity**: Harder to test in isolation
8. **Security Perimeter**: Shares security boundary with platform
9. **Performance Impact**: Heavy analytics may affect platform performance
10. **Flexibility**: Limited architectural choices

#### Use Cases

- **Small to medium deployments**: Moderate analytics workloads
- **Startup/MVP**: Rapid development and deployment
- **Tight Integration**: When deep integration with other modules is critical
- **Resource Constraints**: Limited operational resources
- **Unified Platform**: When platform consistency is prioritized

#### Configuration Example

```yaml
# integrated-deployment.yaml
deployment:
  architecture: "integrated"

  platform:
    namespace: "llm-platform"
    serviceMesh: "istio"

  analytics:
    module:
      enabled: true
      replicas: 3
      resources:
        requests:
          cpu: "1000m"
          memory: "2Gi"
        limits:
          cpu: "4000m"
          memory: "8Gi"

    integration:
      directCommunication: true
      useServiceMesh: true

    database:
      shared: true
      schema: "analytics"
      poolSize: 50

    events:
      platformEventBus: true
      topics:
        - "platform.events"
        - "platform.metrics"

    observability:
      logging:
        shared: true
        level: "info"

      monitoring:
        shared: true
        exportMetrics: true

      tracing:
        shared: true
        samplingRate: 0.1
```

---

### Architecture 3: Distributed Data Node Cluster

#### Overview
Horizontal scaling approach with data sharding, distributed processing, and high availability.

#### Architecture Diagram (Textual)

```
                        [Load Balancer]
                              |
                    +---------+---------+
                    |                   |
              [Coordinator 1]     [Coordinator 2]
              (Leader Election)   (Standby)
                    |                   |
        +-----------+-------------------+-----------+
        |           |           |           |       |
   [Data Node 1] [Data Node 2] [Data Node 3] ... [Data Node N]
   (Shard 1,4,7) (Shard 2,5,8) (Shard 3,6,9)
        |           |           |           |       |
   [Local DB]  [Local DB]  [Local DB]  [Local DB]
   (Shard 1,4) (Shard 2,5) (Shard 3,6)
        |           |           |           |       |
        +-----------+-----------+-----------+-------+
                              |
                    [Distributed Cache]
                    (Redis Cluster)
                              |
                    [Message Queue Cluster]
                    (Kafka Cluster)
```

#### Components

**Coordinator Layer:**
- **Leader Coordinator**: Coordinates cluster operations
- **Standby Coordinator**: Hot standby for failover
- **Cluster Management**: Node registration and health monitoring
- **Shard Assignment**: Manages shard distribution

**Data Node Layer:**
- **Data Nodes**: Process and store sharded data
- **Local Storage**: Each node maintains subset of data
- **Query Processor**: Handles queries for local shards
- **Replication Manager**: Manages data replication

**Shared Infrastructure:**
- **Distributed Cache**: Shared cache for hot data
- **Message Queue**: Event distribution and coordination
- **Service Discovery**: Dynamic node discovery (Consul/etcd)

#### Sharding Strategy

**Hash-Based Sharding:**
```typescript
class ShardingStrategy {
  private shardCount: number;
  private replicationFactor: number;

  constructor(shardCount: number, replicationFactor: number = 3) {
    this.shardCount = shardCount;
    this.replicationFactor = replicationFactor;
  }

  /**
   * Determine shard for a given key
   */
  getShard(key: string): number {
    const hash = this.hashCode(key);
    return Math.abs(hash) % this.shardCount;
  }

  /**
   * Get replica shards for data redundancy
   */
  getReplicaShards(primaryShard: number): number[] {
    const replicas: number[] = [];
    for (let i = 1; i < this.replicationFactor; i++) {
      replicas.push((primaryShard + i) % this.shardCount);
    }
    return replicas;
  }

  /**
   * Get nodes responsible for a shard
   */
  getNodesForShard(shard: number, nodeCount: number): number[] {
    const nodesPerShard = Math.ceil(nodeCount / this.shardCount);
    const startNode = (shard * nodesPerShard) % nodeCount;
    const nodes: number[] = [];

    for (let i = 0; i < this.replicationFactor; i++) {
      nodes.push((startNode + i) % nodeCount);
    }

    return nodes;
  }

  private hashCode(str: string): number {
    let hash = 0;
    for (let i = 0; i < str.length; i++) {
      const char = str.charCodeAt(i);
      hash = ((hash << 5) - hash) + char;
      hash = hash & hash;
    }
    return hash;
  }
}
```

**Time-Based Sharding:**
```typescript
class TimeBasedSharding {
  private shardDuration: number; // milliseconds

  constructor(shardDurationDays: number = 7) {
    this.shardDuration = shardDurationDays * 24 * 60 * 60 * 1000;
  }

  getShard(timestamp: number): string {
    const shardId = Math.floor(timestamp / this.shardDuration);
    return `shard_${shardId}`;
  }

  getShardRange(shard: string): { start: number; end: number } {
    const shardId = parseInt(shard.split('_')[1]);
    return {
      start: shardId * this.shardDuration,
      end: (shardId + 1) * this.shardDuration
    };
  }

  getShardsForTimeRange(start: number, end: number): string[] {
    const startShard = Math.floor(start / this.shardDuration);
    const endShard = Math.floor(end / this.shardDuration);
    const shards: string[] = [];

    for (let i = startShard; i <= endShard; i++) {
      shards.push(`shard_${i}`);
    }

    return shards;
  }
}
```

#### Leader Election

**Raft-Based Election:**
```typescript
interface RaftNode {
  id: string;
  state: 'follower' | 'candidate' | 'leader';
  currentTerm: number;
  votedFor: string | null;
  log: LogEntry[];
  commitIndex: number;
  lastApplied: number;
}

class RaftLeaderElection {
  private node: RaftNode;
  private electionTimeout: number;
  private heartbeatInterval: number;

  constructor(nodeId: string) {
    this.node = {
      id: nodeId,
      state: 'follower',
      currentTerm: 0,
      votedFor: null,
      log: [],
      commitIndex: 0,
      lastApplied: 0
    };

    this.electionTimeout = this.randomTimeout(150, 300);
    this.heartbeatInterval = 50;
  }

  /**
   * Start election process
   */
  startElection(): void {
    this.node.state = 'candidate';
    this.node.currentTerm++;
    this.node.votedFor = this.node.id;

    // Request votes from other nodes
    this.requestVotes();
  }

  /**
   * Handle vote request
   */
  handleVoteRequest(candidateId: string, term: number): boolean {
    if (term > this.node.currentTerm) {
      this.node.currentTerm = term;
      this.node.votedFor = null;
      this.node.state = 'follower';
    }

    if (this.node.votedFor === null || this.node.votedFor === candidateId) {
      this.node.votedFor = candidateId;
      return true;
    }

    return false;
  }

  /**
   * Become leader
   */
  becomeLeader(): void {
    this.node.state = 'leader';
    this.startHeartbeat();
  }

  /**
   * Send heartbeat to followers
   */
  private startHeartbeat(): void {
    setInterval(() => {
      if (this.node.state === 'leader') {
        this.sendHeartbeat();
      }
    }, this.heartbeatInterval);
  }

  private requestVotes(): void {
    // Implementation for requesting votes from cluster
  }

  private sendHeartbeat(): void {
    // Implementation for sending heartbeat to followers
  }

  private randomTimeout(min: number, max: number): number {
    return Math.floor(Math.random() * (max - min + 1)) + min;
  }
}
```

#### Fault Tolerance Mechanisms

**Health Monitoring:**
```typescript
class HealthMonitor {
  private nodes: Map<string, NodeHealth> = new Map();
  private checkInterval: number = 5000; // 5 seconds

  startMonitoring(): void {
    setInterval(() => {
      this.checkAllNodes();
    }, this.checkInterval);
  }

  private async checkAllNodes(): Promise<void> {
    for (const [nodeId, health] of this.nodes.entries()) {
      try {
        const isHealthy = await this.checkNode(nodeId);
        this.updateNodeHealth(nodeId, isHealthy);
      } catch (error) {
        this.handleNodeFailure(nodeId, error);
      }
    }
  }

  private async checkNode(nodeId: string): Promise<boolean> {
    // Perform health check
    // - TCP connectivity
    // - HTTP health endpoint
    // - Response time
    // - Resource utilization
    return true;
  }

  private updateNodeHealth(nodeId: string, isHealthy: boolean): void {
    const health = this.nodes.get(nodeId);
    if (health) {
      health.isHealthy = isHealthy;
      health.lastCheck = Date.now();

      if (isHealthy) {
        health.consecutiveFailures = 0;
      } else {
        health.consecutiveFailures++;

        if (health.consecutiveFailures >= 3) {
          this.markNodeAsFailed(nodeId);
        }
      }
    }
  }

  private handleNodeFailure(nodeId: string, error: any): void {
    console.error(`Node ${nodeId} health check failed:`, error);
    this.updateNodeHealth(nodeId, false);
  }

  private markNodeAsFailed(nodeId: string): void {
    // Trigger failover procedures
    // - Reassign shards
    // - Promote replicas
    // - Alert administrators
  }
}
```

**Automatic Failover:**
```typescript
class FailoverManager {
  private shardingStrategy: ShardingStrategy;
  private clusterState: ClusterState;

  async handleNodeFailure(failedNodeId: string): Promise<void> {
    // 1. Identify affected shards
    const affectedShards = this.getAffectedShards(failedNodeId);

    // 2. Find replica nodes
    const failoverPlan = this.createFailoverPlan(affectedShards);

    // 3. Promote replicas
    await this.executeFailover(failoverPlan);

    // 4. Redistribute load
    await this.redistributeShards();

    // 5. Update cluster state
    this.updateClusterState(failedNodeId, 'failed');
  }

  private getAffectedShards(nodeId: string): number[] {
    return this.clusterState.getShardsForNode(nodeId);
  }

  private createFailoverPlan(shards: number[]): FailoverPlan {
    const plan: FailoverPlan = { assignments: [] };

    for (const shard of shards) {
      const replicaNodes = this.clusterState.getReplicaNodesForShard(shard);
      const healthyReplica = replicaNodes.find(n => n.isHealthy);

      if (healthyReplica) {
        plan.assignments.push({
          shard,
          fromNode: null,
          toNode: healthyReplica.id,
          action: 'promote'
        });
      } else {
        // No healthy replica, need to restore from backup
        plan.assignments.push({
          shard,
          fromNode: null,
          toNode: this.selectHealthyNode(),
          action: 'restore'
        });
      }
    }

    return plan;
  }

  private async executeFailover(plan: FailoverPlan): Promise<void> {
    for (const assignment of plan.assignments) {
      if (assignment.action === 'promote') {
        await this.promoteReplica(assignment.shard, assignment.toNode);
      } else if (assignment.action === 'restore') {
        await this.restoreFromBackup(assignment.shard, assignment.toNode);
      }
    }
  }

  private async redistributeShards(): Promise<void> {
    // Rebalance shards across healthy nodes
  }

  private updateClusterState(nodeId: string, status: string): void {
    this.clusterState.setNodeStatus(nodeId, status);
  }

  private selectHealthyNode(): string {
    return this.clusterState.getHealthyNodes()[0].id;
  }

  private async promoteReplica(shard: number, nodeId: string): Promise<void> {
    // Promote replica to primary
  }

  private async restoreFromBackup(shard: number, nodeId: string): Promise<void> {
    // Restore shard from backup
  }
}
```

#### Pros

1. **Horizontal Scalability**: Add nodes to increase capacity
2. **High Availability**: No single point of failure
3. **Fault Tolerance**: Automatic failover and recovery
4. **Performance**: Distributed query processing
5. **Data Locality**: Sharding improves query performance
6. **Elastic Scaling**: Scale up/down based on demand
7. **Geographic Distribution**: Nodes in multiple regions
8. **Resource Efficiency**: Distribute load across cluster
9. **Consistency**: Configurable consistency levels
10. **Resilience**: Survives multiple node failures

#### Cons

1. **Complexity**: Significant operational complexity
2. **Consistency Challenges**: CAP theorem trade-offs
3. **Network Overhead**: Cross-node communication latency
4. **Data Rebalancing**: Complex shard redistribution
5. **Debugging Difficulty**: Distributed systems are hard to debug
6. **Operational Overhead**: Requires specialized expertise
7. **Cost**: Higher infrastructure costs
8. **Query Complexity**: Distributed queries are more complex
9. **Data Consistency**: Eventual consistency challenges
10. **Split-Brain Risk**: Potential for cluster partitioning

#### Use Cases

- **Massive Scale**: Petabyte-scale analytics
- **Global Distribution**: Multi-region deployments
- **High Availability**: 99.99%+ uptime requirements
- **Real-time Analytics**: Low-latency distributed queries
- **IoT/Streaming**: High-velocity data ingestion

#### Configuration Example

```yaml
# distributed-deployment.yaml
deployment:
  architecture: "distributed"

  cluster:
    name: "analytics-cluster"
    sharding:
      strategy: "hash"  # hash, range, time-based
      shardCount: 100
      replicationFactor: 3

    coordination:
      algorithm: "raft"  # raft, paxos, zab
      coordinators: 3
      electionTimeout: 300
      heartbeatInterval: 50

  dataNodes:
    replicas: 15
    autoscaling:
      enabled: true
      minReplicas: 10
      maxReplicas: 50
      targetCPUUtilization: 70

    resources:
      requests:
        cpu: "2000m"
        memory: "8Gi"
      limits:
        cpu: "8000m"
        memory: "32Gi"

    storage:
      type: "ssd"
      size: "1Ti"
      storageClass: "fast-ssd"

  failover:
    enabled: true
    healthCheck:
      interval: 5000
      timeout: 2000
      failureThreshold: 3

    automatic: true
    redistributeOnFailure: true

  consistency:
    level: "eventual"  # strong, eventual, causal
    quorum:
      read: 2  # N/2 + 1
      write: 2
```

---

## Deployment Specifications

### Docker Containerization Strategy

#### Multi-Stage Dockerfile

```dockerfile
# Stage 1: Build
FROM node:18-alpine AS builder

WORKDIR /app

# Copy dependency files
COPY package*.json ./
COPY tsconfig.json ./

# Install dependencies
RUN npm ci --only=production && \
    npm cache clean --force

# Copy source code
COPY src ./src

# Build application
RUN npm run build

# Stage 2: Production
FROM node:18-alpine AS production

# Add non-root user
RUN addgroup -g 1001 analytics && \
    adduser -D -u 1001 -G analytics analytics

WORKDIR /app

# Copy built artifacts
COPY --from=builder --chown=analytics:analytics /app/dist ./dist
COPY --from=builder --chown=analytics:analytics /app/node_modules ./node_modules
COPY --from=builder --chown=analytics:analytics /app/package*.json ./

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD node healthcheck.js

# Switch to non-root user
USER analytics

# Expose port
EXPOSE 8080

# Start application
CMD ["node", "dist/index.js"]
```

#### Docker Compose (Local Development)

```yaml
# docker-compose.yaml
version: '3.8'

services:
  analytics-api:
    build:
      context: .
      dockerfile: Dockerfile
    ports:
      - "8080:8080"
    environment:
      - NODE_ENV=development
      - DATABASE_URL=postgresql://postgres:password@postgres:5432/analytics
      - REDIS_URL=redis://redis:6379
      - KAFKA_BROKERS=kafka:9092
    depends_on:
      - postgres
      - redis
      - kafka
    volumes:
      - ./src:/app/src
      - ./config:/app/config
    networks:
      - analytics-network

  postgres:
    image: timescale/timescaledb:latest-pg14
    ports:
      - "5432:5432"
    environment:
      - POSTGRES_USER=postgres
      - POSTGRES_PASSWORD=password
      - POSTGRES_DB=analytics
    volumes:
      - postgres-data:/var/lib/postgresql/data
    networks:
      - analytics-network

  redis:
    image: redis:7-alpine
    ports:
      - "6379:6379"
    volumes:
      - redis-data:/data
    networks:
      - analytics-network

  kafka:
    image: confluentinc/cp-kafka:7.4.0
    ports:
      - "9092:9092"
    environment:
      - KAFKA_BROKER_ID=1
      - KAFKA_ZOOKEEPER_CONNECT=zookeeper:2181
      - KAFKA_ADVERTISED_LISTENERS=PLAINTEXT://kafka:9092
      - KAFKA_OFFSETS_TOPIC_REPLICATION_FACTOR=1
    depends_on:
      - zookeeper
    networks:
      - analytics-network

  zookeeper:
    image: confluentinc/cp-zookeeper:7.4.0
    ports:
      - "2181:2181"
    environment:
      - ZOOKEEPER_CLIENT_PORT=2181
      - ZOOKEEPER_TICK_TIME=2000
    networks:
      - analytics-network

volumes:
  postgres-data:
  redis-data:

networks:
  analytics-network:
    driver: bridge
```

---

### Kubernetes Manifests

#### Namespace

```yaml
# k8s/namespace.yaml
apiVersion: v1
kind: Namespace
metadata:
  name: llm-analytics
  labels:
    name: llm-analytics
    environment: production
```

#### ConfigMap

```yaml
# k8s/configmap.yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: analytics-config
  namespace: llm-analytics
data:
  NODE_ENV: "production"
  LOG_LEVEL: "info"
  API_PORT: "8080"
  METRICS_PORT: "9090"

  # Database
  DB_HOST: "timescaledb-service"
  DB_PORT: "5432"
  DB_NAME: "analytics"
  DB_POOL_SIZE: "50"

  # Cache
  REDIS_HOST: "redis-service"
  REDIS_PORT: "6379"
  REDIS_CLUSTER_MODE: "true"

  # Message Queue
  KAFKA_BROKERS: "kafka-0.kafka-headless:9092,kafka-1.kafka-headless:9092,kafka-2.kafka-headless:9092"
  KAFKA_CONSUMER_GROUP: "analytics-hub"

  # Integration
  REGISTRY_API_URL: "http://registry-service:8080/api/v1"
  POLICY_ENGINE_API_URL: "http://policy-engine-service:8080/api/v1"
  OBSERVATORY_API_URL: "http://observatory-service:8080/api/v1"
```

#### Secret

```yaml
# k8s/secret.yaml
apiVersion: v1
kind: Secret
metadata:
  name: analytics-secrets
  namespace: llm-analytics
type: Opaque
stringData:
  DB_PASSWORD: "changeme"
  REDIS_PASSWORD: "changeme"
  API_KEY: "changeme"
  JWT_SECRET: "changeme"
```

#### Deployment

```yaml
# k8s/deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: analytics-api
  namespace: llm-analytics
  labels:
    app: analytics-api
    version: v1
spec:
  replicas: 3
  strategy:
    type: RollingUpdate
    rollingUpdate:
      maxSurge: 1
      maxUnavailable: 0
  selector:
    matchLabels:
      app: analytics-api
  template:
    metadata:
      labels:
        app: analytics-api
        version: v1
      annotations:
        prometheus.io/scrape: "true"
        prometheus.io/port: "9090"
        prometheus.io/path: "/metrics"
    spec:
      serviceAccountName: analytics-api

      # Init containers
      initContainers:
      - name: wait-for-db
        image: busybox:1.35
        command: ['sh', '-c', 'until nc -z timescaledb-service 5432; do echo waiting for db; sleep 2; done;']

      # Main container
      containers:
      - name: analytics-api
        image: analytics-hub:1.0.0
        imagePullPolicy: IfNotPresent

        ports:
        - name: http
          containerPort: 8080
          protocol: TCP
        - name: metrics
          containerPort: 9090
          protocol: TCP

        env:
        - name: NODE_ENV
          valueFrom:
            configMapKeyRef:
              name: analytics-config
              key: NODE_ENV

        - name: DB_PASSWORD
          valueFrom:
            secretKeyRef:
              name: analytics-secrets
              key: DB_PASSWORD

        envFrom:
        - configMapRef:
            name: analytics-config

        resources:
          requests:
            cpu: 500m
            memory: 1Gi
          limits:
            cpu: 2000m
            memory: 4Gi

        livenessProbe:
          httpGet:
            path: /health/live
            port: 8080
          initialDelaySeconds: 30
          periodSeconds: 10
          timeoutSeconds: 5
          failureThreshold: 3

        readinessProbe:
          httpGet:
            path: /health/ready
            port: 8080
          initialDelaySeconds: 10
          periodSeconds: 5
          timeoutSeconds: 3
          failureThreshold: 3

        volumeMounts:
        - name: config
          mountPath: /app/config
          readOnly: true
        - name: tmp
          mountPath: /tmp

      volumes:
      - name: config
        configMap:
          name: analytics-config
      - name: tmp
        emptyDir: {}

      # Security
      securityContext:
        runAsNonRoot: true
        runAsUser: 1001
        fsGroup: 1001

      # Affinity
      affinity:
        podAntiAffinity:
          preferredDuringSchedulingIgnoredDuringExecution:
          - weight: 100
            podAffinityTerm:
              labelSelector:
                matchExpressions:
                - key: app
                  operator: In
                  values:
                  - analytics-api
              topologyKey: kubernetes.io/hostname
```

#### Service

```yaml
# k8s/service.yaml
apiVersion: v1
kind: Service
metadata:
  name: analytics-api-service
  namespace: llm-analytics
  labels:
    app: analytics-api
spec:
  type: ClusterIP
  ports:
  - name: http
    port: 8080
    targetPort: 8080
    protocol: TCP
  - name: metrics
    port: 9090
    targetPort: 9090
    protocol: TCP
  selector:
    app: analytics-api
  sessionAffinity: None
```

#### HorizontalPodAutoscaler

```yaml
# k8s/hpa.yaml
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: analytics-api-hpa
  namespace: llm-analytics
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: analytics-api
  minReplicas: 3
  maxReplicas: 20
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 70
  - type: Resource
    resource:
      name: memory
      target:
        type: Utilization
        averageUtilization: 80
  behavior:
    scaleDown:
      stabilizationWindowSeconds: 300
      policies:
      - type: Percent
        value: 50
        periodSeconds: 60
    scaleUp:
      stabilizationWindowSeconds: 0
      policies:
      - type: Percent
        value: 100
        periodSeconds: 30
      - type: Pods
        value: 2
        periodSeconds: 30
      selectPolicy: Max
```

#### Ingress

```yaml
# k8s/ingress.yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: analytics-api-ingress
  namespace: llm-analytics
  annotations:
    kubernetes.io/ingress.class: nginx
    cert-manager.io/cluster-issuer: letsencrypt-prod
    nginx.ingress.kubernetes.io/ssl-redirect: "true"
    nginx.ingress.kubernetes.io/rate-limit: "100"
spec:
  tls:
  - hosts:
    - analytics.llmplatform.io
    secretName: analytics-tls
  rules:
  - host: analytics.llmplatform.io
    http:
      paths:
      - path: /
        pathType: Prefix
        backend:
          service:
            name: analytics-api-service
            port:
              number: 8080
```

#### StatefulSet (for Data Nodes in Distributed Architecture)

```yaml
# k8s/statefulset.yaml
apiVersion: apps/v1
kind: StatefulSet
metadata:
  name: analytics-data-node
  namespace: llm-analytics
spec:
  serviceName: analytics-data-node
  replicas: 10
  selector:
    matchLabels:
      app: analytics-data-node
  template:
    metadata:
      labels:
        app: analytics-data-node
    spec:
      containers:
      - name: data-node
        image: analytics-data-node:1.0.0
        ports:
        - containerPort: 8080
          name: http
        - containerPort: 9090
          name: peer
        volumeMounts:
        - name: data
          mountPath: /var/lib/analytics
        resources:
          requests:
            cpu: 2000m
            memory: 8Gi
          limits:
            cpu: 8000m
            memory: 32Gi
  volumeClaimTemplates:
  - metadata:
      name: data
    spec:
      accessModes: [ "ReadWriteOnce" ]
      storageClassName: fast-ssd
      resources:
        requests:
          storage: 1Ti
```

---

### Helm Charts

#### Chart.yaml

```yaml
# helm/Chart.yaml
apiVersion: v2
name: llm-analytics-hub
description: A Helm chart for LLM Analytics Hub
type: application
version: 1.0.0
appVersion: "1.0.0"

keywords:
  - analytics
  - llm
  - monitoring
  - metrics

maintainers:
  - name: Analytics Team
    email: analytics@llmplatform.io

dependencies:
  - name: postgresql
    version: 12.x.x
    repository: https://charts.bitnami.com/bitnami
    condition: postgresql.enabled

  - name: redis
    version: 17.x.x
    repository: https://charts.bitnami.com/bitnami
    condition: redis.enabled

  - name: kafka
    version: 22.x.x
    repository: https://charts.bitnami.com/bitnami
    condition: kafka.enabled
```

#### values.yaml

```yaml
# helm/values.yaml
# Default values for llm-analytics-hub

## Deployment architecture
## Options: standalone, integrated, distributed
architecture: standalone

## Global settings
global:
  imageRegistry: ""
  imagePullSecrets: []

## Image configuration
image:
  registry: docker.io
  repository: llmplatform/analytics-hub
  tag: "1.0.0"
  pullPolicy: IfNotPresent

## Replica count
replicaCount: 3

## Service account
serviceAccount:
  create: true
  annotations: {}
  name: ""

## Pod annotations
podAnnotations:
  prometheus.io/scrape: "true"
  prometheus.io/port: "9090"

## Pod security context
podSecurityContext:
  runAsNonRoot: true
  runAsUser: 1001
  fsGroup: 1001

## Container security context
securityContext:
  allowPrivilegeEscalation: false
  capabilities:
    drop:
    - ALL
  readOnlyRootFilesystem: true

## Service configuration
service:
  type: ClusterIP
  port: 8080
  metricsPort: 9090

## Ingress configuration
ingress:
  enabled: true
  className: nginx
  annotations:
    cert-manager.io/cluster-issuer: letsencrypt-prod
  hosts:
    - host: analytics.llmplatform.io
      paths:
        - path: /
          pathType: Prefix
  tls:
    - secretName: analytics-tls
      hosts:
        - analytics.llmplatform.io

## Resource limits
resources:
  requests:
    cpu: 500m
    memory: 1Gi
  limits:
    cpu: 2000m
    memory: 4Gi

## Autoscaling
autoscaling:
  enabled: true
  minReplicas: 3
  maxReplicas: 20
  targetCPUUtilizationPercentage: 70
  targetMemoryUtilizationPercentage: 80

## PostgreSQL (TimescaleDB)
postgresql:
  enabled: true
  image:
    registry: docker.io
    repository: timescale/timescaledb
    tag: latest-pg14
  auth:
    username: analytics
    password: changeme
    database: analytics
  primary:
    persistence:
      enabled: true
      size: 500Gi
      storageClass: "fast-ssd"

## Redis
redis:
  enabled: true
  architecture: replication
  auth:
    enabled: true
    password: changeme
  master:
    persistence:
      enabled: true
      size: 8Gi
  replica:
    replicaCount: 3
    persistence:
      enabled: true
      size: 8Gi

## Kafka
kafka:
  enabled: true
  replicaCount: 5
  auth:
    clientProtocol: plaintext
  persistence:
    enabled: true
    size: 100Gi

## Configuration
config:
  logLevel: info
  apiPort: 8080
  metricsPort: 9090

  ## Integration endpoints
  integration:
    registry:
      url: http://registry-service:8080/api/v1
    policyEngine:
      url: http://policy-engine-service:8080/api/v1
    observatory:
      url: http://observatory-service:8080/api/v1

## Monitoring
monitoring:
  enabled: true
  serviceMonitor:
    enabled: true
    interval: 30s
```

#### templates/deployment.yaml

```yaml
# helm/templates/deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: {{ include "analytics-hub.fullname" . }}
  labels:
    {{- include "analytics-hub.labels" . | nindent 4 }}
spec:
  {{- if not .Values.autoscaling.enabled }}
  replicas: {{ .Values.replicaCount }}
  {{- end }}
  strategy:
    type: RollingUpdate
    rollingUpdate:
      maxSurge: 1
      maxUnavailable: 0
  selector:
    matchLabels:
      {{- include "analytics-hub.selectorLabels" . | nindent 6 }}
  template:
    metadata:
      annotations:
        {{- toYaml .Values.podAnnotations | nindent 8 }}
      labels:
        {{- include "analytics-hub.selectorLabels" . | nindent 8 }}
    spec:
      {{- with .Values.global.imagePullSecrets }}
      imagePullSecrets:
        {{- toYaml . | nindent 8 }}
      {{- end }}
      serviceAccountName: {{ include "analytics-hub.serviceAccountName" . }}
      securityContext:
        {{- toYaml .Values.podSecurityContext | nindent 8 }}

      initContainers:
      - name: wait-for-db
        image: busybox:1.35
        command:
        - sh
        - -c
        - |
          until nc -z {{ include "analytics-hub.postgresql.host" . }} {{ .Values.postgresql.service.port }}; do
            echo "Waiting for PostgreSQL..."
            sleep 2
          done

      containers:
      - name: {{ .Chart.Name }}
        securityContext:
          {{- toYaml .Values.securityContext | nindent 12 }}
        image: "{{ .Values.image.registry }}/{{ .Values.image.repository }}:{{ .Values.image.tag | default .Chart.AppVersion }}"
        imagePullPolicy: {{ .Values.image.pullPolicy }}

        ports:
        - name: http
          containerPort: {{ .Values.config.apiPort }}
          protocol: TCP
        - name: metrics
          containerPort: {{ .Values.config.metricsPort }}
          protocol: TCP

        env:
        - name: DB_HOST
          value: {{ include "analytics-hub.postgresql.host" . }}
        - name: DB_PASSWORD
          valueFrom:
            secretKeyRef:
              name: {{ include "analytics-hub.fullname" . }}
              key: db-password

        livenessProbe:
          httpGet:
            path: /health/live
            port: http
          initialDelaySeconds: 30
          periodSeconds: 10

        readinessProbe:
          httpGet:
            path: /health/ready
            port: http
          initialDelaySeconds: 10
          periodSeconds: 5

        resources:
          {{- toYaml .Values.resources | nindent 12 }}
```

---

## Operational Patterns

### Health Check Endpoints

```typescript
// src/health/health-controller.ts
import { Request, Response } from 'express';

export class HealthController {
  /**
   * Liveness probe - Is the service running?
   * Returns 200 if the process is alive
   */
  async liveness(req: Request, res: Response): Promise<void> {
    res.status(200).json({
      status: 'UP',
      timestamp: new Date().toISOString()
    });
  }

  /**
   * Readiness probe - Is the service ready to accept traffic?
   * Checks all dependencies
   */
  async readiness(req: Request, res: Response): Promise<void> {
    const checks = await Promise.allSettled([
      this.checkDatabase(),
      this.checkCache(),
      this.checkMessageQueue(),
      this.checkExternalAPIs()
    ]);

    const allHealthy = checks.every(c => c.status === 'fulfilled' && c.value);

    if (allHealthy) {
      res.status(200).json({
        status: 'READY',
        timestamp: new Date().toISOString(),
        checks: {
          database: 'UP',
          cache: 'UP',
          messageQueue: 'UP',
          externalAPIs: 'UP'
        }
      });
    } else {
      res.status(503).json({
        status: 'NOT_READY',
        timestamp: new Date().toISOString(),
        checks: {
          database: checks[0].status === 'fulfilled' ? 'UP' : 'DOWN',
          cache: checks[1].status === 'fulfilled' ? 'UP' : 'DOWN',
          messageQueue: checks[2].status === 'fulfilled' ? 'UP' : 'DOWN',
          externalAPIs: checks[3].status === 'fulfilled' ? 'UP' : 'DOWN'
        }
      });
    }
  }

  /**
   * Startup probe - Has the service completed initialization?
   */
  async startup(req: Request, res: Response): Promise<void> {
    if (this.isInitialized()) {
      res.status(200).json({
        status: 'STARTED',
        timestamp: new Date().toISOString()
      });
    } else {
      res.status(503).json({
        status: 'STARTING',
        timestamp: new Date().toISOString()
      });
    }
  }

  private async checkDatabase(): Promise<boolean> {
    // Check database connectivity
    return true;
  }

  private async checkCache(): Promise<boolean> {
    // Check cache connectivity
    return true;
  }

  private async checkMessageQueue(): Promise<boolean> {
    // Check message queue connectivity
    return true;
  }

  private async checkExternalAPIs(): Promise<boolean> {
    // Check external API connectivity
    return true;
  }

  private isInitialized(): boolean {
    // Check if service initialization is complete
    return true;
  }
}
```

### Graceful Shutdown

```typescript
// src/server.ts
import { Server } from 'http';
import express from 'express';

export class GracefulShutdownHandler {
  private server: Server;
  private isShuttingDown = false;
  private activeConnections = new Set<any>();

  constructor(server: Server) {
    this.server = server;
    this.setupShutdownHandlers();
  }

  private setupShutdownHandlers(): void {
    // Handle termination signals
    process.on('SIGTERM', () => this.shutdown('SIGTERM'));
    process.on('SIGINT', () => this.shutdown('SIGINT'));

    // Track active connections
    this.server.on('connection', (conn) => {
      this.activeConnections.add(conn);
      conn.on('close', () => {
        this.activeConnections.delete(conn);
      });
    });
  }

  private async shutdown(signal: string): Promise<void> {
    if (this.isShuttingDown) {
      return;
    }

    console.log(`Received ${signal}, starting graceful shutdown...`);
    this.isShuttingDown = true;

    // 1. Stop accepting new requests
    this.server.close(() => {
      console.log('HTTP server closed');
    });

    // 2. Wait for active requests to complete (with timeout)
    await this.waitForActiveRequests(30000); // 30 seconds

    // 3. Close database connections
    await this.closeDatabaseConnections();

    // 4. Close cache connections
    await this.closeCacheConnections();

    // 5. Close message queue connections
    await this.closeMessageQueueConnections();

    // 6. Flush any pending logs
    await this.flushLogs();

    console.log('Graceful shutdown complete');
    process.exit(0);
  }

  private async waitForActiveRequests(timeout: number): Promise<void> {
    const start = Date.now();

    while (this.activeConnections.size > 0) {
      if (Date.now() - start > timeout) {
        console.warn(`Shutdown timeout reached, forcefully closing ${this.activeConnections.size} connections`);
        this.activeConnections.forEach(conn => conn.destroy());
        break;
      }

      await this.sleep(100);
    }
  }

  private async closeDatabaseConnections(): Promise<void> {
    console.log('Closing database connections...');
    // Implementation
  }

  private async closeCacheConnections(): Promise<void> {
    console.log('Closing cache connections...');
    // Implementation
  }

  private async closeMessageQueueConnections(): Promise<void> {
    console.log('Closing message queue connections...');
    // Implementation
  }

  private async flushLogs(): Promise<void> {
    console.log('Flushing logs...');
    // Implementation
  }

  private sleep(ms: number): Promise<void> {
    return new Promise(resolve => setTimeout(resolve, ms));
  }
}
```

### Rolling Update Strategy

**Zero-Downtime Deployment Process:**

```
1. Pre-deployment validation
   - Run health checks on current deployment
   - Verify new version is available
   - Check resource availability

2. Start new pods (25% at a time)
   - Deploy new version pods
   - Wait for readiness probes to pass
   - Verify new pods are healthy

3. Shift traffic gradually
   - Route 10% traffic to new pods
   - Monitor error rates and latency
   - If healthy, increase to 25%, 50%, 75%, 100%
   - If issues detected, rollback immediately

4. Terminate old pods
   - Send SIGTERM to old pods
   - Wait for graceful shutdown (30s max)
   - Force kill if timeout exceeded

5. Post-deployment validation
   - Verify all new pods are healthy
   - Check application metrics
   - Monitor for errors
```

**Kubernetes Rolling Update Configuration:**

```yaml
spec:
  strategy:
    type: RollingUpdate
    rollingUpdate:
      maxSurge: 1        # Max pods above desired count
      maxUnavailable: 0  # Min pods always available

  minReadySeconds: 10   # Wait 10s after ready before continuing

  progressDeadlineSeconds: 600  # Rollout must complete in 10 min
```

### Blue-Green Deployment

**Process:**

```
BLUE Environment (Current Production)
  |
  v
[Deploy GREEN Environment]
  |
  v
[Run Tests on GREEN]
  |
  v
{Tests Pass?}
  |
  +--[NO]---> [Destroy GREEN] --> [Alert Team]
  |
  +--[YES]--> [Switch Traffic to GREEN]
  |              |
  |              v
  |           [Monitor GREEN]
  |              |
  |              v
  |           {Healthy?}
  |              |
  |              +--[NO]---> [Rollback to BLUE]
  |              |
  |              +--[YES]--> [Destroy BLUE]
  |                            |
  |                            v
  |                         [Complete]
  v
GREEN becomes new Production
```

**Implementation:**

```yaml
# Blue environment
apiVersion: v1
kind: Service
metadata:
  name: analytics-api
spec:
  selector:
    app: analytics-api
    version: blue
  ports:
  - port: 8080

---
# Green deployment
apiVersion: apps/v1
kind: Deployment
metadata:
  name: analytics-api-green
spec:
  replicas: 3
  selector:
    matchLabels:
      app: analytics-api
      version: green
  template:
    metadata:
      labels:
        app: analytics-api
        version: green
    spec:
      containers:
      - name: analytics-api
        image: analytics-hub:2.0.0

---
# Switch traffic by updating service selector
# kubectl patch service analytics-api -p '{"spec":{"selector":{"version":"green"}}}'
```

### Canary Deployment

**Process:**

```
[Deploy Canary Version]
  |
  v
[Route 5% Traffic to Canary]
  |
  v
[Monitor Metrics] --> {Healthy?}
  |                       |
  |                       +--[NO]---> [Rollback]
  |                       |
  |                       +--[YES]--> [Increase to 10%]
  |
  v
[Continue Gradual Increase: 25%, 50%, 75%, 100%]
  |
  v
[Replace Stable with Canary]
```

**Istio Configuration:**

```yaml
apiVersion: networking.istio.io/v1beta1
kind: VirtualService
metadata:
  name: analytics-api
spec:
  hosts:
  - analytics-api
  http:
  - match:
    - headers:
        canary:
          exact: "true"
    route:
    - destination:
        host: analytics-api
        subset: canary
      weight: 100
  - route:
    - destination:
        host: analytics-api
        subset: stable
      weight: 95
    - destination:
        host: analytics-api
        subset: canary
      weight: 5

---
apiVersion: networking.istio.io/v1beta1
kind: DestinationRule
metadata:
  name: analytics-api
spec:
  host: analytics-api
  subsets:
  - name: stable
    labels:
      version: v1
  - name: canary
    labels:
      version: v2
```

**Flagger Canary Configuration:**

```yaml
apiVersion: flagger.app/v1beta1
kind: Canary
metadata:
  name: analytics-api
spec:
  targetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: analytics-api

  service:
    port: 8080

  analysis:
    interval: 1m
    threshold: 5
    maxWeight: 50
    stepWeight: 10

    metrics:
    - name: request-success-rate
      thresholdRange:
        min: 99
      interval: 1m

    - name: request-duration
      thresholdRange:
        max: 500
      interval: 1m

    webhooks:
    - name: load-test
      url: http://flagger-loadtester/
      timeout: 5s
      metadata:
        cmd: "hey -z 1m -q 10 -c 2 http://analytics-api-canary:8080/"
```

---

## Deployment Decision Trees

### Decision Tree 1: Choosing Deployment Architecture

```
START: What is your deployment scale?

├─[Small (<1000 req/s)]
│  │
│  └─ What are your operational constraints?
│     │
│     ├─[Limited ops team]
│     │  └─ **CHOOSE: Integrated Platform Module**
│     │     - Minimal operational overhead
│     │     - Shared infrastructure
│     │     - Simplified deployment
│     │
│     └─[Dedicated analytics team]
│        └─ **CHOOSE: Standalone Analytics Service**
│           - Better isolation
│           - Independent scaling
│           - Custom optimizations

├─[Medium (1000-10000 req/s)]
│  │
│  └─ What is your availability requirement?
│     │
│     ├─[99.9% (3 nines)]
│     │  └─ **CHOOSE: Standalone Analytics Service**
│     │     - Independent scaling
│     │     - Better fault isolation
│     │     - Multi-AZ deployment
│     │
│     └─[99.99% (4 nines)]
│        └─ **CHOOSE: Distributed Data Node Cluster**
│           - High availability
│           - Automatic failover
│           - Multi-region support

└─[Large (>10000 req/s)]
   │
   └─ **CHOOSE: Distributed Data Node Cluster**
      - Horizontal scalability
      - Geographic distribution
      - Sharded data storage
      - High throughput
```

### Decision Tree 2: Choosing Database Technology

```
START: What is your primary query pattern?

├─[Time-series analytics]
│  │
│  └─ What is your data retention period?
│     │
│     ├─[<90 days]
│     │  └─ **CHOOSE: TimescaleDB**
│     │     - Excellent time-series support
│     │     - PostgreSQL compatibility
│     │     - Good for structured data
│     │
│     └─[>90 days]
│        └─ What is your query latency requirement?
│           │
│           ├─[<100ms]
│           │  └─ **CHOOSE: ClickHouse**
│           │     - Ultra-fast queries
│           │     - Excellent compression
│           │     - Column-oriented storage
│           │
│           └─[<1s acceptable]
│              └─ **CHOOSE: TimescaleDB with archival**
│                 - Hot data in TimescaleDB
│                 - Cold data in S3/Glacier
│                 - Query federation

├─[Ad-hoc analytics]
│  │
│  └─ **CHOOSE: ClickHouse**
│     - OLAP workloads
│     - Complex aggregations
│     - Fast scan performance

└─[Mixed workloads]
   │
   └─ **CHOOSE: Multi-store approach**
      - TimescaleDB for real-time metrics
      - ClickHouse for historical analytics
      - Redis for hot data cache
```

### Decision Tree 3: Choosing Deployment Strategy

```
START: What is your rollback time requirement?

├─[Instant rollback needed]
│  │
│  └─ What is your acceptable downtime?
│     │
│     ├─[Zero downtime]
│     │  └─ **CHOOSE: Blue-Green Deployment**
│     │     - Instant rollback
│     │     - No downtime
│     │     - Higher resource cost
│     │
│     └─[Brief downtime acceptable]
│        └─ **CHOOSE: Rolling Update**
│           - Gradual rollout
│           - Resource efficient
│           - Quick rollback

├─[Gradual validation needed]
│  │
│  └─ What percentage of users can test?
│     │
│     ├─[Small subset (1-10%)]
│     │  └─ **CHOOSE: Canary Deployment**
│     │     - Gradual rollout
│     │     - Risk mitigation
│     │     - A/B testing capability
│     │
│     └─[Specific user groups]
│        └─ **CHOOSE: Feature Flags + Canary**
│           - Targeted rollout
│           - Fine-grained control
│           - Easy rollback per feature

└─[Testing in production]
   │
   └─ **CHOOSE: Shadow Deployment**
      - Test with production traffic
      - No impact on users
      - Validate performance
```

---

## Cloud Provider Configurations

### AWS Deployment

#### Architecture on AWS

```
[Route 53]
    |
    v
[Application Load Balancer]
    |
    v
[EKS Cluster]
    |
    +-- [Ingress Controller]
    |       |
    |       v
    +-- [Analytics Pods]
    |       |
    |       v
    +-- [RDS Aurora PostgreSQL (TimescaleDB)]
    |
    +-- [ElastiCache Redis Cluster]
    |
    +-- [MSK (Managed Kafka)]
    |
    +-- [S3] (Long-term storage)
    |
    +-- [CloudWatch] (Monitoring)
```

#### Terraform Configuration

```hcl
# terraform/aws/main.tf

# EKS Cluster
module "eks" {
  source = "terraform-aws-modules/eks/aws"

  cluster_name    = "analytics-cluster"
  cluster_version = "1.27"

  vpc_id     = module.vpc.vpc_id
  subnet_ids = module.vpc.private_subnets

  eks_managed_node_groups = {
    analytics = {
      min_size     = 3
      max_size     = 20
      desired_size = 5

      instance_types = ["m5.2xlarge"]
      capacity_type  = "ON_DEMAND"

      labels = {
        workload = "analytics"
      }

      taints = []
    }
  }
}

# RDS Aurora (TimescaleDB)
resource "aws_rds_cluster" "analytics" {
  cluster_identifier      = "analytics-cluster"
  engine                  = "aurora-postgresql"
  engine_version          = "14.6"
  database_name           = "analytics"
  master_username         = "admin"
  master_password         = random_password.db_password.result

  backup_retention_period = 30
  preferred_backup_window = "03:00-04:00"

  vpc_security_group_ids = [aws_security_group.rds.id]
  db_subnet_group_name   = aws_db_subnet_group.analytics.name

  enabled_cloudwatch_logs_exports = ["postgresql"]

  tags = {
    Environment = "production"
    Service     = "analytics"
  }
}

resource "aws_rds_cluster_instance" "analytics" {
  count              = 3
  identifier         = "analytics-instance-${count.index}"
  cluster_identifier = aws_rds_cluster.analytics.id
  instance_class     = "db.r6g.2xlarge"
  engine             = aws_rds_cluster.analytics.engine
  engine_version     = aws_rds_cluster.analytics.engine_version
}

# ElastiCache Redis
resource "aws_elasticache_replication_group" "analytics" {
  replication_group_id       = "analytics-cache"
  replication_group_description = "Analytics cache cluster"

  engine                     = "redis"
  engine_version             = "7.0"
  node_type                  = "cache.r6g.xlarge"
  num_cache_clusters         = 6
  parameter_group_name       = "default.redis7.cluster.on"

  subnet_group_name          = aws_elasticache_subnet_group.analytics.name
  security_group_ids         = [aws_security_group.redis.id]

  automatic_failover_enabled = true
  multi_az_enabled          = true

  at_rest_encryption_enabled = true
  transit_encryption_enabled = true

  tags = {
    Environment = "production"
    Service     = "analytics"
  }
}

# MSK (Kafka)
resource "aws_msk_cluster" "analytics" {
  cluster_name           = "analytics-events"
  kafka_version          = "3.4.0"
  number_of_broker_nodes = 6

  broker_node_group_info {
    instance_type   = "kafka.m5.2xlarge"
    client_subnets  = module.vpc.private_subnets
    security_groups = [aws_security_group.msk.id]

    storage_info {
      ebs_storage_info {
        volume_size = 1000
      }
    }
  }

  encryption_info {
    encryption_in_transit {
      client_broker = "TLS"
      in_cluster    = true
    }
  }

  logging_info {
    broker_logs {
      cloudwatch_logs {
        enabled   = true
        log_group = aws_cloudwatch_log_group.msk.name
      }
    }
  }

  tags = {
    Environment = "production"
    Service     = "analytics"
  }
}

# S3 for long-term storage
resource "aws_s3_bucket" "analytics_archive" {
  bucket = "analytics-archive-${data.aws_caller_identity.current.account_id}"

  tags = {
    Environment = "production"
    Service     = "analytics"
  }
}

resource "aws_s3_bucket_lifecycle_configuration" "analytics_archive" {
  bucket = aws_s3_bucket.analytics_archive.id

  rule {
    id     = "archive-old-data"
    status = "Enabled"

    transition {
      days          = 90
      storage_class = "GLACIER"
    }

    expiration {
      days = 2555  # 7 years
    }
  }
}
```

---

### GCP Deployment

#### Architecture on GCP

```
[Cloud DNS]
    |
    v
[Cloud Load Balancing]
    |
    v
[GKE Cluster]
    |
    +-- [Ingress Controller]
    |       |
    |       v
    +-- [Analytics Pods]
    |       |
    |       v
    +-- [Cloud SQL (PostgreSQL)]
    |
    +-- [Memorystore (Redis)]
    |
    +-- [Pub/Sub] (Event streaming)
    |
    +-- [Cloud Storage] (Long-term storage)
    |
    +-- [Cloud Monitoring]
```

#### Terraform Configuration

```hcl
# terraform/gcp/main.tf

# GKE Cluster
resource "google_container_cluster" "analytics" {
  name     = "analytics-cluster"
  location = var.region

  remove_default_node_pool = true
  initial_node_count       = 1

  network    = google_compute_network.analytics.name
  subnetwork = google_compute_subnetwork.analytics.name

  release_channel {
    channel = "REGULAR"
  }

  workload_identity_config {
    workload_pool = "${var.project_id}.svc.id.goog"
  }
}

resource "google_container_node_pool" "analytics" {
  name       = "analytics-pool"
  location   = var.region
  cluster    = google_container_cluster.analytics.name
  node_count = 3

  autoscaling {
    min_node_count = 3
    max_node_count = 20
  }

  node_config {
    machine_type = "n2-standard-8"

    oauth_scopes = [
      "https://www.googleapis.com/auth/cloud-platform"
    ]

    workload_metadata_config {
      mode = "GKE_METADATA"
    }
  }
}

# Cloud SQL (PostgreSQL with TimescaleDB)
resource "google_sql_database_instance" "analytics" {
  name             = "analytics-db"
  database_version = "POSTGRES_14"
  region           = var.region

  settings {
    tier = "db-custom-8-32768"  # 8 vCPU, 32GB RAM

    availability_type = "REGIONAL"

    backup_configuration {
      enabled                        = true
      start_time                     = "03:00"
      point_in_time_recovery_enabled = true
      transaction_log_retention_days = 7
      backup_retention_settings {
        retained_backups = 30
      }
    }

    ip_configuration {
      ipv4_enabled    = false
      private_network = google_compute_network.analytics.id
    }

    database_flags {
      name  = "shared_preload_libraries"
      value = "timescaledb"
    }
  }
}

# Memorystore (Redis)
resource "google_redis_instance" "analytics" {
  name           = "analytics-cache"
  tier           = "STANDARD_HA"
  memory_size_gb = 32
  region         = var.region

  redis_version = "REDIS_7_0"

  authorized_network = google_compute_network.analytics.id

  redis_configs = {
    maxmemory-policy = "allkeys-lru"
  }
}

# Pub/Sub Topics
resource "google_pubsub_topic" "analytics_events" {
  name = "analytics-events"

  message_retention_duration = "604800s"  # 7 days
}

resource "google_pubsub_subscription" "analytics_events" {
  name  = "analytics-events-sub"
  topic = google_pubsub_topic.analytics_events.name

  ack_deadline_seconds = 20

  retry_policy {
    minimum_backoff = "10s"
    maximum_backoff = "600s"
  }
}

# Cloud Storage
resource "google_storage_bucket" "analytics_archive" {
  name     = "analytics-archive-${var.project_id}"
  location = var.region

  lifecycle_rule {
    condition {
      age = 90
    }
    action {
      type          = "SetStorageClass"
      storage_class = "NEARLINE"
    }
  }

  lifecycle_rule {
    condition {
      age = 365
    }
    action {
      type          = "SetStorageClass"
      storage_class = "COLDLINE"
    }
  }
}
```

---

### Azure Deployment

#### Architecture on Azure

```
[Azure DNS]
    |
    v
[Application Gateway]
    |
    v
[AKS Cluster]
    |
    +-- [Ingress Controller]
    |       |
    |       v
    +-- [Analytics Pods]
    |       |
    |       v
    +-- [Azure Database for PostgreSQL]
    |
    +-- [Azure Cache for Redis]
    |
    +-- [Event Hubs] (Kafka-compatible)
    |
    +-- [Blob Storage] (Long-term storage)
    |
    +-- [Azure Monitor]
```

#### Terraform Configuration

```hcl
# terraform/azure/main.tf

# AKS Cluster
resource "azurerm_kubernetes_cluster" "analytics" {
  name                = "analytics-cluster"
  location            = azurerm_resource_group.analytics.location
  resource_group_name = azurerm_resource_group.analytics.name
  dns_prefix          = "analytics"

  default_node_pool {
    name       = "default"
    node_count = 3
    vm_size    = "Standard_D8s_v3"

    enable_auto_scaling = true
    min_count          = 3
    max_count          = 20
  }

  identity {
    type = "SystemAssigned"
  }

  network_profile {
    network_plugin    = "azure"
    load_balancer_sku = "standard"
  }
}

# Azure Database for PostgreSQL
resource "azurerm_postgresql_flexible_server" "analytics" {
  name                   = "analytics-db"
  resource_group_name    = azurerm_resource_group.analytics.name
  location               = azurerm_resource_group.analytics.location
  version                = "14"
  administrator_login    = "adminuser"
  administrator_password = random_password.db_password.result

  storage_mb = 524288  # 512 GB

  sku_name   = "GP_Standard_D8s_v3"

  backup_retention_days        = 30
  geo_redundant_backup_enabled = true

  high_availability {
    mode                      = "ZoneRedundant"
    standby_availability_zone = "2"
  }
}

# Azure Cache for Redis
resource "azurerm_redis_cache" "analytics" {
  name                = "analytics-cache"
  location            = azurerm_resource_group.analytics.location
  resource_group_name = azurerm_resource_group.analytics.name
  capacity            = 3
  family              = "P"
  sku_name            = "Premium"

  enable_non_ssl_port = false
  minimum_tls_version = "1.2"

  redis_configuration {
    maxmemory_policy = "allkeys-lru"
  }

  zone = "1"
}

# Event Hubs (Kafka-compatible)
resource "azurerm_eventhub_namespace" "analytics" {
  name                = "analytics-events"
  location            = azurerm_resource_group.analytics.location
  resource_group_name = azurerm_resource_group.analytics.name
  sku                 = "Standard"
  capacity            = 5

  kafka_enabled = true
}

resource "azurerm_eventhub" "analytics_events" {
  name                = "analytics-events"
  namespace_name      = azurerm_eventhub_namespace.analytics.name
  resource_group_name = azurerm_resource_group.analytics.name
  partition_count     = 32
  message_retention   = 7
}

# Blob Storage
resource "azurerm_storage_account" "analytics" {
  name                     = "analyticsarchive${random_string.suffix.result}"
  resource_group_name      = azurerm_resource_group.analytics.name
  location                 = azurerm_resource_group.analytics.location
  account_tier             = "Standard"
  account_replication_type = "GRS"

  blob_properties {
    versioning_enabled = true

    delete_retention_policy {
      days = 30
    }
  }
}

resource "azurerm_storage_management_policy" "analytics" {
  storage_account_id = azurerm_storage_account.analytics.id

  rule {
    name    = "archive-old-data"
    enabled = true

    filters {
      blob_types = ["blockBlob"]
    }

    actions {
      base_blob {
        tier_to_cool_after_days_since_modification_greater_than    = 30
        tier_to_archive_after_days_since_modification_greater_than = 90
        delete_after_days_since_modification_greater_than          = 2555
      }
    }
  }
}
```

---

## On-Premise Deployment

### Hardware Requirements

#### Minimum Configuration (Small Deployment)

**Control Plane:**
- 3x servers (high availability)
- 8 cores CPU
- 32 GB RAM
- 500 GB SSD

**Worker Nodes:**
- 5x servers
- 16 cores CPU
- 64 GB RAM
- 2 TB SSD

**Database:**
- 3x servers (replication)
- 16 cores CPU
- 128 GB RAM
- 4 TB NVMe SSD (RAID 10)

**Total:** 11 servers

#### Recommended Configuration (Medium Deployment)

**Control Plane:**
- 5x servers
- 16 cores CPU
- 64 GB RAM
- 1 TB SSD

**Worker Nodes:**
- 15x servers
- 32 cores CPU
- 128 GB RAM
- 4 TB NVMe SSD

**Database:**
- 5x servers
- 32 cores CPU
- 256 GB RAM
- 8 TB NVMe SSD (RAID 10)

**Total:** 25 servers

### Installation Guide

#### 1. Prerequisites

```bash
# Update all systems
sudo apt update && sudo apt upgrade -y

# Install required packages
sudo apt install -y \
  apt-transport-https \
  ca-certificates \
  curl \
  gnupg \
  lsb-release \
  ntp

# Configure NTP for time synchronization
sudo systemctl enable ntp
sudo systemctl start ntp
```

#### 2. Kubernetes Installation (kubeadm)

```bash
# Install containerd
cat <<EOF | sudo tee /etc/modules-load.d/containerd.conf
overlay
br_netfilter
EOF

sudo modprobe overlay
sudo modprobe br_netfilter

# Install kubectl, kubeadm, kubelet
curl -s https://packages.cloud.google.com/apt/doc/apt-key.gpg | sudo apt-key add -
cat <<EOF | sudo tee /etc/apt/sources.list.d/kubernetes.list
deb https://apt.kubernetes.io/ kubernetes-xenial main
EOF

sudo apt update
sudo apt install -y kubelet kubeadm kubectl
sudo apt-mark hold kubelet kubeadm kubectl

# Initialize control plane (on first master node)
sudo kubeadm init \
  --control-plane-endpoint="LOAD_BALANCER_DNS:6443" \
  --upload-certs \
  --pod-network-cidr=10.244.0.0/16

# Install CNI (Calico)
kubectl apply -f https://docs.projectcalico.org/manifests/calico.yaml

# Join other control plane nodes
sudo kubeadm join LOAD_BALANCER_DNS:6443 \
  --token <token> \
  --discovery-token-ca-cert-hash sha256:<hash> \
  --control-plane \
  --certificate-key <cert-key>

# Join worker nodes
sudo kubeadm join LOAD_BALANCER_DNS:6443 \
  --token <token> \
  --discovery-token-ca-cert-hash sha256:<hash>
```

#### 3. Storage Configuration

```bash
# Install Rook-Ceph for distributed storage
kubectl create -f https://raw.githubusercontent.com/rook/rook/release-1.11/deploy/examples/crds.yaml
kubectl create -f https://raw.githubusercontent.com/rook/rook/release-1.11/deploy/examples/operator.yaml
kubectl create -f https://raw.githubusercontent.com/rook/rook/release-1.11/deploy/examples/cluster.yaml

# Create storage classes
cat <<EOF | kubectl apply -f -
apiVersion: storage.k8s.io/v1
kind: StorageClass
metadata:
  name: fast-ssd
provisioner: rook-ceph.rbd.csi.ceph.com
parameters:
  clusterID: rook-ceph
  pool: replicapool
  imageFormat: "2"
  imageFeatures: layering
allowVolumeExpansion: true
reclaimPolicy: Retain
EOF
```

#### 4. Database Installation

```bash
# Install TimescaleDB using Helm
helm repo add timescale https://charts.timescale.com/
helm install timescaledb timescale/timescaledb-single \
  --set replicaCount=3 \
  --set persistentVolumes.data.size=2Ti \
  --set persistentVolumes.wal.size=500Gi \
  --set resources.requests.memory=128Gi \
  --set resources.requests.cpu=16
```

#### 5. Deploy Analytics Hub

```bash
# Add Helm repository
helm repo add analytics https://charts.analytics.local/

# Install Analytics Hub
helm install analytics-hub analytics/llm-analytics-hub \
  --namespace llm-analytics \
  --create-namespace \
  --values values-onprem.yaml
```

#### 6. Configure Networking

```yaml
# values-onprem.yaml
ingress:
  enabled: true
  className: nginx
  hosts:
    - host: analytics.company.local
      paths:
        - path: /
          pathType: Prefix
  tls:
    - secretName: analytics-tls
      hosts:
        - analytics.company.local

# Install NGINX Ingress Controller
helm install ingress-nginx ingress-nginx/ingress-nginx \
  --namespace ingress-nginx \
  --create-namespace \
  --set controller.service.type=LoadBalancer \
  --set controller.service.externalIPs[0]=<EXTERNAL_IP>
```

#### 7. Monitoring Setup

```bash
# Install Prometheus and Grafana
helm repo add prometheus-community https://prometheus-community.github.io/helm-charts
helm install prometheus prometheus-community/kube-prometheus-stack \
  --namespace monitoring \
  --create-namespace
```

---

## Summary

This comprehensive deployment guide provides:

1. **Three Deployment Architectures**: Standalone, Integrated, and Distributed, with detailed pros/cons and use cases
2. **Complete Deployment Specifications**: Docker, Kubernetes manifests, Helm charts, and operators
3. **Operational Patterns**: Health checks, graceful shutdown, rolling updates, blue-green, and canary deployments
4. **Decision Trees**: Guides for choosing the right architecture, database, and deployment strategy
5. **Cloud Provider Configurations**: Production-ready Terraform configurations for AWS, GCP, and Azure
6. **On-Premise Deployment**: Complete hardware specs and installation guide for on-premise deployments

These specifications enable deploying LLM-Analytics-Hub in any environment with appropriate scalability, reliability, and operational efficiency.
