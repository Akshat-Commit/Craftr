import React, { useState } from "react";
import { motion } from "motion/react";
import { Link } from "react-router-dom";
import { Zap, Shield, Cpu, ArrowRight, ChevronDown } from "lucide-react";
import { useDownload } from "../hooks/useDownload";

export default function Home() {
  const { isMac, isWindows, triggerDownload } = useDownload();
  const [showWinTooltip, setShowWinTooltip] = useState(false);
  const [showMacTooltip, setShowMacTooltip] = useState(false);

  const handleDownloadWindows = () => {
    if (isWindows || (!isMac && !isWindows)) {
      triggerDownload('windows');
    } else {
      setShowWinTooltip(true);
      setTimeout(() => setShowWinTooltip(false), 3000);
    }
  };

  const handleDownloadMac = () => {
    if (isMac) {
      triggerDownload('macos');
    } else {
      setShowMacTooltip(true);
      setTimeout(() => setShowMacTooltip(false), 3000);
    }
  };

  return (
    <div className="relative isolate pt-12 overflow-hidden">
      {/* Background Gradients */}
      <div className="absolute inset-x-0 -top-40 -z-10 transform-gpu overflow-hidden blur-3xl sm:-top-80">
        <div 
          className="relative left-[calc(50%-11rem)] aspect-[1155/678] w-[36.125rem] -translate-x-1/2 rotate-[30deg] bg-gradient-to-tr from-[#AAFF00] to-[#1a1a1a] opacity-10 sm:left-[calc(50%-30rem)] sm:w-[72.1875rem]"
          style={{ clipPath: 'polygon(74.1% 44.1%, 100% 61.6%, 97.5% 26.9%, 85.5% 0.1%, 80.7% 2%, 72.5% 32.5%, 60.2% 62.4%, 52.4% 68.1%, 47.5% 58.3%, 45.2% 34.5%, 27.5% 76.7%, 0.1% 64.9%, 17.9% 100%, 27.6% 76.8%, 76.1% 97.7%, 74.1% 44.1%)' }}
        />
      </div>

      {/* Hero Section */}
      <section className="mx-auto max-w-7xl px-6 py-12 lg:py-20">
        <div className="text-center">
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.5 }}
            className="sm:mt-[-40px]"
          >
             <span className="inline-flex items-center gap-2 rounded-full border border-white/10 bg-white/5 px-3 py-1 text-xs font-semibold uppercase tracking-widest text-zinc-400">
               Now in Version 1.0.0
             </span>
          </motion.div>
          
          <motion.h1 
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.5, delay: 0.1 }}
            className="mt-8 text-5xl font-bold tracking-tight text-white sm:text-7xl lg:text-8xl"
          >
            Better Prompts. <br />
            <span className="text-[#AAFF00]">Two Keystrokes.</span>
          </motion.h1>
          
          <motion.p 
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.5, delay: 0.2 }}
            className="mx-auto mt-6 max-w-2xl text-lg leading-8 text-zinc-400 sm:text-xl"
          >
            Craftr sits in your system tray and instantly enhances or compresses any prompt — in any app, anywhere on your desktop.
          </motion.p>
          
          <motion.div 
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.5, delay: 0.3 }}
            className="mt-10 flex flex-col items-center justify-center gap-6"
          >
            <div className="flex flex-col sm:flex-row gap-4 w-full max-w-sm sm:max-w-none justify-center">
              <div className="relative">
                <button
                  onClick={handleDownloadWindows}
                  className={`flex items-center justify-center gap-2 rounded-2xl px-8 py-4 text-sm font-bold transition-all hover:scale-105 cursor-pointer w-full sm:w-auto ${
                    isWindows || (!isMac && !isWindows)
                      ? "bg-[#AAFF00] text-black shadow-[0_0_20px_rgba(170,255,0,0.3)] opacity-100" 
                      : "bg-white/10 text-white border border-white/10 opacity-60"
                  }`}
                >
                  Download for Windows
                </button>
                {showWinTooltip && (
                  <div className="absolute -top-12 left-1/2 -translate-x-1/2 bg-white text-black text-[10px] font-bold py-1 px-3 rounded shadow-xl whitespace-nowrap z-50">
                    Switch to Windows to download Windows version
                    <div className="absolute top-full left-1/2 -translate-x-1/2 border-8 border-transparent border-t-white" />
                  </div>
                )}
              </div>

              <div className="relative">
                <button
                  onClick={handleDownloadMac}
                  className={`flex items-center justify-center gap-2 rounded-2xl px-8 py-4 text-sm font-bold transition-all hover:scale-105 cursor-pointer w-full sm:w-auto ${
                    isMac 
                      ? "bg-[#AAFF00] text-black shadow-[0_0_20px_rgba(170,255,0,0.3)] opacity-100" 
                      : "bg-white/10 text-white border border-white/10 opacity-60"
                  }`}
                >
                  Download for macOS
                </button>
                {showMacTooltip && (
                  <div className="absolute -top-12 left-1/2 -translate-x-1/2 bg-white text-black text-[10px] font-bold py-1 px-3 rounded shadow-xl whitespace-nowrap z-50">
                    Switch to macOS to download Mac version
                    <div className="absolute top-full left-1/2 -translate-x-1/2 border-8 border-transparent border-t-white" />
                  </div>
                )}
              </div>
            </div>
            
            <div className="flex flex-col items-center gap-4">
              <p className="text-xs font-medium text-zinc-500 tracking-wide uppercase">
                Free • No account required • Windows 10+ & macOS 12+
              </p>
              <Link
                to="/pricing"
                className="group flex items-center gap-1 text-sm font-semibold leading-6 text-zinc-400 hover:text-[#AAFF00] transition-colors"
              >
                Learn about Pro Features <ArrowRight size={14} className="transition-transform group-hover:translate-x-1" />
              </Link>
            </div>
          </motion.div>

          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            transition={{ duration: 1, delay: 0.8 }}
            className="mt-20 flex flex-col items-center gap-2 text-zinc-500 animate-bounce cursor-pointer"
            onClick={() => document.getElementById('features')?.scrollIntoView({ behavior: 'smooth' })}
          >
            <span className="text-[10px] font-bold uppercase tracking-widest">How it works</span>
            <ChevronDown size={14} />
          </motion.div>
        </div>
      </section>

      {/* Feature Cards */}
      <section id="features" className="mx-auto max-w-7xl px-6 py-24 sm:py-32">
        <div className="grid grid-cols-1 gap-8 md:grid-cols-3">
          {[
            {
              title: "Works Everywhere",
              desc: "From browsers to IDEs to Slack—if you can select it, Craftr can enhance it. Zero integration required.",
              icon: Zap
            },
            {
              title: "Two Modes",
              desc: "Instantly enhance for better results or compress to save tokens. Switch modes with a simple toggle.",
              icon: Cpu
            },
            {
              title: "Privacy First",
              desc: "Craftr uses your own API keys. We never touch your data, and nothing is stored on our servers.",
              icon: Shield
            }
          ].map((feature, i) => (
            <motion.div
              key={i}
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
              transition={{ delay: i * 0.1 }}
              className="group rounded-3xl border border-white/5 bg-zinc-900/50 p-8 hover:border-[#AAFF00]/50 transition-colors"
            >
              <div className="mb-6 flex h-12 w-12 items-center justify-center rounded-xl bg-[#AAFF00]/10 text-[#AAFF00]">
                <feature.icon size={24} />
              </div>
              <h3 className="text-xl font-bold text-white">{feature.title}</h3>
              <p className="mt-4 text-zinc-400">{feature.desc}</p>
            </motion.div>
          ))}
        </div>
      </section>

      {/* How it Works */}
      <section className="border-t border-white/5 bg-zinc-900/20 py-24 sm:py-32">
        <div className="mx-auto max-w-7xl px-6">
          <div className="text-center">
            <h2 className="text-3xl font-bold tracking-tight text-white sm:text-5xl">How it Works</h2>
            <p className="mt-4 text-zinc-400">Master your productivity in three steps.</p>
          </div>
          
          <div className="mt-20 grid grid-cols-1 gap-12 lg:grid-cols-3">
            {[
              { step: "01", title: "Select", desc: "Select your prompt text in any application." },
              { step: "02", title: "Shortcut", desc: "Press Ctrl+E (Windows) or Cmd+E (macOS)." },
              { step: "03", title: "Refined", desc: "The enhanced prompt replaces your text instantly." }
            ].map((item, i) => (
              <div key={i} className="relative">
                <span className="text-7xl font-black text-[#AAFF00]/10 absolute -top-8 left-0">{item.step}</span>
                <div className="relative">
                  <h3 className="text-2xl font-bold text-white mb-2">{item.title}</h3>
                  <p className="text-zinc-500 leading-relaxed">{item.desc}</p>
                </div>
              </div>
            ))}
          </div>
        </div>
      </section>

      {/* Final CTA */}
      <section className="py-24 sm:py-32">
        <div className="mx-auto max-w-7xl px-6">
          <div className="relative isolate overflow-hidden bg-zinc-900 px-6 py-24 text-center shadow-2xl rounded-3xl border border-white/5">
             <h2 className="mx-auto max-w-2xl text-3xl font-bold tracking-tight text-white sm:text-4xl">
               Ready to craft better prompts?
             </h2>
             <p className="mx-auto mt-6 max-w-xl text-lg leading-8 text-zinc-400">
               Join thousands of power users who have optimized their AI workflow with Craftr.
             </p>
             <div className="mt-10 flex items-center justify-center gap-x-6">
               <Link
                 to="/download"
                 className="rounded-full bg-white px-8 py-3 text-sm font-semibold text-black hover:bg-[#AAFF00] transition-colors"
               >
                 Start free today
               </Link>
             </div>
          </div>
        </div>
      </section>
    </div>
  );
}
