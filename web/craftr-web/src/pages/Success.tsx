import { motion } from "motion/react";
import { Link, useSearchParams } from "react-router-dom";
import { CheckCircle2, Copy, Mail, ExternalLink } from "lucide-react";
import { useState } from "react";

export default function Success() {
  const [searchParams] = useSearchParams();
  const sessionId = searchParams.get("session_id");
  const [copied, setCopied] = useState(false);

  const copySupport = () => {
    navigator.clipboard.writeText("support@craftr.app");
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="pt-32 pb-24 min-h-screen flex items-center justify-center">
      <div className="mx-auto max-w-2xl px-6 text-center">
        <motion.div
          initial={{ scale: 0.8, opacity: 0 }}
          animate={{ scale: 1, opacity: 1 }}
          className="mb-8 flex justify-center"
        >
          <div className="flex h-20 w-20 items-center justify-center rounded-full bg-[#AAFF00]/10 text-[#AAFF00]">
            <CheckCircle2 size={48} />
          </div>
        </motion.div>

        <h1 className="text-4xl font-bold tracking-tight text-white mb-4">Payment Successful!</h1>
        <p className="text-lg text-zinc-400 mb-12">
          Thank you for upgrading to Craftr Pro. Your productivity is about to level up.
        </p>

        <div className="grid grid-cols-1 gap-6 text-left">
          <div className="rounded-3xl border border-[#AAFF00]/20 bg-zinc-900/50 p-8 shadow-[0_0_50px_-12px_rgba(170,255,0,0.1)]">
            <h3 className="text-xl font-bold text-white mb-6">Next Steps</h3>
            
            <div className="space-y-6">
              <div className="flex gap-4">
                <div className="flex h-8 w-8 shrink-0 items-center justify-center rounded-full bg-zinc-800 text-[#AAFF00] text-sm font-bold">1</div>
                <div>
                  <h4 className="text-white font-medium mb-1">Check your email</h4>
                  <p className="text-zinc-500 text-sm">We've sent your license key to your email. If you don't see it, check your spam folder.</p>
                </div>
              </div>

              <div className="flex gap-4">
                <div className="flex h-8 w-8 shrink-0 items-center justify-center rounded-full bg-zinc-800 text-[#AAFF00] text-sm font-bold">2</div>
                <div>
                  <h4 className="text-white font-medium mb-1">Activate in App</h4>
                  <p className="text-zinc-500 text-sm">Open Craftr on your desktop, go to Settings &gt; License, and paste your key.</p>
                </div>
              </div>

              <div className="flex gap-4">
                <div className="flex h-8 w-8 shrink-0 items-center justify-center rounded-full bg-zinc-800 text-[#AAFF00] text-sm font-bold">3</div>
                <div>
                  <h4 className="text-white font-medium mb-1">Enjoy Pro</h4>
                  <p className="text-zinc-500 text-sm">You now have unlimited enhancements and priority model access.</p>
                </div>
              </div>
            </div>
          </div>

          <div className="flex flex-col sm:flex-row gap-4 justify-center mt-8">
            <Link 
              to="/" 
              className="px-8 py-3 rounded-2xl bg-white text-black font-bold text-sm hover:bg-[#AAFF00] transition-colors inline-flex items-center gap-2"
            >
              Back to Home
            </Link>
            <button 
              onClick={copySupport}
              className="px-8 py-3 rounded-2xl bg-zinc-900 border border-white/5 text-white font-bold text-sm hover:bg-zinc-800 transition-colors inline-flex items-center gap-2"
            >
              <Mail size={16} />
              {copied ? "Email Copied!" : "Support Email"}
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
