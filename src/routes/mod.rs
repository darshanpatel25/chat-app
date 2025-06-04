    use actix_web::web;

    use crate::controllers::userController;
    use crate::middleware::auth;
    use crate::controllers::ticket_controller;
    use crate::middleware::role_check;

    pub fn init(cfg: &mut web::ServiceConfig) {
        cfg.service(userController::register_user);
        cfg.service(userController::login_user);
        cfg.service(
            web::scope("/auth") // Optional: namespace it under /auth
                .wrap(auth::AuthMiddleware)
                .service(userController::protected_test)
        );
                // Create ticket - requires authentication only
        cfg.service(
            web::resource("/tickets/create")
                .wrap(auth::AuthMiddleware)
                .wrap(role_check::RoleCheck::new(vec!["project_manager","admin"]))
                .route(web::post().to(ticket_controller::create_ticket))
        );
        cfg.service(
            web::resource("/tickets/all")
                .wrap(auth::AuthMiddleware)
                .wrap(role_check::RoleCheck::new(vec!["project_manager", "admin"]))
                .route(web::get().to(ticket_controller::get_all_tickets))
        );
        cfg.service(
            web::resource("/tickets/status")
                .wrap(auth::AuthMiddleware)
                .wrap(role_check::RoleCheck::new(vec!["developer","qa","project_manager","admin"]))
                .route(web::put().to(ticket_controller::update_ticket_status))
        );
        cfg.service(
            web::resource("/tickets/assign")
                .wrap(auth::AuthMiddleware)
                .wrap(role_check::RoleCheck::new(vec!["pm","admin"]))
                .route(web::put().to(ticket_controller::assign_ticket))
        );
        // cfg.service(
        //     web::resource("/tickets/my")
        //         .wrap(auth::AuthMiddleware)
        //         .route(web::get().to(ticket_controller::get_my_tickets))
        // );
    }
