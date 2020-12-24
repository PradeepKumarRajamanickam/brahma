pub trait StateEvent
{
    fn get_state_id(&self) -> usize;
    fn get_instance_id(&self) -> usize;
}