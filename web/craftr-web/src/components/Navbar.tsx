import { Link } from "react-router-dom";
import { Download } from "lucide-react";
import Logo from "./Logo";
import { useDownload } from "../hooks/useDownload";

export default function Navbar() {
  const { triggerDownload } = useDownload();

  return (
    <nav className="fixed top-0 left-0 right-0 z-50 border-b border-white/5 bg-black/50 backdrop-blur-xl">
      <div className="mx-auto flex h-16 max-w-7xl items-center justify-between px-4 sm:px-6 lg:px-8">
        <Link to="/" className="flex items-center gap-2 group">
          <Logo size={32} />
          <span className="text-xl font-bold tracking-tight text-white">Craftr</span>
        </Link>
        
        <div className="hidden items-center gap-8 md:flex">
          <Link to="/" className="text-sm font-medium text-zinc-400 transition-colors hover:text-[#AAFF00]">Home</Link>
          <Link to="/pricing" className="text-sm font-medium text-zinc-400 transition-colors hover:text-[#AAFF00]">Pricing</Link>
          <Link to="/changelog" className="text-sm font-medium text-zinc-400 transition-colors hover:text-[#AAFF00]">Changelog</Link>
        </div>

        <div className="flex items-center gap-4">
          <button
            onClick={() => triggerDownload()}
            className="flex items-center gap-2 rounded-full bg-white px-4 py-2 text-sm font-semibold text-black transition-all hover:bg-[#AAFF00] cursor-pointer"
          >
            <Download size={16} />
            <span>Download</span>
          </button>
        </div>
      </div>
    </nav>
  );
}
