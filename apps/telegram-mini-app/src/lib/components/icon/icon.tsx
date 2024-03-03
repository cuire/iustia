export type IconType = {
  default: React.FC;
  solid?: React.FC;
};

type IconProps = {
  icon: IconType;
  className?: string;
  strokeWidth?: number;
  solid?: boolean;
};

type IconComponent<P extends IconProps = IconProps> = React.FC<P>;

export const Icon: IconComponent = (props) => {
  return (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      fill={props.solid ? "currentColor" : "none"}
      viewBox="0 0 24 24"
      strokeWidth={props.strokeWidth ?? 1.5}
      stroke={props.solid ? "none" : "currentColor"}
      className={props.className ?? "w-6 h-6"}
    >
      {props.icon[props.solid && props.icon.solid ? "solid" : "default"]?.({})}
    </svg>
  );
};
