use async_trait::async_trait;
use futures::{Stream, StreamExt};

struct StateMachine<State> {
    current_state: State
}

#[async_trait]
trait StateMachineXXXX {
    type State;
    type Input;

    async fn receive(&self, input: &Self::Input) -> StateMachine<Self::State>;
}

impl<State, Input> StateMachine<State> where Self: StateMachineXXXX<State=State, Input=Input> {
    async fn execute<InputStream>(&self, initial_state: &State, inputs: InputStream) -> &State where InputStream: Stream<Item = Input> {
        inputs.fold(initial_state, ...)
        &self.current_state
    }
}