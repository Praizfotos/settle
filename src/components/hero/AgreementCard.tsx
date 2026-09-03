export function AgreementCard() {
  const milestones = [
    { name: "Discovery", amount: "$500", status: "completed" },
    { name: "Design", amount: "$750", status: "active" },
    { name: "Development", amount: "$1,250", status: "pending" },
  ];

  return (
    <div
      className="rounded-[18px] bg-white p-4"
      style={{
        boxShadow: "0 20px 60px rgba(0,0,0,0.18), 0 4px 16px rgba(0,0,0,0.10)",
        border: "1px solid rgba(0,0,0,0.05)",
      }}
    >
      {/* Header */}
      <div className="flex items-start justify-between mb-3">
        <div>
          <p className="text-[10px] font-medium text-gray-400 mb-0.5 uppercase tracking-wide">Agreement #1048</p>
          <p className="text-[13px] font-semibold text-gray-900 leading-tight">Website Development</p>
        </div>
        <span className="text-[10px] font-semibold px-2 py-0.5 rounded-full" style={{ background: "#DCFCE7", color: "#15803D" }}>
          Active
        </span>
      </div>

      {/* Parties */}
      <div className="flex items-center gap-2 mb-3 pb-3" style={{ borderBottom: "1px solid #F3F4F6" }}>
        <div className="flex-1">
          <p className="text-[9px] text-gray-400 mb-0.5">Client</p>
          <p className="text-[11px] font-semibold text-gray-700">Acme Studio</p>
        </div>
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#9CA3AF" strokeWidth="2" strokeLinecap="round" aria-hidden="true">
          <path d="M5 12h14M12 5l7 7-7 7" />
        </svg>
        <div className="flex-1 text-right">
          <p className="text-[9px] text-gray-400 mb-0.5">Provider</p>
          <p className="text-[11px] font-semibold text-gray-700">Praiz Digital</p>
        </div>
      </div>

      {/* Total */}
      <div className="flex items-center justify-between mb-3">
        <p className="text-[10px] text-gray-400">Total</p>
        <div className="flex items-center gap-1.5">
          <p className="text-[15px] font-bold text-gray-900">$2,500</p>
          <span className="text-[9px] font-semibold px-1.5 py-0.5 rounded" style={{ background: "#EFF6FF", color: "#1254D8" }}>USDC</span>
        </div>
      </div>

      {/* Milestones */}
      <div className="space-y-2">
        <p className="text-[9px] font-semibold uppercase tracking-wide text-gray-400">Milestones</p>
        {milestones.map((m) => (
          <MilestoneRow key={m.name} {...m} />
        ))}
      </div>

      {/* Footer */}
      <div className="mt-3 pt-3 flex items-center gap-1.5" style={{ borderTop: "1px solid #F3F4F6" }}>
        <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="#16A34A" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round" aria-hidden="true">
          <path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z" />
        </svg>
        <span className="text-[10px] font-medium" style={{ color: "#16A34A" }}>Funds secured in escrow</span>
      </div>
    </div>
  );
}

function MilestoneRow({ name, amount, status }: { name: string; amount: string; status: string }) {
  const statusStyles: Record<string, { bg: string; text: string; label: string }> = {
    completed: { bg: "#DCFCE7", text: "#15803D", label: "Completed" },
    active:    { bg: "#DBEAFE", text: "#1254D8", label: "In progress" },
    pending:   { bg: "#F3F4F6", text: "#6B7280", label: "Pending" },
  };
  const s = statusStyles[status];

  return (
    <div className="flex items-center justify-between">
      <div className="flex items-center gap-1.5">
        {status === "completed" ? (
          <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="#15803D" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round" aria-hidden="true">
            <path d="M20 6L9 17l-5-5" />
          </svg>
        ) : status === "active" ? (
          <div className="w-3 h-3 rounded-full border-2 border-[#1254D8] flex items-center justify-center">
            <div className="w-1.5 h-1.5 rounded-full bg-[#1254D8]" />
          </div>
        ) : (
          <div className="w-3 h-3 rounded-full border border-gray-300" />
        )}
        <span className="text-[11px] text-gray-700 font-medium">{name}</span>
      </div>
      <div className="flex items-center gap-2">
        <span className="text-[11px] font-semibold text-gray-900">{amount}</span>
        <span className="text-[9px] font-medium px-1.5 py-0.5 rounded-full" style={{ background: s.bg, color: s.text }}>
          {s.label}
        </span>
      </div>
    </div>
  );
}
