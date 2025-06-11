mod api;
//get
pub use api::auth;
pub use api::create_room;
pub use api::join_room;
pub use api::logout;
pub use api::rooms;

//post
pub use api::login;
pub use api::signup;

mod static_file;
//get
pub use static_file::home;
