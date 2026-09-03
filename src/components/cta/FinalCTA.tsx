export function FinalCTA() {
  return (
    <section
      aria-labelledby="final-cta-heading"
      className="py-24 sm:py-32 bg-white text-center"
    >
      <div className="max-w-[640px] mx-auto px-6 md:px-10">
        <h2
          id="final-cta-heading"
          className="font-bold tracking-[-0.025em] text-gray-900 leading-[1.1] mb-5"
          style={{ fontSize: "clamp(30px, 4vw, 52px)" }}
        >
          Make every agreement
          <br />
          easier to trust.
        </h2>
        <p className="text-[16px] text-gray-500 leading-relaxed mb-9 max-w-[460px] mx-auto">
          Settle gives modern businesses the infrastructure to create, fund,
          manage, and complete agreements with confidence.
        </p>
        <div className="flex flex-col sm:flex-row items-center justify-center gap-3">
          <a
            href="#get-started"
            className="w-full sm:w-auto inline-flex items-center justify-center gap-2 px-7 py-3 text-[14px] font-semibold text-white rounded-[10px] hover:opacity-90 active:scale-[0.98] transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-500"
            style={{ background: "linear-gradient(135deg, #1254D8, #2F70E8)", boxShadow: "0 2px 10px rgba(18,84,216,0.3)" }}
          >
            Create your first agreement
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round" aria-hidden="true">
              <path d="M5 12h14M12 5l7 7-7 7" />
            </svg>
          </a>
          <a
            href="#features"
            className="w-full sm:w-auto inline-flex items-center justify-center px-7 py-3 text-[14px] font-semibold text-gray-600 rounded-[10px] hover:bg-gray-50 active:scale-[0.98] transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-gray-300"
            style={{ border: "1px solid #E5E7EB" }}
          >
            Explore the platform
          </a>
        </div>
      </div>
    </section>
  );
}
