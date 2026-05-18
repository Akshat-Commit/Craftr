import { cn } from "../lib/utils";

interface LogoProps {
  className?: string;
  size?: number;
}

export default function Logo({ className, size = 32 }: LogoProps) {
  return (
    <img
      src="/logo.png"
      alt="Craftr Logo"
      width={size}
      height={size}
      className={cn("rounded-lg", className)}
    />
  );
}
