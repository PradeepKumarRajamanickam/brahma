use crate::traits::StateEvent;

#[derive(Property)]
pub(crate) struct OnEnterState
{
    pub(crate) state_id: usize,
    pub(crate) instance_id: usize,
}

impl StateEvent for OnEnterState
{
    fn get_state_id(&self) -> usize
    {
        self.state_id
    }

    fn get_instance_id(&self) -> usize
    {
        self.instance_id
    }
}