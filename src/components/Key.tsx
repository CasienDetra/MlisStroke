import { specialKeysObj } from "../config/keySymbols";

interface KeyProps {
  value: string;
}

const keyLabelMap: Record<string, string> = {
  Escape: "Esc",
};

export function Key({ value }: KeyProps) {
  const displayLabel = keyLabelMap[value] || specialKeysObj[value] || value;

  return <div className="key key-regular">{displayLabel}</div>;
}
