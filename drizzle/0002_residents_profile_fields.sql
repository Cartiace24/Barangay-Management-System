ALTER TABLE `residents` ADD `nationality` text DEFAULT 'Filipino' NOT NULL;--> statement-breakpoint
ALTER TABLE `residents` ADD `address` text;--> statement-breakpoint
ALTER TABLE `residents` ADD `barangay` text;--> statement-breakpoint
ALTER TABLE `residents` ADD `municipality` text;--> statement-breakpoint
ALTER TABLE `residents` ADD `province` text;--> statement-breakpoint
ALTER TABLE `residents` ADD `occupation` text;--> statement-breakpoint
ALTER TABLE `residents` ADD `registered_at` text DEFAULT CURRENT_TIMESTAMP NOT NULL;