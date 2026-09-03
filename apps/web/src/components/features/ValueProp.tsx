export function ValueProp() {
  return (
    <section
      id="features"
      aria-labelledby="value-prop-heading"
      className="py-20 sm:py-28 bg-white"
    >
      <div className="max-w-[1280px] mx-auto px-6 md:px-10 text-center">
        <p className="text-[11px] font-semibold uppercase tracking-[0.18em] text-[#1254D8] mb-4">
          The Platform
        </p>
        <h2
          id="value-prop-heading"
          className="font-bold tracking-[-0.025em] text-gray-900 leading-[1.1] mb-5"
          style={{ fontSize: "clamp(32px, 4vw, 52px)" }}
        >
          Everything you need to make{" "}
          <br className="hidden sm:block" />
          business payments trustworthy.
        </h2>
        <p className="text-[16px] sm:text-[17px] text-gray-500 leading-relaxed max-w-[560px] mx-auto">
          From the first agreement to the final settlement, Settle gives every
          party a clear and verifiable financial workflow.
        </p>
      </div>
    </section>
  );
}
