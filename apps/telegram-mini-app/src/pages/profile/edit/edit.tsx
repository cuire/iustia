import { Input, List } from "@telegram-apps/telegram-ui";

export const EditRoute: React.FC = () => {
  return (
    <List
      style={{
        width: 400,
        maxWidth: "100%",
        margin: "auto",
        background: "var(--tgui--secondary_bg_color)",
      }}
    >
      <Input
        header="Input"
        placeholder="I am usual input, just leave me alone"
      />
      <Input
        status="error"
        header="Input"
        placeholder="I am error input, don't make my mistakes..."
      />
      <Input
        status="focused"
        header="Input"
        placeholder="I am focused input, are u focused on me?"
      />
      <Input disabled header="Input" placeholder="I am disabled input" />
    </List>
  );
};
