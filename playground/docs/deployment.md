---
title: Deployment Guide
tags: [devops, kubernetes, deployment]
author:
---
# Deployment Guide

## Prerequisites

- Docker 24+
- kubectl configured
- Helm 3.x

## Quick Deploy

```bash
# Build the container
docker build -t myapp:latest .

# Deploy to staging
kubectl apply -f k8s/staging/

# Verify pods are running
kubectl get pods -n staging
```

## Environment Variables

```yaml
env:
  - name: DATABASE_URL
    value: postgres://user:pass@db:5432/myapp
  - name: REDIS_URL
    value: redis://cache:6379
  - name: LOG_LEVEL
    value: info
```

## Rollback

```bash
# Check deployment history
kubectl rollout history deployment/myapp

# Rollback to previous version
kubectl rollout undo deployment/myapp
```

> **Important**: Always run database migrations before deploying new code.

- [ ] Update Kubernetes manifests for v2.2
- [ ] Configure auto-scaling rules
- [ ] Set up monitoring dashboards
- [x] Write deployment documentation
- [x] Create staging environment

#devops #kubernetes #deployment
