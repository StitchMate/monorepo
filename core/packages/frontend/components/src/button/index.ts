import { LitElement, html, css } from 'lit';
import { customElement } from 'lit/decorators.js';

@customElement('sm-button')
class SMButton extends LitElement {
  static override styles = css`
    :host {}
    @unocss-placeholder
  `;

  override render() {
    return html`
      <button bg="red-500">
        <slot></slot>
      </button>
    `;
  }
}

declare global {
  interface HTMLElementTagNameMap {
    'sm-button': SMButton;
  }
}