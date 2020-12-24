#[derive(Default)]
pub struct Machine;

// State Lanes
#[derive(Default)]
#[allow(non_camel_case_types)]
pub(crate) struct STATE_Start_LANE_OnEnter;
#[derive(Default)]
#[allow(non_camel_case_types)]
pub(crate) struct STATE_Choice_LANE_OnEnter;
#[derive(Default)]
#[allow(non_camel_case_types)]
pub(crate) struct STATE_A_LANE_OnEnter;
#[derive(Default)]
#[allow(non_camel_case_types)]
pub(crate) struct STATE_B_LANE_OnEnter;
#[derive(Default)]
#[allow(non_camel_case_types)]
pub(crate) struct STATE_End_LANE_OnEnter;

// Transit Lanes
#[derive(Default)]
#[allow(non_camel_case_types)]
pub(crate) struct Transit_Start_TO_Choice_LANE_OnEnter;
#[derive(Default)]
#[allow(non_camel_case_types)]
pub(crate) struct Transit_Choice_TO_A_LANE_OnAction;
#[derive(Default)]
#[allow(non_camel_case_types)]
pub(crate) struct Transit_Choice_TO_B_LANE_OnAction;
#[derive(Default)]
#[allow(non_camel_case_types)]
pub(crate) struct Transit_A_TO_End_LANE_OnEnter;
#[derive(Default)]
#[allow(non_camel_case_types)]
pub(crate) struct Transit_B_TO_End_LANE_OnEnter;
