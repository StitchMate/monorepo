use async_trait::async_trait;

#[async_trait]
pub trait TState<T>
where
    T: Sync + Send,
{
    async fn entry(&mut self, _context: &mut T) {}
    async fn exit(&mut self, _context: &mut T) {}
    async fn update(&mut self, _context: &mut T) {}
}
