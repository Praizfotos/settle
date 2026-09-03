const categories = [
  { icon: "briefcase", label: "Agencies" },
  { icon: "users", label: "Independent Teams" },
  { icon: "building", label: "Service Businesses" },
  { icon: "code", label: "Developers" },
  { icon: "globe", label: "Global Contractors" },
  { icon: "layers", label: "Suppliers" },
];

export function TrustStrip() {
  return (
    <section aria-label="Trusted by business types" className="py-10 sm:py-12">
      <div className="max-w-[1280px] mx-auto px-6 md:px-10">
        <p className="text-center text-[11px] font-semibold uppercase tracking-[0.18em] text-gray-400 mb-7">
          Built for every kind of business agreement
        </p>
        <div className="flex flex-wrap items-center justify-center gap-x-8 gap-y-4 sm:gap-x-12">
          {categories.map((cat) => (
            <div key={cat.label} className="flex items-center gap-2 group">
              <CategoryIcon name={cat.icon} />
              <span className="text-[13px] font-medium text-gray-400 group-hover:text-gray-600 transition-colors">
                {cat.label}
              </span>
            </div>
          ))}
        </div>
      </div>
    </section>
  );
}

function CategoryIcon({ name }: { name: string }) {
  const icons: Record<string, React.ReactNode> = {
    briefcase: (
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.8" strokeLinecap="round" strokeLinejoin="round" aria-hidden="true" className="text-gray-400">
        <rect x="2" y="7" width="20" height="14" rx="2" /><path d="M16 7V5a2 2 0 0 0-2-2h-4a2 2 0 0 0-2 2v2" />
      </svg>
    ),
    users: (
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.8" strokeLinecap="round" strokeLinejoin="round" aria-hidden="true" className="text-gray-400">
        <path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2" /><circle cx="9" cy="7" r="4" /><path d="M23 21v-2a4 4 0 0 0-3-3.87" /><path d="M16 3.13a4 4 0 0 1 0 7.75" />
      </svg>
    ),
    building: (
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.8" strokeLinecap="round" strokeLinejoin="round" aria-hidden="true" className="text-gray-400">
        <path d="M3 21h18M9 21V7l6-4v18M3 21V11l6-4" />
      </svg>
    ),
    code: (
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.8" strokeLinecap="round" strokeLinejoin="round" aria-hidden="true" className="text-gray-400">
        <polyline points="16 18 22 12 16 6" /><polyline points="8 6 2 12 8 18" />
      </svg>
    ),
    globe: (
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.8" strokeLinecap="round" strokeLinejoin="round" aria-hidden="true" className="text-gray-400">
        <circle cx="12" cy="12" r="10" /><path d="M2 12h20M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10A15.3 15.3 0 0 1 12 2z" />
      </svg>
    ),
    layers: (
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.8" strokeLinecap="round" strokeLinejoin="round" aria-hidden="true" className="text-gray-400">
        <polygon points="12 2 2 7 12 12 22 7 12 2" /><polyline points="2 17 12 22 22 17" /><polyline points="2 12 12 17 22 12" />
      </svg>
    ),
  };
  return <span className="text-gray-400">{icons[name]}</span>;
}
