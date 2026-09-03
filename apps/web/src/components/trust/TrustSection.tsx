const pillars = [
  {
    icon: (
      <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.8" strokeLinecap="round" strokeLinejoin="round" aria-hidden="true">
        <path d="M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5" />
      </svg>
    ),
    title: "Programmable settlement",
    description: "Payment conditions are encoded at creation time. Rules execute deterministically — not manually.",
  },
  {
    icon: (
      <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.8" strokeLinecap="round" strokeLinejoin="round" aria-hidden="true">
        <path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z" /><circle cx="12" cy="12" r="3" />
      </svg>
    ),
    title: "Transparent state",
    description: "Agreement state is verifiable on-chain. Both parties always see the same source of truth.",
  },
  {
    icon: (
      <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.8" strokeLinecap="round" strokeLinejoin="round" aria-hidden="true">
        <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" /><polyline points="14 2 14 8 20 8" /><line x1="16" y1="13" x2="8" y2="13" /><line x1="16" y1="17" x2="8" y2="17" /><polyline points="10 9 9 9 8 9" />
      </svg>
    ),
    title: "Auditable activity",
    description: "Every event — funding, milestone approvals, releases, disputes — is indexed and permanently retrievable.",
  },
  {
    icon: (
      <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.8" strokeLinecap="round" strokeLinejoin="round" aria-hidden="true">
        <rect x="3" y="11" width="18" height="11" rx="2" ry="2" /><path d="M7 11V7a5 5 0 0 1 10 0v4" />
      </svg>
    ),
    title: "Permission-aware workflows",
    description: "Fine-grained authorization separates who can approve, fund, and release. Multi-party control built in.",
  },
];

export function TrustSection() {
  return (
    <section
      id="security"
      aria-labelledby="trust-heading"
      className="py-20 sm:py-28 bg-white"
    >
      <div className="max-w-[1280px] mx-auto px-6 md:px-10">
        {/* Heading */}
        <div className="text-center mb-14 sm:mb-16">
          <p className="text-[11px] font-semibold uppercase tracking-[0.18em] text-[#1254D8] mb-4">
            Infrastructure
          </p>
          <h2
            id="trust-heading"
            className="font-bold tracking-[-0.025em] text-gray-900 leading-[1.15]"
            style={{ fontSize: "clamp(28px, 3.5vw, 44px)" }}
          >
            Designed for financial integrity.
          </h2>
        </div>

        {/* Pillars */}
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-5">
          {pillars.map((p) => (
            <div
              key={p.title}
              className="p-6 rounded-[16px] group hover:-translate-y-1 transition-transform duration-200"
              style={{
                background: "#F9FAFB",
                border: "1px solid rgba(0,0,0,0.06)",
              }}
            >
              <div
                className="w-10 h-10 rounded-[10px] flex items-center justify-center mb-4 text-[#1254D8]"
                style={{ background: "#EFF6FF" }}
              >
                {p.icon}
              </div>
              <h3 className="text-[14px] font-bold text-gray-900 mb-2">{p.title}</h3>
              <p className="text-[13px] text-gray-500 leading-relaxed">{p.description}</p>
            </div>
          ))}
        </div>
      </div>
    </section>
  );
}
