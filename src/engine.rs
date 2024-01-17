use crate::{responses::Ctx, traits::Method, Methods};
use futures::future::BoxFuture;
use std::{collections::HashMap, future::Future, sync::Arc};

pub struct SafeVkBot {
    methods: Arc<Methods>,
    commands: HashMap<String, Box<dyn Fn(Arc<Methods>) -> BoxFuture<'static, ()>>>,
    watching: Vec<Box<dyn Fn(Arc<Methods>) -> BoxFuture<'static, ()>>>,
}

impl SafeVkBot {
    /// Creates a new instance for `SafeVkBot`
    pub fn create(token: &str) -> Self {
        SafeVkBot {
            methods: Arc::new(Methods::new(token.to_string())),
            commands: HashMap::new(),
            watching: Vec::new(),
        }
    }

    /// Creates a new command that bot will listen
    pub fn command<F, Fut>(mut self, trigger: &str, handler: F) -> Self
    where
        F: Fn(Arc<Methods>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.commands.insert(
            trigger.to_string(),
            Box::new(move |ctx| Box::pin(handler(ctx))),
        );
        self
    }

    /// Callback for each new request
    pub fn watch<F, Fut>(mut self, handler: F) -> Self
    where
        F: Fn(Arc<Methods>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.watching
            .push(Box::new(move |ctx| Box::pin(handler(ctx))));
        self
    }

    /// Starts a new long poll session
    /// For more info: https://dev.vk.com/en/api/bots-long-poll/getting-started
    pub async fn start_polling(&self, group_id: u32) {
        let (s, mut r) = tokio::sync::mpsc::channel(10);
        let methods = self.methods.clone();

        tokio::spawn(async move {
            loop {
                let response = methods.long_poll(group_id).await;
                let update = methods
                    .connect(&response.server, response.key, response.ts, 25)
                    .await;

                s.send(update).await.expect("unable to send");
            }
        });

        while let Some(update) = r.recv().await {
            self.update_state(update).await;
        }
    }

    ///
    async fn update_state(&self, update: Ctx) {
        let mut state = self.write_state().await;

        for watch in &self.watching {
            tokio::spawn(watch(self.methods.clone()));
        }

        if let Some(command) = self.parse_command(&update) {
            if let Some(handler) = self.commands.get(&command) {
                tokio::spawn(handler(self.methods.clone()));
            }
        }

        *state = update;
    }

    async fn write_state(&self) -> tokio::sync::RwLockWriteGuard<'_, Ctx> {
        self.methods.context.write().await
    }

    /// Parses a command
    #[inline]
    fn parse_command(&self, update: &Ctx) -> Option<String> {
        update.updates.iter().find_map(|v| {
            v.object.message.as_ref().and_then(|msg| {
                msg.text
                    .to_lowercase()
                    .split_whitespace()
                    .next()
                    .map(|command| command.to_string())
            })
        })
    }
}
