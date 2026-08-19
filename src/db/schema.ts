import { sql } from "drizzle-orm";
import { index, integer, primaryKey, sqliteTable, text, uniqueIndex } from "drizzle-orm/sqlite-core";

const timestamps = {
  createdAt: text("created_at").notNull().default(sql`CURRENT_TIMESTAMP`),
  updatedAt: text("updated_at").notNull().default(sql`CURRENT_TIMESTAMP`),
};

export const barangayProfile = sqliteTable("barangay_profile", {
  id: integer("id").primaryKey({ autoIncrement: true }),
  name: text("name").notNull(),
  municipality: text("municipality").notNull(),
  province: text("province").notNull(),
  address: text("address"),
  contactNumber: text("contact_number"),
  email: text("email"),
  logoPath: text("logo_path"),
  ...timestamps,
});

export const roles = sqliteTable("roles", {
  id: integer("id").primaryKey({ autoIncrement: true }),
  code: text("code").notNull().unique(),
  name: text("name").notNull(),
  description: text("description"),
  status: text("status", { enum: ["active", "inactive"] }).notNull().default("active"),
  ...timestamps,
});

export const permissions = sqliteTable("permissions", {
  id: integer("id").primaryKey({ autoIncrement: true }),
  code: text("code").notNull().unique(),
  name: text("name").notNull(),
  module: text("module").notNull(),
  description: text("description"),
  ...timestamps,
}, (table) => [index("permissions_module_idx").on(table.module)]);

export const rolePermissions = sqliteTable("role_permissions", {
  roleId: integer("role_id").notNull().references(() => roles.id, { onDelete: "cascade" }),
  permissionId: integer("permission_id").notNull().references(() => permissions.id, { onDelete: "cascade" }),
  createdAt: text("created_at").notNull().default(sql`CURRENT_TIMESTAMP`),
}, (table) => [primaryKey({ columns: [table.roleId, table.permissionId] })]);

export const officialPositions = sqliteTable("official_positions", {
  id: integer("id").primaryKey({ autoIncrement: true }),
  title: text("title").notNull(),
  displayOrder: integer("display_order").notNull().default(0),
  status: text("status", { enum: ["active", "inactive"] }).notNull().default("active"),
  ...timestamps,
}, (table) => [uniqueIndex("official_positions_title_unique").on(table.title)]);

export const users = sqliteTable("users", {
  id: integer("id").primaryKey({ autoIncrement: true }),
  username: text("username").notNull().unique(),
  passwordHash: text("password_hash").notNull(),
  fullName: text("full_name").notNull(),
  officialPositionId: integer("official_position_id").references(() => officialPositions.id, { onDelete: "set null" }),
  roleId: integer("role_id").references(() => roles.id, { onDelete: "set null" }),
  status: text("status", { enum: ["active", "inactive", "locked"] }).notNull().default("active"),
  lastLoginAt: text("last_login_at"),
  ...timestamps,
}, (table) => [index("users_role_idx").on(table.roleId), index("users_official_position_idx").on(table.officialPositionId)]);

export const residents = sqliteTable("residents", {
  id: integer("id").primaryKey({ autoIncrement: true }),
  residentCode: text("resident_code").notNull().unique(),
  firstName: text("first_name").notNull(),
  middleName: text("middle_name"),
  lastName: text("last_name").notNull(),
  suffix: text("suffix"),
  birthDate: text("birth_date"),
  sex: text("sex", { enum: ["male", "female", "other", "unspecified"] }).notNull().default("unspecified"),
  civilStatus: text("civil_status"),
  contactNumber: text("contact_number"),
  email: text("email"),
  status: text("status", { enum: ["active", "inactive", "deceased", "moved_out"] }).notNull().default("active"),
  ...timestamps,
}, (table) => [index("residents_name_idx").on(table.lastName, table.firstName), index("residents_status_idx").on(table.status)]);

export const households = sqliteTable("households", {
  id: integer("id").primaryKey({ autoIncrement: true }),
  householdCode: text("household_code").notNull().unique(),
  addressLine: text("address_line").notNull(),
  purok: text("purok"),
  status: text("status", { enum: ["active", "inactive"] }).notNull().default("active"),
  ...timestamps,
}, (table) => [index("households_purok_idx").on(table.purok), index("households_status_idx").on(table.status)]);

export const householdMembers = sqliteTable("household_members", {
  householdId: integer("household_id").notNull().references(() => households.id, { onDelete: "cascade" }),
  residentId: integer("resident_id").notNull().references(() => residents.id, { onDelete: "cascade" }),
  relationshipToHead: text("relationship_to_head").notNull(),
  isHouseholdHead: integer("is_household_head", { mode: "boolean" }).notNull().default(false),
  joinedAt: text("joined_at").notNull().default(sql`CURRENT_TIMESTAMP`),
}, (table) => [primaryKey({ columns: [table.householdId, table.residentId] }), index("household_members_resident_idx").on(table.residentId)]);

export const documentTypes = sqliteTable("document_types", {
  id: integer("id").primaryKey({ autoIncrement: true }),
  code: text("code").notNull().unique(),
  name: text("name").notNull(),
  description: text("description"),
  feeCentavos: integer("fee_centavos").notNull().default(0),
  validityDays: integer("validity_days"),
  status: text("status", { enum: ["active", "inactive"] }).notNull().default("active"),
  ...timestamps,
});

export const documentRequests = sqliteTable("document_requests", {
  id: integer("id").primaryKey({ autoIncrement: true }),
  requestCode: text("request_code").notNull().unique(),
  residentId: integer("resident_id").notNull().references(() => residents.id, { onDelete: "restrict" }),
  documentTypeId: integer("document_type_id").notNull().references(() => documentTypes.id, { onDelete: "restrict" }),
  purpose: text("purpose").notNull(),
  status: text("status", { enum: ["pending", "approved", "rejected", "cancelled", "issued"] }).notNull().default("pending"),
  requestedAt: text("requested_at").notNull().default(sql`CURRENT_TIMESTAMP`),
  reviewedByUserId: integer("reviewed_by_user_id").references(() => users.id, { onDelete: "set null" }),
  reviewedAt: text("reviewed_at"),
  rejectionReason: text("rejection_reason"),
  ...timestamps,
}, (table) => [index("document_requests_resident_idx").on(table.residentId), index("document_requests_status_idx").on(table.status), index("document_requests_type_idx").on(table.documentTypeId)]);

export const documents = sqliteTable("documents", {
  id: integer("id").primaryKey({ autoIncrement: true }),
  documentNumber: text("document_number").notNull().unique(),
  documentRequestId: integer("document_request_id").references(() => documentRequests.id, { onDelete: "set null" }),
  residentId: integer("resident_id").notNull().references(() => residents.id, { onDelete: "restrict" }),
  documentTypeId: integer("document_type_id").notNull().references(() => documentTypes.id, { onDelete: "restrict" }),
  issuedByUserId: integer("issued_by_user_id").references(() => users.id, { onDelete: "set null" }),
  status: text("status", { enum: ["issued", "voided"] }).notNull().default("issued"),
  issuedAt: text("issued_at").notNull().default(sql`CURRENT_TIMESTAMP`),
  expiresAt: text("expires_at"),
  qrToken: text("qr_token").notNull().unique(),
  voidedAt: text("voided_at"),
  voidReason: text("void_reason"),
  ...timestamps,
}, (table) => [index("documents_resident_idx").on(table.residentId), index("documents_status_idx").on(table.status), index("documents_type_idx").on(table.documentTypeId)]);

export const auditLogs = sqliteTable("audit_logs", {
  id: integer("id").primaryKey({ autoIncrement: true }),
  userId: integer("user_id").references(() => users.id, { onDelete: "set null" }),
  action: text("action").notNull(),
  entityType: text("entity_type").notNull(),
  entityId: text("entity_id"),
  detailsJson: text("details_json"),
  createdAt: text("created_at").notNull().default(sql`CURRENT_TIMESTAMP`),
}, (table) => [index("audit_logs_user_idx").on(table.userId), index("audit_logs_entity_idx").on(table.entityType, table.entityId), index("audit_logs_created_at_idx").on(table.createdAt)]);
