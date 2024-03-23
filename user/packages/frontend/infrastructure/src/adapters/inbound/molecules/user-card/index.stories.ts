import { Meta, StoryObj } from "@storybook/web-components";
import { withActions } from "@storybook/addon-actions/decorator";
import { html } from "lit";
import "./index.component";

const meta: Meta = {
  title: "user/UserCard",
  component: "sm-usercard",
  parameters: {
    actions: {
      handles: [],
    },
  },
  decorators: [withActions],
  argTypes: {
    user: {
        control: {
            type: 'object'
        }
    }
  },
};

export default meta;

type Story = StoryObj;

export const UserPresent: Story = {
  render: ({ user }) => {
    return html` <sm-user-card .user=${user}></sm-user-card> `;
  },
  args: {
    user: {
        name: {
            first: "Test",
            last: "User"
        }
    }
  },
};
