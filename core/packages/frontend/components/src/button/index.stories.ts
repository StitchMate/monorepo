import { Meta, StoryObj } from "@storybook/web-components";
import { withActions } from '@storybook/addon-actions/decorator';
import { html } from "lit";
import(".").catch((_err) => {
    //In case some our components are already defined
});

const meta: Meta = {
  title: "core/Button",
  component: "sm-button",
  parameters: {
    actions: {
        handles: ['click']
    }
  },
  decorators: [withActions],
  argTypes: {
    title: {
      control: {
        type: "text",
      },
    },
  },
};

export default meta;

type Story = StoryObj;

export const Primary: Story = {
  render: ({ title }) => {
    return html` <sm-button> ${title} </sm-button> `;
  },
  args: {
    title: "My Button",
  },
};
