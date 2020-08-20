mod error;

use human_panic::setup_panic;

use crate::error::Error;

fn main() {
    setup_panic!();

    // TODO: everything
    let result = Result::<(), Error>::Ok(());

    match result {
        Ok(()) => ::std::process::exit(0),
        Err(e) => {
            eprintln!("Error: {}", e);
            ::std::process::exit(1);
        }
    }
}
