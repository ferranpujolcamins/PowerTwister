use std::{pin::Pin, sync::Arc, task::{Poll, Context}};

use async_trait::async_trait;
use futures::{Stream, stream::{self, BoxStream, StreamExt}, executor};


trait HasInputType {
    type Input;
}
trait HasOutputType {
    type Output;
}

#[async_trait]
trait State: Sized + Send {
    type Output;
    type Events: Stream + Send;

    async fn receive_events(self, events: Self::Events) -> Option<(Self::Output, (Self, Self::Events))>;

    // TODO: can we get rid of the box?
    fn drive<'a>(self, events: Self::Events) -> BoxStream<'a, Self::Output> where Self: 'a, Self::Events: 'a {
        stream::unfold((self, events), |(current_state, e)| async {
            current_state.receive_events(e).await
        }).boxed()
    }
}



enum A {
    X(i32),
    Y(i32)
}

#[async_trait]
impl State for A {
    type Events = BoxStream<'static, i32>;
    type Output = i32;
    
    async fn receive_events(self, events: Self::Events) -> Option<(Self::Output, (Self, Self::Events))> {
        match self {
            A::X(i) => {
                //println!("X: {}", i);
                Some((0, (A::Y(i), events)))
            },
            A::Y(i) => {
                //println!("Y: {}", i);
                if i < 15 {
                    Some((1, (A::X(i+1), events)))
                } else {
                    None
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        use futures::Stream;

        let a = A::X(0);
        let events = Box::pin(stream::empty());
        let s = a.drive(events);
        println!("{}", executor::block_on(s.fold("".to_string(), |acc, e| async move { format!("{}{}", acc, e) })));
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