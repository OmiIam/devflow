# DevFlow API Reference

Base URL (development): `http://localhost:8000`

## Health

### GET /health
Checks server liveness.

**Response 200**
```json
{
  "status": "ok",
  "timestamp": "2026-02-15T11:30:00Z",
  "version": "0.1.0"
}
```

## Auth

### POST /auth/register
Create a new user and receive a JWT.

**Request**
```json
{
  "name": "First User",
  "email": "user@example.com",
  "password": "SuperSecure123!"
}
```

**Response 200**
```json
{
  "user": {
    "id": "uuid",
    "name": "First User",
    "email": "user@example.com"
  },
  "token": "<jwt>"
}
```

### POST /auth/login
Authenticate existing user.

**Request**
```json
{
  "email": "user@example.com",
  "password": "SuperSecure123!"
}
```

**Response 200** – same shape as register.

### GET /auth/me
Return the authenticated user.

Headers: `Authorization: Bearer <token>`.

**Response 200**
```json
{
  "user": {
    "id": "uuid",
    "name": "First User",
    "email": "user@example.com"
  }
}
```

### POST /auth/logout
Stateless acknowledgement that the token is no longer used (clients should delete it).

Headers: `Authorization: Bearer <token>`.

**Response 200**
```json
{ "message": "success" }
```

## Focus

### POST /focus/score
Calculates a focus score from session data.

**Request**
```json
{
  "duration_minutes": 25,
  "interruptions": 1,
  "state": "completed"
}
```

**Response 200**
```json
{ "score": 0.73 }
```

**Response 422** – invalid duration.
```json
{ "error": "duration_minutes must be positive" }
```
