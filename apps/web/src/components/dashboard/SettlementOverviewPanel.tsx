export function SettlementOverviewPanel() {
  const bars = [40, 65, 50, 80, 60, 90, 72, 85, 68, 95, 78, 88];
  const maxH = 80;

  return (
    <div
      className="w-full max-w-[440px] rounded-[20px] bg-white p-6"
      style={{
        boxShadow: "0 8px 40px rgba(0,0,0,0.08), 0 1px 4px rgba(0,0,0,0.04)",
        border: "1px solid rgba(0,0,0,0.06)",
      }}
    >
      {/* Header */}
      <div className="flex items-start justify-between mb-1">
        <div>
          <p className="text-[11px] font-semibold uppercase tracking-wide text-gray-400 mb-1">
            Settlement Overview
          </p>
          <p className="text-[32px] font-bold text-gray-900 leading-none">$24,860</p>
        </div>
        <span
          className="text-[11px] font-semibold px-2.5 py-1 rounded-full mt-1"
          style={{ background: "#DCFCE7", color: "#15803D" }}
        >
          +12.4%
        </span>
      </div>
      <p className="text-[12px] text-gray-400 mb-5">Total settled this month</p>

      {/* Time filters */}
      <div className="flex items-center gap-1 mb-5">
        {["Today", "Week", "Month", "Year"].map((f, i) => (
          <button
            key={f}
            className={`px-3 py-1 text-[11px] font-medium rounded-full transition-colors ${
              i === 2
                ? "bg-[#EFF6FF] text-[#1254D8]"
                : "text-gray-400 hover:text-gray-600 hover:bg-gray-50"
            }`}
          >
            {f}
          </button>
        ))}
      </div>

      {/* Bar chart */}
      <div className="flex items-end gap-1.5 mb-6 h-[88px]" aria-label="Settlement chart">
        {bars.map((v, i) => (
          <div key={i} className="flex-1 flex flex-col items-center justify-end h-full">
            <div
              className="w-full rounded-t-[4px] transition-all"
              style={{
                height: `${(v / 100) * maxH}px`,
                background:
                  i === bars.length - 3
                    ? "linear-gradient(180deg, #1254D8 0%, #2F70E8 100%)"
                    : i === bars.length - 1 || i === bars.length - 2
                    ? "linear-gradient(180deg, #60A5FA 0%, #93C5FD 100%)"
                    : "#EFF6FF",
              }}
            />
          </div>
        ))}
      </div>

      {/* Stats row */}
      <div className="grid grid-cols-2 sm:grid-cols-4 gap-3">
        {[
          { label: "Incoming", value: "$14,200", color: "#1254D8" },
          { label: "Escrowed", value: "$7,450", color: "#6366F1" },
          { label: "Released", value: "$3,210", color: "#15803D" },
          { label: "Disputed", value: "$0", color: "#6B7280" },
        ].map((s) => (
          <div key={s.label} className="bg-gray-50 rounded-[10px] p-2.5">
            <p className="text-[9px] font-medium text-gray-400 mb-1">{s.label}</p>
            <p className="text-[13px] font-bold" style={{ color: s.color }}>
              {s.value}
            </p>
          </div>
        ))}
      </div>
    </div>
  );
}
