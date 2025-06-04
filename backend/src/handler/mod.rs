mod api;
//get
pub use api::greeting;

//post
pub use api::login;
pub use api::signup;

mod static_file;
//get
pub use static_file::home;
