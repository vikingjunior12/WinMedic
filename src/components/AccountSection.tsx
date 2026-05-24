import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useAppStore } from "../store/useAppStore";

export function AccountSection() {
  const { username, password, confirmPassword, add_to_admins, set } = useAppStore();
  const [showPw, setShowPw] = useState(false);
  const [genLoading, setGenLoading] = useState(false);

  const passwordsMatch = password === confirmPassword;
  const hasUser = username.trim().length > 0;

  async function generatePw() {
    setGenLoading(true);
    try {
      const pw = await invoke<string>("generate_password", { length: 16 });
      set("password", pw);
      set("confirmPassword", pw);
    } catch {
      // ignore
    } finally {
      setGenLoading(false);
    }
  }

  return (
    <section className="card">
      <h2 className="card-title">User Account</h2>
      <div className="form-group">
        <label>Username</label>
        <input
          type="text"
          value={username}
          onChange={(e) => set("username", e.target.value)}
          placeholder="e.g. student01"
          maxLength={20}
        />
      </div>
      <div className="form-group">
        <label>Password</label>
        <div className="input-row">
          <input
            type={showPw ? "text" : "password"}
            value={password}
            onChange={(e) => set("password", e.target.value)}
            placeholder="Password"
            disabled={!hasUser}
          />
          <button
            className="btn-icon"
            onClick={() => setShowPw((v) => !v)}
            title={showPw ? "Hide" : "Show"}
          >
            {showPw ? "🙈" : "👁"}
          </button>
          <button
            className="btn-icon"
            onClick={generatePw}
            disabled={genLoading}
            title="Generate password"
          >
            🔑
          </button>
        </div>
      </div>
      <div className="form-group">
        <label>Confirm Password</label>
        <input
          type={showPw ? "text" : "password"}
          value={confirmPassword}
          onChange={(e) => set("confirmPassword", e.target.value)}
          placeholder="Repeat password"
          disabled={!hasUser}
          className={hasUser && confirmPassword && !passwordsMatch ? "input-error" : ""}
        />
        {hasUser && confirmPassword && !passwordsMatch && (
          <span className="field-error">Passwords do not match</span>
        )}
      </div>
      <label className="checkbox-row">
        <input
          type="checkbox"
          checked={add_to_admins}
          onChange={(e) => set("add_to_admins", e.target.checked)}
          disabled={!hasUser}
        />
        <span>Add to "Administrators" group</span>
      </label>
    </section>
  );
}
