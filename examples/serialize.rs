extern crate libc;
extern crate rctl;

#[cfg(feature="serialize")]
extern crate serde_json;

#[cfg(feature="serialize")]
fn main() {

    let uid = unsafe { libc::getuid() };

    let rule = rctl::Rule {
        subject: rctl::Subject::user_id(uid),
        resource: rctl::Resource::OpenFiles,
        limit: rctl::Limit::amount(1000),
        action: rctl::Action::Deny
    };

    let serialized = serde_json::to_string(&rule)
        .expect("Could not serialize RCTL rule.");

    println!("{}", serialized);
}

#[cfg(not(feature="serialize"))]
fn main() {
   println!("Run `cargo build --features=serialize` to enable this example");
}
