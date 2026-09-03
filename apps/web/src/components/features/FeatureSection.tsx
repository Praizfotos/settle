import { ReactNode } from "react";

interface FeatureSectionProps {
  eyebrow: string;
  heading: string;
  description: string;
  ctaLabel: string;
  ctaHref: string;
  visual: ReactNode;
  reverse?: boolean;
  id?: string;
}

export function FeatureSection({
  eyebrow,
  heading,
  description,
  ctaLabel,
  ctaHref,
  visual,
  reverse = false,
  id,
}: FeatureSectionProps) {
  return (
    <section
      id={id}
      aria-labelledby={id ? `${id}-heading` : undefined}
      className="py-16 sm:py-24 bg-white"
    >
      <div className="max-w-[1280px] mx-auto px-6 md:px-10">
        <div
          className={`flex flex-col ${
            reverse ? "lg:flex-row-reverse" : "lg:flex-row"
          } items-center gap-12 lg:gap-16`}
        >
          {/* Text */}
          <div className="flex-1 max-w-[480px]">
            <p className="text-[11px] font-semibold uppercase tracking-[0.18em] text-[#1254D8] mb-4">
              {eyebrow}
            </p>
            <h2
              id={id ? `${id}-heading` : undefined}
              className="font-bold tracking-[-0.025em] text-gray-900 leading-[1.15] mb-5"
              style={{ fontSize: "clamp(26px, 3.2vw, 40px)" }}
            >
              {heading}
            </h2>
            <p className="text-[15px] sm:text-[16px] text-gray-500 leading-relaxed mb-8">
              {description}
            </p>
            <a
              href={ctaHref}
              className="inline-flex items-center gap-1.5 text-[14px] font-semibold text-[#1254D8] hover:text-[#0D4FD7] transition-colors group focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-500 rounded"
            >
              {ctaLabel}
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
            {visual}
          </div>
        </div>
      </div>
    </section>
  );
}
