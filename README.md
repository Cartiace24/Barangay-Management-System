# Barangay Management System

An open-source, offline-first desktop application designed to help small Philippine barangays manage basic administrative records and daily operations.

> **Status:** Early development — currently completed through Step 8.

## About

The Barangay Management System is intended to provide a simple, practical, and accessible management solution for barangay offices.

The system is designed primarily for small barangays that may only have one computer and limited or unreliable internet access.

The goal is to keep the core system completely functional offline while providing a clean and easy-to-use desktop interface.

### Initial Development Barangay

**Brgy. Pulong Matong**  
General Tinio, Nueva Ecija

This is sample development data. The finished system will allow the barangay profile and information to be changed from within the application.

## Planned Features

- Resident management
- Household management
- Barangay officials and user management
- User roles and permissions
- Barangay profile and settings
- Document and certificate requests
- Blotter records
- Payment and receipt records
- Reports
- Database backup and restore
- Audit logs
- Offline operation

## Technology Stack

### Frontend

- React
- TypeScript
- Vite
- Tailwind CSS
- shadcn/ui

### Desktop

- Tauri 2
- Rust

### Database

- SQLite
- Drizzle ORM

The application is designed around a local-first architecture:

```text
React + TypeScript
        ↓
      Tauri
        ↓
      Rust
        ↓
     SQLite
