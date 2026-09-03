export function TrustScoreCard() {
  return (
    <div
      className="rounded-[16px] bg-white p-3.5"
      style={{
        boxShadow: "0 16px 48px rgba(0,0,0,0.16), 0 2px 8px rgba(0,0,0,0.08)",
        border: "1px solid rgba(0,0,0,0.05)",
      }}
    >
      <p className="text-[9px] font-semibold uppercase tracking-wide text-gray-400 mb-2">Trust Score</p>

      {/* Score ring */}
      <div className="flex items-center gap-2.5 mb-2">
        <div className="relative w-10 h-10 flex-shrink-0">
          <svg viewBox="0 0 36 36" className="w-full h-full -rotate-90" aria-hidden="true">
            <circle cx="18" cy="18" r="15.5" fill="none" stroke="#EFF6FF" strokeWidth="3" />
            <circle
              cx="18" cy="18" r="15.5"
              fill="none"
              stroke="#1254D8"
              strokeWidth="3"
              strokeDasharray="97.4 100"
              strokeLinecap="round"
            />
          </svg>
          <span className="absolute inset-0 flex items-center justify-center text-[8px] font-bold text-gray-900">
            98
          </span>
        </div>
        <div>
          <p className="text-[18px] font-bold text-gray-900 leading-none">98.4</p>
          <p className="text-[9px] font-medium mt-0.5" style={{ color: "#15803D" }}>Excellent</p>
        </div>
      </div>

      <div className="space-y-1">
        <div className="flex justify-between items-center">
          <span className="text-[9px] text-gray-400">Agreements</span>
          <span className="text-[9px] font-semibold text-gray-700">128</span>
        </div>
        <div className="flex justify-between items-center">
          <span className="text-[9px] text-gray-400">On-time</span>
          <span className="text-[9px] font-semibold" style={{ color: "#15803D" }}>96.8%</span>
        </div>
      </div>
    </div>
  );
}
