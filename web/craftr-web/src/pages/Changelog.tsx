import { motion } from "motion/react";
import { Plus, Bug, Zap } from "lucide-react";

const updates = [
  {
    version: "1.0.0",
    date: "May 12, 2026",
    title: "The Initial Launch",
    description: "Today marking the official launch of Craftr! A productivity powerhouse born out of the need for faster, cleaner AI interactions.",
    changes: [
      { type: "new", text: "Global shortcut support (Ctrl+E / Cmd+E)" },
      { type: "new", text: "Dual modes: Enhance for results, Compress for tokens" },
      { type: "new", text: "Groq Llama 3 integration for near-instant responses" },
      { type: "new", text: "System tray residency for zero-ui footprint" },
      { type: "new", text: "Stripe-powered Pro features for power users" }
    ]
  }
];

export default function Changelog() {
  return (
    <div className="pt-32 pb-24">
      <div className="mx-auto max-w-3xl px-6">
        <div className="text-center mb-20">
          <h1 className="text-4xl font-bold tracking-tight text-white sm:text-6xl">Changelog</h1>
          <p className="mt-6 text-lg leading-8 text-zinc-400">
            Follow our journey as we refine the future of prompt engineering.
          </p>
          
          <div className="mt-10 mx-auto max-w-sm">
            <div className="flex gap-2 p-1.5 rounded-full bg-zinc-900 border border-white/5">
              <input 
                type="email" 
                placeholder="Join the newsletter..." 
                className="flex-1 bg-transparent px-4 text-sm outline-none placeholder:text-zinc-600"
              />
              <button className="rounded-full bg-white px-4 py-2 text-xs font-bold text-black hover:bg-[#AAFF00] transition-colors">
                Subscribe
              </button>
            </div>
          </div>
        </div>

        <div className="space-y-24">
          {updates.map((update, i) => (
            <motion.div
              key={update.version}
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
              className="relative pl-12 border-l border-white/10"
            >
              <div className="absolute -left-[5px] top-0 h-2.5 w-2.5 rounded-full bg-[#AAFF00] shadow-[0_0_10px_#AAFF00]" />
              
              <div className="flex flex-col sm:flex-row sm:items-baseline sm:justify-between mb-4">
                <h3 className="text-2xl font-bold text-white leading-none mb-2 md:mb-0">v{update.version} — {update.title}</h3>
                <span className="text-xs font-mono text-zinc-500">{update.date}</span>
              </div>

              <p className="text-zinc-400 text-sm mb-8 leading-relaxed">
                {update.description}
              </p>

              <div className="space-y-4">
                {update.changes.map((change, j) => (
                  <div key={j} className="flex items-start gap-4">
                    <span className={`mt-1 flex h-4 w-4 shrink-0 items-center justify-center rounded text-[8px] font-bold uppercase ${
                      change.type === 'new' ? 'bg-[#AAFF00]/10 text-[#AAFF00]' :
                      change.type === 'fix' ? 'bg-red-500/10 text-red-500' : 'bg-blue-500/10 text-blue-500'
                    }`}>
                      {change.type === 'new' ? <Plus size={8} /> : 
                       change.type === 'fix' ? <Bug size={8} /> : 
                       <Zap size={8} />}
                    </span>
                    <p className="text-sm text-zinc-300">{change.text}</p>
                  </div>
                ))}
              </div>
            </motion.div>
          ))}
        </div>
      </div>
    </div>
  );
}
