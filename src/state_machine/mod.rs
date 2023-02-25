use std::{pin::Pin, rc::Rc, sync::Arc};

use async_trait::async_trait;
use futures::{Stream, stream};


trait HasInputType {
    type Input;
}
trait HasOutputType {
    type Output;
}

#[async_trait]
trait State: Sized {
    type Output;
    type Events: Stream + Sync;

    async fn receive_events(&self, events: &Self::Events) -> Option<(Self::Output, &Self)>;

    // TODO: can we get rid of the box?
    async fn drive<'s, 'e: 's>(&'s self, events: &'e Self::Events) -> Box<dyn Stream<Item = Self::Output> +'s>  {
        Box::new(stream::unfold(self, |current_state| async {
            current_state.receive_events(events).await
        }))
    }
}

enum A {
    X,
    Y
}

#[async_trait]
impl State for A {
    type Events = Arc<dyn Stream<Item = i32>>;
    
    async fn receive_events(&self, events: &Self::Events) -> Option<(i32, &Self)> {

    }
}

/* 

async fn state(events: Stream) -> Output {
    f();
    g();
    events.await;
    h();
}

async fn other_State(events: Stream) -> i32 {
    events.await;

}
*/