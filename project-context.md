# Barangay Management System — Project Context

## 1. Project Overview

This project is a **desktop Barangay Management System** designed for use by barangay personnel to manage residents, households, officials, documents, requests, records, and administrative information.

The application is intended to be:

* Desktop-first
* Offline-capable
* Local-first
* Persistent across application restarts
* Independent of external database servers
* Suitable for a Windows desktop environment
* Maintainable and expandable over time

The system must prioritize reliability, simplicity, data integrity, and ease of use.

---

# 2. Core Requirements

The application must:

* Run as a desktop application.
* Work without an internet connection for core functionality.
* Store operational data locally.
* Use a local SQLite database.
* Persist data after application restart.
* Persist data after computer restart.
* Keep the production database outside the source-code directory.
* Use database migrations for schema changes.
* Keep database access separate from React UI components.
* Provide proper CRUD functionality for applicable records.
* Maintain relationships between residents, households, officials, users, documents, and other records.
* Avoid unnecessary duplication of data.
* Provide a clean and professional administrative interface.

Internet connectivity must NOT be required for normal database operations.

---

# 3. Technology Stack

## Desktop Framework

**Tauri**

Tauri is responsible for packaging the application as a desktop application and providing the secure boundary between the frontend and backend.

## Frontend

* React
* TypeScript
* Tailwind CSS

The frontend is responsible for:

* UI
* Navigation
* Forms
* Tables
* Dashboards
* User interactions
* Validation presentation
* Calling backend services/commands

The frontend must not directly access SQLite.

## Backend

**Rust through Tauri**

Rust is responsible for:

* SQLite connection
* Database operations
* Database initialization
* Migration execution
* Backend services
* Tauri commands
* Filesystem operations where necessary

## Database

**SQLite**

SQLite is the only production database.

## ORM / Schema / Migrations

**Drizzle ORM**

Drizzle is used to:

* Define the database schema
* Generate migrations
* Maintain versioned database changes

Drizzle must remain the source of truth for database schema changes.

---

# 4. Database Architecture

The application uses a completely local SQLite database.

Do NOT introduce:

* MySQL
* PostgreSQL
* MongoDB
* Firebase
* Supabase
* External database servers
* Cloud databases
* Remote database dependencies

The production database must remain local to the user's computer.

---

# 5. Database Location

The production SQLite database must be stored in the appropriate **Tauri application-data directory**.

Do NOT store the production database in:

* The project root
* `src/`
* `src-tauri/`
* `node_modules/`
* The Git repository
* The application's source-code directory

The database must persist through:

* Application restart
* Computer restart
* Normal application updates

The exact production database path must be resolved through Tauri's application-data directory APIs rather than hardcoded to a development path.

---

# 6. Database Schema

The initial database architecture contains these tables:

1. `barangay_profile`
2. `users`
3. `roles`
4. `permissions`
5. `role_permissions`
6. `official_positions`
7. `residents`
8. `households`
9. `household_members`
10. `document_types`
11. `document_requests`
12. `documents`
13. `audit_logs`

Do not create unnecessary future tables unless a current feature genuinely requires them.

---

# 7. Core Relationships

## Users

Users should reference:

* Their role
* Their official position where applicable

## Roles

Roles should connect to permissions through:

`role_permissions`

This provides role-based access control.

## Households

Households relate to residents through:

`household_members`

A resident should not need duplicated household information when a relationship can represent it properly.

## Documents

Documents and document-related records should reference residents where appropriate.

## Audit Logs

Important administrative actions should be capable of being recorded in `audit_logs`.

---

# 8. Database Design Rules

Use:

* Primary keys
* Foreign keys
* Appropriate indexes
* Unique constraints where appropriate
* Timestamps
* Status fields where appropriate

Avoid:

* Unnecessary duplicated data
* Hardcoded relationships in React
* SQL inside React components
* Unnecessary tables
* Unnecessary complexity

Database relationships should be represented through proper foreign keys and relationship tables where appropriate.

---

# 9. Database Migration Rules

All production schema changes must use **Drizzle migrations**.

Never manually modify the production database schema as a substitute for a migration.

When a schema change is required:

1. Modify the Drizzle schema.
2. Generate the appropriate migration.
3. Review the generated SQL.
4. Test the migration.
5. Apply it through the application's migration system.

Existing migrations must not be casually rewritten once they have been used against production databases.

---

# 10. Database Access Architecture

Database access must remain outside React UI components.

The intended architecture is:

```text
React UI
    ↓
Frontend Service Layer
    ↓
Tauri Commands
    ↓
Rust Backend
    ↓
SQLite
```

React components must not contain raw SQL.

Database functionality should be exposed through reusable service functions and typed Tauri commands.

Example conceptual structure:

```text
src/
├── components/
├── pages/
├── services/
│   └── ...
└── ...

src-tauri/
├── src/
│   ├── db/
│   ├── services/
│   ├── commands/
│   └── ...
└── ...
```

The exact directory structure may differ if the existing project already has a sensible architecture.

Do not reorganize the entire project unnecessarily.

---

# 11. Current Database Implementation Status

## STEP 2 — DATABASE ARCHITECTURE

The previous implementation session has already worked on the database foundation.

Reported completed work includes:

* Drizzle SQLite schema
* Initial Drizzle migration
* Rust/Tauri migration runner
* Local SQLite database
* Tauri application-data database location
* Requested 13 tables
* Foreign keys
* Unique constraints
* Indexes
* Timestamps/status fields where appropriate
* Backend database/service boundary
* Typed frontend service boundary
* Database migration/persistence tests

The generated migration reportedly contains all requested tables.

The backend migration test reportedly:

1. Created the SQLite database.
2. Created all requested tables.
3. Inserted test data.
4. Reopened the database.
5. Confirmed the inserted data persisted.
6. Confirmed migrations were not reapplied unnecessarily.

The previous session ended because the coding session/token limit was reached.

### IMPORTANT

Do NOT assume the implementation is correct merely because the previous coding agent reported success.

When continuing development, inspect the actual project files and verify the implementation.

Do not rebuild Step 2 from scratch.

Do not replace the current architecture unless a genuine technical problem is discovered.

---

# 12. Current Functional Status

The database foundation exists, but the system is not yet a complete CRUD application.

The application still needs user-facing functionality for managing records.

Future CRUD functionality will include operations such as:

* Create
* Read
* Update
* Delete
* Search
* Filter
* View details

These should be implemented through the established frontend-service → Tauri-command → Rust → SQLite architecture.

Do not put SQL directly into React components.

---

# 13. Planned Application Modules

The system is expected to eventually contain the following major modules.

## Dashboard

Display useful administrative information such as:

* Resident count
* Household count
* Pending document requests
* Recent activity
* Other relevant statistics

Do not overpopulate the dashboard with unnecessary information.

---

## Resident Management

The resident module should eventually support:

* Resident list
* Search
* Filtering
* Resident details
* Add resident
* Edit resident
* Delete/archive resident where appropriate
* Resident status
* Household relationship
* Document history where appropriate

---

## Household Management

The household module should support:

* Household list
* Household details
* Add household
* Edit household
* Delete/archive household where appropriate
* Assign residents
* Remove residents
* View household members
* Household head designation where appropriate

Household membership should use the `household_members` relationship table.

---

## Barangay Officials

The system should support barangay officials and positions including, at minimum:

* Barangay Captain
* SK Chairman
* Barangay Secretary
* Barangay Treasurer

The `official_positions` table should represent positions instead of hardcoding positions throughout the UI.

---

## Users

The system should eventually support:

* User accounts
* User roles
* Role assignment
* Official position assignment where appropriate
* Account status
* Permissions

---

## Roles and Permissions

The system should use role-based access control.

Roles connect to permissions through:

`role_permissions`

Avoid hardcoding permission logic throughout individual UI components.

---

## Certificate / Document Management

The system should eventually support:

* Document types
* Document requests
* Generated/issued documents
* Resident association
* Request status
* Dates
* Relevant administrative information

Potential document types may include barangay certificates and clearances, but the exact list should be configurable rather than permanently hardcoded where practical.

---

## Blotter / Incident Management

Blotter functionality may be added when the project reaches that stage.

Do not create unnecessary database tables before they are actually required.

---

## Audit Logs

Important actions should eventually be recorded, including appropriate actions such as:

* Login
* Record creation
* Record modification
* Record deletion
* Document issuance
* Permission-related changes

Audit logging should be implemented carefully and should not unnecessarily expose sensitive information in the UI.

---

# 14. UI / UX Direction

The application should look like a professional modern administrative desktop application.

Priorities:

* Clean
* Minimal
* Professional
* Easy to navigate
* Consistent
* Practical
* Fast to understand
* Suitable for long-term administrative use

Avoid:

* Excessive gradients
* Excessive animations
* "AI slop" aesthetics
* Overly decorative interfaces
* Excessive glassmorphism
* Unnecessary visual effects
* Cluttered dashboards

The UI should prioritize usability over visual novelty.

---

# 15. Navigation

The application should eventually have a clear navigation structure.

A possible structure:

```text
Dashboard

Residents
    ├── All Residents
    └── Add Resident

Households
    ├── All Households
    └── Add Household

Documents
    ├── Requests
    ├── Documents
    └── Document Types

Officials

Users & Roles

Reports

Audit Logs

Settings
```

This is a conceptual structure.

Do not blindly implement every item at once.

Build modules incrementally.

---

# 16. Data Integrity

Data integrity is a major priority.

Before deleting records, consider whether the record is referenced elsewhere.

Foreign key relationships must be respected.

Avoid deleting records in a way that leaves orphaned data.

Where appropriate, consider archival/status-based removal rather than permanent deletion.

The final behavior should be determined per module rather than applying one deletion strategy everywhere.

---

# 17. Validation

Forms should validate required information before submitting data.

Validation should exist at the appropriate layers.

Frontend validation improves user experience.

Backend/database validation protects data integrity.

Never rely exclusively on frontend validation.

---

# 18. Error Handling

Errors should be handled gracefully.

Do not expose raw Rust errors, SQL errors, stack traces, or internal implementation details directly to ordinary users.

The UI should provide understandable messages.

Example:

Instead of exposing:

```text
FOREIGN KEY constraint failed
```

the UI could present an appropriate message explaining why the requested operation could not be completed.

Detailed technical errors may still be logged for debugging.

---

# 19. Security

The application is local-first, but local data should still be treated as important administrative information.

Avoid:

* Hardcoded passwords
* Hardcoded secrets
* API keys in frontend code
* Unnecessary external network requests
* Exposing database internals to the UI

Authentication and authorization should eventually be handled through the application's backend/service architecture.

---

# 20. Offline Requirement

Core functionality must work without internet access.

The following should not require internet:

* Opening the application
* Viewing residents
* Creating residents
* Editing residents
* Household management
* User management
* Officials management
* Document management
* Searching local records
* Reading local records

External integrations, if ever added, must be optional and must not break the core offline application.

---

# 21. Backup and Restore

A backup and restore system should eventually be implemented.

The exact design should be decided later.

A likely approach is to allow authorized users to export/import the local database or a validated backup format.

Do not implement backup functionality prematurely unless it is part of the current development step.

---

# 22. Development Rules

When working on this project:

### Rule 1 — Inspect before changing

Always inspect the existing implementation before modifying it.

Do not assume the project is empty.

### Rule 2 — Preserve working architecture

Do not replace working components or architecture without a clear technical reason.

### Rule 3 — Small steps

Implement the system incrementally.

Do not attempt to build the entire application in one giant change.

### Rule 4 — Test changes

After implementing a feature:

* Compile/build it.
* Run appropriate tests.
* Verify the affected functionality.
* Check for regressions.

### Rule 5 — Do not silently change requirements

If an implementation decision would change an established requirement, explain the issue before making the change.

### Rule 6 — Avoid unnecessary dependencies

Do not add packages simply because they are convenient.

Prefer the existing stack.

### Rule 7 — Keep database logic separated

Never place raw SQL directly inside React components.

### Rule 8 — Migrations are required

Database schema changes must use migrations.

### Rule 9 — Don't overbuild

Only implement functionality required by the current development step.

Do not create speculative tables, APIs, or modules simply because they might be useful later.

---

# 23. Tauri Rules

The application is a Tauri desktop application.

Backend functionality should remain within the Tauri/Rust boundary where appropriate.

The frontend should communicate with Rust through Tauri commands/services.

Production filesystem paths must use Tauri's appropriate application-data/path APIs.

Do not hardcode development machine paths.

---

# 24. Git Rules

GitHub is used for source control.

Do not commit:

* Production databases
* Local user data
* Secrets
* Passwords
* API keys
* Machine-specific files

The production SQLite database belongs in the application's data directory, not the repository.

---

# 25. Current Development Roadmap

The project should be developed in controlled steps.

### Step 1 — Project/Foundation

Establish the desktop application foundation and project structure.

### Step 2 — Database Architecture

Completed/in progress from the previous session.

Includes:

* SQLite
* Drizzle schema
* Migrations
* Tauri database initialization
* Database service boundary
* Database persistence testing

### Step 3 — Database Service / CRUD Foundation

Next major stage.

Create reusable backend/frontend service functionality that allows the application to:

* Create records
* Read records
* Update records
* Delete/archive records where appropriate

Begin with the core entities rather than implementing every module simultaneously.

### Step 4 — Resident Management

Build the complete resident management UI and backend functionality.

### Step 5 — Household Management

Build household management and resident membership relationships.

### Step 6 — Officials

Implement barangay official positions and assignments.

### Step 7 — Users / Roles / Permissions

Implement authentication and authorization.

### Step 8 — Documents

Implement document types, requests, and documents.

### Step 9 — Audit Logs

Implement administrative activity logging.

### Step 10 — Reports / Printing

Add appropriate reports and printable documents.

### Step 11 — Backup / Restore

Implement reliable local backup and restoration.

### Step 12 — Testing / Packaging

Test the complete system and produce the Windows desktop application.

---

# 26. Important Architecture Decisions

These decisions should not be changed casually.

### Decision 1

This is a **desktop application**, not a web-only application.

### Decision 2

The application must be **offline-capable**.

### Decision 3

The production database is **local SQLite**.

### Decision 4

There is **no external database server**.

### Decision 5

Drizzle is used for schema definition and migrations.

### Decision 6

Rust/Tauri owns the production SQLite connection.

### Decision 7

The production database is stored in Tauri's application-data directory.

### Decision 8

React must not directly execute SQL.

### Decision 9

Database access must use a service/backend boundary.

### Decision 10

Database schema changes must use migrations.

---

# 27. Instructions for Claude / Codex

You are working on an existing project.

Before making changes:

1. Read this file.
2. Inspect the current project structure.
3. Inspect relevant existing files.
4. Determine what has already been implemented.
5. Preserve existing working architecture.

When implementing a development step:

1. Explain what you intend to change.
2. Make only the changes required for that step.
3. Reuse existing architecture.
4. Run appropriate tests/builds.
5. Fix errors caused by your changes.
6. Verify the result.
7. Report what changed.
8. Stop when the requested step is complete.

Do not automatically continue into future development steps unless explicitly instructed.

---

# 28. Current Session Note

The previous coding session ran out of tokens while implementing Step 2.

The last reported status was:

> The backend migration test now passes: it creates every requested table, reopens the SQLite file, confirms the inserted row persists, and verifies that the migration is not reapplied.

The previous session also reported that a typed frontend service boundary was added for a later UI health check.

Before continuing development, inspect the actual code and verify these claims.

Do not rebuild Step 2 unless the existing implementation is genuinely broken.

The next intended development stage after verifying Step 2 is **Step 3 — Database Service / CRUD Foundation**.

---

# 29. Final Principle

Build the system as a reliable real-world administrative application, not as a demonstration project.

Prioritize:

**Correctness → Data integrity → Reliability → Maintainability → Usability → Visual polish**

Do not sacrifice the first four for visual features.

The system should remain understandable and maintainable by another developer even after the original development session ends.
