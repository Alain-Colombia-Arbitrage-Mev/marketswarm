//! Minimal Model Context Protocol (MCP) client over the stdio transport.
//!
//! This is intentionally small: it supports the three calls needed to drive a
//! design tool's MCP server (e.g. Figma's `server-figma` or pen.dev) from the
//! command line: `initialize`, `tools/list` and `tools/call`. Messages are
//! JSON-RPC 2.0, one newline-delimited JSON object per line.

use anyhow::{bail, ensure, Context, Result};
use serde_json::{json, Value};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, Command};

pub const PROTOCOL_VERSION: &str = "2024-11-05";

pub struct Mcp {
    child: Child,
    stdin: tokio::process::ChildStdin,
    lines: tokio::io::Lines<BufReader<tokio::process::ChildStdout>>,
    next_id: u64,
}

impl Mcp {
    /// Spawns `command` through `sh -c`, so compound commands such as
    /// `npx -y @modelcontextprotocol/server-figma --stdio` work unchanged.
    /// The child inherits the environment (API keys, PATH, etc.).
    pub async fn connect(command: &str) -> Result<Self> {
        ensure!(
            !command.trim().is_empty() && command.len() <= 2000,
            "MCP server command is empty or too long"
        );
        let mut child = Command::new("sh")
            .arg("-c")
            .arg(command)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::inherit())
            .spawn()
            .context("Failed to spawn MCP server command")?;
        let stdin = child.stdin.take().context("MCP server stdin unavailable")?;
        let stdout = child.stdout.take().context("MCP server stdout unavailable")?;
        Ok(Self {
            child,
            stdin,
            lines: BufReader::new(stdout).lines(),
            next_id: 1,
        })
    }

    async fn send(&mut self, payload: &Value) -> Result<()> {
        let mut line = serde_json::to_vec(payload)?;
        line.push(b'\n');
        self.stdin.write_all(&line).await.context("MCP write failed")?;
        self.stdin.flush().await.context("MCP flush failed")?;
        Ok(())
    }

    async fn receive(&mut self, id: u64) -> Result<Value> {
        while let Some(line) = self
            .lines
            .next_line()
            .await
            .context("MCP read failed")?
        {
            let msg: Value =
                serde_json::from_str(&line).context("MCP sent an invalid JSON line")?;
            if msg.get("id").and_then(Value::as_u64) == Some(id) {
                if let Some(err) = msg.get("error") {
                    bail!(
                        "MCP returned an error: {}",
                        serde_json::to_string(err).unwrap_or_else(|_| "<unprintable>".into())
                    );
                }
                return Ok(msg.get("result").cloned().unwrap_or(Value::Null));
            }
            // A server->client request must be answered or the server may block.
            if msg.get("method").is_some() && msg.get("id").is_some() {
                let _ = self
                    .send(&json!({"jsonrpc":"2.0","id":msg["id"],
                        "error":{"code":-32601,"message":"Method not supported by the market-swarm MCP client"}}))
                    .await;
            }
            // Otherwise it is a notification; ignore it.
        }
        bail!("MCP server closed the stream before responding")
    }

    pub async fn request(&mut self, method: &str, params: Value) -> Result<Value> {
        let id = self.next_id;
        self.next_id = self.next_id.wrapping_add(1);
        self.send(&json!({"jsonrpc":"2.0","id":id,"method":method,"params":params}))
            .await?;
        self.receive(id).await
    }

    /// `initialize` plus the required `notifications/initialized` handshake.
    pub async fn initialize(&mut self) -> Result<Value> {
        let result = self
            .request(
                "initialize",
                json!({"protocolVersion": PROTOCOL_VERSION, "capabilities": {},
                    "clientInfo": {"name": "market-swarm", "version": env!("CARGO_PKG_VERSION")}}),
            )
            .await?;
        self.send(&json!({"jsonrpc":"2.0","method":"notifications/initialized"}))
            .await?;
        Ok(result)
    }

    pub async fn list_tools(&mut self) -> Result<Value> {
        self.request("tools/list", json!({})).await
    }

    pub async fn call_tool(&mut self, name: &str, args: Value) -> Result<Value> {
        self.request("tools/call", json!({"name": name, "arguments": args})).await
    }
}

impl Drop for Mcp {
    fn drop(&mut self) {
        // Best-effort cleanup; the child may already have exited.
        let _ = self.child.start_kill();
    }
}
