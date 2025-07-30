
pub mod add;
pub mod branch;
pub mod checkout;
pub mod commit;
pub mod diff;
pub mod init;
pub mod status;
pub mod switch;
pub mod log;
pub mod merge;
pub mod base;
pub mod shared;

//基础trait，定义所有命令的共同接口
pub trait Command {
    fn execute(&mut self) -> i32;
    fn run(&mut self) -> Result<(), Box<dyn std::error::Error>>;
}
