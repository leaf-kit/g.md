---
title: Architecture Overview
tags: [architecture, system-design]
author: mink
---
# Architecture Overview

## System Components

The system follows a **microservices architecture** with the following components:

1. **API Gateway** — handles routing and authentication
2. **User Service** — manages user data and profiles
3. **Notification Service** — sends emails and push notifications
4. **Database Layer** — PostgreSQL with Redis caching

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Frontend | React + TypeScript |
| Backend | Rust (Actix-web) |
| Database | PostgreSQL 15 |
| Cache | Redis 7 |
| Queue | RabbitMQ |
| Deploy | Kubernetes |

## Data Flow

```
Client → API Gateway → Service → Database
                    ↓
              Message Queue → Notification Service
```

## Performance Requirements

- API latency < **50ms** p99
- Throughput: **10,000 requests/sec**
- Uptime: **99.99%**

> The architecture is designed for horizontal scaling.

## Related Documents

- [API Guide](api-guide.md)
- [Deployment Guide](deployment.md)
- [[Sprint Backlog]]

![System Diagram](../images/architecture.png)

#architecture #system-design #microservices
