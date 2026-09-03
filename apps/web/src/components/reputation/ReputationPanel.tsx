const stats = [
  { label: "Completed agreements", value: "128", highlight: false },
  { label: "On-time completion", value: "96.8%", highlight: true },
  { label: "Dispute rate", value: "1.4%", highlight: false },
  { label: "Settled volume", value: "$84,200", highlight: true },
];

export function ReputationPanel() {
  return (
    <div
      className="w-full max-w-[480px] rounded-[20px] bg-white p-6 sm:p-8"
      style={{
        boxShadow: "0 8px 40px rgba(0,0,0,0.08), 0 1px 4px rgba(0,0,0,0.04)",
        border: "1px solid rgba(0,0,0,0.06)",
      }}
    >
      {/* Score header */}
      <div className="flex items-center gap-6 mb-7 pb-6" style={{ borderBottom: "1px solid #F3F4F6" }}>
        {/* Ring */}
        <div className="relative w-20 h-20 flex-shrink-0">
          <svg viewBox="0 0 36 36" className="w-full h-full -rotate-90" aria-hidden="true">
            <circle cx="18" cy="18" r="15.5" fill="none" stroke="#EFF6FF" strokeWidth="2.5" />
            <circle
              cx="18" cy="18" r="15.5"
              fill="none"
              stroke="url(#rep-grad)"
              strokeWidth="2.5"
              strokeDasharray="97 100"
              strokeLinecap="round"
            />
            <defs>
              <linearGradient id="rep-grad" x1="0%" y1="0%" x2="100%" y2="0%">
                <stop offset="0%" stopColor="#1254D8" />
                <stop offset="100%" stopColor="#2F70E8" />
              </linearGradient>
            </defs>
          </svg>
          <div className="absolute inset-0 flex flex-col items-center justify-center">
            <span className="text-[16px] font-bold text-gray-900 leading-none">98.7</span>
          </div>
        </div>
        {/* Label */}
        <div>
          <div className="flex items-center gap-2 mb-1">
            <span
              className="text-[11px] font-bold px-2.5 py-0.5 rounded-full"
              style={{ background: "#DCFCE7", color: "#15803D" }}
            >
              Excellent
            </span>
          </div>
          <p className="text-[22px] font-bold text-gray-900 leading-tight">Business Reputation</p>
          <p className="text-[12px] text-gray-400 mt-0.5">Derived from verified settlements</p>
        </div>
      </div>

      {/* Stats grid */}
      <div className="grid grid-cols-2 gap-3">
        {stats.map((s) => (
          <div
            key={s.label}
            className="p-4 rounded-[12px]"
            style={{
              background: s.highlight ? "#EFF6FF" : "#F9FAFB",
              border: `1px solid ${s.highlight ? "#DBEAFE" : "#F3F4F6"}`,
            }}
          >
            <p className="text-[22px] font-bold mb-1" style={{ color: s.highlight ? "#1254D8" : "#111827" }}>
              {s.value}
            </p>
            <p className="text-[11px] text-gray-500 leading-snug">{s.label}</p>
          </div>
        ))}
      </div>

      {/* Footer note */}
      <p className="text-[11px] text-gray-400 mt-5 pt-4" style={{ borderTop: "1px solid #F3F4F6" }}>
        Score calculated from 128 completed on-chain settlement records.
      </p>
    </div>
  );
}
