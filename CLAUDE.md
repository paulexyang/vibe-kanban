# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Core Commands

### Development
- `pnpm dev` - Run frontend (port 3000) and backend (port 3001) with hot reload
- `pnpm build` - Build both frontend and backend for production
- `pnpm generate-types` - Regenerate TypeScript types from Rust models (run after backend model changes)

### Testing
- `cargo test --workspace` - Run all Rust tests
- `cd frontend && npx tsc --noEmit` - Type-check frontend code

### Linting and Formatting
- `cd frontend && npm run lint` - Lint frontend code (max 100 warnings allowed)
- `cd frontend && npm run lint:fix` - Auto-fix frontend lint issues
- `cd frontend && npm run format` - Format frontend code with Prettier
- `cargo fmt --all` - Format Rust code
- `cargo clippy --all --all-targets --all-features` - Lint Rust code

## Architecture Overview

Vibe Kanban orchestrates AI coding agents (Claude Code, Gemini CLI, Codex, Amp) using a kanban-style interface. The architecture consists of:

### Backend (Rust + Axum)
- **Entry**: `backend/src/main.rs` - Axum server setup with CORS and static file serving
- **API Routes**: `backend/src/routes/` - RESTful endpoints under `/api/*`
- **Database**: SQLite with SQLx, migrations in `backend/migrations/`
- **Services**: `backend/src/services/` - Core business logic
  - `git.rs` - Git worktree management for parallel task execution
  - `setup.rs` - Project setup automation
  - `process.rs` - AI agent process management
- **Executors**: `backend/src/executors/` - AI agent integrations (Claude, Gemini, etc.)
- **MCP Server**: `backend/src/mcp/` - Model Context Protocol implementation

### Frontend (React + TypeScript + Vite)
- **Entry**: `frontend/src/App.tsx` - Main React component with routing
- **Pages**: `frontend/src/pages/` - Route components
  - `ProjectOverview.tsx` - Kanban board interface
  - `TaskDetail.tsx` - Task execution monitoring
- **Components**: `frontend/src/components/` - Reusable UI components (shadcn/ui based)
- **API Client**: `frontend/src/lib/api.ts` - Type-safe API client
- **Shared Types**: `shared/types.ts` - Auto-generated from Rust models

### Key Workflows

1. **Task Execution Flow**:
   - User creates task → Backend creates git worktree → Executor runs AI agent → Real-time updates via polling → PR creation on completion

2. **Type Safety Flow**:
   - Modify Rust models → Run `pnpm generate-types` → TypeScript types updated → Frontend compilation catches type errors

3. **Database Schema Changes**:
   - Create migration file in `backend/migrations/` → SQLx applies on startup → Update models in `backend/src/models/`

## Important Notes

- **Git Worktrees**: Tasks execute in isolated git worktrees to enable parallel execution
- **Port Configuration**: Dev server automatically finds free ports, stored in `.vibe-kanban-dev-ports.json`
- **Database Location**: Development DB at `~/.vibe-kanban/vibe_kanban.db`
- **Sound Alerts**: Built-in notification sounds for task completion/failure
- **Authentication**: GitHub token required for PR operations, stored securely
- **MCP Support**: Vibe Kanban acts as an MCP server for agent communication