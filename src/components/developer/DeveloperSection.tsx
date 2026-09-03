"use client";

import { useState } from "react";

const tabs = ["TypeScript", "REST API", "CLI", "Webhooks"];

const codeExamples: Record<string, string> = {
  TypeScript: `import { Settle } from "@settle/sdk";

const settle = new Settle({ apiKey: process.env.SETTLE_KEY });

const agreement = await settle.agreements.create({
  client: "client_acme_studio",
  provider: "provider_praiz_digital",
  asset: "USDC",
  amount: 2500,
  milestones: [
    { name: "Discovery", amount: 500 },
    { name: "Design",    amount: 750 },
    { name: "Delivery",  amount: 1250 },
  ],
  disputeWindowDays: 7,
  releaseCondition: "manual_approval",
});

console.log(agreement.id); // agr_1048`,

  "REST API": `POST /v1/agreements
Authorization: Bearer sk_live_...
Content-Type: application/json

{
  "client":   "client_acme_studio",
  "provider": "provider_praiz_digital",
  "asset":    "USDC",
  "amount":   2500,
  "milestones": [
    { "name": "Discovery", "amount": 500  },
    { "name": "Design",    "amount": 750  },
    { "name": "Delivery",  "amount": 1250 }
  ]
}

// 201 Created
{
  "id":     "agr_1048",
  "status": "draft",
  "fundUrl": "https://settle.io/fund/agr_1048"
}`,

  CLI: `# Install the Settle CLI
npm install -g @settle/cli

# Authenticate
settle auth login

# Create an agreement from a config file
settle agreements create --file agreement.json

# Fund a draft agreement
settle agreements fund agr_1048

# Check agreement status
settle agreements status agr_1048
# → Active · Milestone 2/3 in progress`,

  Webhooks: `// Register a webhook endpoint
POST /v1/webhooks
{
  "url":    "https://your-app.com/settle/events",
  "events": ["agreement.funded", "milestone.approved",
             "settlement.released", "dispute.opened"]
}

// Example payload: milestone.approved
{
  "event":       "milestone.approved",
  "agreementId": "agr_1048",
  "milestone":   { "name": "Design", "amount": 750 },
  "releasedAt":  "2026-09-02T14:32:11Z",
  "txHash":      "0xabc...def"
}`,
};

export function DeveloperSection() {
  const [activeTab, setActiveTab] = useState("TypeScript");

  return (
    <section
      id="developers"
      aria-labelledby="dev-heading"
      className="py-20 sm:py-28"
      style={{ background: "#0F172A" }}
    >
      <div className="max-w-[1280px] mx-auto px-6 md:px-10">
        <div className="flex flex-col lg:flex-row items-start gap-12 lg:gap-16">
          {/* Left: text */}
          <div className="flex-1 max-w-[420px] pt-2">
            <p className="text-[11px] font-semibold uppercase tracking-[0.18em] mb-4" style={{ color: "#76A7F5" }}>
              Developer First
            </p>
            <h2
              id="dev-heading"
              className="font-bold tracking-[-0.025em] text-white leading-[1.15] mb-5"
              style={{ fontSize: "clamp(26px, 3.2vw, 40px)" }}
            >
              Built for builders.
            </h2>
            <p className="text-[15px] leading-relaxed mb-8" style={{ color: "#94A3B8" }}>
              Integrate programmable agreements and settlements directly into
              your application with a clean SDK, typed contracts, and
              real-time webhooks.
            </p>

            <div className="space-y-3 mb-8">
              {[
                "TypeScript SDK with full type safety",
                "REST API with OpenAPI spec",
                "Real-time webhook events",
                "Stellar Soroban contract ABIs",
              ].map((item) => (
                <div key={item} className="flex items-center gap-2.5">
                  <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="#1254D8" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round" aria-hidden="true">
                    <path d="M20 6L9 17l-5-5" />
                  </svg>
                  <span className="text-[13px]" style={{ color: "#CBD5E1" }}>{item}</span>
                </div>
              ))}
            </div>

            <div className="flex flex-col sm:flex-row gap-3">
              <a
                href="#api-docs"
                className="inline-flex items-center justify-center gap-2 px-5 py-2.5 text-[13px] font-semibold rounded-[10px] text-white hover:opacity-90 transition-opacity focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-400"
                style={{ background: "linear-gradient(135deg, #1254D8, #2F70E8)" }}
              >
                Explore API
                <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round" aria-hidden="true">
                  <path d="M5 12h14M12 5l7 7-7 7" />
                </svg>
              </a>
              <a
                href="#sdk-docs"
                className="inline-flex items-center justify-center px-5 py-2.5 text-[13px] font-semibold rounded-[10px] hover:bg-white/10 transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-slate-400"
                style={{ color: "#94A3B8", border: "1px solid rgba(255,255,255,0.12)" }}
              >
                Read SDK Docs
              </a>
            </div>
          </div>

          {/* Right: code block */}
          <div className="flex-1 w-full">
            {/* Tab bar */}
            <div
              className="flex items-center gap-1 px-4 pt-3 pb-0 rounded-t-[16px]"
              style={{ background: "#1E293B", borderBottom: "1px solid rgba(255,255,255,0.06)" }}
              role="tablist"
              aria-label="Code examples"
            >
              {tabs.map((tab) => (
                <button
                  key={tab}
                  role="tab"
                  aria-selected={activeTab === tab}
                  onClick={() => setActiveTab(tab)}
                  className={`px-3.5 py-2.5 text-[12px] font-medium rounded-t-[8px] transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-400 ${
                    activeTab === tab
                      ? "text-white border-b-2 border-[#1254D8] -mb-px"
                      : "text-slate-400 hover:text-slate-200"
                  }`}
                >
                  {tab}
                </button>
              ))}
            </div>

            {/* Code */}
            <div
              className="rounded-b-[16px] p-5 overflow-x-auto"
              style={{ background: "#1E293B" }}
            >
              <pre
                className="text-[12.5px] leading-[1.75]"
                style={{ color: "#E2E8F0", fontFamily: "ui-monospace, 'Cascadia Code', 'Fira Code', Consolas, monospace" }}
              >
                <code>{codeExamples[activeTab]}</code>
              </pre>
            </div>

            {/* Copy hint */}
            <p className="text-[11px] mt-3" style={{ color: "#475569" }}>
              Install via{" "}
              <code
                className="px-1.5 py-0.5 rounded text-[11px]"
                style={{ background: "#1E293B", color: "#76A7F5" }}
              >
                npm install @settle/sdk
              </code>
            </p>
          </div>
        </div>
      </div>
    </section>
  );
}
