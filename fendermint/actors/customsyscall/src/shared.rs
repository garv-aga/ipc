use num_derive::FromPrimitive;

pub const CUSTOMSYSCALL_ACTOR_NAME: &str = "customsyscall";

#[derive(FromPrimitive)]
#[repr(u64)]
pub enum Method {
    Invoke = frc42_dispatch::method_hash!("Invoke"),
}