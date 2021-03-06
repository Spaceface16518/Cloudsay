use actix_web::{
    actix::System,
    http::Method,
    middleware::Logger,
    server::HttpServer,
    App,
};
use lib::handler;
use pretty_env_logger;
use std::{
    env::var,
    net::{Ipv4Addr, SocketAddrV4},
};

mod lib;

fn main() {
    pretty_env_logger::init();

    // Register the actix runtime
    let system = System::new("cloudsay");

    // Get the desired port from an environmental variable (default is 80)
    let port: u16 = if let Ok(p) = var("PORT") {
        p.parse().expect("Could not parse $PORT")
    } else {
        80
    };

    // Build the http server
    HttpServer::new(|| {
        App::new()
            // Add some logging, using pretty_env_logger.
            .middleware(Logger::default())
            // Register our handler with the actix application
            .resource("/", |r| r.method(Method::GET).with(handler))
    })
    // Bind it to the correct port, and use the generic 0.0.0.0 ip address
    .bind(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), port))
    .expect("Could not bind to specified port")
    // Start the server (this is not actually done here, it simply registers
    // this action into the actix runtime. It will be preformed later)
    .start();

    // Run the actix runtime
    system.run();
}
