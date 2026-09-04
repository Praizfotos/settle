import type { Metadata } from "next";
import { Inter } from "next/font/google";
import { WalletProvider } from "@/lib/wallet";
import "./globals.css";

const inter = Inter({
  subsets: ["latin"],
  variable: "--font-inter",
  display: "swap",
  weight: ["400", "500", "600", "700"],
});

export const metadata: Metadata = {
  title: "Settle — Programmable Agreements. Verifiable Payments.",
  description:
    "Settle is open payment and settlement infrastructure that helps businesses create programmable agreements, fund milestones, manage escrow, resolve disputes, and build verifiable transaction history.",
  keywords: ["escrow", "programmable agreements", "settlement", "fintech", "milestones", "payments"],
  openGraph: {
    title: "Settle — Programmable Agreements. Verifiable Payments.",
    description:
      "Turn business agreements into predictable settlements. Built for businesses, developers, and modern financial workflows.",
    type: "website",
  },
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en" className={inter.variable}>
      <body className="antialiased">
        <WalletProvider>{children}</WalletProvider>
      </body>
    </html>
  );
}
