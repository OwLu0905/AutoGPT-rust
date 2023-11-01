#[macro_export]
macro_rules! get_function_string {
    ($func:ident) => {{
        stringify!($func)
    }};
}

#[macro_use]
mod ai_functions;
mod apis;
mod helpers;
mod models;

use helpers::command_line::get_user_response;

fn main() {
    let usr_req = get_user_response("build something");
    dbg!(usr_req);
}
