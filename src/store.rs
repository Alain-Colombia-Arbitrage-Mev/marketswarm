use crate::schema::{AgentResult, Evidence};
use anyhow::{ensure, Context, Result};
use chrono::Utc;
use rusqlite::{params, Connection, OptionalExtension};
use std::path::{Path, PathBuf};

#[derive(Clone)]
pub struct Store {
    path: PathBuf,
}
impl Store {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
            std::fs::create_dir_all(parent)?;
        }
        let s = Self { path };
        s.conn()?.execute_batch("PRAGMA journal_mode=WAL;
            CREATE TABLE IF NOT EXISTS evidence (id TEXT PRIMARY KEY, hash TEXT NOT NULL, body TEXT NOT NULL);
            CREATE TABLE IF NOT EXISTS runs (id TEXT PRIMARY KEY, request TEXT NOT NULL, created TEXT NOT NULL, status TEXT NOT NULL, report TEXT);
            CREATE TABLE IF NOT EXISTS tasks (run_id TEXT NOT NULL, role TEXT NOT NULL, body TEXT NOT NULL, PRIMARY KEY(run_id,role));
            CREATE TABLE IF NOT EXISTS events (seq INTEGER PRIMARY KEY AUTOINCREMENT, run_id TEXT NOT NULL, at TEXT NOT NULL, body TEXT NOT NULL);
            CREATE TABLE IF NOT EXISTS leases (run_id TEXT PRIMARY KEY, owner TEXT NOT NULL, expires INTEGER NOT NULL);
            CREATE TABLE IF NOT EXISTS budget (run_id TEXT PRIMARY KEY, reserved REAL NOT NULL);")?;
        Ok(s)
    }
    fn conn(&self) -> Result<Connection> {
        let c = Connection::open(&self.path)?;
        c.busy_timeout(std::time::Duration::from_secs(10))?;
        Ok(c)
    }
    pub fn import(&self, evidence: &[Evidence]) -> Result<usize> {
        let mut c = self.conn()?;
        let tx = c.transaction()?;
        let mut count = 0;
        for e in evidence {
            e.validate()?;
            let body = serde_json::to_string(e)?;
            let existing: Option<String> = tx
                .query_row("SELECT body FROM evidence WHERE id=?", [&e.id], |r| {
                    r.get(0)
                })
                .optional()?;
            if let Some(old) = existing {
                let mut previous: Evidence = serde_json::from_str(&old)?;
                previous.collected_at = e.collected_at;
                ensure!(serde_json::to_string(&previous)? == body, "Evidence id already exists with different metadata/content; version the record");
                continue;
            }
            count += tx.execute(
                "INSERT INTO evidence VALUES (?,?,?)",
                params![e.id, e.content_hash(), body],
            )?;
        }
        tx.commit()?;
        Ok(count)
    }
    pub fn evidence(&self, ids: &[String]) -> Result<Vec<Evidence>> {
        let c = self.conn()?;
        ids.iter()
            .map(|id| {
                let body: String = c
                    .query_row("SELECT body FROM evidence WHERE id=?", [id], |r| r.get(0))
                    .with_context(|| format!("Missing evidence ID: {id}"))?;
                Ok(serde_json::from_str(&body)?)
            })
            .collect()
    }
    pub fn list_evidence(&self) -> Result<Vec<serde_json::Value>> {
        let c = self.conn()?;
        let mut q = c.prepare("SELECT body FROM evidence ORDER BY id")?;
        let rows = q.query_map([], |r| r.get::<_, String>(0))?;
        rows.map(|r| { let e: Evidence = serde_json::from_str(&r?)?; Ok(serde_json::json!({"id": e.id,"provider":e.provider,"kind":e.kind,"observed_at":e.observed_at,"source":e.source,"synthetic_fixture":e.synthetic_fixture})) }).collect()
    }
    pub fn start(&self, id: &str, request: &str) -> Result<()> {
        self.conn()?.execute(
            "INSERT OR IGNORE INTO runs VALUES (?,?,?,'running',NULL)",
            params![id, request, Utc::now().to_rfc3339()],
        )?;
        Ok(())
    }
    pub fn task(&self, run: &str, role: &str) -> Result<Option<AgentResult>> {
        let body: Option<String> = self
            .conn()?
            .query_row(
                "SELECT body FROM tasks WHERE run_id=? AND role=?",
                params![run, role],
                |r| r.get(0),
            )
            .optional()?;
        body.map(|s| Ok(serde_json::from_str(&s)?)).transpose()
    }
    pub fn save_task(&self, run: &str, result: &AgentResult) -> Result<()> {
        self.conn()?.execute(
            "INSERT OR REPLACE INTO tasks VALUES (?,?,?)",
            params![run, result.role, serde_json::to_string(result)?],
        )?;
        Ok(())
    }
    pub fn reserve(&self, run: &str, amount: f64, cap: f64) -> Result<()> {
        ensure!(
            amount.is_finite() && amount >= 0. && cap.is_finite() && cap > 0.,
            "Invalid reservation"
        );
        let mut c = self.conn()?;
        let tx = c.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)?;
        tx.execute("INSERT OR IGNORE INTO budget VALUES (?,0)", [run])?;
        let current: f64 =
            tx.query_row("SELECT reserved FROM budget WHERE run_id=?", [run], |r| {
                r.get(0)
            })?;
        ensure!(
            current + amount <= cap + 1e-12,
            "Run cost reservation would exceed budget"
        );
        tx.execute(
            "UPDATE budget SET reserved=reserved+? WHERE run_id=?",
            params![amount, run],
        )?;
        tx.commit()?;
        Ok(())
    }
    pub fn reserved(&self, run: &str) -> Result<f64> {
        Ok(self
            .conn()?
            .query_row("SELECT reserved FROM budget WHERE run_id=?", [run], |r| {
                r.get(0)
            })
            .optional()?
            .unwrap_or(0.))
    }
    pub fn event(&self, run: &str, event: &serde_json::Value) -> Result<()> {
        self.conn()?.execute(
            "INSERT INTO events (run_id,at,body) VALUES (?,?,?)",
            params![run, Utc::now().to_rfc3339(), serde_json::to_string(event)?],
        )?;
        Ok(())
    }
    pub fn finish(&self, run: &str, status: &str, report: &serde_json::Value) -> Result<()> {
        self.conn()?.execute(
            "UPDATE runs SET status=?, report=? WHERE id=?",
            params![status, serde_json::to_string(report)?, run],
        )?;
        Ok(())
    }
    pub fn report(&self, run: &str) -> Result<Option<serde_json::Value>> {
        let body: Option<Option<String>> = self
            .conn()?
            .query_row("SELECT report FROM runs WHERE id=?", [run], |r| r.get(0))
            .optional()?;
        body.flatten()
            .map(|s| Ok(serde_json::from_str(&s)?))
            .transpose()
    }
    pub fn lease(&self, run: &str, ttl: u64) -> Result<Lease> {
        let owner = format!(
            "{}-{}",
            std::process::id(),
            Utc::now().timestamp_nanos_opt().unwrap_or_default()
        );
        let mut c = self.conn()?;
        let tx = c.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)?;
        tx.execute(
            "DELETE FROM leases WHERE expires < ?",
            [Utc::now().timestamp()],
        )?;
        let inserted = tx.execute(
            "INSERT OR IGNORE INTO leases VALUES (?,?,?)",
            params![run, owner, Utc::now().timestamp() + ttl as i64],
        )?;
        ensure!(
            inserted == 1,
            "This run is active in another worker; wait for its lease or inspect its report"
        );
        tx.commit()?;
        Ok(Lease {
            store: self.clone(),
            run: run.into(),
            owner,
        })
    }
}
pub struct Lease {
    store: Store,
    run: String,
    owner: String,
}
impl Drop for Lease {
    fn drop(&mut self) {
        if let Ok(c) = self.store.conn() {
            let _ = c.execute(
                "DELETE FROM leases WHERE run_id=? AND owner=?",
                params![self.run, self.owner],
            );
        }
    }
}
