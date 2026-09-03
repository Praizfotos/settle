const timeline = [
  { label: "Dispute opened", done: true },
  { label: "Evidence submitted", done: true },
  { label: "Counter-evidence received", done: true },
  { label: "Under review", done: false, active: true },
  { label: "Resolution", done: false },
];

export function DisputePanel() {
  return (
    <div
      className="w-full max-w-[400px] rounded-[20px] bg-white p-6"
      style={{
        boxShadow: "0 8px 40px rgba(0,0,0,0.08), 0 1px 4px rgba(0,0,0,0.04)",
        border: "1px solid rgba(0,0,0,0.06)",
      }}
    >
      {/* Header */}
      <div className="flex items-start justify-between mb-4">
        <div>
          <p className="text-[11px] font-semibold uppercase tracking-wide text-gray-400 mb-1">
            Dispute Center
          </p>
          <p className="text-[15px] font-bold text-gray-900">Dispute #204</p>
        </div>
        <span
          className="text-[10px] font-semibold px-2.5 py-1 rounded-full"
          style={{ background: "#FFFBEB", color: "#92400E" }}
        >
          Under Review
        </span>
      </div>

      {/* Amount */}
      <div
        className="flex items-center justify-between p-3.5 rounded-[12px] mb-5"
        style={{ background: "#FFFBEB", border: "1px solid #FDE68A" }}
      >
        <div>
          <p className="text-[9px] font-semibold uppercase tracking-wide text-amber-600 mb-0.5">
            Disputed Amount
          </p>
          <p className="text-[22px] font-bold text-gray-900">$1,200</p>
        </div>
        <svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="#D97706" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round" aria-hidden="true">
          <path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z" />
          <line x1="12" y1="9" x2="12" y2="13" /><line x1="12" y1="17" x2="12.01" y2="17" />
        </svg>
      </div>

      {/* Timeline */}
      <div className="relative mb-5">
        <div
          className="absolute left-[7px] top-2 bottom-2 w-px"
          style={{ background: "#E5E7EB" }}
          aria-hidden="true"
        />
        <div className="space-y-3.5">
          {timeline.map((step, i) => (
            <div key={i} className="flex items-center gap-3 relative">
              <div
                className="w-3.5 h-3.5 rounded-full flex-shrink-0 relative z-10 flex items-center justify-center"
                style={{
                  background: step.done ? "#D97706" : step.active ? "white" : "#F3F4F6",
                  border: step.active ? "2px solid #D97706" : step.done ? "none" : "1.5px solid #D1D5DB",
                }}
              >
                {step.done && (
                  <svg width="8" height="8" viewBox="0 0 24 24" fill="none" stroke="white" strokeWidth="3" strokeLinecap="round" strokeLinejoin="round" aria-hidden="true">
                    <path d="M20 6L9 17l-5-5" />
                  </svg>
                )}
                {step.active && <div className="w-1.5 h-1.5 rounded-full bg-amber-600" />}
              </div>
              <span
                className="text-[12px] font-medium"
                style={{ color: step.done ? "#111827" : step.active ? "#92400E" : "#9CA3AF" }}
              >
                {step.label}
              </span>
            </div>
          ))}
        </div>
      </div>

      {/* Actions */}
      <div className="flex gap-2.5">
        <button
          className="flex-1 py-2.5 text-[12px] font-semibold text-white rounded-[10px] transition-opacity hover:opacity-90 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-amber-500"
          style={{ background: "linear-gradient(135deg, #D97706, #F59E0B)" }}
        >
          Submit Evidence
        </button>
        <button
          className="flex-1 py-2.5 text-[12px] font-semibold text-gray-700 rounded-[10px] hover:bg-gray-100 transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-gray-300"
          style={{ background: "#F9FAFB", border: "1px solid #E5E7EB" }}
        >
          View Agreement
        </button>
      </div>
    </div>
  );
}
