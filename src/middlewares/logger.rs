use tower_layer::Layer;
use std::fmt;
use std::task::{Context, Poll};
use tower_service::Service;

/// NOTE | When attaching this Layer to a Router, Axum will auto-populate
///     the template variables contained in this file.
/// NOTE | When applying a "layer", it only applies to everything applied
///     before it, in the router.
#[derive(Clone)]
pub struct LoggerMiddleware;

impl<S> Layer<S> for LoggerMiddleware {
    type Service = LoggerService<S>;

    fn layer(&self, service: S) -> Self::Service {
        LoggerService {
            service
        }
    }
}

// This service implements the Log behavior
#[derive(Clone)]
pub struct LoggerService<S> {
    service: S,
}

impl<S, Request> Service<Request> for LoggerService<S>
    where
        S: Service<Request>,
        Request: fmt::Debug,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    /// This is the entry point for the incoming request.
    fn call(&mut self, request: Request) -> Self::Future {
        // Insert log statement here or other functionality
        println!("request = {:?}", request);
        self.service.call(request)
    }
}