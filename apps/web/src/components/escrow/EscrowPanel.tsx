const steps = [
  { label: "Agreement created", done: true },
  { label: "Funds deposited", done: true },
  { label: "Milestone 1 approved", done: true },
  { label: "Milestone 2 in review", done: false, active: true },
  { label: "Final settlement", done: false },
];

export function EscrowPanel() {
  return (
    <div
      className="w-full max-w-[400px] rounded-[20px] bg-white p-6"
      style={{
        boxShadow: "0 8px 40px rgba(0,0,0,0.08), 0 1px 4px rgba(0,0,0,0.04)",
        border: "1px solid rgba(0,0,0,0.06)",
      }}
    >
      {/* Header */}
      <div className="flex items-start justify-between mb-5">
        <div>
          <p className="text-[11px] font-semibold uppercase tracking-wide text-gray-400 mb-1">
            Escrow Balance
          </p>
          <p className="text-[32px] font-bold text-gray-900 leading-none">$12,400</p>
        </div>
        <div
          className="flex items-center gap-1.5 px-3 py-1.5 rounded-full"
          style={{ background: "#DCFCE7" }}
        >
          <svg width="11" height="11" viewBox="0 0 24 24" fill="none" stroke="#15803D" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round" aria-hidden="true">
            <path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z" />
          </svg>
          <span className="text-[10px] font-semibold" style={{ color: "#15803D" }}>
            Funds secured
          </span>
        </div>
      </div>

      {/* Progress bar */}
      <div className="mb-6">
        <div className="flex justify-between mb-1.5">
          <span className="text-[10px] text-gray-400">Progress</span>
          <span className="text-[10px] font-semibold text-gray-600">60%</span>
        </div>
        <div className="h-1.5 rounded-full bg-gray-100">
          <div
            className="h-full rounded-full"
            style={{ width: "60%", background: "linear-gradient(90deg, #1254D8, #2F70E8)" }}
            role="progressbar"
            aria-valuenow={60}
            aria-valuemin={0}
            aria-valuemax={100}
          />
        </div>
      </div>

      {/* Timeline */}
      <div className="relative">
        <div
          className="absolute left-[7px] top-2 bottom-2 w-px"
          style={{ background: "#E5E7EB" }}
          aria-hidden="true"
        />
        <div className="space-y-4">
          {steps.map((step, i) => (
            <div key={i} className="flex items-center gap-3 relative">
              {/* Node */}
              <div
                className="w-3.5 h-3.5 rounded-full flex-shrink-0 relative z-10 flex items-center justify-center"
                style={{
                  background: step.done
                    ? "#1254D8"
                    : step.active
                    ? "white"
                    : "#F3F4F6",
                  border: step.active
                    ? "2px solid #1254D8"
                    : step.done
                    ? "none"
                    : "1.5px solid #D1D5DB",
                }}
              >
                {step.done && (
                  <svg width="8" height="8" viewBox="0 0 24 24" fill="none" stroke="white" strokeWidth="3" strokeLinecap="round" strokeLinejoin="round" aria-hidden="true">
                    <path d="M20 6L9 17l-5-5" />
                  </svg>
                )}
                {step.active && (
                  <div className="w-1.5 h-1.5 rounded-full bg-[#1254D8]" />
                )}
              </div>
              {/* Label */}
              <span
                className="text-[12px] font-medium"
                style={{
                  color: step.done ? "#111827" : step.active ? "#1254D8" : "#9CA3AF",
                }}
              >
                {step.label}
              </span>
              {step.active && (
                <span
                  className="ml-auto text-[9px] font-semibold px-2 py-0.5 rounded-full"
                  style={{ background: "#DBEAFE", color: "#1254D8" }}
                >
                  In progress
                </span>
              )}
            </div>
          ))}
        </div>
      </div>

      {/* Agreement ref */}
      <div
        className="mt-5 pt-4 flex items-center justify-between"
        style={{ borderTop: "1px solid #F3F4F6" }}
      >
        <span className="text-[11px] text-gray-400">Agreement #1048</span>
        <span className="text-[11px] font-medium text-[#1254D8]">View details →</span>
      </div>
    </div>
  );
}
