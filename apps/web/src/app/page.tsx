import { Navigation } from "@/components/navigation/Navigation";
import { HeroSection } from "@/components/hero/HeroSection";
import { TrustStrip } from "@/components/hero/TrustStrip";
import { ValueProp } from "@/components/features/ValueProp";
import { FeatureSection } from "@/components/features/FeatureSection";
import { SettlementOverviewPanel } from "@/components/dashboard/SettlementOverviewPanel";
import { AgreementBuilderPanel } from "@/components/agreements/AgreementBuilderPanel";
import { EscrowPanel } from "@/components/escrow/EscrowPanel";
import { DisputePanel } from "@/components/disputes/DisputePanel";
import { ReputationPanel } from "@/components/reputation/ReputationPanel";
import { DeveloperSection } from "@/components/developer/DeveloperSection";
import { TrustSection } from "@/components/trust/TrustSection";
import { CTASection } from "@/components/cta/CTASection";
import { FinalCTA } from "@/components/cta/FinalCTA";
import { Footer } from "@/components/footer/Footer";

export default function Home() {
  return (
    <>
      <Navigation />

      <main id="main-content">
        {/* 1. Hero */}
        <HeroSection />

        {/* 2. Trust strip */}
        <TrustStrip />

        {/* 3. Value proposition heading */}
        <ValueProp />

        {/* 4. Feature: Settlement Visibility */}
        <div style={{ background: "#F8FAFC" }}>
          <FeatureSection
            id="settlement-visibility"
            eyebrow="Settlement Visibility"
            heading="Know exactly where every payment stands."
            description="Track funded agreements, pending milestones, releases, refunds, and disputes from a single source of truth. No more chasing payment status across scattered messages."
            ctaLabel="Explore settlements"
            ctaHref="#settlement-visibility"
            visual={<SettlementOverviewPanel />}
          />
        </div>

        {/* 5. Feature: Programmable Agreements (reversed) */}
        <FeatureSection
          id="agreements"
          eyebrow="Programmable Agreements"
          heading="Turn expectations into rules everyone can verify."
          description="Define milestones, payment conditions, approval windows, expiry rules, and settlement terms before work begins. Both parties agree once — the protocol enforces it."
          ctaLabel="Create an agreement"
          ctaHref="#agreements"
          visual={<AgreementBuilderPanel />}
          reverse
        />

        {/* 6. Feature: Protected Payments / Escrow */}
        <div style={{ background: "#F8FAFC" }}>
          <FeatureSection
            id="escrow"
            eyebrow="Protected Payments"
            heading="Keep funds secure until the work is done."
            description="Funds remain locked according to the agreement terms and only move when the programmed settlement conditions are satisfied. Neither party can act unilaterally."
            ctaLabel="See how escrow works"
            ctaHref="#escrow"
            visual={<EscrowPanel />}
          />
        </div>

        {/* 7. Feature: Dispute Resolution (reversed) */}
        <FeatureSection
          id="disputes"
          eyebrow="Dispute Resolution"
          heading="When something goes wrong, the rules are already clear."
          description="Give both parties a structured process for submitting evidence, reviewing a dispute, and resolving settlement — without relying on scattered messages and screenshots."
          ctaLabel="Learn about disputes"
          ctaHref="#disputes"
          visual={<DisputePanel />}
          reverse
        />

        {/* 8. Feature: Verifiable History — wider layout */}
        <ReputationFeature />

        {/* 9. Developer section */}
        <DeveloperSection />

        {/* 10. Trust / Security pillars */}
        <TrustSection />

        {/* 11. Big blue CTA */}
        <CTASection />

        {/* 12. Final CTA */}
        <FinalCTA />
      </main>

      <Footer />
    </>
  );
}

function ReputationFeature() {
  return (
    <section
      id="reputation"
      aria-labelledby="reputation-heading"
      className="py-16 sm:py-24"
      style={{ background: "#F8FAFC" }}
    >
      <div className="max-w-[1280px] mx-auto px-6 md:px-10">
        <div className="flex flex-col lg:flex-row items-center gap-12 lg:gap-16">
          {/* Text */}
          <div className="flex-1 max-w-[480px]">
            <p className="text-[11px] font-semibold uppercase tracking-[0.18em] text-[#1254D8] mb-4">
              Verifiable History
            </p>
            <h2
              id="reputation-heading"
              className="font-bold tracking-[-0.025em] text-gray-900 leading-[1.15] mb-5"
              style={{ fontSize: "clamp(26px, 3.2vw, 40px)" }}
            >
              Turn completed work into portable trust.
            </h2>
            <p className="text-[15px] sm:text-[16px] text-gray-500 leading-relaxed mb-6">
              Completed settlements create a verifiable record of successful
              business activity. Your track record is derived from real
              on-chain outcomes — not self-reported numbers.
            </p>
            <ul className="space-y-2.5 mb-8">
              {[
                "Scores derived from verified settlement events",
                "Portable across clients and platforms",
                "Supports stronger commercial relationships",
              ].map((item) => (
                <li key={item} className="flex items-start gap-2.5">
                  <svg
                    width="15"
                    height="15"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="#1254D8"
                    strokeWidth="2.5"
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    className="mt-0.5 flex-shrink-0"
                    aria-hidden="true"
                  >
                    <path d="M20 6L9 17l-5-5" />
                  </svg>
                  <span className="text-[14px] text-gray-600">{item}</span>
                </li>
              ))}
            </ul>
            <a
              href="#reputation"
              className="inline-flex items-center gap-1.5 text-[14px] font-semibold text-[#1254D8] hover:text-[#0D4FD7] transition-colors group focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-500 rounded"
            >
              View reputation model
              <svg
                width="14"
                height="14"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                strokeWidth="2.5"
                strokeLinecap="round"
                strokeLinejoin="round"
                className="transition-transform group-hover:translate-x-0.5"
                aria-hidden="true"
              >
                <path d="M5 12h14M12 5l7 7-7 7" />
              </svg>
            </a>
          </div>

          {/* Visual */}
          <div className="flex-1 w-full flex justify-center lg:justify-end">
            <ReputationPanel />
          </div>
        </div>
      </div>
    </section>
  );
}
