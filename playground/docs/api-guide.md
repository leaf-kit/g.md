---
title: API Guide
tags: [api, rest, documentation]
author: mink
version: 2.1.0
---
# API Guide

## Authentication

All API requests require a **Bearer token** in the `Authorization` header.

```bash
curl -H "Authorization: Bearer YOUR_TOKEN" https://api.example.com/v1/users
```

## Endpoints

### GET /users

Returns a list of users. Supports **pagination** and **filtering**.

```json
{
  "data": [
    { "id": 1, "name": "Alice", "role": "admin" },
    { "id": 2, "name": "Bob", "role": "user" }
  ],
  "total": 42,
  "page": 1
}
```

### POST /users

Creates a new user. Requires `name` and `email` fields.

```python
import requests

response = requests.post(
    "https://api.example.com/v1/users",
    json={"name": "Charlie", "email": "charlie@example.com"},
    headers={"Authorization": "Bearer YOUR_TOKEN"}
)
print(response.status_code)  # 201
```

### DELETE /users/:id

Deletes a user by ID. Returns `204 No Content` on success.

> **Warning**: This action is irreversible. Use with caution.

## Error Handling

All errors return a standard JSON format:

```json
{
  "error": {
    "code": "NOT_FOUND",
    "message": "User with ID 999 not found"
  }
}
```

## Rate Limiting

- 100 requests per minute for free tier
- 1000 requests per minute for **pro** accounts

See [[Architecture Overview]] for system design details.

#api #rest #documentation
