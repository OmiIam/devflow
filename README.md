<p align="center">
  <strong style="font-size: 1.6em;">DevFlow</strong><br/>
  <em>Task management that understands how developers actually work</em>
</p>

<p align="center">
  <a href="https://github.com/OmiIam/devflow/issues">Issues</a> •
  <a href="https://github.com/OmiIam/devflow/projects">Project Board</a> •
  <a href="./docs/architecture/system-overview.md">Architecture</a> •
  <a href="./docs/decisions">Architectural Decisions</a> •
  <a href="./docs/roadmap.md">Roadmap</a>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/status-early%20architecture-blue" />
  <img src="https://img.shields.io/badge/frontend-authoritative-informational" />
  <img src="https://img.shields.io/badge/license-MIT-green" />
</p>

---

## Overview

DevFlow is a developer-focused task manager designed specifically for how developers *actually* work.

Most task managers almost always treat tasks as static items.  
DevFlow treats **focus, interruption, and context switching** as first class concepts.

This project is intentionally **frontend first**, architecture driven, and scope controlled.  
The goal is not to build “another todo app”, but to explore a more honest productivity model grounded in observable developer behavior.

---

## Problem Statement

Developers routinely lose time not because of task volume, but because of:

- Frequent context switching
- Fragmentation across tools (GitHub, editors, notes, timers)
- Task systems that ignore *focus continuity*

DevFlow addresses this by:
- Modeling context switches explicitly
- Capturing user intent at interaction boundaries
- Keeping the system simple, local, and inspectable in the MVP

This is **not** a medical or cognitive claims product.  
All metrics are approximate and intentionally conservative.

---

## Architectural Philosophy

DevFlow follows a small set of non negotiable principles:

- **Frontend authoritative MVP**  
  The browser is the source of truth in early stages.

- **Explicit scope control**  
  Features exist only if they justify their complexity.

- **Documented decisions over implicit assumptions**  
  Architectural Decision Records (ADRs) are mandatory.

- **Evolution without rewrites**  
  The system must be able to grow without invalidating early work.

For details, see the [System Overview](./docs/architecture/system-overview.md).

---

## High Level Architecture

> MVP architecture (backend deferred)

