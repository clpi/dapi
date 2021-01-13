use super::state::DState;
use tide::Request;
use tide::prelude::*;

pub struct DApi {
    state: DState,
}
