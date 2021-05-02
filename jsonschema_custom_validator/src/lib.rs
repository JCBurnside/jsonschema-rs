
pub trait CustomFormat : Sized +Send + Sync + Clone{
    const NAME : &'static str;
    fn is_valid(&self, input:&str)-> bool;
}