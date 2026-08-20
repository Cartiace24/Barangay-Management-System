mod database;
mod services;

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let database = database::Database::initialize(app.handle())
                .map_err(|error| -> Box<dyn std::error::Error> { Box::new(error) })?;
            services::auth_service::seed_defaults(&database)
                .map_err(|error| -> Box<dyn std::error::Error> { error.into() })?;
            services::document_service::seed_document_types(&database)
                .map_err(|error| -> Box<dyn std::error::Error> { error.into() })?;
            app.manage(database);
            app.manage(services::auth_service::Session(std::sync::Mutex::new(None)));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            services::system_service::get_database_status,
            services::profile_service::get_barangay_profile,
            services::profile_service::authenticate_user,
            services::profile_service::complete_first_time_setup,
            services::profile_service::update_barangay_profile,
            services::profile_service::update_branding_image,
            services::profile_service::remove_branding_image,
            services::profile_service::get_branding_image,
            services::auth_service::login,
            services::auth_service::logout,
            services::auth_service::current_session,
            services::auth_service::list_users,
            services::auth_service::list_roles,
            services::auth_service::list_positions,
            services::auth_service::create_position,
            services::auth_service::create_user,
            services::auth_service::update_user,
            services::auth_service::reset_user_password,
            services::auth_service::change_own_password
            ,services::resident_service::list_residents
            ,services::resident_service::get_resident_profile
            ,services::resident_service::create_resident
            ,services::resident_service::update_resident
            ,services::resident_service::archive_resident
            ,services::household_service::list_households
            ,services::household_service::get_household_profile
            ,services::household_service::list_available_residents
            ,services::household_service::create_household
            ,services::household_service::update_household
            ,services::household_service::add_household_member
            ,services::household_service::remove_household_member
            ,services::household_service::change_household_head
            ,services::document_service::list_document_types
            ,services::document_service::list_document_residents
            ,services::document_service::create_document_type
            ,services::document_service::update_document_type
            ,services::document_service::list_document_requests
            ,services::document_service::create_document_request
            ,services::document_service::update_document_request_status
            ,services::document_service::get_certificate_data
            ,services::blotter_service::list_blotter_cases
            ,services::blotter_service::get_blotter_case
            ,services::blotter_service::create_blotter_case
            ,services::blotter_service::update_blotter_case
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
