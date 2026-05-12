import { useState } from "react";
import { motion } from "motion/react";
import { Check, Info } from "lucide-react";
import { cn } from "../lib/utils";

const plans = [
  {
    name: "Free",
    price: "$0",
    period: "/month",
    desc: "Perfect for casual users",
    features: [
      "10 enhancements per day",
      "Windows + macOS",
      "Groq AI powered",
      "Community support"
    ],
    cta: "Download Free",
    link: "/download",
    highlight: false
  },
  {
    name: "Pro",
    price: "$2",
    period: "/month",
    desc: "For serious power users",
    features: [
      "Unlimited enhancements",
      "Priority AI model",
      "Windows + macOS",
      "Email support",
      "Early beta access"
    ],
    cta: "Get Pro",
    planId: "pro",
    highlight: true
  },
  {
    name: "Lifetime",
    price: "$15",
    period: " once",
    desc: "The ultimate investment",
    features: [
      "Everything in Pro",
      "Forever, no subscription",
      "Priority support",
      "Founder's badge"
    ],
    cta: "Buy Lifetime",
    planId: "lifetime",
    highlight: false
  }
];

const faqs = [
  {
    q: "What AI model does Craftr use?",
    a: "By default, Craftr uses Llama 3 via Groq for lightning-fast results. Pro users get access to more advanced models for complex prompt engineering."
  },
  {
    q: "Is my data private?",
    a: "Completely. Craftr is a local app. Your prompts go directly from your machine to the AI provider using your own API key. We never see or store your prompts."
  },
  {
    q: "Can I cancel anytime?",
    a: "Yes. For the Pro subscription, you can cancel whenever you want through our billing portal. The Lifetime plan is a one-time payment—no strings attached."
  },
  {
    q: "What is a license key?",
    a: "It's a unique code (CRFTR-XXXX-XXXX-XXXX) that validates your Pro status. You'll receive it via email instantly after your purchase."
  }
];

export default function Pricing() {
  const [loading, setLoading] = useState<string | null>(null);

  const handleCheckout = async (plan: string) => {
    if (plan === "Free") return;
    
    setLoading(plan);
    try {
      const response = await fetch("/api/stripe/checkout", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ plan, email: "" }) // Email will be collected in Stripe Checkout
      });
      const data = await response.json();
      if (data.url) {
        window.location.href = data.url;
      }
    } catch (err) {
      console.error(err);
    } finally {
      setLoading(null);
    }
  };

  return (
    <div className="pt-32 pb-24">
      <div className="mx-auto max-w-7xl px-6">
        <div className="text-center">
          <h1 className="text-4xl font-bold tracking-tight text-white sm:text-6xl">Simple Pricing</h1>
          <p className="mt-6 text-lg leading-8 text-zinc-400">
            Choose the plan that fits your prompt engineering workflow.
          </p>
        </div>

        <div className="mt-20 grid grid-cols-1 gap-8 md:grid-cols-3">
          {plans.map((plan) => (
            <div
              key={plan.name}
              className={cn(
                "relative flex flex-col rounded-3xl p-8 transition-all",
                plan.highlight 
                  ? "bg-zinc-900 border-2 border-[#AAFF00] scale-105 z-10" 
                  : "bg-zinc-900/50 border border-white/5"
              )}
            >
              {plan.highlight && (
                <span className="absolute -top-4 left-1/2 -translate-x-1/2 rounded-full bg-[#AAFF00] px-3 py-1 text-xs font-bold text-black uppercase">
                  Recommended
                </span>
              )}
              
              <div className="mb-8">
                <h3 className="text-lg font-bold text-white uppercase tracking-widest">{plan.name}</h3>
                <div className="mt-4 flex items-baseline gap-1">
                  <span className="text-5xl font-bold tracking-tight text-white">{plan.price}</span>
                  <span className="text-sm font-medium text-zinc-500">{plan.period}</span>
                </div>
                <p className="mt-2 text-sm text-zinc-400">{plan.desc}</p>
              </div>

              <ul className="mb-10 flex-1 space-y-4">
                {plan.features.map((feature) => (
                  <li key={feature} className="flex items-center gap-3 text-sm text-zinc-300">
                    <Check size={18} className="text-[#AAFF00]" />
                    {feature}
                  </li>
                ))}
              </ul>

              {plan.planId ? (
                <button
                  onClick={() => handleCheckout(plan.planId!)}
                  disabled={loading !== null}
                  className={cn(
                    "w-full rounded-2xl py-4 text-sm font-bold transition-all disabled:opacity-50",
                    plan.highlight
                      ? "bg-[#AAFF00] text-black hover:bg-[#99ee00]"
                      : "bg-white text-black hover:bg-zinc-200"
                  )}
                >
                  {loading === plan.planId ? "Connecting..." : plan.cta}
                </button>
              ) : (
                <a
                  href={plan.link}
                  className="w-full rounded-2xl bg-zinc-800 py-4 text-center text-sm font-bold text-white transition-all hover:bg-zinc-700"
                >
                  {plan.cta}
                </a>
              )}
            </div>
          ))}
        </div>

        {/* FAQs */}
        <div className="mt-32 max-w-3xl mx-auto">
          <h2 className="text-3xl font-bold text-white mb-12 text-center">Frequently Asked Questions</h2>
          <div className="space-y-6">
            {faqs.map((faq, i) => (
              <div key={i} className="rounded-2xl border border-white/5 bg-zinc-900/50 p-6">
                <h4 className="text-lg font-bold text-white mb-2">{faq.q}</h4>
                <p className="text-zinc-400 leading-relaxed text-sm">{faq.a}</p>
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
}
