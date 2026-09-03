export function CTASection() {
  return (
    <section aria-labelledby="cta-heading" className="py-10 sm:py-12">
      <div className="max-w-[1280px] mx-auto px-4 sm:px-6 md:px-10">
        <div
          className="relative overflow-hidden rounded-[28px] px-8 sm:px-16 py-16 sm:py-20 text-center"
          style={{
            background: "linear-gradient(135deg, #0D4FD7 0%, #1254D8 40%, #2F70E8 100%)",
          }}
        >
          {/* Background decoration */}
          <CTADecoration />

          <div className="relative z-10 max-w-[580px] mx-auto">
            <p className="text-[11px] font-semibold uppercase tracking-[0.18em] mb-5" style={{ color: "rgba(255,255,255,0.60)" }}>
              Get started
            </p>
            <h2
              id="cta-heading"
              className="font-bold tracking-[-0.025em] text-white leading-[1.1] mb-5"
              style={{ fontSize: "clamp(28px, 4vw, 48px)" }}
            >
              Build business relationships
              <br />
              around rules, not assumptions.
            </h2>
            <p className="text-[15px] sm:text-[16px] leading-relaxed mb-8" style={{ color: "rgba(255,255,255,0.70)" }}>
              Create your first programmable agreement with Settle. One source of
              truth for both parties, from start to settlement.
            </p>
            <div className="flex flex-col sm:flex-row items-center justify-center gap-3">
              <a
                href="#get-started"
                className="w-full sm:w-auto inline-flex items-center justify-center gap-2 px-7 py-3 text-[14px] font-semibold text-[#1254D8] bg-white rounded-[10px] hover:bg-gray-50 active:scale-[0.98] transition-all duration-150 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-white"
                style={{ boxShadow: "0 2px 8px rgba(0,0,0,0.15)" }}
              >
                Get Started
                <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round" aria-hidden="true">
                  <path d="M5 12h14M12 5l7 7-7 7" />
                </svg>
              </a>
              <a
                href="#developers"
                className="w-full sm:w-auto inline-flex items-center justify-center px-7 py-3 text-[14px] font-semibold rounded-[10px] hover:bg-white/10 active:scale-[0.98] transition-all duration-150 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-white/40"
                style={{ color: "rgba(255,255,255,0.90)", border: "1px solid rgba(255,255,255,0.25)" }}
              >
                Read the Documentation
              </a>
            </div>
          </div>
        </div>
      </div>
    </section>
  );
}

function CTADecoration() {
  return (
    <svg
      className="absolute inset-0 w-full h-full pointer-events-none"
      aria-hidden="true"
    >
      <defs>
        <pattern id="cta-grid" width="48" height="48" patternUnits="userSpaceOnUse">
          <path d="M 48 0 L 0 0 0 48" fill="none" stroke="white" strokeWidth="0.6" />
        </pattern>
      </defs>
      <rect width="100%" height="100%" fill="url(#cta-grid)" opacity="0.06" />
      {/* Settlement network nodes */}
      <circle cx="10%" cy="30%" r="60" fill="white" opacity="0.03" />
      <circle cx="90%" cy="70%" r="80" fill="white" opacity="0.03" />
      <circle cx="50%" cy="10%" r="40" fill="white" opacity="0.04" />
      {/* Lines */}
      <line x1="10%" y1="30%" x2="50%" y2="50%" stroke="white" strokeWidth="0.8" opacity="0.08" />
      <line x1="90%" y1="70%" x2="50%" y2="50%" stroke="white" strokeWidth="0.8" opacity="0.08" />
      <line x1="50%" y1="10%" x2="50%" y2="50%" stroke="white" strokeWidth="0.8" opacity="0.08" />
      {/* Nodes */}
      <circle cx="10%" cy="30%" r="3" fill="white" opacity="0.2" />
      <circle cx="90%" cy="70%" r="3" fill="white" opacity="0.2" />
      <circle cx="50%" cy="10%" r="3" fill="white" opacity="0.2" />
      <circle cx="50%" cy="50%" r="5" fill="white" opacity="0.25" />
    </svg>
  );
}
