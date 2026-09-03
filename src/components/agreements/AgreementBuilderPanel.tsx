export function AgreementBuilderPanel() {
  const milestones = [
    { num: "01", name: "Discovery", amount: "$500" },
    { num: "02", name: "Design", amount: "$750" },
    { num: "03", name: "Delivery", amount: "$1,250" },
  ];

  return (
    <div
      className="w-full max-w-[400px] rounded-[20px] bg-white p-6"
      style={{
        boxShadow: "0 8px 40px rgba(0,0,0,0.08), 0 1px 4px rgba(0,0,0,0.04)",
        border: "1px solid rgba(0,0,0,0.06)",
      }}
    >
      <p className="text-[15px] font-bold text-gray-900 mb-5">Create Agreement</p>

      {/* Form fields */}
      <div className="space-y-3 mb-5">
        {[
          { label: "Client", placeholder: "Acme Studio" },
          { label: "Provider", placeholder: "Praiz Digital" },
          { label: "Asset", placeholder: "USDC" },
          { label: "Total Amount", placeholder: "$2,500" },
        ].map((f) => (
          <div key={f.label}>
            <label className="block text-[10px] font-semibold text-gray-500 mb-1 uppercase tracking-wide">
              {f.label}
            </label>
            <div
              className="w-full px-3 py-2.5 rounded-[8px] text-[12px] text-gray-400"
              style={{ background: "#F9FAFB", border: "1px solid #E5E7EB" }}
              aria-label={f.label}
            >
              {f.placeholder}
            </div>
          </div>
        ))}
      </div>

      {/* Milestones */}
      <div className="mb-5">
        <p className="text-[10px] font-semibold uppercase tracking-wide text-gray-500 mb-2.5">
          Milestones
        </p>
        <div className="space-y-2">
          {milestones.map((m) => (
            <div
              key={m.num}
              className="flex items-center gap-3 px-3 py-2.5 rounded-[8px]"
              style={{ background: "#F9FAFB", border: "1px solid #E5E7EB" }}
            >
              <span className="text-[10px] font-bold text-gray-400 w-5 flex-shrink-0">
                {m.num}
              </span>
              <span className="flex-1 text-[12px] font-medium text-gray-700">
                {m.name}
              </span>
              <span className="text-[12px] font-bold text-gray-900">{m.amount}</span>
            </div>
          ))}
        </div>
      </div>

      {/* Conditions */}
      <div className="grid grid-cols-2 gap-3 mb-5">
        <div>
          <p className="text-[9px] font-semibold uppercase tracking-wide text-gray-400 mb-1.5">
            Payment Condition
          </p>
          <div
            className="px-3 py-2 rounded-[8px] text-[11px] font-medium text-[#1254D8]"
            style={{ background: "#EFF6FF", border: "1px solid #DBEAFE" }}
          >
            Release after approval
          </div>
        </div>
        <div>
          <p className="text-[9px] font-semibold uppercase tracking-wide text-gray-400 mb-1.5">
            Dispute Window
          </p>
          <div
            className="px-3 py-2 rounded-[8px] text-[11px] font-medium text-gray-700"
            style={{ background: "#F9FAFB", border: "1px solid #E5E7EB" }}
          >
            7 days
          </div>
        </div>
      </div>

      {/* CTA */}
      <button
        className="w-full py-2.5 text-[13px] font-semibold text-white rounded-[10px] transition-all hover:opacity-90 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-500"
        style={{ background: "linear-gradient(135deg, #1254D8 0%, #2F70E8 100%)" }}
      >
        Create Agreement
      </button>
    </div>
  );
}
