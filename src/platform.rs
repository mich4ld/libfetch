use crate::shared::procfs::Memory;

pub trait Platform {
    fn name(&self) -> Option<String>;

    fn memory(&self) -> Option<Memory>;

    fn shell(&self) -> Option<String>;
    
    fn kernel(&self) -> Option<String>;

    fn uptime(&self) -> Option<usize>;

    fn user(&self) -> Option<String>;

    fn hostname(&self) -> Option<String>;

    fn desktop(&self) -> Option<String>;
}
