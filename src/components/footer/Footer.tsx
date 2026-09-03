import { SettleLogo } from "@/components/layout/SettleLogo";

const columns = [
  {
    heading: "Product",
    links: [
      { label: "Agreements", href: "#agreements" },
      { label: "Escrow", href: "#escrow" },
      { label: "Milestones", href: "#milestones" },
      { label: "Disputes", href: "#disputes" },
      { label: "Reputation", href: "#reputation" },
    ],
  },
  {
    heading: "Developers",
    links: [
      { label: "API Reference", href: "#api" },
      { label: "SDK", href: "#sdk" },
      { label: "Documentation", href: "#docs" },
      { label: "GitHub", href: "#github" },
      { label: "Changelog", href: "#changelog" },
    ],
  },
  {
    heading: "Company",
    links: [
      { label: "About", href: "#about" },
      { label: "Security", href: "#security" },
      { label: "Contact", href: "#contact" },
      { label: "Careers", href: "#careers" },
    ],
  },
  {
    heading: "Resources",
    links: [
      { label: "Documentation", href: "#documentation" },
      { label: "Guides", href: "#guides" },
      { label: "Blog", href: "#blog" },
      { label: "Help Center", href: "#help" },
    ],
  },
];

export function Footer() {
  return (
    <footer
      aria-label="Site footer"
      className="bg-white"
      style={{ borderTop: "1px solid #F3F4F6" }}
    >
      <div className="max-w-[1280px] mx-auto px-6 md:px-10 py-14 sm:py-16">
        {/* Top: logo + columns */}
        <div className="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-5 gap-8 sm:gap-10 mb-12">
          {/* Brand col */}
          <div className="col-span-2 sm:col-span-3 lg:col-span-1 mb-4 lg:mb-0">
            <div className="mb-4">
              <SettleLogo />
            </div>
            <p className="text-[13px] text-gray-400 leading-relaxed max-w-[220px]">
              Programmable agreement and settlement infrastructure for modern
              businesses.
            </p>
          </div>

          {/* Link columns */}
          {columns.map((col) => (
            <nav
              key={col.heading}
              aria-label={`${col.heading} links`}
            >
              <p className="text-[11px] font-bold uppercase tracking-[0.12em] text-gray-900 mb-4">
                {col.heading}
              </p>
              <ul className="space-y-2.5" role="list">
                {col.links.map((link) => (
                  <li key={link.label}>
                    <a
                      href={link.href}
                      className="text-[13px] text-gray-400 hover:text-gray-700 transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-500 rounded"
                    >
                      {link.label}
                    </a>
                  </li>
                ))}
              </ul>
            </nav>
          ))}
        </div>

        {/* Bottom bar */}
        <div
          className="flex flex-col sm:flex-row items-center justify-between gap-4 pt-7"
          style={{ borderTop: "1px solid #F3F4F6" }}
        >
          <p className="text-[12px] text-gray-400">
            &copy; {new Date().getFullYear()} Settle. All rights reserved.
          </p>
          <div className="flex items-center gap-5">
            {["Privacy", "Terms", "Security"].map((item) => (
              <a
                key={item}
                href={`#${item.toLowerCase()}`}
                className="text-[12px] text-gray-400 hover:text-gray-600 transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-500 rounded"
              >
                {item}
              </a>
            ))}
          </div>
        </div>
      </div>
    </footer>
  );
}
