import { useAppStore } from "../store/useAppStore";

const STATUS_ICON: Record<string, string> = {
  success: "✓",
  failed: "✗",
  skipped: "–",
  cancelled: "⊘",
};

const STATUS_CLASS: Record<string, string> = {
  success: "step-success",
  failed: "step-failed",
  skipped: "step-skipped",
  cancelled: "step-cancelled",
};

export function StatusPanel() {
  const { isRunning, progress, steps, currentStep } = useAppStore();

  return (
    <div className="status-panel">
      <div className="progress-bar-wrap">
        <div
          className="progress-bar-fill"
          style={{ width: `${Math.round(progress * 100)}%` }}
        />
      </div>
      <div className="progress-label">
        {isRunning ? `${Math.round(progress * 100)}%` : "Ready"}
      </div>

      {isRunning && currentStep && (
        <div className="step-running">
          <span className="step-running-dot" />
          <span className="step-running-name">{currentStep}</span>
          <span className="step-running-dots" />
        </div>
      )}

      <div className="step-list">
        {steps.map((step, i) => (
          <div key={i} className={`step-row ${STATUS_CLASS[step.status] ?? ""}`}>
            <span className="step-icon">{STATUS_ICON[step.status] ?? "?"}</span>
            <span className="step-name">{step.name}</span>
            {step.message && step.message !== "Dry Run" && (
              <span className="step-msg">{step.message}</span>
            )}
          </div>
        ))}
      </div>
    </div>
  );
}
