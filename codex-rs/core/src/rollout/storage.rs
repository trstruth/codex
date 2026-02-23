use std::io;
use std::path::{Path, PathBuf};

use async_trait::async_trait;

use crate::Config;
use crate::rollout::list::{Cursor, ThreadSortKey, ThreadsPage};
use crate::rollout::recorder::RolloutRecorder;
use crate::state_db::StateDbHandle;
use codex_protocol::ThreadId;
use codex_protocol::protocol::InitialHistory;
use codex_protocol::protocol::RolloutItem;
use codex_protocol::protocol::SessionSource;

/// Live-session persistence operations used by active sessions.
#[async_trait]
pub trait LiveRolloutRecorder: Send + Sync {
    fn rollout_path(&self) -> &Path;
    fn state_db(&self) -> Option<StateDbHandle>;

    async fn record_items(&self, items: &[RolloutItem]) -> io::Result<()>;
    async fn persist(&self) -> io::Result<()>;
    async fn flush(&self) -> io::Result<()>;
    async fn shutdown(&self) -> io::Result<()>;
}

/// Rollout storage operations used when reading historical sessions.
#[async_trait]
pub trait RolloutHistoryStore: Send + Sync {
    async fn list_threads(
        &self,
        config: &Config,
        page_size: usize,
        cursor: Option<&Cursor>,
        sort_key: ThreadSortKey,
        allowed_sources: &[SessionSource],
        model_providers: Option<&[String]>,
        default_provider: &str,
    ) -> io::Result<ThreadsPage>;

    async fn list_archived_threads(
        &self,
        config: &Config,
        page_size: usize,
        cursor: Option<&Cursor>,
        sort_key: ThreadSortKey,
        allowed_sources: &[SessionSource],
        model_providers: Option<&[String]>,
        default_provider: &str,
    ) -> io::Result<ThreadsPage>;

    async fn find_latest_thread_path(
        &self,
        config: &Config,
        page_size: usize,
        cursor: Option<&Cursor>,
        sort_key: ThreadSortKey,
        allowed_sources: &[SessionSource],
        model_providers: Option<&[String]>,
        default_provider: &str,
        filter_cwd: Option<&Path>,
    ) -> io::Result<Option<PathBuf>>;

    async fn get_rollout_history(&self, path: &Path) -> io::Result<InitialHistory>;
    async fn load_rollout_items(
        &self,
        path: &Path,
    ) -> io::Result<(Vec<RolloutItem>, Option<ThreadId>, usize)>;
}

/// Default file-system based rollout history store.
#[derive(Debug, Default, Clone)]
pub struct FileRolloutStore;

#[async_trait]
impl RolloutHistoryStore for FileRolloutStore {
    async fn list_threads(
        &self,
        config: &Config,
        page_size: usize,
        cursor: Option<&Cursor>,
        sort_key: ThreadSortKey,
        allowed_sources: &[SessionSource],
        model_providers: Option<&[String]>,
        default_provider: &str,
    ) -> io::Result<ThreadsPage> {
        RolloutRecorder::list_threads(
            config,
            page_size,
            cursor,
            sort_key,
            allowed_sources,
            model_providers,
            default_provider,
        )
        .await
    }

    async fn list_archived_threads(
        &self,
        config: &Config,
        page_size: usize,
        cursor: Option<&Cursor>,
        sort_key: ThreadSortKey,
        allowed_sources: &[SessionSource],
        model_providers: Option<&[String]>,
        default_provider: &str,
    ) -> io::Result<ThreadsPage> {
        RolloutRecorder::list_archived_threads(
            config,
            page_size,
            cursor,
            sort_key,
            allowed_sources,
            model_providers,
            default_provider,
        )
        .await
    }

    async fn find_latest_thread_path(
        &self,
        config: &Config,
        page_size: usize,
        cursor: Option<&Cursor>,
        sort_key: ThreadSortKey,
        allowed_sources: &[SessionSource],
        model_providers: Option<&[String]>,
        default_provider: &str,
        filter_cwd: Option<&Path>,
    ) -> io::Result<Option<PathBuf>> {
        RolloutRecorder::find_latest_thread_path(
            config,
            page_size,
            cursor,
            sort_key,
            allowed_sources,
            model_providers,
            default_provider,
            filter_cwd,
        )
        .await
    }

    async fn get_rollout_history(&self, path: &Path) -> io::Result<InitialHistory> {
        RolloutRecorder::get_rollout_history(path).await
    }

    async fn load_rollout_items(
        &self,
        path: &Path,
    ) -> io::Result<(Vec<RolloutItem>, Option<ThreadId>, usize)> {
        RolloutRecorder::load_rollout_items(path).await
    }
}

#[async_trait]
impl LiveRolloutRecorder for RolloutRecorder {
    fn rollout_path(&self) -> &Path {
        self.rollout_path()
    }

    fn state_db(&self) -> Option<StateDbHandle> {
        self.state_db()
    }

    async fn record_items(&self, items: &[RolloutItem]) -> io::Result<()> {
        self.record_items(items).await
    }

    async fn persist(&self) -> io::Result<()> {
        self.persist().await
    }

    async fn flush(&self) -> io::Result<()> {
        self.flush().await
    }

    async fn shutdown(&self) -> io::Result<()> {
        self.shutdown().await
    }
}
