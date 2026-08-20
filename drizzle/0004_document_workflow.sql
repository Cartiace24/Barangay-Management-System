ALTER TABLE `document_requests` ADD `requested_by_user_id` integer REFERENCES users(id);--> statement-breakpoint
ALTER TABLE `document_requests` ADD `approved_by_user_id` integer REFERENCES users(id);--> statement-breakpoint
ALTER TABLE `document_requests` ADD `approved_at` text;--> statement-breakpoint
ALTER TABLE `document_requests` ADD `released_by_user_id` integer REFERENCES users(id);--> statement-breakpoint
ALTER TABLE `document_requests` ADD `released_at` text;