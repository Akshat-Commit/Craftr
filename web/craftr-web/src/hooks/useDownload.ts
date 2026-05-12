import { useState, useEffect } from "react";

export function useDownload() {
  const [isMac, setIsMac] = useState(false);
  const [isWindows, setIsWindows] = useState(false);

  useEffect(() => {
    const ua = navigator.userAgent.toLowerCase();
    setIsMac(ua.includes("mac"));
    setIsWindows(ua.includes("windows"));
  }, []);

  const downloadPaths = {
    windows: "https://github.com/your-username/craftr-desktop/releases/latest/download/craftr-windows-x64.exe",
    macos: "https://github.com/your-username/craftr-desktop/releases/latest/download/craftr-macos.dmg",
  };

  const triggerDownload = (platform?: 'windows' | 'macos') => {
    const target = platform || (isMac ? 'macos' : 'windows');
    window.location.href = downloadPaths[target];
  };

  return { isMac, isWindows, triggerDownload };
}
