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
    windows: "https://github.com/Akshat-Commit/Craftr/releases/download/v1.0.0/craftr-windows-x64.exe",
    macos: "#",
  };

  const triggerDownload = (platform?: 'windows' | 'macos') => {
    const target = platform || (isMac ? 'macos' : 'windows');
    window.location.href = downloadPaths[target];
  };

  return { isMac, isWindows, triggerDownload };
}
