CREATE TABLE `blotter_cases` (
	`id` integer PRIMARY KEY AUTOINCREMENT NOT NULL,
	`case_number` text NOT NULL,
	`reported_at` text DEFAULT CURRENT_TIMESTAMP NOT NULL,
	`complainant` text NOT NULL,
	`respondent` text,
	`incident_at` text,
	`incident_location` text,
	`incident_description` text NOT NULL,
	`action_taken` text,
	`status` text DEFAULT 'open' NOT NULL,
	`assigned_official_id` integer,
	`resolution` text,
	`resolved_at` text,
	`closed_at` text,
	`created_at` text DEFAULT CURRENT_TIMESTAMP NOT NULL,
	`updated_at` text DEFAULT CURRENT_TIMESTAMP NOT NULL,
	FOREIGN KEY (`assigned_official_id`) REFERENCES `users`(`id`) ON UPDATE no action ON DELETE set null
);
--> statement-breakpoint
CREATE UNIQUE INDEX `blotter_cases_case_number_unique` ON `blotter_cases` (`case_number`);--> statement-breakpoint
CREATE INDEX `blotter_cases_status_idx` ON `blotter_cases` (`status`);--> statement-breakpoint
CREATE INDEX `blotter_cases_reported_at_idx` ON `blotter_cases` (`reported_at`);--> statement-breakpoint
CREATE INDEX `blotter_cases_assigned_idx` ON `blotter_cases` (`assigned_official_id`);