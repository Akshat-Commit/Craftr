import { Link } from "react-router-dom";
import { Github, Twitter } from "lucide-react";
import Logo from "./Logo";

export default function Footer() {
  return (
    <footer className="border-t border-white/5 bg-black py-12">
      <div className="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
        <div className="flex flex-col items-start justify-between gap-8 md:flex-row">
          <div className="flex flex-col gap-4">
            <Link to="/" className="flex items-center gap-2">
              <Logo size={32} />
              <span className="text-xl font-bold tracking-tight text-white">Craftr</span>
            </Link>
            <p className="max-w-xs text-sm text-zinc-500">
              Enhance and compress your AI prompts anywhere on your desktop with simple global shortcuts.
            </p>
          </div>
          
          <div className="grid grid-cols-2 gap-12 sm:grid-cols-3">
            <div className="flex flex-col gap-3">
              <h4 className="text-xs font-bold uppercase tracking-widest text-zinc-400">Product</h4>
              <Link to="/pricing" className="text-sm text-zinc-500 hover:text-white">Pricing</Link>
              <Link to="/download" className="text-sm text-zinc-500 hover:text-white">Download</Link>
              <Link to="/changelog" className="text-sm text-zinc-500 hover:text-white">Changelog</Link>
            </div>
            <div className="flex flex-col gap-3">
              <h4 className="text-xs font-bold uppercase tracking-widest text-zinc-400">Legal</h4>
              <Link to="#" className="text-sm text-zinc-500 hover:text-white">Privacy</Link>
              <Link to="#" className="text-sm text-zinc-500 hover:text-white">Terms</Link>
            </div>
            <div className="flex flex-col gap-3">
              <h4 className="text-xs font-bold uppercase tracking-widest text-zinc-400">Social</h4>
              <Link to="#" className="flex items-center gap-2 text-sm text-zinc-500 hover:text-white">
                <Twitter size={16} /> @craftrapp
              </Link>
              <a href="https://github.com/Akshat-Commit" target="_blank" rel="noopener noreferrer" className="flex items-center gap-2 text-sm text-zinc-500 hover:text-white">
                <Github size={16} /> GitHub
              </a>
            </div>
          </div>
        </div>
        
        <div className="mt-12 border-t border-white/5 pt-8 flex flex-col sm:flex-row items-center justify-between gap-4 text-xs text-zinc-600">
          <div>
            © {new Date().getFullYear()} Craftr. Built for better prompts.
          </div>
          <div className="font-medium">
            Built by <a href="https://github.com/Akshat-Commit" target="_blank" rel="noopener noreferrer" className="text-zinc-500 hover:text-white transition-colors">Akshat Jain</a>
          </div>
        </div>
      </div>
    </footer>
  );
}
