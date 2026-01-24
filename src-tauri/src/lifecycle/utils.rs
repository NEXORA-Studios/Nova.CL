use std::future::Future;
use std::sync::Arc;

use crate::lifecycle::{CommandError, CommandHandler, CommandInput, CommandOutput};

pub fn async_cmd<F, Fut>(f: F) -> Arc<CommandHandler>
where
    F: Fn(CommandInput) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<CommandOutput, CommandError>> + Send + 'static,
{
    Arc::new(move |input| Box::pin(f(input)))
}

pub fn sync_cmd<F>(f: F) -> Arc<CommandHandler>
where
    F: Fn(CommandInput) -> Result<CommandOutput, CommandError> + Send + Sync + 'static,
{
    Arc::new(move |input| {
        let result = f(input);
        Box::pin(async move { result })
    })
}