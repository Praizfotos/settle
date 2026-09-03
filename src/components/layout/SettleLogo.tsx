export function SettleLogo({ size = "default" }: { size?: "default" | "large" | "white" }) {
  const textColor = size === "white" ? "#FFFFFF" : "#111827";
  const iconSize = size === "large" ? 28 : 22;

  return (
    <div className="flex items-center gap-2" aria-label="Settle">
      {/* Geometric mark: two arcs converging to a center point */}
      <svg
        width={iconSize}
        height={iconSize}
        viewBox="0 0 28 28"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
        aria-hidden="true"
      >
        {/* Left arc */}
        <path
          d="M4 14C4 8.477 8.477 4 14 4"
          stroke="#1254D8"
          strokeWidth="2.5"
          strokeLinecap="round"
        />
        {/* Right arc */}
        <path
          d="M24 14C24 19.523 19.523 24 14 24"
          stroke="#2F70E8"
          strokeWidth="2.5"
          strokeLinecap="round"
        />
        {/* Center convergence dot */}
        <circle cx="14" cy="14" r="3" fill="url(#settle-grad)" />
        {/* Left trail dot */}
        <circle cx="5.5" cy="9" r="1.5" fill="#1254D8" opacity="0.5" />
        {/* Right trail dot */}
        <circle cx="22.5" cy="19" r="1.5" fill="#2F70E8" opacity="0.5" />
        <defs>
          <radialGradient id="settle-grad" cx="50%" cy="50%" r="50%">
            <stop offset="0%" stopColor="#2F70E8" />
            <stop offset="100%" stopColor="#1254D8" />
          </radialGradient>
        </defs>
      </svg>

      <span
        style={{
          color: textColor,
          fontSize: size === "large" ? "20px" : "15px",
          fontWeight: 700,
          letterSpacing: "-0.02em",
          lineHeight: 1,
        }}
      >
        Settle
      </span>
    </div>
  );
}
