use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Instant;

use pin_project::pin_project;

#[pin_project]
struct TimedMetric<Fut>
where
    Fut: Future,
{
    #[pin]
    inner: Fut,
    operation: &'static str,
    route: &'static str,
    start: Option<Instant>,
}

impl<Fut> Future for TimedMetric<Fut>
where
    Fut: Future,
{
    type Output = Fut::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let this = self.project();
        let start = this.start.get_or_insert_with(Instant::now);

        match this.inner.poll(cx) {
            // If the inner future is still pending, this wrapper is still pending.
            Poll::Pending => Poll::Pending,

            // If the inner future is done, measure the elapsed time and finish this wrapper future.
            Poll::Ready(v) => {
                let histogram = metrics::histogram!("riot_api", "operation" => *this.operation, "route" => *this.route);
                histogram.record(start.elapsed());
                Poll::Ready(v)
            }
        }
    }
}

pub fn timed<Fut>(future: Fut, operation: &'static str, route: &'static str) -> impl Future<Output = Fut::Output>
where
    Fut: Future,
{
    TimedMetric {
        inner: future,
        operation,
        route,
        start: None,
    }
}