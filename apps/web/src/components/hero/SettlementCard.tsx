export function SettlementCard() {
  return (
    <div
      className="rounded-[16px] bg-white p-3.5"
      style={{
        boxShadow: "0 16px 48px rgba(0,0,0,0.16), 0 2px 8px rgba(0,0,0,0.08)",
        border: "1px solid rgba(0,0,0,0.05)",
      }}
    >
      <p className="text-[9px] font-semibold uppercase tracking-wide text-gray-400 mb-2">Settlement</p>
      <p className="text-[22px] font-bold text-gray-900 leading-none mb-1.5">$750.00</p>

      <div className="flex items-center gap-1 mb-2.5">
        <svg width="11" height="11" viewBox="0 0 24 24" fill="none" stroke="#16A34A" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round" aria-hidden="true">
          <path d="M20 6L9 17l-5-5" />
        </svg>
        <span className="text-[10px] font-medium" style={{ color: "#15803D" }}>Milestone approved</span>
      </div>

      <div className="pt-2.5" style={{ borderTop: "1px solid #F3F4F6" }}>
        <p className="text-[9px] text-gray-400 mb-0.5">Released to provider</p>
        <p className="text-[10px] font-semibold text-gray-700">Praiz Digital</p>
      </div>

      <div className="mt-2.5 flex items-center gap-1.5 px-2.5 py-1.5 rounded-[8px]" style={{ background: "#DCFCE7" }}>
        <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="#15803D" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round" aria-hidden="true">
          <path d="M20 6L9 17l-5-5" />
        </svg>
        <span className="text-[10px] font-semibold" style={{ color: "#15803D" }}>Confirmed</span>
      </div>
    </div>
  );
}
