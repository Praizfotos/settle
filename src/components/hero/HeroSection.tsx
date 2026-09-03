import { AgreementCard } from "@/components/hero/AgreementCard";
import { SettlementCard } from "@/components/hero/SettlementCard";
import { TrustScoreCard } from "@/components/hero/TrustScoreCard";

export function HeroSection() {
  return (
    <section
      aria-label="Hero"
      className="pt-[60px]"
    >
      <div className="max-w-[1280px] mx-auto px-4 sm:px-6 md:px-10 py-4 sm:py-6">
        {/* Large blue rounded hero container */}
        <div
          className="relative overflow-hidden rounded-[28px] sm:rounded-[32px] min-h-[540px] sm:min-h-[600px]"
          style={{
            background:
              "linear-gradient(135deg, #0D4FD7 0%, #1254D8 35%, #2F70E8 70%, #4A84EE 100%)",
          }}
        >
          {/* Subtle background grid */}
          <HeroGrid />

          {/* Inner glow */}
          <div
            className="absolute inset-0 pointer-events-none"
            style={{
              background:
                "radial-gradient(ellipse 70% 60% at 50% -10%, rgba(255,255,255,0.08) 0%, transparent 70%)",
            }}
            aria-hidden="true"
          />

          {/* Content */}
          <div className="relative z-10 flex flex-col lg:flex-row items-center gap-8 lg:gap-12 px-8 sm:px-12 lg:px-16 pt-14 sm:pt-16 pb-16 sm:pb-20">
            {/* Left: text */}
            <div className="flex-1 max-w-[520px] text-center lg:text-left animate-fade-in-up">
              {/* Eyebrow */}
              <p
                className="text-[11px] font-semibold tracking-[0.18em] uppercase mb-5 animation-delay-100"
                style={{ color: "rgba(255,255,255,0.65)" }}
              >
                Payments, without the trust gap
              </p>

              {/* Headline */}
              <h1
                className="font-bold leading-[1.0] tracking-[-0.025em] mb-6 animation-delay-200"
                style={{
                  fontSize: "clamp(40px, 5.5vw, 64px)",
                  color: "#FFFFFF",
                }}
              >
                Turn agreements
                <br />
                into predictable
                <br />
                <span style={{ color: "rgba(255,255,255,0.75)" }}>
                  settlements.
                </span>
              </h1>

              {/* Sub */}
              <p
                className="text-[15px] sm:text-[16px] leading-[1.65] mb-8 animation-delay-300"
                style={{ color: "rgba(255,255,255,0.72)", maxWidth: "420px" }}
              >
                Settle gives businesses a programmable way to fund milestones,
                secure payments, resolve disputes, and create verifiable
                transaction history.
              </p>

              {/* CTAs */}
              <div className="flex flex-col sm:flex-row items-center lg:items-start gap-3 mb-7 animation-delay-400">
                <a
                  href="#get-started"
                  className="w-full sm:w-auto inline-flex items-center justify-center gap-2 px-6 py-3 text-[14px] font-semibold text-[#1254D8] bg-white rounded-[10px] hover:bg-gray-50 active:scale-[0.98] transition-all duration-150 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-white focus-visible:ring-offset-2 focus-visible:ring-offset-blue-600"
                  style={{ boxShadow: "0 2px 8px rgba(0,0,0,0.15)" }}
                >
                  Get Started
                  <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round" aria-hidden="true">
                    <path d="M5 12h14M12 5l7 7-7 7" />
                  </svg>
                </a>
                <a
                  href="#features"
                  className="w-full sm:w-auto inline-flex items-center justify-center px-6 py-3 text-[14px] font-semibold rounded-[10px] hover:bg-white/10 active:scale-[0.98] transition-all duration-150 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-white/50"
                  style={{
                    color: "rgba(255,255,255,0.90)",
                    border: "1px solid rgba(255,255,255,0.25)",
                  }}
                >
                  Explore the Platform
                </a>
              </div>

              {/* Trust note */}
              <p
                className="text-[12px] animation-delay-500"
                style={{ color: "rgba(255,255,255,0.45)" }}
              >
                Built for businesses, developers, and modern financial workflows.
              </p>
            </div>

            {/* Right: floating UI cards */}
            <div
              className="flex-1 relative w-full max-w-[420px] lg:max-w-none h-[380px] sm:h-[420px] lg:h-[440px] animate-fade-in animation-delay-300"
              aria-hidden="true"
            >
              {/* Main agreement card */}
              <div className="absolute top-0 right-0 lg:right-4 w-[260px] sm:w-[280px] animate-float-a">
                <AgreementCard />
              </div>

              {/* Settlement card */}
              <div className="absolute bottom-[60px] sm:bottom-[40px] left-0 lg:left-2 w-[180px] sm:w-[195px] animate-float-b animation-delay-200">
                <SettlementCard />
              </div>

              {/* Trust score */}
              <div className="absolute top-[160px] sm:top-[170px] left-[30px] sm:left-[20px] w-[150px] sm:w-[160px] animate-float-a animation-delay-400">
                <TrustScoreCard />
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>
  );
}

function HeroGrid() {
  return (
    <svg
      className="absolute inset-0 w-full h-full pointer-events-none"
      aria-hidden="true"
      style={{ opacity: 0.07 }}
    >
      <defs>
        <pattern
          id="hero-grid"
          width="48"
          height="48"
          patternUnits="userSpaceOnUse"
        >
          <path
            d="M 48 0 L 0 0 0 48"
            fill="none"
            stroke="white"
            strokeWidth="0.8"
          />
        </pattern>
      </defs>
      <rect width="100%" height="100%" fill="url(#hero-grid)" />
      {/* Decorative nodes */}
      <circle cx="15%" cy="25%" r="2" fill="white" opacity="0.4" />
      <circle cx="30%" cy="65%" r="1.5" fill="white" opacity="0.3" />
      <circle cx="80%" cy="20%" r="2" fill="white" opacity="0.3" />
      <circle cx="72%" cy="75%" r="1.5" fill="white" opacity="0.25" />
      {/* Connection lines */}
      <line x1="15%" y1="25%" x2="30%" y2="65%" stroke="white" strokeWidth="0.6" opacity="0.2" />
      <line x1="80%" y1="20%" x2="72%" y2="75%" stroke="white" strokeWidth="0.6" opacity="0.2" />
    </svg>
  );
}
