export const Theme: React.FC = () => {
  return (
    <div className="grid h-full grid-cols-3 gap-4">
      <div className="bg-base-100">base-100</div>
      <div className="bg-base-200">base-200</div>
      <div className="bg-base-content text-base-100">base-content</div>
      <div className="bg-primary">primary</div>
      <div className="bg-primary-focus">primary-focus</div>
      <div className="bg-primary-content text-base-100">primary-content</div>
      <div className="bg-secondary">secondary</div>
      <div className="bg-secondary-focus">secondary-focus</div>
      <div className="bg-secondary-content">secondary-content</div>
      <div className="bg-info">info</div>
      <div className="bg-error">error</div>
      <div className="bg-accent">accent</div>
    </div>
  );
};
