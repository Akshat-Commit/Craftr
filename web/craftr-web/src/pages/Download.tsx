import { useEffect, useState } from "react";
import { motion } from "motion/react";
import { Apple, Monitor, Download as DownloadIcon, Terminal, Key, Rocket } from "lucide-react";

export default function Download() {
  const [isMac, setIsMac] = useState(false);

  useEffect(() => {
    setIsMac(navigator.userAgent.toLowerCase().includes("mac"));
  }, []);

  const versions = {
    windows: { 
      ver: "1.0.0", 
      date: "May 12, 2026", 
      hash: "sha256:d4f3e...9a21",
      link: "/releases/craftr-windows-x64.exe" 
    },
    macos: { 
      ver: "1.0.0", 
      date: "May 12, 2026", 
      hash: "sha256:f12e8...b7d4",
      link: "/releases/craftr-macos.dmg" 
    }
  };

  const platforms = [
    { id: 'macos', name: 'macOS', icon: Apple, info: versions.macos, highlight: isMac, bg: 'bg-white/10', text: 'text-white' },
    { id: 'windows', name: 'Windows', icon: Monitor, info: versions.windows, highlight: !isMac, bg: 'bg-blue-500/10', text: 'text-blue-500' }
  ].sort((a, b) => (a.highlight === b.highlight ? 0 : a.highlight ? -1 : 1));

  return (
    <div className="pt-32 pb-24">
      <div className="mx-auto max-w-7xl px-6 text-center">
        <h1 className="text-4xl font-bold tracking-tight text-white sm:text-6xl">Get Craftr</h1>
        <p className="mt-6 text-lg leading-8 text-zinc-400">
          Available on your favorite desktop platforms.
        </p>

        <div className="mt-16 grid grid-cols-1 gap-8 md:grid-cols-2">
          {platforms.map((p) => (
            <motion.div
              key={p.id}
              id={p.id}
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              className={`rounded-3xl border p-8 flex flex-col items-center transition-all ${
                p.highlight 
                  ? "bg-zinc-900 border-[#AAFF00] shadow-[0_0_50px_-12px_rgba(170,255,0,0.2)]" 
                  : "bg-zinc-900/50 border-white/5 grayscale opacity-60 hover:grayscale-0 hover:opacity-100"
              }`}
            >
              <div className={`mb-6 flex h-16 w-16 items-center justify-center rounded-2xl ${p.bg} ${p.text}`}>
                <p.icon size={32} />
              </div>
              <h3 className="text-2xl font-bold text-white mb-2">{p.name}</h3>
              <p className="text-zinc-500 text-sm mb-8">Version {p.info.ver} • {p.info.date}</p>
              
              <a 
                href={p.info.link}
                className={`flex w-full items-center justify-center gap-2 rounded-2xl py-4 font-bold text-black transition-transform hover:scale-[1.02] ${
                  p.highlight ? "bg-[#AAFF00]" : "bg-white"
                }`}
              >
                <DownloadIcon size={18} />
                Download {p.id === 'windows' ? '.exe' : '.dmg'}
              </a>
              <p className="mt-4 text-[10px] font-mono text-zinc-600 truncate max-w-full">
                {p.info.hash}
              </p>
            </motion.div>
          ))}
        </div>

        {/* Setup Guide */}
        <div className="mt-32 max-w-4xl mx-auto text-left">
          <h2 className="text-3xl font-bold text-center text-white mb-16">Quick Setup Guide</h2>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-12">
            {[
              { icon: DownloadIcon, title: "Install", desc: "Download the installer for your OS and run it." },
              { icon: Key, title: "API Key", desc: "Get a free Groq API key at console.groq.com." },
              { icon: Rocket, title: "Ignite", desc: "Paste your key in Craftr and start using shortcuts." }
            ].map((item, i) => (
              <div key={i} className="flex flex-col gap-4">
                <div className="flex h-12 w-12 items-center justify-center rounded-xl bg-zinc-800 text-[#AAFF00]">
                  <item.icon size={24} />
                </div>
                <h4 className="text-lg font-bold text-white">{i + 1}. {item.title}</h4>
                <p className="text-zinc-400 text-sm leading-relaxed">{item.desc}</p>
              </div>
            ))}
          </div>
        </div>

        {/* Requirements */}
        <div className="mt-32 border-t border-white/5 pt-16">
          <h3 className="text-xl font-bold text-white mb-8">System Requirements</h3>
          <div className="flex flex-wrap justify-center gap-8">
            <div className="text-sm text-zinc-500 p-6 rounded-2xl bg-zinc-900/30 border border-white/5">
              <span className="block font-bold text-zinc-300 mb-2">Windows</span>
              Windows 10/11 (64-bit)
            </div>
            <div className="text-sm text-zinc-500 p-6 rounded-2xl bg-zinc-900/30 border border-white/5">
              <span className="block font-bold text-zinc-300 mb-2">macOS</span>
              macOS 12+ (Intel + Apple Silicon)
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
