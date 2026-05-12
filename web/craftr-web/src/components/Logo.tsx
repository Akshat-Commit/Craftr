import { cn } from "../lib/utils";

interface LogoProps {
  className?: string;
  size?: number;
}

export default function Logo({ className, size = 32 }: LogoProps) {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 100 100"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
      className={cn("text-[#AAFF00]", className)}
    >
      {/* Circle "C" */}
      <path
        d="M75 25C68.3 18.3 59.2 14 50 14C30.1 14 14 30.1 14 50C14 69.9 30.1 86 50 86C59.2 86 68.3 81.7 75 75"
        stroke="currentColor"
        strokeWidth="12"
        strokeLinecap="round"
      />
      {/* Lightning Bolt */}
      <path
        d="M60 30L40 55H55L40 80L65 50H50L60 30Z"
        fill="currentColor"
      />
    </svg>
  );
}
