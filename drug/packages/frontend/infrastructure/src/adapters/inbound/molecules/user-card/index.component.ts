/* @ts-ignore */
import("@stitchmate/core-components").catch((err) => {
  //In case core components is already registered in the broweser
});
import { LitElement, html, css } from 'lit';
import { customElement, property } from 'lit/decorators.js';

type User = {
    name: {
        first: String,
        last: String
    }
};

export class SMUserCard extends LitElement {
  static override styles = css`
    :host {}
    @unocss-placeholder
  `;

  @property({type: Object})
  user?: User;

  override render() {
    return html`
      <div>
        <p color="blue-500">${this.user?.name.first} ${this.user?.name.last}</p>
        <sm-button>View Profile</sm-button>
      </div>
    `;
  }
}