use dashmap::DashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::time::sleep;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct SessionState {
    pub session_id: String,
    pub api_key: String,
    pub last_activity: SystemTime,
}

impl SessionState {
    pub fn new(api_key: String) -> Self {
        Self {
            session_id: Uuid::new_v4().to_string(),
            api_key,
            last_activity: SystemTime::now(),
        }
    }

    pub fn is_expired(&self, timeout: Duration) -> bool {
        self.last_activity
            .elapsed()
            .map(|elapsed| elapsed > timeout)
            .unwrap_or(true)
    }

    pub fn update_activity(&mut self) {
        self.last_activity = SystemTime::now();
    }
}

pub struct SessionManager {
    sessions: Arc<DashMap<String, SessionState>>,
    timeout: Duration,
}

impl SessionManager {
    pub fn new(timeout_secs: u64) -> Self {
        Self {
            sessions: Arc::new(DashMap::new()),
            timeout: Duration::from_secs(timeout_secs),
        }
    }

    pub fn new_session(&self, api_key: String) -> SessionState {
        let session = SessionState::new(api_key);
        let session_id = session.session_id.clone();
        self.sessions.insert(session_id.clone(), session.clone());
        session
    }

    pub fn get_session(&self, session_id: &str) -> Option<SessionState> {
        self.sessions.get(session_id).map(|entry| entry.clone())
    }

    pub fn update_activity(&self, session_id: &str) -> bool {
        if let Some(mut entry) = self.sessions.get_mut(session_id) {
            entry.update_activity();
            true
        } else {
            false
        }
    }

    pub fn remove_session(&self, session_id: &str) -> bool {
        self.sessions.remove(session_id).is_some()
    }

    pub fn cleanup_expired_sessions(&self) {
        let timeout = self.timeout;
        self.sessions
            .retain(|_id, session| !session.is_expired(timeout));
    }

    pub fn start_cleanup_task(self: Arc<Self>) {
        tokio::spawn(async move {
            loop {
                sleep(Duration::from_secs(60)).await;
                self.cleanup_expired_sessions();
            }
        });
    }
}
